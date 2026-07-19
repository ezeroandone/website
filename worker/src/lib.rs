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

    // Rate limiting
    let ip = req
        .headers()
        .get("CF-Connecting-IP")?
        .unwrap_or_else(|| "unknown".to_string());
    let kv = env.kv("EZO_AUTH")?;
    if let Err(e) = middleware::rate_limit::rate_limit_check(&kv, &ip).await {
        return router::error_to_response(e);
    }

    // Session context (may be None for public routes)
    let session_ctx = middleware::auth::parse_session_context(&req, &env).await;

    // Route dispatch
    let response = router::router_dispatch(&req, session_ctx, &env).await;

    // Attach CORS headers to all responses
    middleware::cors::attach_cors_headers(response)
}
