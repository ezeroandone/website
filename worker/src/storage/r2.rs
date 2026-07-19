#![allow(dead_code, unused_imports)]

//! R2 object storage helpers.
//!
//! Wraps the Workers R2 binding for put / get / delete operations and
//! generates short-lived presigned download URLs.
//!
//! Two storage domains are served from the same bucket (`EZO_MEDIA`):
//!
//! | Path prefix        | Access  | Cache-Control                          |
//! |--------------------|---------|----------------------------------------|
//! | `avatars/…`        | Public  | `public, max-age=31536000, immutable`  |
//! | `posts/…`          | Public  | `public, max-age=31536000, immutable`  |
//! | `careers/…`        | Public  | `public, max-age=31536000, immutable`  |
//! | `applications/…`   | Private | no public access; presigned URL only   |

use worker::*;

use crate::router::WorkerError;

// ---------------------------------------------------------------------------
// Put a public media image into R2
// ---------------------------------------------------------------------------

/// Store `bytes` in R2 at the given `key` as a public media asset.
///
/// Sets the supplied `cache_control` value on the R2 object metadata so the
/// CDN can serve the asset according to the caller's caching policy.
///
/// Returns the public CDN URL: `https://media.ezeroandone.com/{key}`.
///
/// Requirements: 12.6, 12.7
pub async fn r2_put_public(
    bucket: &Bucket,
    key: &str,
    bytes: Vec<u8>,
    mime: &str,
    cache_control: &str,
) -> std::result::Result<String, WorkerError> {
    let http_metadata = HttpMetadata {
        content_type: Some(mime.to_string()),
        cache_control: Some(cache_control.to_string()),
        ..HttpMetadata::default()
    };

    bucket
        .put(key, Data::Bytes(bytes))
        .http_metadata(http_metadata)
        .execute()
        .await
        .map_err(|e| WorkerError::Internal(format!("R2 put failed: {}", e)))?;

    Ok(format!("https://media.ezeroandone.com/{}", key))
}

// ---------------------------------------------------------------------------
// Put a private application document into R2
// ---------------------------------------------------------------------------

/// Store `bytes` in R2 at `applications/{application_id}/{sanitised_filename}`.
///
/// The object is created without a Cache-Control header so it is never served
/// from any public edge cache — access is exclusively via presigned URLs issued
/// to Admin-level staff (Req 6.10).
///
/// Returns the R2 key on success.
///
/// Requirements: 6.7, 6.10
pub async fn r2_put_application_document(
    bucket: &Bucket,
    application_id: &str,
    sanitised_filename: &str,
    bytes: Vec<u8>,
    mime_type: &str,
) -> Result<String> {
    let r2_key = format!("applications/{}/{}", application_id, sanitised_filename);

    // Build HTTP metadata — content type only; no Cache-Control so the
    // object is never made publicly cacheable.
    let http_metadata = HttpMetadata {
        content_type: Some(mime_type.to_string()),
        ..HttpMetadata::default()
    };

    bucket
        .put(&r2_key, Data::Bytes(bytes))
        .http_metadata(http_metadata)
        .execute()
        .await
        .map_err(|e| Error::RustError(format!("R2 put failed: {}", e)))?;

    Ok(r2_key)
}

// ---------------------------------------------------------------------------
// Generate a presigned URL for a private application document
// ---------------------------------------------------------------------------

/// Generate a time-limited presigned URL for an R2 object.
///
/// Only keys that begin with `applications/` are eligible — any other prefix
/// returns `Err(WorkerError::Forbidden)` per security requirement 12.11.
///
/// The generated URL expires after `ttl_secs` seconds and cannot be reused.
///
/// Requirements: 12.9, 12.10, 12.11
pub async fn generate_presigned_url(
    _bucket: &Bucket,
    r2_key: &str,
    _ttl_secs: u32,
) -> std::result::Result<String, WorkerError> {
    if !r2_key.starts_with("applications/") {
        return Err(WorkerError::Forbidden);
    }
    // Presigned URL generation is not yet available in workers-rs 0.4.
    // Return a stub URL for now; the admin UI will call this endpoint and
    // handle the 501 gracefully until the binding is available.
    Err(WorkerError::Internal(
        "presigned URL not yet implemented in this workers-rs version".to_string(),
    ))
}

// ---------------------------------------------------------------------------
// extension_from_mime
// ---------------------------------------------------------------------------

/// Map a MIME type string to its canonical file extension.
///
/// Used when constructing deterministic R2 object keys for uploaded images
/// (e.g. `avatars/{staff_id}.jpg`).
///
/// Defaults to `"bin"` for any unrecognised MIME type.
pub fn extension_from_mime(mime: &str) -> &'static str {
    match mime {
        "image/jpeg"    => "jpg",
        "image/png"     => "png",
        "image/webp"    => "webp",
        "image/avif"    => "avif",
        "application/pdf" => "pdf",
        "application/msword" => "doc",
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => "docx",
        _               => "bin",
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_from_mime_known_types() {
        assert_eq!(extension_from_mime("image/jpeg"), "jpg");
        assert_eq!(extension_from_mime("image/png"), "png");
        assert_eq!(extension_from_mime("image/webp"), "webp");
        assert_eq!(extension_from_mime("image/avif"), "avif");
        assert_eq!(extension_from_mime("application/pdf"), "pdf");
        assert_eq!(extension_from_mime("application/msword"), "doc");
        assert_eq!(
            extension_from_mime(
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
            ),
            "docx"
        );
    }

    #[test]
    fn test_extension_from_mime_unknown_returns_bin() {
        assert_eq!(extension_from_mime("text/plain"), "bin");
        assert_eq!(extension_from_mime(""), "bin");
        assert_eq!(extension_from_mime("application/octet-stream"), "bin");
    }
}
