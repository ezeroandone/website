#![allow(dead_code, unused_imports)]

//! Staff signing-key provisioning.
//! Generates, stores, and rotates per-staff ECDSA signing keys used for
//! QR identity verification tokens.

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use worker::D1Database;
use worker::wasm_bindgen::JsValue;

use crate::router::{DbError, ValidationError, WorkerError};

/// Validate that `pem` is a structurally correct PEM-encoded public key.
///
/// Accepted headers (and their matching footers):
/// - `-----BEGIN PUBLIC KEY-----` / `-----END PUBLIC KEY-----`  (PKCS#8 SubjectPublicKeyInfo)
/// - `-----BEGIN EC PUBLIC KEY-----` / `-----END EC PUBLIC KEY-----`
///
/// Full ASN.1 / DER parsing is intentionally **not** performed; the function
/// only checks that the PEM envelope is well-formed and the base64 body is
/// non-empty and decodable.
pub fn is_valid_public_key_pem(pem: &str) -> bool {
    let pem = pem.trim();

    let valid_pairs = [
        (
            "-----BEGIN PUBLIC KEY-----",
            "-----END PUBLIC KEY-----",
        ),
        (
            "-----BEGIN EC PUBLIC KEY-----",
            "-----END EC PUBLIC KEY-----",
        ),
    ];

    for (header, footer) in &valid_pairs {
        if pem.starts_with(header) && pem.ends_with(footer) {
            // Extract everything between the header and footer lines.
            let body = pem
                .strip_prefix(header)
                .unwrap()
                .strip_suffix(footer)
                .unwrap()
                // Strip whitespace / newlines that PEM wraps the body with.
                .replace('\n', "")
                .replace('\r', "");
            let body = body.trim();

            if body.is_empty() {
                return false;
            }

            // The body must be valid standard-alphabet base64.
            return STANDARD.decode(body).is_ok();
        }
    }

    false
}

/// Store (or replace) the PEM-encoded public signing key for `staff_id` in D1.
///
/// Errors:
/// - `WorkerError::Validation(ValidationError::InvalidKeyFormat)` — PEM validation failed
/// - `WorkerError::Db(DbError::NotFound)` — no staff row matched `staff_id`
/// - `WorkerError::Db(DbError::Query(_))` — D1 execution error
pub async fn provision_signing_key(
    staff_id: &str,
    public_key_pem: &str,
    db: &D1Database,
) -> Result<(), WorkerError> {
    if !is_valid_public_key_pem(public_key_pem) {
        return Err(WorkerError::Validation(ValidationError::InvalidKeyFormat));
    }

    let result = db
        .prepare(
            "UPDATE staff SET signing_public_key = ?1, updated_at = unixepoch() WHERE id = ?2",
        )
        .bind(&[
            JsValue::from_str(public_key_pem),
            JsValue::from_str(staff_id),
        ])
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?
        .run()
        .await
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?;

    // D1 result meta contains rows_written (number of rows modified).
    let rows_written = result
        .meta()
        .ok()
        .flatten()
        .and_then(|m| m.rows_written)
        .unwrap_or(0);

    if rows_written == 0 {
        return Err(WorkerError::Db(DbError::NotFound));
    }

    Ok(())
}
