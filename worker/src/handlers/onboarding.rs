#![allow(dead_code, unused_imports)]

//! Onboarding wizard handlers for new staff members.
//!
//! GET   /api/me                      — return the authenticated user's own staff profile
//! GET   /api/onboarding/status      — return current onboarding step and completion state
//! PATCH /api/onboarding/profile     — update staff profile fields (name, job_title, bio, avatar_url)
//! POST  /api/onboarding/signing-key — provision staff signing public key
//! POST  /api/onboarding/complete    — mark onboarding as completed

use worker::*;

use crate::crypto::signing_key::provision_signing_key;
use crate::middleware::auth::SessionContext;
use crate::router::{DbError, ValidationError, WorkerError, error_to_response};

// ---------------------------------------------------------------------------
// GET /api/me
// ---------------------------------------------------------------------------

/// Return the authenticated staff member's own profile record.
///
/// This is used by the admin profile settings page so users can view and
/// update their own information after onboarding is complete.
///
/// Requirements: 4.4 (profile management)
pub async fn get_me(
    _req: &Request,
    env: &Env,
    ctx: SessionContext,
) -> Result<Response> {
    let db = env.d1("DB").map_err(|e| {
        WorkerError::Db(DbError::Query(e.to_string()))
    })?;

    let row = db
        .prepare(
            "SELECT id, email, username, name, job_title, bio, avatar_url, role \
             FROM staff WHERE id = ?1",
        )
        .bind(&[ctx.staff_id.clone().into()])?
        .first::<serde_json::Value>(None)
        .await
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?
        .ok_or(WorkerError::NotFound)?;

    Response::from_json(&row)
        .map(|r| r.with_status(200))
        .map_err(|e| WorkerError::Internal(e.to_string()).into())
}


/// authenticated staff member.
///
/// Step derivation:
/// - `onboarding_completed = true`           → step 3, completed = true
/// - `signing_public_key IS NOT NULL`         → step 3, completed = false  (awaiting ack)
/// - `name != ''`                             → step 2, completed = false  (needs signing key)
/// - otherwise                               → step 1, completed = false  (needs profile)
///
/// Requirements: 4.10
pub async fn get_status(
    _req: &Request,
    env: &Env,
    ctx: SessionContext,
) -> Result<Response> {
    let db = env.d1("DB").map_err(|e| {
        WorkerError::Db(DbError::Query(e.to_string()))
    })?;

    // Query only the columns needed for step derivation
    let row = db
        .prepare(
            "SELECT id, signing_public_key, onboarding_completed, name \
             FROM staff WHERE id = ?1",
        )
        .bind(&[ctx.staff_id.clone().into()])?
        .first::<serde_json::Value>(None)
        .await
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?
        .ok_or(WorkerError::NotFound)?;

    // Extract fields from the JSON row returned by D1
    let onboarding_completed = row
        .get("onboarding_completed")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let signing_public_key = row
        .get("signing_public_key")
        .and_then(|v| if v.is_null() { None } else { v.as_str().map(|s| s.to_string()) });

    let name = row
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    // Derive the current step
    let (step, completed) = if onboarding_completed {
        (3u8, true)
    } else if signing_public_key.is_some() {
        (3u8, false)
    } else if !name.is_empty() {
        (2u8, false)
    } else {
        (1u8, false)
    };

    Response::from_json(&serde_json::json!({
        "step": step,
        "completed": completed,
    }))
    .map(|r| r.with_status(200))
    .map_err(|e| {
        error_to_response(WorkerError::Internal(e.to_string())).unwrap_err()
    })
}

// ---------------------------------------------------------------------------
// PATCH /api/onboarding/profile
// ---------------------------------------------------------------------------

/// Update the staff profile fields supplied in the request body.
///
/// Accepts a JSON body with any combination of:
/// `{ "name": "...", "job_title": "...", "bio": "...", "avatar_url": "..." }`
///
/// All fields are optional; absent fields default to an empty string.
///
/// Requirements: 4.4
pub async fn patch_profile(
    mut req: Request,
    env: &Env,
    ctx: SessionContext,
) -> Result<Response> {
    // Parse the JSON body; tolerate an empty / missing body by falling back to
    // an empty object so that all fields simply default to empty strings.
    let body: serde_json::Value = req
        .json()
        .await
        .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));

    let name      = body.get("name")      .and_then(|v| v.as_str()).unwrap_or("").to_string();
    let job_title = body.get("job_title") .and_then(|v| v.as_str()).unwrap_or("").to_string();
    let bio       = body.get("bio")       .and_then(|v| v.as_str()).unwrap_or("").to_string();
    let avatar_url= body.get("avatar_url").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let db = env.d1("DB").map_err(|e| {
        WorkerError::Db(DbError::Query(e.to_string()))
    })?;

    db.prepare(
        "UPDATE staff \
         SET name=?1, job_title=?2, bio=?3, avatar_url=?4, updated_at=unixepoch() \
         WHERE id=?5",
    )
    .bind(&[
        name.into(),
        job_title.into(),
        bio.into(),
        avatar_url.into(),
        ctx.staff_id.clone().into(),
    ])?
    .run()
    .await
    .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?;

    Response::from_json(&serde_json::json!({ "message": "Profile updated" }))
        .map(|r| r.with_status(200))
        .map_err(|e| {
            error_to_response(WorkerError::Internal(e.to_string())).unwrap_err()
        })
}

// ---------------------------------------------------------------------------
// POST /api/onboarding/signing-key
// ---------------------------------------------------------------------------

/// Validate and store the staff member's public signing key.
///
/// Accepts a JSON body: `{ "public_key_pem": "-----BEGIN PUBLIC KEY-----..." }`
///
/// - HTTP 400 with a descriptive error on invalid PEM format
///   (`ValidationError::InvalidKeyFormat`).
/// - HTTP 200 on success.
///
/// Requirements: 4.5, 4.6, 4.7
pub async fn post_signing_key(
    mut req: Request,
    env: &Env,
    ctx: SessionContext,
) -> Result<Response> {
    // Parse the JSON body
    let body: serde_json::Value = req
        .json()
        .await
        .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));

    let public_key_pem = body
        .get("public_key_pem")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let db = env.d1("DB").map_err(|e| {
        WorkerError::Db(DbError::Query(e.to_string()))
    })?;

    match provision_signing_key(&ctx.staff_id, &public_key_pem, &db).await {
        Ok(()) => {
            // HTTP 200 — key provisioned successfully
            Response::from_json(&serde_json::json!({ "message": "Signing key provisioned" }))
                .map(|r| r.with_status(200))
                .map_err(|e| {
                    error_to_response(WorkerError::Internal(e.to_string())).unwrap_err()
                })
        }
        Err(WorkerError::Validation(ValidationError::InvalidKeyFormat)) => {
            // HTTP 400 with a descriptive message — Requirement 4.6
            Response::from_json(&serde_json::json!({
                "error": "Invalid public key format: expected a PEM-encoded Ed25519 or ECDSA P-256 public key"
            }))
            .map(|r| r.with_status(400))
            .map_err(|e| {
                error_to_response(WorkerError::Internal(e.to_string())).unwrap_err()
            })
        }
        Err(e) => error_to_response(e),
    }
}

// ---------------------------------------------------------------------------
// POST /api/onboarding/complete
// ---------------------------------------------------------------------------

/// Mark the staff member's onboarding as completed.
///
/// Sets `onboarding_completed = 1` on the staff row and returns HTTP 200
/// with `{ "message": "Onboarding complete" }`.
///
/// Requirements: 4.8, 4.9
pub async fn post_complete(
    _req: &Request,
    env: &Env,
    ctx: SessionContext,
) -> Result<Response> {
    let db = env.d1("DB").map_err(|e| {
        WorkerError::Db(DbError::Query(e.to_string()))
    })?;

    db.prepare(
        "UPDATE staff \
         SET onboarding_completed = 1, updated_at = unixepoch() \
         WHERE id = ?1",
    )
    .bind(&[ctx.staff_id.clone().into()])?
    .run()
    .await
    .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?;

    Response::from_json(&serde_json::json!({ "message": "Onboarding complete" }))
        .map(|r| r.with_status(200))
        .map_err(|e| {
            error_to_response(WorkerError::Internal(e.to_string())).unwrap_err()
        })
}
