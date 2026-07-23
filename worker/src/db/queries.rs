#![allow(dead_code, unused_imports)]

//! Typed D1 query helpers.
//! Each function wraps a prepared statement and deserialises the result
//! into the appropriate domain model from `crate::models`.
//!
//! This module also contains the KV cache-aside helper used by content,
//! team, and career handlers to reduce D1 read load within Free Tier budgets.

use std::result::Result;

use serde::{Deserialize, Serialize};
use worker::kv::KvStore;
use worker::*;

use crate::router::WorkerError;

// ---------------------------------------------------------------------------
// KV cache-aside helper
// ---------------------------------------------------------------------------

/// Serve `T` from KV cache when available; otherwise call `fetch_fn` to load
/// from D1, attempt to write the result back to KV, and return the value.
///
/// # Generic parameters
/// * `T`   — response type; must be JSON-serialisable/deserialisable.
/// * `F`   — async factory that fetches from D1 on cache miss.
/// * `Fut` — the `Future` returned by `F`.
///
/// # Behaviour
/// | KV state | Action |
/// |---|---|
/// | HIT  (valid JSON)  | Deserialise and return immediately — no D1 query (Req 13.4) |
/// | HIT  (corrupt JSON)| Log warning, fall through to fetch_fn |
/// | MISS or KV error   | Call `fetch_fn`, populate cache, return value  (Req 13.5) |
/// | KV write failure   | Log warning, **do not** surface to caller      (Req 13.6) |
///
/// Requirements: 13.1, 13.2, 13.3, 13.4, 13.5, 13.6
pub async fn get_cached_or_fetch<T, F, Fut>(
    kv: &KvStore,
    cache_key: &str,
    ttl_secs: u64,
    fetch_fn: F,
) -> Result<T, WorkerError>
where
    T: Serialize + for<'de> Deserialize<'de>,
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T, WorkerError>>,
{
    // ── 1. KV GET ────────────────────────────────────────────────────────────
    match kv.get(cache_key).text().await {
        Ok(Some(json_str)) => {
            // Cache HIT — attempt to deserialise (Requirement 13.4).
            match serde_json::from_str::<T>(&json_str) {
                Ok(value) => return Ok(value),
                Err(e) => {
                    // Stale / corrupt entry — fall through to re-fetch.
                    console_warn!(
                        "kv cache deserialise error for key '{}': {}",
                        cache_key,
                        e
                    );
                }
            }
        }
        Ok(None) => {
            // Cache miss — will call fetch_fn below.
        }
        Err(e) => {
            // KV read error — log and fall through to D1 (non-fatal).
            console_warn!("kv get error for key '{}': {}", cache_key, e);
        }
    }

    // ── 2. Cache miss: call fetch_fn to obtain data from D1 (Req 13.5) ───────
    let value = fetch_fn().await?;

    // ── 3. Populate KV cache (non-fatal on failure, Requirement 13.6) ────────
    match serde_json::to_string(&value) {
        Ok(json_str) => {
            match kv.put(cache_key, json_str) {
                Ok(builder) => {
                    if let Err(e) = builder.expiration_ttl(ttl_secs).execute().await {
                        // Req 13.6: KV write failure must NOT be surfaced to the caller.
                        console_warn!("kv put error for key '{}': {}", cache_key, e);
                    }
                }
                Err(e) => {
                    console_warn!("kv put builder error for key '{}': {}", cache_key, e);
                }
            }
        }
        Err(e) => {
            console_warn!("kv cache serialise error for key '{}': {}", cache_key, e);
        }
    }

    Ok(value)
}
