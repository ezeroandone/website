use worker::kv::KvStore;

use crate::router::WorkerError;

const MAX_REQUESTS: u32 = 5;
const WINDOW_SECS: u64 = 60;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct RateEntry {
    pub(crate) count: u32,
    pub(crate) window_start: u64,
}

/// Pure, testable core of the rate-limit logic.
///
/// Takes the existing KV entry (or `None` for the first request in this window)
/// and the current Unix timestamp in seconds.  Returns the updated `RateEntry`
/// that should be written back to KV and a boolean indicating whether the
/// request is **allowed** (`true`) or should be **rate-limited** (`false`).
///
/// The logic mirrors `rate_limit_check` exactly but without any I/O:
/// - If no entry exists, start a new window with `count = 1`.
/// - If the window has expired (`now >= window_start + WINDOW_SECS`), reset
///   the window and set `count = 1`.
/// - Otherwise increment `count`.
/// - Return `allowed = count <= MAX_REQUESTS`.
pub(crate) fn rate_limit_check_inner(entry: Option<RateEntry>, now: u64) -> (RateEntry, bool) {
    let new_entry = match entry {
        None => RateEntry {
            count: 1,
            window_start: now,
        },
        Some(mut e) => {
            if now.saturating_sub(e.window_start) >= WINDOW_SECS {
                // Window expired — start fresh.
                e.count = 1;
                e.window_start = now;
            } else {
                e.count += 1;
            }
            e
        }
    };
    let allowed = new_entry.count <= MAX_REQUESTS;
    (new_entry, allowed)
}

/// Check the per-IP rate limit using KV_AUTH.
/// Returns `Err(WorkerError::RateLimited)` when the caller exceeds the threshold.
pub async fn rate_limit_check(kv: &KvStore, ip: &str) -> Result<(), WorkerError> {
    let key = format!("rl:{}", ip);

    // Current Unix timestamp in seconds
    let now = (worker::js_sys::Date::now() / 1000.0) as u64;

    // Read existing entry from KV (None means first request for this IP/window)
    let raw = kv
        .get(&key)
        .text()
        .await
        .map_err(|e| WorkerError::Internal(e.to_string()))?;

    let existing = match raw {
        Some(text) => Some(
            serde_json::from_str::<RateEntry>(&text)
                .map_err(|e| WorkerError::Internal(e.to_string()))?,
        ),
        None => None,
    };

    let (entry, allowed) = rate_limit_check_inner(existing, now);

    // Serialize and write back with TTL 60 s regardless of outcome
    let serialized =
        serde_json::to_string(&entry).map_err(|e| WorkerError::Internal(e.to_string()))?;

    kv.put(&key, serialized)
        .map_err(|e| WorkerError::Internal(e.to_string()))?
        .expiration_ttl(WINDOW_SECS)
        .execute()
        .await
        .map_err(|e| WorkerError::Internal(e.to_string()))?;

    if !allowed {
        return Err(WorkerError::RateLimited);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /// Baseline timestamp used across all tests (arbitrary fixed value).
    const T0: u64 = 1_700_000_000;

    // ── Deterministic unit tests ─────────────────────────────────────────────

    /// Requests 1–5 within the same window must all be allowed.
    #[test]
    fn five_requests_in_window_all_allowed() {
        let mut entry: Option<RateEntry> = None;
        for i in 1..=5 {
            let (new_entry, allowed) = rate_limit_check_inner(entry, T0);
            assert!(
                allowed,
                "request {} should be allowed (count = {})",
                i,
                new_entry.count
            );
            entry = Some(new_entry);
        }
    }

    /// The 6th request in the same 60-second window must be rate-limited.
    #[test]
    fn sixth_request_in_window_is_rate_limited() {
        let mut entry: Option<RateEntry> = None;
        // Make 5 allowed requests.
        for _ in 1..=5 {
            let (new_entry, _) = rate_limit_check_inner(entry, T0);
            entry = Some(new_entry);
        }
        // 6th request in the same window.
        let (_, allowed) = rate_limit_check_inner(entry, T0);
        assert!(!allowed, "6th request in same window must be rate-limited");
    }

    /// After exactly WINDOW_SECS have elapsed the counter resets and the
    /// next request is allowed again.
    #[test]
    fn request_after_window_expiry_is_allowed() {
        // Exhaust the limit.
        let mut entry: Option<RateEntry> = None;
        for _ in 1..=6 {
            let (new_entry, _) = rate_limit_check_inner(entry, T0);
            entry = Some(new_entry);
        }
        // Advance time by exactly WINDOW_SECS.
        let later = T0 + WINDOW_SECS;
        let (new_entry, allowed) = rate_limit_check_inner(entry, later);
        assert!(
            allowed,
            "first request after window expiry must be allowed, count = {}",
            new_entry.count
        );
        assert_eq!(
            new_entry.count, 1,
            "counter must reset to 1 after window expiry"
        );
        assert_eq!(
            new_entry.window_start, later,
            "window_start must be updated to the current time"
        );
    }

    /// The very first request (no existing entry) must be allowed.
    #[test]
    fn first_request_no_entry_is_allowed() {
        let (entry, allowed) = rate_limit_check_inner(None, T0);
        assert!(allowed, "first request with no prior entry must be allowed");
        assert_eq!(entry.count, 1);
        assert_eq!(entry.window_start, T0);
    }

    // ── Property 14: Rate-limiter enforces per-IP threshold ──────────────────

    proptest! {
        /// **Property 14: Rate-limiter enforces per-IP threshold**
        ///
        /// **Validates: Requirements 1.12, 19.10**
        ///
        /// For any arbitrary window-start timestamp and any request count
        /// `n ∈ [1, MAX_REQUESTS]`, all `n` requests within the same 60-second
        /// window must be allowed.  The `(n+1)`-th request in the same window
        /// must be rate-limited (`allowed = false`).
        ///
        /// Additionally, after the window expires the counter resets and the
        /// next request is always allowed.
        #[test]
        fn prop_rate_limit_threshold(
            window_start in 0_u64..=u64::MAX / 2,
            // n requests that should all be allowed (1 to MAX_REQUESTS)
            n in 1_u32..=MAX_REQUESTS,
        ) {
            let now = window_start;

            // Simulate `n` requests inside the window.
            let mut entry: Option<RateEntry> = None;
            for i in 1..=n {
                let (new_entry, allowed) = rate_limit_check_inner(entry, now);
                prop_assert!(
                    allowed,
                    "request {} (of {}) must be allowed within the window (count = {})",
                    i, n, new_entry.count
                );
                entry = Some(new_entry);
            }

            // Pad up to exactly MAX_REQUESTS if n < MAX_REQUESTS so the
            // (n+1)-th request can cross the threshold.
            let mut padded_entry = entry.clone();
            for _ in (n + 1)..=MAX_REQUESTS {
                let (new_entry, _) = rate_limit_check_inner(padded_entry, now);
                padded_entry = Some(new_entry);
            }

            // The very next request (MAX_REQUESTS + 1) must be rate-limited.
            let (_, limited) = rate_limit_check_inner(padded_entry, now);
            prop_assert!(
                !limited,
                "request {} must be rate-limited within the same window",
                MAX_REQUESTS + 1
            );

            // After window expiry, the first request must be allowed again.
            let expired_now = window_start.saturating_add(WINDOW_SECS);
            let (reset_entry, allowed_after_expiry) = rate_limit_check_inner(entry, expired_now);
            prop_assert!(
                allowed_after_expiry,
                "first request after window expiry must be allowed (count = {})",
                reset_entry.count
            );
            prop_assert_eq!(
                reset_entry.count,
                1,
                "counter must reset to 1 after window expiry"
            );
        }
    }
}
