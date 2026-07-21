#![allow(dead_code)]

use worker::*;

use crate::handlers::admin as admin_handlers;
use crate::handlers::auth as auth_handlers;
use crate::handlers::careers as careers_handlers;
use crate::handlers::content as content_handlers;
use crate::handlers::onboarding as onboarding_handlers;
use crate::handlers::team as team_handlers;
use crate::handlers::verify as verify_handlers;
use crate::middleware::auth::{auth_middleware, onboarding_guard, SessionContext};
use crate::models::staff::Role;

/// Central error type for all Worker errors.
#[derive(Debug)]
pub enum WorkerError {
    Auth(AuthError),
    Db(DbError),
    Validation(ValidationError),
    NotFound,
    Forbidden,
    RateLimited,
    Internal(String),
}

#[derive(Debug)]
pub enum AuthError {
    Unauthorized,
    InvalidToken,
    Expired,
    DomainNotAllowed,
    InvalidEmail,
}

#[derive(Debug)]
pub enum DbError {
    NotFound,
    Conflict,
    Query(String),
}

#[derive(Debug)]
pub enum ValidationError {
    InvalidInput(String),
    InvalidMimeType,
    FileTooLarge,
    TooManyFiles,
    InvalidKeyFormat,
    InvalidTransition,
}

impl From<WorkerError> for worker::Error {
    fn from(e: WorkerError) -> Self {
        worker::Error::RustError(format!("{:?}", e))
    }
}

/// Pure mapping from a `WorkerError` variant to its HTTP status code.
///
/// Extracted for testability: native-host unit tests cannot construct a
/// `worker::Response` (wasm-bindgen), so they call this function directly.
/// `error_to_response` delegates here so the mapping is never duplicated.
///
/// Mapping (design section 15.2 / Requirements 17.1–17.6):
/// - `Auth(_)`        → 401
/// - `Forbidden`      → 403
/// - `NotFound`       → 404
/// - `RateLimited`    → 429
/// - `Validation(_)`  → 400
/// - `Db(_)`          → 500
/// - `Internal(_)`    → 500
pub(crate) fn error_to_status(err: &WorkerError) -> u16 {
    match err {
        WorkerError::Auth(_) => 401,
        WorkerError::Forbidden => 403,
        WorkerError::NotFound => 404,
        WorkerError::RateLimited => 429,
        WorkerError::Validation(_) => 400,
        WorkerError::Db(_) | WorkerError::Internal(_) => 500,
    }
}

/// Returns the canonical plain-text body message for a given HTTP status code.
fn status_message(status: u16) -> &'static str {
    match status {
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        429 => "Too Many Requests",
        _ => "Internal Server Error",
    }
}

/// Map a `WorkerError` to an HTTP `Response`.
pub fn error_to_response(err: WorkerError) -> Result<Response> {
    let status = error_to_status(&err);
    let message = status_message(status);

    // Log full detail for 5xx errors (visible only in Workers logs)
    if status == 500 {
        console_error!("Internal error: {:?}", err);
    }

    Response::from_json(&serde_json::json!({ "error": message }))
        .map(|r| r.with_status(status))
        .map_err(|e| e.into())
}

// ---------------------------------------------------------------------------
// Placeholder handler — returns 404 until the real handler is implemented.
// ---------------------------------------------------------------------------
fn not_implemented() -> Result<Response> {
    error_to_response(WorkerError::NotFound)
}

// ---------------------------------------------------------------------------
// Router
// ---------------------------------------------------------------------------

/// Dispatch an incoming request to the correct handler.
///
/// Route protection follows the table in design section 11.3:
/// - Public routes are dispatched directly.
/// - Protected routes call `auth_middleware` first; on `Err(resp)` that
///   response is returned immediately.
/// - The `onboarding_guard` is applied for all authenticated sessions on
///   protected paths, redirecting incomplete staff to /onboarding.
///
/// Requirements: 3.4, 3.5, 3.6, 3.7
pub async fn router_dispatch(
    req: &Request,
    session_ctx: Option<SessionContext>,
    env: &Env,
) -> Result<Response> {
    let path = req.path();
    let method = req.method();

    // ── Onboarding guard (authenticated sessions only) ──────────────────────
    // Applied before routing so that incomplete staff can't access protected
    // resources by guessing URLs. Bypass paths (/api/auth, /api/onboarding)
    // are checked inside the guard itself.
    if let Some(redirect) = onboarding_guard(session_ctx.as_ref(), &path) {
        return redirect;
    }

    // ── Public routes (no auth required) ────────────────────────────────────

    // GET /api/insights
    if method == Method::Get && path == "/api/insights" {
        return content_handlers::list_insights(req, env).await;
    }

    // GET /api/insights/:slug
    if method == Method::Get && path.starts_with("/api/insights/") {
        let slug = path.trim_start_matches("/api/insights/");
        return content_handlers::get_insight(req, env, slug).await;
    }

    // GET /api/work
    if method == Method::Get && path == "/api/work" {
        return content_handlers::list_work(req, env).await;
    }

    // GET /api/work/:slug
    if method == Method::Get && path.starts_with("/api/work/") {
        let slug = path.trim_start_matches("/api/work/");
        return content_handlers::get_work(req, env, slug).await;
    }

    // GET /api/capabilities
    if method == Method::Get && path == "/api/capabilities" {
        return content_handlers::list_capabilities(req, env).await;
    }

    // GET /api/capabilities/:slug
    if method == Method::Get && path.starts_with("/api/capabilities/") {
        let slug = path.trim_start_matches("/api/capabilities/");
        return content_handlers::get_capability(req, env, slug).await;
    }

    // GET /api/team
    if method == Method::Get && path == "/api/team" {
        return team_handlers::list_team(req, env).await;
    }

    // GET /api/team/:username
    if method == Method::Get && path.starts_with("/api/team/") {
        let username = path.trim_start_matches("/api/team/");
        return team_handlers::get_team_member(req, env, username).await;
    }

    // GET /api/careers
    if method == Method::Get && path == "/api/careers" {
        return careers_handlers::list_careers(req, env).await;
    }

    // GET /api/careers/:slug
    // Note: must be checked before the more-specific apply sub-routes.
    if method == Method::Get && path.starts_with("/api/careers/") {
        let rest = path.trim_start_matches("/api/careers/");
        // Only match if no further path segments (i.e. not /api/careers/:slug/apply/...)
        if !rest.contains('/') {
            return careers_handlers::get_career(req, env, rest).await;
        }
    }

    // POST /api/careers/:slug/apply
    if method == Method::Post {
        let segments: Vec<&str> = path.split('/').collect();
        // /api/careers/:slug/apply  → ["", "api", "careers", slug, "apply"]
        if segments.len() == 5
            && segments[1] == "api"
            && segments[2] == "careers"
            && segments[4] == "apply"
        {
            let slug = segments[3].to_string();
            return careers_handlers::apply(req.clone()?, env, &slug).await;
        }
    }

    // POST /api/careers/:slug/apply/documents
    if method == Method::Post {
        let segments: Vec<&str> = path.split('/').collect();
        // /api/careers/:slug/apply/documents → ["", "api", "careers", slug, "apply", "documents"]
        if segments.len() == 6
            && segments[1] == "api"
            && segments[2] == "careers"
            && segments[4] == "apply"
            && segments[5] == "documents"
        {
            let slug = segments[3].to_string();
            return careers_handlers::upload_document(req.clone()?, env, &slug).await;
        }
    }

    // POST /api/auth/request
    if method == Method::Post && path == "/api/auth/request" {
        return auth_handlers::request_magic_link(req.clone()?, env).await;
    }

    // GET /api/auth/callback
    if method == Method::Get && path == "/api/auth/callback" {
        return auth_handlers::callback(req.clone()?, env).await;
    }

    // GET /api/verify  — public, no auth required (Requirement 3.4)
    if method == Method::Get && path == "/api/verify" {
        return verify_handlers::handle_verify(req, env).await;
    }

    // ── Staff-protected routes (Role::Staff required) ────────────────────────

    // POST /api/auth/logout  (Staff role required)
    if method == Method::Post && path == "/api/auth/logout" {
        let _ctx = match auth_middleware(req, env, Role::Staff).await {
            Ok(ctx) => ctx,
            Err(resp) => return Ok(resp),
        };
        return auth_handlers::logout(req.clone()?, env).await;
    }

    // GET /api/onboarding/status
    if method == Method::Get && path == "/api/onboarding/status" {
        let ctx = match auth_middleware(req, env, Role::Staff).await {
            Ok(ctx) => ctx,
            Err(resp) => return Ok(resp),
        };
        return onboarding_handlers::get_status(req, env, ctx).await;
    }

    // GET /api/me — return the authenticated user's own staff record
    if method == Method::Get && path == "/api/me" {
        let ctx = match auth_middleware(req, env, Role::Staff).await {
            Ok(ctx) => ctx,
            Err(resp) => return Ok(resp),
        };
        return onboarding_handlers::get_me(req, env, ctx).await;
    }

    // PATCH /api/onboarding/profile
    if method == Method::Patch && path == "/api/onboarding/profile" {
        let ctx = match auth_middleware(req, env, Role::Staff).await {
            Ok(ctx) => ctx,
            Err(resp) => return Ok(resp),
        };
        return onboarding_handlers::patch_profile(req.clone()?, env, ctx).await;
    }

    // POST /api/onboarding/signing-key
    if method == Method::Post && path == "/api/onboarding/signing-key" {
        let ctx = match auth_middleware(req, env, Role::Staff).await {
            Ok(ctx) => ctx,
            Err(resp) => return Ok(resp),
        };
        return onboarding_handlers::post_signing_key(req.clone()?, env, ctx).await;
    }

    // POST /api/onboarding/complete
    if method == Method::Post && path == "/api/onboarding/complete" {
        let ctx = match auth_middleware(req, env, Role::Staff).await {
            Ok(ctx) => ctx,
            Err(resp) => return Ok(resp),
        };
        return onboarding_handlers::post_complete(req, env, ctx).await;
    }

    // POST /api/upload/avatar
    if method == Method::Post && path == "/api/upload/avatar" {
        let ctx = match auth_middleware(req, env, Role::Staff).await {
            Ok(ctx) => ctx,
            Err(resp) => return Ok(resp),
        };
        return admin_handlers::handle_upload_avatar(req.clone()?, ctx, env).await;
    }

    // ── Admin-protected routes (Role::Admin required) ────────────────────────

    // GET /api/admin/careers
    if method == Method::Get && path == "/api/admin/careers" {
        let ctx = match auth_middleware(req, env, Role::Admin).await {
            Ok(ctx) => ctx,
            Err(resp) => return Ok(resp),
        };
        return admin_handlers::list_admin_careers(req, env, ctx).await;
    }

    // POST /api/admin/careers
    if method == Method::Post && path == "/api/admin/careers" {
        let ctx = match auth_middleware(req, env, Role::Admin).await {
            Ok(ctx) => ctx,
            Err(resp) => return Ok(resp),
        };
        return admin_handlers::create_career(req.clone()?, env, ctx).await;
    }

    // PATCH /api/admin/careers/:id
    if method == Method::Patch && path.starts_with("/api/admin/careers/") {
        let id = path.trim_start_matches("/api/admin/careers/").to_string();
        let ctx = match auth_middleware(req, env, Role::Admin).await {
            Ok(ctx) => ctx,
            Err(resp) => return Ok(resp),
        };
        return admin_handlers::patch_career(req.clone()?, env, ctx, &id).await;
    }

    // PATCH /api/admin/applications/:id/status
    if method == Method::Patch {
        let segments: Vec<&str> = path.split('/').collect();
        // /api/admin/applications/:id/status → ["", "api", "admin", "applications", id, "status"]
        if segments.len() == 6
            && segments[1] == "api"
            && segments[2] == "admin"
            && segments[3] == "applications"
            && segments[5] == "status"
        {
            let id = segments[4].to_string();
            let ctx = match auth_middleware(req, env, Role::Admin).await {
                Ok(ctx) => ctx,
                Err(resp) => return Ok(resp),
            };
            return admin_handlers::patch_application_status(req.clone()?, env, ctx, &id).await;
        }
    }

    // POST /api/admin/applications/:id/hire
    if method == Method::Post {
        let segments: Vec<&str> = path.split('/').collect();
        // /api/admin/applications/:id/hire → ["", "api", "admin", "applications", id, "hire"]
        if segments.len() == 6
            && segments[1] == "api"
            && segments[2] == "admin"
            && segments[3] == "applications"
            && segments[5] == "hire"
        {
            let id = segments[4].to_string();
            let ctx = match auth_middleware(req, env, Role::Admin).await {
                Ok(ctx) => ctx,
                Err(resp) => return Ok(resp),
            };
            return admin_handlers::hire_applicant(req.clone()?, env, ctx, &id).await;
        }
    }

    // GET /api/admin/applications/:id/documents
    if method == Method::Get {
        let segments: Vec<&str> = path.split('/').collect();
        // /api/admin/applications/:id/documents → ["", "api", "admin", "applications", id, "documents"]
        if segments.len() == 6
            && segments[1] == "api"
            && segments[2] == "admin"
            && segments[3] == "applications"
            && segments[5] == "documents"
        {
            let id = segments[4].to_string();
            let ctx = match auth_middleware(req, env, Role::Admin).await {
                Ok(ctx) => ctx,
                Err(resp) => return Ok(resp),
            };
            return admin_handlers::handle_get_application_documents(&id, ctx, env).await;
        }
    }

    // GET /api/admin/applications/:id/documents/:doc_id/url
    if method == Method::Get {
        let segments: Vec<&str> = path.split('/').collect();
        // /api/admin/applications/:id/documents/:doc_id/url
        // → ["", "api", "admin", "applications", id, "documents", doc_id, "url"]
        if segments.len() == 8
            && segments[1] == "api"
            && segments[2] == "admin"
            && segments[3] == "applications"
            && segments[5] == "documents"
            && segments[7] == "url"
        {
            let id = segments[4].to_string();
            let doc_id = segments[6].to_string();
            let ctx = match auth_middleware(req, env, Role::Admin).await {
                Ok(ctx) => ctx,
                Err(resp) => return Ok(resp),
            };
            return admin_handlers::handle_get_document_presigned_url(&id, &doc_id, ctx, env).await;
        }
    }

    // GET /api/admin/applications  (list all)
    if method == Method::Get && path == "/api/admin/applications" {
        let ctx = match auth_middleware(req, env, Role::Admin).await {
            Ok(ctx) => ctx,
            Err(resp) => return Ok(resp),
        };
        return admin_handlers::list_applications(req, env, ctx).await;
    }

    // POST /api/admin/staff/:id/confirm
    if method == Method::Post {
        let segments: Vec<&str> = path.split('/').collect();
        // /api/admin/staff/:id/confirm → ["", "api", "admin", "staff", id, "confirm"]
        if segments.len() == 6
            && segments[1] == "api"
            && segments[2] == "admin"
            && segments[3] == "staff"
            && segments[5] == "confirm"
        {
            let id = segments[4].to_string();
            let ctx = match auth_middleware(req, env, Role::Admin).await {
                Ok(ctx) => ctx,
                Err(resp) => return Ok(resp),
            };
            return admin_handlers::confirm_staff(req, env, ctx, &id).await;
        }
    }

    // PATCH /api/admin/staff/:id/role
    if method == Method::Patch {
        let segments: Vec<&str> = path.split('/').collect();
        // /api/admin/staff/:id/role → ["", "api", "admin", "staff", id, "role"]
        if segments.len() == 6
            && segments[1] == "api"
            && segments[2] == "admin"
            && segments[3] == "staff"
            && segments[5] == "role"
        {
            let id = segments[4].to_string();
            let ctx = match auth_middleware(req, env, Role::Admin).await {
                Ok(ctx) => ctx,
                Err(resp) => return Ok(resp),
            };
            return admin_handlers::patch_staff_role(req.clone()?, env, ctx, &id).await;
        }
    }

    // GET /api/admin/staff
    if method == Method::Get && path == "/api/admin/staff" {
        let ctx = match auth_middleware(req, env, Role::Admin).await {
            Ok(ctx) => ctx,
            Err(resp) => return Ok(resp),
        };
        return admin_handlers::list_staff(req, env, ctx).await;
    }

    // GET /api/admin/content  (list all posts incl. drafts)
    if method == Method::Get && path == "/api/admin/content" {
        let ctx = match auth_middleware(req, env, Role::Admin).await {
            Ok(ctx) => ctx,
            Err(resp) => return Ok(resp),
        };
        return admin_handlers::list_content(req, env, ctx).await;
    }

    // POST /api/admin/content
    if method == Method::Post && path == "/api/admin/content" {
        let ctx = match auth_middleware(req, env, Role::Admin).await {
            Ok(ctx) => ctx,
            Err(resp) => return Ok(resp),
        };
        return admin_handlers::create_content(req.clone()?, env, ctx).await;
    }

    // PATCH /api/admin/content/:id  (note: must come before SuperAdmin DELETE)
    if method == Method::Patch && path.starts_with("/api/admin/content/") {
        let id = path.trim_start_matches("/api/admin/content/").to_string();
        let ctx = match auth_middleware(req, env, Role::Admin).await {
            Ok(ctx) => ctx,
            Err(resp) => return Ok(resp),
        };
        return admin_handlers::patch_content(req.clone()?, env, ctx, &id).await;
    }

    // POST /api/upload/post/:id/cover
    if method == Method::Post {
        let segments: Vec<&str> = path.split('/').collect();
        // /api/upload/post/:id/cover → ["", "api", "upload", "post", id, "cover"]
        if segments.len() == 6
            && segments[1] == "api"
            && segments[2] == "upload"
            && segments[3] == "post"
            && segments[5] == "cover"
        {
            let id = segments[4].to_string();
            let ctx = match auth_middleware(req, env, Role::Admin).await {
                Ok(ctx) => ctx,
                Err(resp) => return Ok(resp),
            };
            return admin_handlers::handle_upload_post_cover(&id, req.clone()?, ctx, env).await;
        }
    }

    // POST /api/upload/post/:id/media
    if method == Method::Post {
        let segments: Vec<&str> = path.split('/').collect();
        // /api/upload/post/:id/media → ["", "api", "upload", "post", id, "media"]
        if segments.len() == 6
            && segments[1] == "api"
            && segments[2] == "upload"
            && segments[3] == "post"
            && segments[5] == "media"
        {
            let id = segments[4].to_string();
            let ctx = match auth_middleware(req, env, Role::Admin).await {
                Ok(ctx) => ctx,
                Err(resp) => return Ok(resp),
            };
            return admin_handlers::handle_upload_post_media(&id, req.clone()?, ctx, env).await;
        }
    }

    // POST /api/upload/career/:id/hero
    if method == Method::Post {
        let segments: Vec<&str> = path.split('/').collect();
        // /api/upload/career/:id/hero → ["", "api", "upload", "career", id, "hero"]
        if segments.len() == 6
            && segments[1] == "api"
            && segments[2] == "upload"
            && segments[3] == "career"
            && segments[5] == "hero"
        {
            let id = segments[4].to_string();
            let ctx = match auth_middleware(req, env, Role::Admin).await {
                Ok(ctx) => ctx,
                Err(resp) => return Ok(resp),
            };
            return admin_handlers::handle_upload_career_hero(&id, req.clone()?, ctx, env).await;
        }
    }

    // GET /api/clients  (public)
    if method == Method::Get && path == "/api/clients" {
        return admin_handlers::list_clients_public(req, env).await;
    }

    // GET /api/admin/clients
    if method == Method::Get && path == "/api/admin/clients" {
        let ctx = match auth_middleware(req, env, Role::Admin).await {
            Ok(ctx) => ctx,
            Err(resp) => return Ok(resp),
        };
        return admin_handlers::list_clients(req, env, ctx).await;
    }

    // POST /api/admin/clients
    if method == Method::Post && path == "/api/admin/clients" {
        let ctx = match auth_middleware(req, env, Role::Admin).await {
            Ok(ctx) => ctx,
            Err(resp) => return Ok(resp),
        };
        return admin_handlers::create_client(req.clone()?, env, ctx).await;
    }

    // PATCH /api/admin/clients/:id
    if method == Method::Patch && path.starts_with("/api/admin/clients/") {
        let id = path.trim_start_matches("/api/admin/clients/").to_string();
        if !id.contains('/') {
            let ctx = match auth_middleware(req, env, Role::Admin).await {
                Ok(ctx) => ctx,
                Err(resp) => return Ok(resp),
            };
            return admin_handlers::patch_client(req.clone()?, env, ctx, &id).await;
        }
    }

    // DELETE /api/admin/clients/:id
    if method == Method::Delete && path.starts_with("/api/admin/clients/") {
        let id = path.trim_start_matches("/api/admin/clients/").to_string();
        if !id.contains('/') {
            let ctx = match auth_middleware(req, env, Role::Admin).await {
                Ok(ctx) => ctx,
                Err(resp) => return Ok(resp),
            };
            return admin_handlers::delete_client(req, env, ctx, &id).await;
        }
    }

    // GET /api/admin/content/:id/team
    if method == Method::Get {
        let segments: Vec<&str> = path.split('/').collect();
        if segments.len() == 6
            && segments[1] == "api"
            && segments[2] == "admin"
            && segments[3] == "content"
            && segments[5] == "team"
        {
            let id = segments[4].to_string();
            let ctx = match auth_middleware(req, env, Role::Admin).await {
                Ok(ctx) => ctx,
                Err(resp) => return Ok(resp),
            };
            return admin_handlers::list_post_team(req, env, ctx, &id).await;
        }
    }

    // POST /api/admin/content/:id/team
    if method == Method::Post {
        let segments: Vec<&str> = path.split('/').collect();
        if segments.len() == 6
            && segments[1] == "api"
            && segments[2] == "admin"
            && segments[3] == "content"
            && segments[5] == "team"
        {
            let id = segments[4].to_string();
            let ctx = match auth_middleware(req, env, Role::Admin).await {
                Ok(ctx) => ctx,
                Err(resp) => return Ok(resp),
            };
            return admin_handlers::add_post_team_member(req.clone()?, env, ctx, &id).await;
        }
    }

    // DELETE /api/admin/content/:post_id/team/:member_id
    if method == Method::Delete {
        let segments: Vec<&str> = path.split('/').collect();
        if segments.len() == 7
            && segments[1] == "api"
            && segments[2] == "admin"
            && segments[3] == "content"
            && segments[5] == "team"
        {
            let post_id = segments[4].to_string();
            let member_id = segments[6].to_string();
            let ctx = match auth_middleware(req, env, Role::Admin).await {
                Ok(ctx) => ctx,
                Err(resp) => return Ok(resp),
            };
            return admin_handlers::remove_post_team_member(req, env, ctx, &post_id, &member_id).await;
        }
    }

    // POST /api/upload/client/:id/logo
    if method == Method::Post {
        let segments: Vec<&str> = path.split('/').collect();
        if segments.len() == 6
            && segments[1] == "api"
            && segments[2] == "upload"
            && segments[3] == "client"
            && segments[5] == "logo"
        {
            let id = segments[4].to_string();
            let ctx = match auth_middleware(req, env, Role::Admin).await {
                Ok(ctx) => ctx,
                Err(resp) => return Ok(resp),
            };
            return admin_handlers::handle_upload_client_logo(&id, req.clone()?, ctx, env).await;
        }
    }

    // ── SuperAdmin-protected routes (Role::SuperAdmin required) ──────────────

    // DELETE /api/admin/content/:id
    if method == Method::Delete && path.starts_with("/api/admin/content/") {
        let id = path.trim_start_matches("/api/admin/content/").to_string();
        let ctx = match auth_middleware(req, env, Role::SuperAdmin).await {
            Ok(ctx) => ctx,
            Err(resp) => return Ok(resp),
        };
        return admin_handlers::delete_content(req, env, ctx, &id).await;
    }

    // DELETE /api/admin/*  (catch-all SuperAdmin delete)
    if method == Method::Delete && path.starts_with("/api/admin/") {
        let _ctx = match auth_middleware(req, env, Role::SuperAdmin).await {
            Ok(ctx) => ctx,
            Err(resp) => return Ok(resp),
        };
        return not_implemented(); // handlers::admin::admin_delete_catchall
    }

    // ── Fallthrough — no route matched ──────────────────────────────────────
    error_to_response(WorkerError::NotFound)
}
