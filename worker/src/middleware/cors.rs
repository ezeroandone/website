use worker::*;

const ALLOWED_ORIGIN: &str = "https://ezeroandone.io";
const ALLOWED_METHODS: &str = "GET, POST, PATCH, DELETE, OPTIONS";
const ALLOWED_HEADERS: &str = "Content-Type, Authorization, Cookie";

/// Return an HTTP 200 response with all required CORS preflight headers.
pub fn cors_preflight_response() -> Result<Response> {
    let mut headers = Headers::new();
    headers.set("Access-Control-Allow-Origin", ALLOWED_ORIGIN)?;
    headers.set("Access-Control-Allow-Methods", ALLOWED_METHODS)?;
    headers.set("Access-Control-Allow-Headers", ALLOWED_HEADERS)?;
    headers.set("Access-Control-Allow-Credentials", "true")?;
    Ok(Response::empty()?.with_headers(headers))
}

/// Append CORS headers to any outgoing response.
pub fn attach_cors_headers(res: Result<Response>) -> Result<Response> {
    let mut res = res?;
    let headers = res.headers_mut();
    headers.set("Access-Control-Allow-Origin", ALLOWED_ORIGIN)?;
    headers.set("Access-Control-Allow-Credentials", "true")?;
    Ok(res)
}
