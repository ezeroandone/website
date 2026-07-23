#![allow(dead_code, unused_imports)]

//! Magic-link token generation.
//! Produces a cryptographically random, URL-safe token and stores it in KV
//! with an expiry.  Verifies the token on callback and marks it consumed.

use worker::js_sys::{Array, Function, Object, Reflect, Uint8Array};
use worker::wasm_bindgen::JsCast;

/// Encode a byte slice as a lowercase hex string.
///
/// Each byte becomes exactly two lowercase hex digits, so 32 bytes → 64 chars.
/// This is the pure-Rust encoding logic that `generate_secure_token` applies to
/// the bytes produced by the Web Crypto API.  Exposed as `pub` so tests can
/// invoke it directly with deterministic or random byte inputs.
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Generate a cryptographically secure 32-byte random token encoded as a
/// 64-character lowercase hex string.
///
/// Uses the Web Crypto API (`crypto.getRandomValues`) available in the
/// Cloudflare Workers runtime as a global `crypto` object.
pub fn generate_secure_token() -> String {
    // In Cloudflare Workers `crypto` is a global — not on `window`.
    let global: Object = worker::js_sys::global().unchecked_into();

    let crypto_val = Reflect::get(&global, &"crypto".into())
        .expect("crypto global must exist in Workers runtime");

    let buf = Uint8Array::new_with_length(32);

    // Call crypto.getRandomValues(buf)
    let get_random_values: Function = Reflect::get(&crypto_val, &"getRandomValues".into())
        .expect("getRandomValues must exist on crypto")
        .unchecked_into();

    let args = Array::of1(&buf);
    Reflect::apply(&get_random_values, &crypto_val, &args)
        .expect("getRandomValues must succeed");

    bytes_to_hex(&buf.to_vec())
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::bytes_to_hex;
    use rand::RngCore;
    use std::collections::HashSet;

    /// Helper: generate one token using `rand::thread_rng()` as the CSPRNG.
    ///
    /// This mirrors the production logic of `generate_secure_token` — 32 random
    /// bytes encoded via `bytes_to_hex` — but uses `rand` instead of `web_sys`
    /// so the test can run on a native host (no browser runtime required).
    fn generate_test_token() -> String {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        bytes_to_hex(&bytes)
    }

    /// **Validates: Requirements 1.3**
    ///
    /// Property 2 (partial): token uniqueness and format.
    ///
    /// Generates 1 000 tokens and asserts:
    /// 1. All tokens are distinct (no collisions in a set of 1 000).
    /// 2. Every token is exactly 64 characters long.
    /// 3. Every character in every token is a lowercase hex digit (`[0-9a-f]`).
    #[test]
    fn prop_token_uniqueness_and_format() {
        const N: usize = 1_000;
        let mut seen: HashSet<String> = HashSet::with_capacity(N);

        for _ in 0..N {
            let token = generate_test_token();

            // --- format: exactly 64 characters ---
            assert_eq!(
                token.len(),
                64,
                "token must be exactly 64 chars, got {} chars: {:?}",
                token.len(),
                token
            );

            // --- format: only lowercase hex digits ---
            let all_hex = token.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f'));
            assert!(
                all_hex,
                "token must match ^[0-9a-f]{{64}}$, got: {:?}",
                token
            );

            // --- uniqueness ---
            let is_new = seen.insert(token.clone());
            assert!(
                is_new,
                "duplicate token detected: {:?} (after {} unique tokens)",
                token,
                seen.len() - 1
            );
        }

        assert_eq!(seen.len(), N, "expected {N} distinct tokens, got {}", seen.len());
    }

    /// Sanity-check for `bytes_to_hex`: known input → known output.
    #[test]
    fn bytes_to_hex_known_values() {
        assert_eq!(bytes_to_hex(&[0x00]), "00");
        assert_eq!(bytes_to_hex(&[0xff]), "ff");
        assert_eq!(bytes_to_hex(&[0xde, 0xad, 0xbe, 0xef]), "deadbeef");
        assert_eq!(bytes_to_hex(&[0u8; 32]).len(), 64);
    }
}
