#![allow(dead_code)]

//! Public R2 object proxy.
//! GET /media/:key — serves a file from the EZO_MEDIA R2 bucket.

use worker::*;
use crate::router::WorkerError;

pub async fn serve_r2_object(_req: &Request, env: &Env, key: &str) -> Result<Response> {
    let bucket = env.bucket("EZO_MEDIA")
        .map_err(|e| WorkerError::Internal(e.to_string()))?;

    match bucket.get(key).execute().await {
        Ok(Some(obj)) => {
            let body = obj.body().ok_or_else(|| WorkerError::Internal("No body".into()))?;
            let bytes = body.bytes().await.map_err(|e| WorkerError::Internal(e.to_string()))?;
            let mime = obj.http_metadata().content_type
                .unwrap_or_else(|| "application/octet-stream".to_string());
            Response::from_bytes(bytes)
                .map(|r| r.with_headers({
                    let mut h = Headers::new();
                    let _ = h.set("Content-Type", &mime);
                    let _ = h.set("Cache-Control", "public, max-age=31536000, immutable");
                    let _ = h.set("Access-Control-Allow-Origin", "*");
                    h
                }))
                .map_err(|e| WorkerError::Internal(e.to_string()).into())
        }
        Ok(None) => {
            crate::router::error_to_response(WorkerError::NotFound)
        }
        Err(e) => {
            crate::router::error_to_response(WorkerError::Internal(e.to_string()))
        }
    }
}
