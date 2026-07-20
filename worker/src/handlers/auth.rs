#![allow(dead_code)]

//! Magic-link authentication handlers.
//!
//! POST /api/auth/request  — request a magic-link login email   (Req 1.1–1.5, 19.3)
//! GET  /api/auth/callback — verify token, issue session cookie  (Req 1.6–1.10, 2.1, 19.1, 19.9)
//! POST /api/auth/logout   — clear session cookie                (Req 1.11)

use worker::js_sys;
use worker::*;

use crate::crypto::jwt::sign_jwt;
use crate::crypto::token::generate_secure_token;
use crate::email::send_magic_link;
use crate::middleware::rate_limit::rate_limit_check;
use crate::models::session::StaffClaims;
use crate::models::staff::Staff;
use crate::router::{error_to_response, AuthError, DbError, WorkerError};

// ---------------------------------------------------------------------------
// Corporate domain allowlist
// ---------------------------------------------------------------------------

const CORPORATE_DOMAINS: &[&str] = &["ezeroandone.com", "ezeroandone.io"];

/// Validate that the given email belongs to an allowed corporate domain.
///
/// Returns `Err(WorkerError::Forbidden)` if the domain is not on the allowlist.
/// The error message deliberately does **not** reveal which domains are allowed
/// (Req 1.2).
fn validate_corporate_email(email: &str) -> std::result::Result<(), WorkerError> {
    let domain = email
        .rsplit_once('@')
        .map(|(_, d)| d)
        .unwrap_or("");

    if CORPORATE_DOMAINS.contains(&domain) {
        Ok(())
    } else {
        Err(WorkerError::Forbidden)
    }
}

// ---------------------------------------------------------------------------
// Cookie helpers
// ---------------------------------------------------------------------------

/// Build the `Set-Cookie` header value for the session cookie.
///
/// The returned string always includes `HttpOnly`, `Secure`, and
/// `SameSite=Strict` — the three mandatory security attributes for the
/// session cookie (Requirements 2.5, 19.9, and Glossary: Session_Cookie).
///
/// Extracted as a standalone, target-agnostic function so it can be unit-
/// tested with `cargo test` on the native target (no WASM runtime needed).
pub fn build_session_cookie(jwt: &str) -> String {
    format!(
        "session={}; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=86400",
        jwt
    )
}

// ---------------------------------------------------------------------------
// POST /api/auth/request
// ---------------------------------------------------------------------------

/// Handle `POST /api/auth/request`.
///
/// Expects a JSON body `{ "email": "<corporate email>" }`.
///
/// Flow:
/// 1. Parse email from body.
/// 2. Validate against corporate domain allowlist (403 on failure, no domain leak).
/// 3. Rate-limit check (429 if exceeded).
/// 4. Generate a 64-char secure token.
/// 5. Store `ml:{token}` → `{ "email": email }` in `EZO_AUTH` KV with TTL 900 s.
/// 6. Send magic-link email.
/// 7. Return 200 `{ "message": "Magic link sent" }`.
///
/// Requirements: 1.1, 1.2, 1.3, 1.4, 1.5, 19.3
pub async fn request_magic_link(mut req: Request, env: &Env) -> Result<Response> {
    // ── 1. Parse body ────────────────────────────────────────────────────────
    let body: serde_json::Value = req
        .json()
        .await
        .map_err(|_| WorkerError::Validation(crate::router::ValidationError::InvalidInput(
            "Invalid JSON body".into(),
        )))?;

    let email = body
        .get("email")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_lowercase();

    if email.is_empty() || !email.contains('@') {
        return error_to_response(WorkerError::Auth(AuthError::InvalidEmail));
    }

    // ── 2. Corporate domain allowlist (Req 1.2) ──────────────────────────────
    if let Err(e) = validate_corporate_email(&email) {
        return error_to_response(e);
    }

    // ── 3. Rate-limit check (Req 1.3) ────────────────────────────────────────
    let kv = env
        .kv("EZO_AUTH")
        .map_err(|e| WorkerError::Internal(e.to_string()))?;

    // Best-effort IP extraction from CF-Connecting-IP header.
    let ip = req
        .headers()
        .get("CF-Connecting-IP")
        .unwrap_or(None)
        .unwrap_or_else(|| "unknown".to_string());

    rate_limit_check(&kv, &ip)
        .await
        .map_err(|e| e)?;

    // ── 4. Generate token (Req 1.4) ──────────────────────────────────────────
    let token = generate_secure_token();

    // ── 5. Store in KV with TTL 900 s (Req 1.4, 19.3) ───────────────────────
    let kv_value = serde_json::json!({ "email": email }).to_string();
    let kv_key = format!("ml:{}", token);

    kv.put(&kv_key, kv_value)
        .map_err(|e| WorkerError::Internal(e.to_string()))?
        .expiration_ttl(900)
        .execute()
        .await
        .map_err(|e| WorkerError::Internal(e.to_string()))?;

    // ── 6. Send magic-link email (Req 1.5) ───────────────────────────────────
    send_magic_link(&email, &token, env)
        .await
        .map_err(|e| e)?;

    // ── 7. Return 200 ────────────────────────────────────────────────────────
    Response::from_json(&serde_json::json!({ "message": "Magic link sent" }))
        .map(|r| r.with_status(200))
}

// ---------------------------------------------------------------------------
// GET /api/auth/callback
// ---------------------------------------------------------------------------

/// Handle `GET /api/auth/callback`.
///
/// Expects a `token` query-string parameter.
///
/// Flow:
/// 1. Extract `token` query param; 401 if absent.
/// 2. Fetch `ml:{token}` from KV; 401 if missing (expired or never issued).
/// 3. Immediately delete the KV entry (single-use token, Req 19.1).
/// 4. Parse `email` from the KV JSON value.
/// 5. Query D1 for existing staff; insert a new row if not found.
/// 6. Sign a `StaffClaims` JWT valid for 86 400 s.
/// 7. Set `session` HttpOnly cookie.
/// 8. Redirect to `/onboarding` or `/dashboard` depending on `onboarding_completed`.
///
/// Requirements: 1.6, 1.7, 1.8, 1.9, 1.10, 2.1, 19.1, 19.9
pub async fn callback(req: Request, env: &Env) -> Result<Response> {
    // ── 1. Extract token query param ─────────────────────────────────────────
    let url = req.url()?;
    let token = url
        .query_pairs()
        .find(|(k, _)| k == "token")
        .map(|(_, v)| v.to_string());

    let token = match token {
        Some(t) if !t.is_empty() => t,
        _ => return error_to_response(WorkerError::Auth(AuthError::Unauthorized)),
    };

    // ── 2. Fetch from KV ─────────────────────────────────────────────────────
    let kv = env
        .kv("EZO_AUTH")
        .map_err(|e| WorkerError::Internal(e.to_string()))?;

    let kv_key = format!("ml:{}", token);

    let raw = kv
        .get(&kv_key)
        .text()
        .await
        .map_err(|e| WorkerError::Internal(e.to_string()))?;

    let kv_value = match raw {
        Some(v) => v,
        None => return error_to_response(WorkerError::Auth(AuthError::Unauthorized)),
    };

    // ── 3. Delete KV entry immediately (single-use, Req 19.1) ───────────────
    kv.delete(&kv_key)
        .await
        .map_err(|e| WorkerError::Internal(e.to_string()))?;

    // ── 4. Parse email from KV value ─────────────────────────────────────────
    let parsed: serde_json::Value = serde_json::from_str(&kv_value)
        .map_err(|e| WorkerError::Internal(format!("KV parse error: {e}")))?;

    let email = parsed
        .get("email")
        .and_then(|v| v.as_str())
        .ok_or_else(|| WorkerError::Internal("KV value missing email field".into()))?
        .to_string();

    // ── 5. Look up or create staff row ───────────────────────────────────────
    let db = env
        .d1("DB")
        .map_err(|e| WorkerError::Internal(e.to_string()))?;

    let stmt = db.prepare(
        "SELECT id, email, username, name, job_title, bio, avatar_url, role, \
         onboarding_completed, created_at, updated_at \
         FROM staff WHERE email = ?1",
    );

    let staff: Option<Staff> = stmt
        .bind(&[email.clone().into()])
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?
        .first::<Staff>(None)
        .await
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?;

    let staff = match staff {
        Some(s) => s,
        None => {
            // Generate a unique username: local-part + 4-hex random suffix
            let local = email.split('@').next().unwrap_or("user");
            // Reuse generate_secure_token and take first 4 hex chars for the suffix
            let suffix = &generate_secure_token()[..4];
            let username = format!("{}{}", local, suffix);

            db.prepare("INSERT INTO staff (email, username) VALUES (?1, ?2)")
                .bind(&[email.clone().into(), username.into()])
                .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?
                .run()
                .await
                .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?;

            // Re-query to get the auto-generated id and defaults
            db.prepare(
                "SELECT id, email, username, name, job_title, bio, avatar_url, role, \
                 onboarding_completed, created_at, updated_at \
                 FROM staff WHERE email = ?1",
            )
            .bind(&[email.clone().into()])
            .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?
            .first::<Staff>(None)
            .await
            .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?
            .ok_or_else(|| WorkerError::Internal("Staff row missing after insert".into()))?
        }
    };

    // ── 6. Sign JWT (Req 2.1, 19.9) ──────────────────────────────────────────
    let now = (js_sys::Date::now() / 1000.0) as i64;

    let claims = StaffClaims {
        sub: staff.id.clone(),
        email: staff.email.clone(),
        role: staff.role,
        onboarded: staff.onboarding_completed,
        exp: now + 86_400,
    };

    let jwt_secret = env
        .secret("JWT_SECRET")
        .map_err(|e| WorkerError::Internal(format!("Missing JWT_SECRET: {e}")))?
        .to_string();

    let jwt = sign_jwt(&claims, jwt_secret.as_bytes())?;

    // ── 7. Build session cookie ───────────────────────────────────────────────
    let cookie = build_session_cookie(&jwt);

    // ── 8. Redirect based on onboarding status ───────────────────────────────
    let redirect_path = if staff.onboarding_completed {
        "/dashboard"
    } else {
        "/onboarding"
    };

    let mut headers = Headers::new();
    headers
        .set("Set-Cookie", &cookie)
        .map_err(|e| WorkerError::Internal(format!("Failed to set cookie header: {e}")))?;
    headers
        .set("Location", redirect_path)
        .map_err(|e| WorkerError::Internal(format!("Failed to set Location header: {e}")))?;

    Ok(Response::empty()
        .map_err(|e| WorkerError::Internal(e.to_string()))?
        .with_headers(headers)
        .with_status(302))
}

// ---------------------------------------------------------------------------
// POST /api/auth/logout
// ---------------------------------------------------------------------------

/// Handle `POST /api/auth/logout`.
///
/// Clears the `session` cookie by issuing a replacement with `Max-Age=0`.
/// Returns 200 `{ "message": "Logged out" }`.
///
/// Requirements: 1.11
pub async fn logout(_req: Request, _env: &Env) -> Result<Response> {
    let cookie = "session=; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=0";

    let mut headers = Headers::new();
    headers
        .set("Set-Cookie", cookie)
        .map_err(|e| WorkerError::Internal(format!("Failed to set cookie header: {e}")))?;
    headers
        .set("Content-Type", "application/json")
        .map_err(|e| WorkerError::Internal(format!("Failed to set Content-Type: {e}")))?;

    let body = serde_json::json!({ "message": "Logged out" }).to_string();

    Ok(
        Response::from_body(ResponseBody::Body(body.into_bytes()))
            .map_err(|e| WorkerError::Internal(e.to_string()))?
            .with_headers(headers)
            .with_status(200),
    )
}

// ---------------------------------------------------------------------------
// Unit tests — cookie security attributes (Requirements 2.5, 19.9)
// ---------------------------------------------------------------------------
//
// These tests run on the **native** host target via `cargo test`.
// They exercise `build_session_cookie`, which is pure-Rust and carries no
// WASM-only dependencies, so no wasm runner is required.
//
// Validates:
//   Requirement 2.5  — session cookie must be present on protected routes;
//                      this test confirms the attributes when the cookie IS set.
//   Requirement 19.9 — the session cookie must carry HttpOnly, Secure, and
//                      SameSite=Strict.

#[cfg(test)]
mod tests {
    use super::build_session_cookie;

    /// A realistic-looking JWT placeholder used across all cookie tests.
    const SAMPLE_JWT: &str =
        "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.\
         eyJzdWIiOiJ1c2VyMTIzIiwiZW1haWwiOiJ0ZXN0QGV6ZXJvYW5kb25lLmNvbSIsInJvbGUiOiJTdGFmZiIsIm9uYm9hcmRlZCI6dHJ1ZSwiZXhwIjoxNzAwMDAwMDAwfQ.\
         SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";

    // ── Core security-attribute assertions ───────────────────────────────────

    /// Requirement 19.9 — HttpOnly flag must be present.
    #[test]
    fn cookie_contains_httponly() {
        let cookie = build_session_cookie(SAMPLE_JWT);
        assert!(
            cookie.contains("HttpOnly"),
            "Set-Cookie header must contain 'HttpOnly'; got: {cookie}"
        );
    }

    /// Requirement 19.9 — Secure flag must be present.
    #[test]
    fn cookie_contains_secure() {
        let cookie = build_session_cookie(SAMPLE_JWT);
        assert!(
            cookie.contains("Secure"),
            "Set-Cookie header must contain 'Secure'; got: {cookie}"
        );
    }

    /// Requirement 19.9 — SameSite=Strict must be present.
    #[test]
    fn cookie_contains_samesite_strict() {
        let cookie = build_session_cookie(SAMPLE_JWT);
        assert!(
            cookie.contains("SameSite=Strict"),
            "Set-Cookie header must contain 'SameSite=Strict'; got: {cookie}"
        );
    }

    /// All three required attributes present in a single assertion (belt-and-braces).
    #[test]
    fn cookie_contains_all_three_security_attributes() {
        let cookie = build_session_cookie(SAMPLE_JWT);
        let missing: Vec<&str> = ["HttpOnly", "Secure", "SameSite=Strict"]
            .iter()
            .copied()
            .filter(|attr| !cookie.contains(attr))
            .collect();
        assert!(
            missing.is_empty(),
            "Set-Cookie header is missing security attributes {missing:?}; got: {cookie}"
        );
    }

    // ── Cookie name and JWT payload ──────────────────────────────────────────

    /// The cookie name must be exactly `session`.
    #[test]
    fn cookie_name_is_session() {
        let cookie = build_session_cookie(SAMPLE_JWT);
        assert!(
            cookie.starts_with("session="),
            "Set-Cookie header must start with 'session='; got: {cookie}"
        );
    }

    /// The JWT value must be embedded verbatim immediately after `session=`.
    #[test]
    fn cookie_embeds_jwt_verbatim() {
        let jwt = "header.payload.sig";
        let cookie = build_session_cookie(jwt);
        assert!(
            cookie.contains(&format!("session={jwt}")),
            "Set-Cookie header must embed the JWT verbatim; got: {cookie}"
        );
    }

    // ── Attribute completeness ───────────────────────────────────────────────

    /// Path=/ must be present so the cookie is sent on all routes.
    #[test]
    fn cookie_contains_path_root() {
        let cookie = build_session_cookie(SAMPLE_JWT);
        assert!(
            cookie.contains("Path=/"),
            "Set-Cookie header must contain 'Path=/'; got: {cookie}"
        );
    }

    /// Max-Age must be set (non-zero, indicating a session that expires).
    #[test]
    fn cookie_contains_max_age() {
        let cookie = build_session_cookie(SAMPLE_JWT);
        assert!(
            cookie.contains("Max-Age="),
            "Set-Cookie header must contain 'Max-Age='; got: {cookie}"
        );
    }
}
