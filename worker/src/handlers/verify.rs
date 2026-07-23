#![allow(dead_code, unused_imports)]

//! QR-code identity verification handler.
//!
//! GET /api/verify?token=<jwt>
//!
//! Validates a short-lived QR identity JWT and returns a sanitised identity
//! response. All failure modes return the same generic HTTP 401 to avoid
//! leaking implementation details (Requirements 9.6, 19.4).

use worker::*;

use crate::crypto::jwt::verify_identity_jwt;
use crate::models::staff::{LifecycleStatus, Role};
use crate::router::{DbError, WorkerError, error_to_response};

// ---------------------------------------------------------------------------
// Internal response struct (never exposes raw JWT claims)
// ---------------------------------------------------------------------------

/// The JSON body returned on a successful `GET /api/verify` call.
///
/// Requirements: 9.4, 9.7 — no `sub`, `exp`, `iss`, `email`, `id`, or
/// `signing_public_key` fields are included.
#[derive(serde::Serialize)]
struct IdentityResponse {
    /// Staff display name.
    name: String,
    /// Public CDN URL of the staff avatar.
    photo_url: String,
    /// Lifecycle status as a string (e.g. `"Confirmed"`).
    identity_status: String,
    /// Clearance level 1–4 derived from role × lifecycle status.
    clearance_level: u8,
    /// Current Unix timestamp (seconds) at the moment of verification.
    verified_at: i64,
}

// ---------------------------------------------------------------------------
// Clearance level mapping
// ---------------------------------------------------------------------------

/// Derive a numeric clearance level from a staff member's role and lifecycle
/// status.
///
/// Mapping per Requirement 9.5:
/// - non-Confirmed (Probation | Inactive) → 1
/// - Confirmed + Staff                    → 2
/// - Confirmed + Admin                    → 3
/// - Confirmed + SuperAdmin               → 4
pub fn map_role_to_clearance(role: &Role, lifecycle_status: &LifecycleStatus) -> u8 {
    match lifecycle_status {
        LifecycleStatus::Confirmed => match role {
            Role::Staff     => 2,
            Role::Admin     => 3,
            Role::SuperAdmin => 4,
            // Public role should never appear on a real staff row, but
            // treating it as non-confirmed is the safe default.
            Role::Public    => 1,
        },
        // Probation or Inactive → clearance 1 regardless of role
        _ => 1,
    }
}

// ---------------------------------------------------------------------------
// Generic 401 helper — used for every failure path
// ---------------------------------------------------------------------------

/// Return `HTTP 401 { "error": "Unauthorized" }`.
///
/// All verify failure modes share this response so callers cannot distinguish
/// expired tokens from invalid signatures, missing tokens, or unknown staff.
/// (Requirements 9.6, 19.4)
fn unauthorized() -> Result<Response> {
    Response::from_json(&serde_json::json!({ "error": "Unauthorized" }))
        .map(|r| r.with_status(401))
}

// ---------------------------------------------------------------------------
// Returns the current Unix time in seconds via the JS `Date` API.
// ---------------------------------------------------------------------------

fn now_unix_seconds() -> i64 {
    use worker::js_sys;
    (js_sys::Date::now() / 1000.0) as i64
}

// ---------------------------------------------------------------------------
// GET /api/verify?token=<jwt>
// ---------------------------------------------------------------------------

/// Validate a QR identity JWT and return a sanitised identity response.
///
/// Algorithm:
/// 1. Extract `token` query param — absent → 401.
/// 2. `verify_identity_jwt(token, QR_SECRET)` — any error → 401.
/// 3. Query `staff` by `claims.sub` — not found → 401.
/// 4. Query `staff_lifecycle` by `staff_id` — not found → 401.
/// 5. Compute `clearance_level` via `map_role_to_clearance`.
/// 6. Return 200 `IdentityResponse`.
///
/// Security: never includes `sub`, `exp`, `iss`, `email`, `id`, or
/// `signing_public_key` in the response body.
///
/// Requirements: 9.4, 9.5, 9.6, 9.7, 19.4
pub async fn handle_verify(req: &Request, env: &Env) -> Result<Response> {
    // ── 1. Extract `token` query param ──────────────────────────────────────
    let url = req.url().map_err(|e| {
        // URL parse failure is internal — still surface as 401 per requirement
        console_error!("verify: URL parse error: {e}");
        worker::Error::RustError("url_parse".to_string())
    })?;

    let token = url
        .query_pairs()
        .find(|(k, _)| k == "token")
        .map(|(_, v)| v.to_string());

    let token = match token {
        Some(t) if !t.is_empty() => t,
        _ => return unauthorized(),
    };

    // ── 2. Verify the QR identity JWT using QR_SECRET ───────────────────────
    let qr_secret_str = match env.secret("QR_SECRET") {
        Ok(s) => s.to_string(),
        Err(e) => {
            console_error!("verify: QR_SECRET binding missing: {e}");
            return unauthorized();
        }
    };

    let claims = match verify_identity_jwt(&token, qr_secret_str.as_bytes()) {
        Ok(c) => c,
        Err(_) => return unauthorized(),
    };

    let staff_id = claims.sub;

    // ── 3. Query staff row ───────────────────────────────────────────────────
    let db = match env.d1("DB") {
        Ok(d) => d,
        Err(e) => {
            console_error!("verify: D1 binding error: {e}");
            return unauthorized();
        }
    };

    let staff_row = match db
        .prepare("SELECT name, avatar_url, role FROM staff WHERE id = ?1")
        .bind(&[staff_id.clone().into()])
        .map_err(|e| {
            console_error!("verify: bind error: {e}");
        })
        .ok()
    {
        Some(stmt) => stmt
            .first::<serde_json::Value>(None)
            .await
            .unwrap_or(None),
        None => return unauthorized(),
    };

    let staff_row = match staff_row {
        Some(row) => row,
        None => return unauthorized(),
    };

    // ── 4. Query staff_lifecycle row ─────────────────────────────────────────
    let lifecycle_row = match db
        .prepare("SELECT status FROM staff_lifecycle WHERE staff_id = ?1")
        .bind(&[staff_id.into()])
        .map_err(|e| {
            console_error!("verify: lifecycle bind error: {e}");
        })
        .ok()
    {
        Some(stmt) => stmt
            .first::<serde_json::Value>(None)
            .await
            .unwrap_or(None),
        None => return unauthorized(),
    };

    let lifecycle_row = match lifecycle_row {
        Some(row) => row,
        None => return unauthorized(),
    };

    // ── Parse role ───────────────────────────────────────────────────────────
    let role_str = staff_row
        .get("role")
        .and_then(|v| v.as_str())
        .unwrap_or("Staff");

    let role = match role_str {
        "SuperAdmin" => Role::SuperAdmin,
        "Admin"      => Role::Admin,
        "Staff"      => Role::Staff,
        _            => Role::Staff,
    };

    // ── Parse lifecycle status ───────────────────────────────────────────────
    let status_str = lifecycle_row
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("Probation");

    let lifecycle_status = match status_str {
        "Confirmed" => LifecycleStatus::Confirmed,
        "Inactive"  => LifecycleStatus::Inactive,
        _           => LifecycleStatus::Probation,
    };

    // ── 5. Compute clearance level ───────────────────────────────────────────
    let clearance_level = map_role_to_clearance(&role, &lifecycle_status);

    // ── 6. Build and return the sanitised response ───────────────────────────
    let name = staff_row
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let photo_url = staff_row
        .get("avatar_url")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let response = IdentityResponse {
        name,
        photo_url,
        identity_status: status_str.to_string(),
        clearance_level,
        verified_at: now_unix_seconds(),
    };

    Response::from_json(&response)
        .map(|r| r.with_status(200))
        .map_err(|e| {
            console_error!("verify: serialise response error: {e}");
            e
        })
}
