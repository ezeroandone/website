use worker::*;

mod crypto;
mod db;
mod email;
mod handlers;
mod middleware;
mod models;
mod router;
mod storage;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // CORS preflight fast-path
    if req.method() == Method::Options {
        return middleware::cors::cors_preflight_response();
    }

    // Rate limiting is applied only in the auth request handler
    // (POST /api/auth/request) to prevent magic-link spam.
    // Do NOT apply it globally — page loads make many parallel API calls
    // that would exhaust a per-IP quota instantly.

    // Session context (may be None for public routes)
    let session_ctx = middleware::auth::parse_session_context(&req, &env).await;

    // Route dispatch
    let response = router::router_dispatch(&req, session_ctx, &env).await;

    // Attach CORS headers to all responses
    middleware::cors::attach_cors_headers(response)
}
