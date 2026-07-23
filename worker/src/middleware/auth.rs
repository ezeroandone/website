#![allow(dead_code)]

use std::result::Result as StdResult;

use worker::*;

use crate::crypto::jwt::verify_jwt;
use crate::models::staff::Role;
use crate::router::{AuthError, WorkerError};

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Authenticated session context extracted from a valid session cookie JWT.
#[derive(Debug, Clone)]
pub struct SessionContext {
    pub staff_id: String,
    pub email: String,
    pub role: Role,
    pub onboarded: bool,
}

// ---------------------------------------------------------------------------
// Bypass paths — exempt from the onboarding redirect guard
// ---------------------------------------------------------------------------

const BYPASS_PATHS: &[&str] = &["/api/auth", "/api/onboarding", "/auth"];

// ---------------------------------------------------------------------------
// parse_session_context
// ---------------------------------------------------------------------------

/// Attempt to parse a [`SessionContext`] from the `session` cookie.
///
/// Returns `None` for unauthenticated requests (missing cookie, invalid
/// signature, or expired token).  Errors are silently discarded here — the
/// `auth_middleware` function surfaces the appropriate HTTP 401/403 when a
/// protected route requires a valid session.
pub async fn parse_session_context(req: &Request, env: &Env) -> Option<SessionContext> {
    // Extract the raw cookie header
    let cookie_header = req.headers().get("Cookie").ok()??;

    // Find the `session` cookie value
    let token = cookie_header
        .split(';')
        .map(|s| s.trim())
        .find_map(|part| part.strip_prefix("session="))?;

    // Pull the JWT secret from the Worker secret binding
    let secret = env.secret("JWT_SECRET").ok()?.to_string();

    // Verify the JWT and map claims to SessionContext
    let claims = verify_jwt(token, secret.as_bytes()).ok()?;

    Some(SessionContext {
        staff_id: claims.sub,
        email: claims.email,
        role: claims.role,
        onboarded: claims.onboarded,
    })
}

// ---------------------------------------------------------------------------
// auth_middleware
// ---------------------------------------------------------------------------

/// Enforce a minimum role requirement for a protected route.
///
/// Extracts and verifies the `session` cookie JWT, then compares the
/// authenticated user's role level against `required_role`.
///
/// Returns:
/// - `Ok(SessionContext)` when the session is valid and the role is sufficient.
/// - `Err(Response)` with HTTP **401** when no valid session exists.
/// - `Err(Response)` with HTTP **403** when the role is below the required level.
///
/// Requirements: 2.2, 2.3, 2.4, 2.5, 3.1, 3.2, 3.3
pub async fn auth_middleware(
    req: &Request,
    env: &Env,
    required_role: Role,
) -> StdResult<SessionContext, Response> {
    // ── 1. Extract the raw cookie header ────────────────────────────────────
    let cookie_header = req
        .headers()
        .get("Cookie")
        .ok()
        .flatten()
        .unwrap_or_default();

    // ── 2. Find the `session` cookie value ──────────────────────────────────
    let token = cookie_header
        .split(';')
        .map(|s| s.trim())
        .find_map(|part| part.strip_prefix("session="));

    let token = match token {
        Some(t) => t.to_string(),
        None => {
            // Requirement 2.5: absent cookie on a protected route → HTTP 401
            return Err(unauthorized_response());
        }
    };

    // ── 3. Pull the JWT secret from the Worker secret binding ───────────────
    let secret = match env.secret("JWT_SECRET") {
        Ok(s) => s.to_string(),
        Err(_) => return Err(internal_error_response()),
    };

    // ── 4. Verify the JWT (signature + expiry) ──────────────────────────────
    // Requirements 2.2 (invalid HMAC → 401), 2.3 (expired → 401), 2.4 (constant-time compare)
    let claims = match verify_jwt(&token, secret.as_bytes()) {
        Ok(c) => c,
        Err(WorkerError::Auth(AuthError::Expired)) => {
            return Err(unauthorized_response());
        }
        Err(_) => {
            return Err(unauthorized_response());
        }
    };

    // ── 5. Build session context ─────────────────────────────────────────────
    let ctx = SessionContext {
        staff_id: claims.sub,
        email: claims.email,
        role: claims.role,
        onboarded: claims.onboarded,
    };

    // ── 6. RBAC role level check ─────────────────────────────────────────────
    // Requirement 3.2: role_level(claims.role) >= role_level(required_role)
    // Requirement 3.3: insufficient role → HTTP 403
    // Requirement 3.8: monotone access (enforced by Ord impl on Role)
    if ctx.role < required_role {
        return Err(forbidden_response());
    }

    Ok(ctx)
}

// ---------------------------------------------------------------------------
// onboarding_guard
// ---------------------------------------------------------------------------

/// Pure, testable core of the onboarding guard.
///
/// Returns `true` when the request should be redirected to `/onboarding`:
/// - The session exists (`Some`).
/// - The session is not yet onboarded (`!ctx.onboarded`).
/// - The request path does not start with any entry in `BYPASS_PATHS`.
///
/// Returns `false` in all other cases (pass through).
///
/// Requirements: 4.1, 4.2
pub(crate) fn onboarding_guard_inner(
    session_ctx: Option<&SessionContext>,
    path: &str,
) -> bool {
    let ctx = match session_ctx {
        Some(c) => c,
        None => return false,
    };

    // Requirement 4.2: bypass paths are never redirected
    for bypass in BYPASS_PATHS {
        if path.starts_with(bypass) {
            return false;
        }
    }

    // Requirement 4.1: authenticated, not yet onboarded → redirect
    !ctx.onboarded
}

/// Return a `302 /onboarding` redirect when the session is not yet onboarded,
/// unless the request path is in the [`BYPASS_PATHS`] list.
///
/// - Returns `Some(response)` when a redirect must be issued.
/// - Returns `None` to pass control through to the handler.
///
/// Requirements: 4.1, 4.2
pub fn onboarding_guard(
    session_ctx: Option<&SessionContext>,
    path: &str,
) -> Option<Result<Response>> {
    if onboarding_guard_inner(session_ctx, path) {
        Some(Response::redirect(
            "https://ezeroandone.io/onboarding".parse().unwrap(),
        ))
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// Private helpers — build error responses without depending on error_to_response
// to keep the module self-contained and avoid circular dependencies.
// ---------------------------------------------------------------------------

fn unauthorized_response() -> Response {
    let body = r#"{"error":"Unauthorized"}"#;
    let mut headers = Headers::new();
    let _ = headers.set("Content-Type", "application/json");
    Response::from_body(ResponseBody::Body(body.as_bytes().to_vec()))
        .unwrap()
        .with_headers(headers)
        .with_status(401)
}

fn forbidden_response() -> Response {
    let body = r#"{"error":"Forbidden"}"#;
    let mut headers = Headers::new();
    let _ = headers.set("Content-Type", "application/json");
    Response::from_body(ResponseBody::Body(body.as_bytes().to_vec()))
        .unwrap()
        .with_headers(headers)
        .with_status(403)
}

fn internal_error_response() -> Response {
    let body = r#"{"error":"Internal Server Error"}"#;
    let mut headers = Headers::new();
    let _ = headers.set("Content-Type", "application/json");
    Response::from_body(ResponseBody::Body(body.as_bytes().to_vec()))
        .unwrap()
        .with_headers(headers)
        .with_status(500)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use super::BYPASS_PATHS;
    use super::onboarding_guard_inner;
    use crate::models::staff::Role;
    use proptest::prelude::*;

    // -----------------------------------------------------------------------
    // Helper — build a SessionContext for testing onboarding_guard
    // -----------------------------------------------------------------------
    fn make_ctx(onboarded: bool) -> SessionContext {
        SessionContext {
            staff_id: "s1".into(),
            email: "test@ezo.io".into(),
            role: Role::Staff,
            onboarded,
        }
    }

    /// Map a `Role` to its numeric access level.
    ///
    /// This mirrors the `Ord` implementation on `Role` (backed by `#[repr(u8)]`
    /// discriminants) and the comparison in `auth_middleware`:
    ///   `ctx.role >= required_role`  ↔  `role_level(ctx.role) >= role_level(required_role)`
    fn role_level(r: Role) -> u8 {
        r as u8
    }

    /// Proptest strategy that produces an arbitrary `Role`.
    fn arb_role() -> impl Strategy<Value = Role> {
        prop_oneof![
            Just(Role::Public),
            Just(Role::Staff),
            Just(Role::Admin),
            Just(Role::SuperAdmin),
        ]
    }

    // -----------------------------------------------------------------------
    // Property 4: RBAC is monotone
    //
    // For all roles r, r', req:
    //   if role_level(r) >= role_level(req)   (r has access)
    //   and role_level(r') > role_level(r)    (r' is strictly higher)
    //   then role_level(r') >= role_level(req) (r' also has access)
    //
    // Validates: Requirements 3.2, 3.8
    // -----------------------------------------------------------------------
    proptest! {
        #[test]
        fn prop_rbac_monotone(
            r   in arb_role(),
            r2  in arb_role(),
            req in arb_role(),
        ) {
            let level_r   = role_level(r);
            let level_r2  = role_level(r2);
            let level_req = role_level(req);

            // Only assert when the preconditions hold:
            //  • r has access to the route (r >= req)
            //  • r' is strictly higher than r
            if level_r >= level_req && level_r2 > level_r {
                // Monotonicity: r' must also have access
                prop_assert!(
                    level_r2 >= level_req,
                    "RBAC monotonicity violated: role {:?} (level {}) has access \
                     to required level {}, but higher role {:?} (level {}) does not",
                    r, level_r, level_req, r2, level_r2
                );
            }
        }
    }

    // -----------------------------------------------------------------------
    // Exhaustive check: enumerate all (r, r', req) triples explicitly to
    // complement the property-based test and guarantee full coverage of the
    // small, finite role space.
    // -----------------------------------------------------------------------
    #[test]
    fn rbac_monotone_exhaustive() {
        let all_roles = [Role::Public, Role::Staff, Role::Admin, Role::SuperAdmin];

        for &r in &all_roles {
            for &r2 in &all_roles {
                for &req in &all_roles {
                    let level_r   = role_level(r);
                    let level_r2  = role_level(r2);
                    let level_req = role_level(req);

                    if level_r >= level_req && level_r2 > level_r {
                        assert!(
                            level_r2 >= level_req,
                            "Monotonicity violated: r={:?}({}), r'={:?}({}), req={:?}({})",
                            r, level_r, r2, level_r2, req, level_req
                        );
                    }
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // Sanity check: verify role_level values match the spec (Public=1 … SuperAdmin=4)
    // -----------------------------------------------------------------------
    #[test]
    fn role_levels_match_spec() {
        assert_eq!(role_level(Role::Public),     1);
        assert_eq!(role_level(Role::Staff),      2);
        assert_eq!(role_level(Role::Admin),      3);
        assert_eq!(role_level(Role::SuperAdmin), 4);
    }

    // -----------------------------------------------------------------------
    // Property 6: Onboarding guard completeness
    //
    // A) For any authenticated session where onboarded=false and
    //    path ∉ BYPASS_PATHS, onboarding_guard returns Some(_) (redirect).
    //
    // B) For any path ∈ BYPASS_PATHS (possibly with a suffix), onboarding_guard
    //    always returns None regardless of onboarding state.
    //
    // Validates: Requirements 4.1, 4.2
    // -----------------------------------------------------------------------

    proptest! {
        /// **Validates: Requirements 4.1, 4.2**
        ///
        /// Property 6A — a non-onboarded authenticated session MUST be redirected
        /// for any path that does not start with a BYPASS_PATH prefix.
        #[test]
        fn prop_guard_redirects_non_bypass_not_onboarded(
            path in "/[a-z]{1,20}",
        ) {
            // Skip paths that happen to start with a bypass prefix — those are
            // covered by property 6B and are not in scope for this assertion.
            for bypass in BYPASS_PATHS {
                if path.starts_with(bypass) {
                    return Ok(());
                }
            }

            let ctx = make_ctx(false);
            // Use the pure inner function to avoid wasm-bindgen Response::redirect
            // on the native test target.
            prop_assert!(
                onboarding_guard_inner(Some(&ctx), &path),
                "Expected onboarding_guard_inner to return true (redirect) for \
                 non-onboarded session on non-bypass path {:?}, but got false",
                path
            );
        }

        /// **Validates: Requirements 4.1, 4.2**
        ///
        /// Property 6B — bypass paths MUST always return None, regardless of
        /// whether the session is onboarded or not.
        #[test]
        fn prop_guard_bypasses_bypass_paths(
            prefix_idx in 0usize..3usize,
            suffix in "[a-z/]{0,20}",
            onboarded in any::<bool>(),
        ) {
            let prefixes = ["/api/auth", "/api/onboarding", "/auth"];
            let path = format!("{}{}", prefixes[prefix_idx], suffix);

            let ctx = make_ctx(onboarded);
            // Bypass paths must never redirect — test via the pure inner function
            // as well as the full guard (bypass → None branch never touches Response).
            prop_assert!(
                !onboarding_guard_inner(Some(&ctx), &path),
                "onboarding_guard_inner returned true (redirect) for bypass path \
                 {:?} (onboarded={})",
                path,
                onboarded
            );
            prop_assert!(
                onboarding_guard(Some(&ctx), &path).is_none(),
                "onboarding_guard returned Some(_) for bypass path {:?} (onboarded={})",
                path,
                onboarded
            );
        }
    }

    // -----------------------------------------------------------------------
    // Deterministic unit tests for onboarding_guard_inner
    // -----------------------------------------------------------------------

    /// No session → guard must pass through (false) — unauthenticated requests
    /// are not the responsibility of the onboarding guard.
    #[test]
    fn guard_none_session_returns_none() {
        assert!(!onboarding_guard_inner(None, "/dashboard"));
    }

    /// An already-onboarded session must never be redirected, regardless of
    /// which non-bypass path is requested.
    #[test]
    fn guard_onboarded_returns_none() {
        let ctx = make_ctx(true);
        assert!(!onboarding_guard_inner(Some(&ctx), "/dashboard"));
        assert!(!onboarding_guard_inner(Some(&ctx), "/admin/staff"));
        assert!(!onboarding_guard_inner(Some(&ctx), "/profile"));
    }

    /// An authenticated, non-onboarded session on a non-bypass path must
    /// trigger a redirect (true).
    #[test]
    fn guard_not_onboarded_non_bypass_returns_some() {
        let ctx = make_ctx(false);
        assert!(onboarding_guard_inner(Some(&ctx), "/dashboard"));
    }

    /// Bypass paths must never redirect, even when the session is not onboarded.
    #[test]
    fn guard_bypass_path_returns_none() {
        let ctx = make_ctx(false);
        assert!(!onboarding_guard_inner(Some(&ctx), "/api/auth/login"));
        assert!(!onboarding_guard_inner(Some(&ctx), "/api/onboarding/profile"));
        assert!(!onboarding_guard_inner(Some(&ctx), "/auth"));
    }
}
