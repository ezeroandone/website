#![allow(dead_code)]

//! Transactional email provider client.
//! Sends magic-link and notification emails via Resend using their HTTP REST
//! API (no native SMTP — Workers do not support raw TCP).

use worker::wasm_bindgen::JsValue;
use worker::{Env, Fetch, Headers, Method, Request, RequestInit};

use crate::router::WorkerError;

const RESEND_API_URL: &str = "https://api.resend.com/emails";
const FROM_ADDRESS: &str = "noreply@ezeroandone.io";
const BASE_URL: &str = "https://ezeroandone.io";

/// Send a magic-link email to `to` containing a sign-in URL built from `token`.
///
/// The callback URL takes the form `https://ezeroandone.io/auth/callback?token={token}`.
/// The link expires in 15 minutes (enforced on the token side, noted in the email body).
pub async fn send_magic_link(to: &str, token: &str, env: &Env) -> Result<(), WorkerError> {
    let api_key = env
        .secret("MAIL_API_KEY")
        .map_err(|e| WorkerError::Internal(format!("Missing MAIL_API_KEY secret: {e}")))?
        .to_string();

    let callback_url = format!("{}/auth/callback?token={}", BASE_URL, token);

    let html = format!(
        "<p>Click <a href=\"{url}\">here</a> to sign in to eZeroAndOne. \
         This link expires in 15 minutes.</p>",
        url = callback_url
    );

    let payload = serde_json::json!({
        "from": FROM_ADDRESS,
        "to": [to],
        "subject": "Your magic link",
        "html": html,
    });

    post_email(&api_key, &payload.to_string()).await
}

/// Send a welcome / onboarding instructions email to `to`.
///
/// Includes a direct link to `/onboarding` and basic next-step instructions.
pub async fn send_onboarding_email(
    to: &str,
    staff_id: &str,
    env: &Env,
) -> Result<(), WorkerError> {
    let api_key = env
        .secret("MAIL_API_KEY")
        .map_err(|e| WorkerError::Internal(format!("Missing MAIL_API_KEY secret: {e}")))?
        .to_string();

    let onboarding_url = format!("{}/onboarding", BASE_URL);

    let html = format!(
        "<h1>Welcome to eZeroAndOne!</h1>\
         <p>Hi there,</p>\
         <p>Your staff account (<code>{staff_id}</code>) has been created. \
         To get started, please complete your onboarding steps:</p>\
         <ol>\
           <li>Visit your <a href=\"{url}\">onboarding dashboard</a>.</li>\
           <li>Upload your profile information and required documents.</li>\
           <li>Review and acknowledge the company policies.</li>\
         </ol>\
         <p>If you have any questions, reach out to your team lead or HR.</p>\
         <p>— The eZeroAndOne team</p>",
        staff_id = staff_id,
        url = onboarding_url
    );

    let payload = serde_json::json!({
        "from": FROM_ADDRESS,
        "to": [to],
        "subject": "Welcome to eZeroAndOne \u{2014} Complete your onboarding",
        "html": html,
    });

    post_email(&api_key, &payload.to_string()).await
}

/// Internal helper: POST a JSON body to the Resend API.
///
/// Returns `Err(WorkerError::Internal)` for any non-2xx HTTP status.
async fn post_email(api_key: &str, json_body: &str) -> Result<(), WorkerError> {
    let mut headers = Headers::new();
    headers
        .set("Content-Type", "application/json")
        .map_err(|e| WorkerError::Internal(format!("Failed to set Content-Type header: {e}")))?;
    headers
        .set("Authorization", &format!("Bearer {}", api_key))
        .map_err(|e| WorkerError::Internal(format!("Failed to set Authorization header: {e}")))?;

    let mut init = RequestInit::new();
    init.with_method(Method::Post);
    init.with_headers(headers);
    init.with_body(Some(JsValue::from_str(json_body)));

    let request = Request::new_with_init(RESEND_API_URL, &init)
        .map_err(|e| WorkerError::Internal(format!("Failed to build Resend request: {e}")))?;

    let mut response = Fetch::Request(request)
        .send()
        .await
        .map_err(|e| WorkerError::Internal(format!("Resend HTTP request failed: {e}")))?;

    let status = response.status_code();
    if !(200..300).contains(&status) {
        let body = response.text().await.unwrap_or_else(|_| "unreadable".to_string());
        return Err(WorkerError::Internal(format!(
            "Resend API returned non-2xx status: {} — {}",
            status, body
        )));
    }

    Ok(())
}
