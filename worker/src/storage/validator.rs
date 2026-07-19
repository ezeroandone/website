#![allow(dead_code, unused_imports)]

//! Upload validation.
//!
//! Provides:
//! - `detect_mime_from_magic_bytes` — inspect raw bytes to determine MIME type
//!   (ignores the client-supplied Content-Type header per Req 6.3 / 12.2)
//! - `sanitise_filename` — strip path-traversal sequences from user-supplied
//!   filenames before they reach R2 (Req 6.9 / 19.6)

use worker::*;

// ---------------------------------------------------------------------------
// Allowed MIME types
// ---------------------------------------------------------------------------

/// MIME types accepted for **application documents** (CV, cover letter, portfolio).
///
/// Requirements: 6.3
pub const ALLOWED_DOC_MIMES: &[&str] = &[
    "application/pdf",
    "application/msword",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
];

/// MIME types accepted for **public media images** (avatar, post cover, etc.).
///
/// Requirements: 12.2
pub const ALLOWED_IMAGE_MIMES: &[&str] = &[
    "image/jpeg",
    "image/png",
    "image/webp",
    "image/avif",
];

// ---------------------------------------------------------------------------
// Size limits
// ---------------------------------------------------------------------------

/// Maximum size for application documents: 10 MB.
///
/// Requirements: 6.5
pub const MAX_DOC_SIZE: usize = 10_485_760;

/// Maximum size for public media images: 4 MB.
///
/// Requirements: 12.4
pub const MAX_IMAGE_SIZE: usize = 4_194_304;

// ---------------------------------------------------------------------------
// Maximum documents per application
// ---------------------------------------------------------------------------

/// Maximum number of documents an application may attach.
///
/// Requirements: 6.8
pub const MAX_DOCS_PER_APPLICATION: i64 = 3;

// ---------------------------------------------------------------------------
// Magic-byte MIME detection
// ---------------------------------------------------------------------------

/// Inspect the first bytes of a file to determine its actual MIME type.
///
/// Returns a static MIME-type string when a known signature is found, or
/// `None` when the bytes do not match any recognised format.
///
/// Signatures checked (in order):
/// - PDF          → `%PDF`  (25 50 44 46)
/// - DOCX (ZIP)   → `PK\x03\x04` (50 4B 03 04)
/// - DOC (OLE2)   → `\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1`
/// - JPEG         → `\xFF\xD8\xFF`
/// - PNG          → `\x89PNG\r\n\x1A\n`
/// - WebP         → `RIFF????WEBP`
/// - AVIF         → `????ftyp` with `avif` / `avis` brand
///
/// Requirements: 6.3, 12.2
pub fn detect_mime_from_magic_bytes(bytes: &[u8]) -> Option<&'static str> {
    // Need at least 12 bytes for WebP / AVIF checks.
    if bytes.is_empty() {
        return None;
    }

    // PDF: %PDF
    if bytes.starts_with(b"%PDF") {
        return Some("application/pdf");
    }

    // DOCX / any ZIP-based Office Open XML: PK\x03\x04
    if bytes.starts_with(&[0x50, 0x4B, 0x03, 0x04]) {
        // Treat all PK-ZIP-origin files as DOCX for the document upload
        // endpoint; the allow-list check in the handler gates the final call.
        return Some(
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        );
    }

    // DOC: OLE2 compound document (legacy Word .doc)
    if bytes.starts_with(&[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1]) {
        return Some("application/msword");
    }

    // JPEG: \xFF\xD8\xFF
    if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return Some("image/jpeg");
    }

    // PNG: \x89PNG\r\n\x1A\n
    if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
        return Some("image/png");
    }

    // WebP: RIFF????WEBP (bytes 0-3 = RIFF, bytes 8-11 = WEBP)
    if bytes.len() >= 12
        && &bytes[0..4] == b"RIFF"
        && &bytes[8..12] == b"WEBP"
    {
        return Some("image/webp");
    }

    // AVIF: ????ftypavif or ????ftypavis (bytes 4-11)
    if bytes.len() >= 12
        && &bytes[4..8] == b"ftyp"
        && (bytes[8..12].eq_ignore_ascii_case(b"avif")
            || bytes[8..12].eq_ignore_ascii_case(b"avis"))
    {
        return Some("image/avif");
    }

    None
}

// ---------------------------------------------------------------------------
// Filename sanitisation
// ---------------------------------------------------------------------------

/// Strip path-traversal sequences from a caller-supplied filename.
///
/// Removes:
/// - All `/` characters (forward-slash)
/// - All `\` characters (backslash, for defence in depth)
/// - All `..` sequences (after the above substitutions are applied)
///
/// The result is a flat filename with no directory component, safe to use
/// as a suffix in an R2 object key.
///
/// If the result after stripping is empty, returns `"unnamed"`.
///
/// Requirements: 6.9, 19.6
pub fn sanitise_filename(name: &str) -> String {
    // 1. Strip forward and back slashes entirely.
    let no_slashes: String = name.chars().filter(|&c| c != '/' && c != '\\').collect();

    // 2. Remove all remaining `..` sequences.
    let no_traversal = no_slashes.replace("..", "");

    // 3. Trim leading/trailing whitespace and dots (defence in depth).
    let trimmed = no_traversal.trim_matches(|c: char| c == '.' || c.is_whitespace());

    // 4. If the result is empty, fall back to a safe placeholder.
    if trimmed.is_empty() {
        "unnamed".to_string()
    } else {
        trimmed.to_string()
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- detect_mime_from_magic_bytes ---

    #[test]
    fn test_pdf_magic() {
        let bytes = b"%PDF-1.4 rest of file";
        assert_eq!(detect_mime_from_magic_bytes(bytes), Some("application/pdf"));
    }

    #[test]
    fn test_docx_magic() {
        // PK\x03\x04 ZIP header
        let bytes = &[0x50, 0x4B, 0x03, 0x04, 0x14, 0x00, 0x00, 0x00];
        assert_eq!(
            detect_mime_from_magic_bytes(bytes),
            Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document")
        );
    }

    #[test]
    fn test_doc_magic() {
        let bytes = &[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1, 0x00];
        assert_eq!(
            detect_mime_from_magic_bytes(bytes),
            Some("application/msword")
        );
    }

    #[test]
    fn test_jpeg_magic() {
        let bytes = &[0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10];
        assert_eq!(detect_mime_from_magic_bytes(bytes), Some("image/jpeg"));
    }

    #[test]
    fn test_png_magic() {
        let bytes = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00];
        assert_eq!(detect_mime_from_magic_bytes(bytes), Some("image/png"));
    }

    #[test]
    fn test_webp_magic() {
        let bytes = b"RIFF\x00\x00\x00\x00WEBP";
        assert_eq!(detect_mime_from_magic_bytes(bytes), Some("image/webp"));
    }

    #[test]
    fn test_avif_magic() {
        let mut bytes = [0u8; 12];
        bytes[4..8].copy_from_slice(b"ftyp");
        bytes[8..12].copy_from_slice(b"avif");
        assert_eq!(detect_mime_from_magic_bytes(&bytes), Some("image/avif"));
    }

    #[test]
    fn test_unknown_returns_none() {
        let bytes = b"\x00\x01\x02\x03\x04";
        assert_eq!(detect_mime_from_magic_bytes(bytes), None);
    }

    #[test]
    fn test_empty_returns_none() {
        assert_eq!(detect_mime_from_magic_bytes(b""), None);
    }

    // --- sanitise_filename ---

    #[test]
    fn test_sanitise_strips_forward_slash() {
        assert!(!sanitise_filename("../../etc/passwd").contains('/'));
    }

    #[test]
    fn test_sanitise_strips_dotdot() {
        let result = sanitise_filename("../../etc/passwd");
        assert!(!result.contains(".."));
    }

    #[test]
    fn test_sanitise_normal_filename_unchanged() {
        let result = sanitise_filename("my_resume.pdf");
        assert_eq!(result, "my_resume.pdf");
    }

    #[test]
    fn test_sanitise_empty_falls_back() {
        assert_eq!(sanitise_filename(""), "unnamed");
        assert_eq!(sanitise_filename(".."), "unnamed");
        assert_eq!(sanitise_filename("/"), "unnamed");
    }

    #[test]
    fn test_sanitise_backslash_stripped() {
        let result = sanitise_filename("..\\Windows\\system32");
        assert!(!result.contains('\\'));
        assert!(!result.contains(".."));
    }
}
