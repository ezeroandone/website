#![allow(dead_code, unused_imports)]

//! JWT sign / verify using HMAC-SHA256 (HS256).
//! Signs session JWTs with JWT_SECRET and QR tokens with QR_SECRET.

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use worker::js_sys;

use crate::models::session::{QrClaims, StaffClaims};
use crate::router::{AuthError, WorkerError};

type HmacSha256 = Hmac<Sha256>;

/// Returns the current Unix time in seconds using the JS `Date` API.
/// This is the only way to get wall-clock time in `wasm32-unknown-unknown`.
fn now_unix_seconds() -> i64 {
    (js_sys::Date::now() / 1000.0) as i64
}

/// Constant-time byte-slice comparison to prevent timing attacks.
/// Returns `true` only when both slices have the same length and content.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff: u8 = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// Compute HMAC-SHA256 and return the raw bytes.
fn hmac_sha256(secret: &[u8], message: &[u8]) -> Vec<u8> {
    let mut mac =
        HmacSha256::new_from_slice(secret).expect("HMAC accepts any key length");
    mac.update(message);
    mac.finalize().into_bytes().to_vec()
}

/// Base64url-encode a byte slice without padding.
fn b64url_encode(data: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(data)
}

/// Base64url-decode a string without padding.
fn b64url_decode(s: &str) -> Result<Vec<u8>, WorkerError> {
    URL_SAFE_NO_PAD
        .decode(s)
        .map_err(|_| WorkerError::Auth(AuthError::InvalidToken))
}

// ── StaffClaims JWT ───────────────────────────────────────────────────────────

/// Sign a `StaffClaims` struct into a compact HS256 JWT string.
///
/// Returns `Err(WorkerError::Internal)` only if JSON serialisation fails
/// (should never happen with well-formed structs).
pub fn sign_jwt(claims: &StaffClaims, secret: &[u8]) -> Result<String, WorkerError> {
    let header_json = r#"{"alg":"HS256","typ":"JWT"}"#;
    let header = b64url_encode(header_json.as_bytes());

    let payload_json = serde_json::to_string(claims)
        .map_err(|e| WorkerError::Internal(format!("JWT serialize: {e}")))?;
    let payload = b64url_encode(payload_json.as_bytes());

    let signing_input = format!("{header}.{payload}");
    let sig = b64url_encode(&hmac_sha256(secret, signing_input.as_bytes()));

    Ok(format!("{signing_input}.{sig}"))
}

/// Inner verify function that accepts an explicit `now` timestamp (Unix seconds).
///
/// Extracted for testability: native-host unit tests cannot call `js_sys::Date::now()`,
/// so they pass a known timestamp directly. The public `verify_jwt` delegates here
/// after resolving wall-clock time from the JS runtime.
pub(crate) fn verify_jwt_at(
    token: &str,
    secret: &[u8],
    now: i64,
) -> Result<StaffClaims, WorkerError> {
    let parts: Vec<&str> = token.splitn(3, '.').collect();
    if parts.len() != 3 {
        return Err(WorkerError::Auth(AuthError::InvalidToken));
    }

    let signing_input = format!("{}.{}", parts[0], parts[1]);
    let expected_sig = hmac_sha256(secret, signing_input.as_bytes());
    let provided_sig =
        b64url_decode(parts[2]).map_err(|_| WorkerError::Auth(AuthError::InvalidToken))?;

    if !constant_time_eq(&expected_sig, &provided_sig) {
        return Err(WorkerError::Auth(AuthError::InvalidToken));
    }

    let payload_bytes = b64url_decode(parts[1])?;
    let claims: StaffClaims = serde_json::from_slice(&payload_bytes)
        .map_err(|_| WorkerError::Auth(AuthError::InvalidToken))?;

    if claims.exp <= now {
        return Err(WorkerError::Auth(AuthError::Expired));
    }

    Ok(claims)
}

/// Verify an HS256 JWT and return the decoded `StaffClaims`.
///
/// Errors:
/// - `WorkerError::Auth(AuthError::InvalidToken)` — malformed, bad signature, or parse failure
/// - `WorkerError::Auth(AuthError::Expired)` — token has passed its `exp`
pub fn verify_jwt(token: &str, secret: &[u8]) -> Result<StaffClaims, WorkerError> {
    verify_jwt_at(token, secret, now_unix_seconds())
}

// ── QrClaims JWT ─────────────────────────────────────────────────────────────

/// Build and sign a short-lived (300 s) QR identity JWT for `staff_id`.
///
/// Panics only if JSON serialisation fails, which cannot happen for this
/// well-known struct layout.
pub fn generate_identity_jwt(staff_id: &str, qr_secret: &[u8]) -> String {
    let claims = QrClaims {
        sub: staff_id.to_string(),
        exp: now_unix_seconds() + 300,
        iss: "ezo-identity".to_string(),
    };

    let header_json = r#"{"alg":"HS256","typ":"JWT"}"#;
    let header = b64url_encode(header_json.as_bytes());

    let payload_json =
        serde_json::to_string(&claims).expect("QrClaims serialisation must not fail");
    let payload = b64url_encode(payload_json.as_bytes());

    let signing_input = format!("{header}.{payload}");
    let sig = b64url_encode(&hmac_sha256(qr_secret, signing_input.as_bytes()));

    format!("{signing_input}.{sig}")
}

/// Verify a QR identity JWT and return the decoded `QrClaims`.
///
/// Errors:
/// - `WorkerError::Auth(AuthError::InvalidToken)` — bad signature, malformed, or wrong issuer
/// - `WorkerError::Auth(AuthError::Expired)` — token has passed its `exp`
pub fn verify_identity_jwt(token: &str, qr_secret: &[u8]) -> Result<QrClaims, WorkerError> {
    let parts: Vec<&str> = token.splitn(3, '.').collect();
    if parts.len() != 3 {
        return Err(WorkerError::Auth(AuthError::InvalidToken));
    }

    let signing_input = format!("{}.{}", parts[0], parts[1]);
    let expected_sig = hmac_sha256(qr_secret, signing_input.as_bytes());
    let provided_sig =
        b64url_decode(parts[2]).map_err(|_| WorkerError::Auth(AuthError::InvalidToken))?;

    if !constant_time_eq(&expected_sig, &provided_sig) {
        return Err(WorkerError::Auth(AuthError::InvalidToken));
    }

    let payload_bytes = b64url_decode(parts[1])?;
    let claims: QrClaims = serde_json::from_slice(&payload_bytes)
        .map_err(|_| WorkerError::Auth(AuthError::InvalidToken))?;

    if claims.exp <= now_unix_seconds() {
        return Err(WorkerError::Auth(AuthError::Expired));
    }

    if claims.iss != "ezo-identity" {
        return Err(WorkerError::Auth(AuthError::InvalidToken));
    }

    Ok(claims)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::staff::Role;
    use proptest::prelude::*;

    // ── Helpers / strategies ──────────────────────────────────────────────────

    /// A fixed "current time" used for the expiry check in all tests.
    ///
    /// Set to the Unix timestamp for 2020-01-01T00:00:00Z so that any `exp`
    /// value of `i64::MAX / 2` (≈ year 146 138 512) is safely in the future
    /// without needing the JS runtime.
    const TEST_NOW: i64 = 1_577_836_800; // 2020-01-01 UTC

    /// Expiry timestamp used for all generated claims: far enough in the future
    /// that `exp > TEST_NOW` always holds, but well within `i64` range.
    const FAR_FUTURE_EXP: i64 = i64::MAX / 2;

    /// Strategy producing a non-empty ASCII-printable string bounded to a
    /// reasonable length (mirrors the convention in `models/tests.rs`).
    fn arb_string() -> impl Strategy<Value = String> {
        "[a-zA-Z0-9 _\\-\\.@/]{1,64}".prop_map(|s| s)
    }

    /// Strategy for `Role`.
    fn arb_role() -> impl Strategy<Value = Role> {
        prop_oneof![
            Just(Role::Public),
            Just(Role::Staff),
            Just(Role::Admin),
            Just(Role::SuperAdmin),
        ]
    }

    /// Strategy for a `StaffClaims` with `exp` set to `FAR_FUTURE_EXP`.
    ///
    /// The `exp` field is pinned to a large future value so the expiry check
    /// inside `verify_jwt_at` always passes when called with `TEST_NOW`.
    fn arb_staff_claims() -> impl Strategy<Value = StaffClaims> {
        (
            arb_string(), // sub
            arb_string(), // email
            arb_role(),
            any::<bool>(), // onboarded
        )
            .prop_map(|(sub, email, role, onboarded)| StaffClaims {
                sub,
                email,
                role,
                onboarded,
                exp: FAR_FUTURE_EXP,
            })
    }

    /// Strategy for a secret of 32–64 bytes drawn from the full byte range.
    fn arb_secret() -> impl Strategy<Value = Vec<u8>> {
        proptest::collection::vec(any::<u8>(), 32..=64)
    }

    // ── Property 3: JWT round-trip fidelity ───────────────────────────────────

    proptest! {
        /// **Property 3: JWT round-trip fidelity**
        ///
        /// **Validates: Requirements 1.9, 2.1, 2.2, 2.3**
        ///
        /// For arbitrary `StaffClaims` with `exp` set to a large future value
        /// and a randomly chosen 32–64 byte secret:
        ///
        ///   `verify_jwt_at(sign_jwt(claims, secret), secret, TEST_NOW) == Ok(claims)`
        ///
        /// This guarantees that the sign → verify pipeline is lossless: the
        /// header and payload are base64url-encoded, the HMAC is computed over
        /// `header.payload`, and the verify function reconstructs the original
        /// struct without any field being dropped or mutated.
        #[test]
        fn prop_jwt_roundtrip(
            claims in arb_staff_claims(),
            secret in arb_secret(),
        ) {
            let token = sign_jwt(&claims, &secret)
                .expect("sign_jwt must not fail for well-formed StaffClaims");

            let recovered = verify_jwt_at(&token, &secret, TEST_NOW)
                .expect("verify_jwt_at must succeed for a freshly signed token with a future exp");

            prop_assert_eq!(
                claims,
                recovered,
                "JWT round-trip must produce an equal StaffClaims"
            );
        }

        /// **Property 3 (negative): wrong secret returns Err**
        ///
        /// **Validates: Requirements 2.2**
        ///
        /// If `verify_jwt_at` is called with a different secret from the one
        /// used to sign the token, it must return `Err` (specifically
        /// `InvalidToken` from the HMAC mismatch).
        ///
        /// The strategy generates two independent secrets; if they happen to be
        /// equal we skip the case (prop_assume) — proptest will regenerate.
        #[test]
        fn prop_jwt_wrong_secret_returns_err(
            claims in arb_staff_claims(),
            sign_secret   in arb_secret(),
            verify_secret in arb_secret(),
        ) {
            prop_assume!(sign_secret != verify_secret);

            let token = sign_jwt(&claims, &sign_secret)
                .expect("sign_jwt must not fail");

            let result = verify_jwt_at(&token, &verify_secret, TEST_NOW);

            prop_assert!(
                result.is_err(),
                "verify_jwt_at with a different secret must return Err, but got Ok"
            );
        }
    }

    // ── Deterministic sanity checks ───────────────────────────────────────────

    /// Verify that the JWT contains exactly three base64url segments separated
    /// by dots, matching the compact serialisation format required by RFC 7519.
    #[test]
    fn jwt_has_three_segments() {
        let claims = StaffClaims {
            sub: "user-001".into(),
            email: "test@example.com".into(),
            role: Role::Staff,
            onboarded: true,
            exp: FAR_FUTURE_EXP,
        };
        let secret = b"a-very-secret-key-that-is-32-byt";
        let token = sign_jwt(&claims, secret).unwrap();
        assert_eq!(token.split('.').count(), 3, "JWT must have exactly three dot-separated segments");
    }

    /// Verify the round-trip for a single known `StaffClaims` value.
    #[test]
    fn jwt_roundtrip_known_value() {
        let claims = StaffClaims {
            sub: "abc-123".into(),
            email: "staff@ezo.io".into(),
            role: Role::Admin,
            onboarded: false,
            exp: FAR_FUTURE_EXP,
        };
        let secret = b"super-secret-key-for-testing-here";
        let token = sign_jwt(&claims, secret).unwrap();
        let recovered = verify_jwt_at(&token, secret, TEST_NOW).unwrap();
        assert_eq!(claims, recovered);
    }

    /// Verify that a tampered payload segment causes `verify_jwt_at` to return `Err`.
    #[test]
    fn jwt_tampered_payload_returns_err() {
        let claims = StaffClaims {
            sub: "tamper-test".into(),
            email: "x@ezo.io".into(),
            role: Role::Staff,
            onboarded: true,
            exp: FAR_FUTURE_EXP,
        };
        let secret = b"another-secret-key-exactly-32-by";
        let token = sign_jwt(&claims, secret).unwrap();

        // Replace the payload segment with a different base64url string.
        let parts: Vec<&str> = token.splitn(3, '.').collect();
        let tampered_payload = "dGFtcGVyZWQ"; // base64url("tampered")
        let tampered_token = format!("{}.{}.{}", parts[0], tampered_payload, parts[2]);

        let result = verify_jwt_at(&tampered_token, secret, TEST_NOW);
        assert!(result.is_err(), "tampered payload must cause verify to return Err");
    }

    // ── Property 12: JWT signature tamper-detection ───────────────────────────

    /// Base64url alphabet (URL-safe, no padding): A-Z, a-z, 0-9, -, _
    const B64URL_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

    proptest! {
        /// **Property 12: JWT signature tamper-detection**
        ///
        /// **Validates: Requirements 2.2, 19.2**
        ///
        /// For any valid JWT and any mutation of a single character in the
        /// signature segment to a *different* valid base64url character, the
        /// verify function must return `Err`.
        ///
        /// Strategy:
        /// 1. Generate arbitrary `StaffClaims` + 32–64 byte secret.
        /// 2. Sign to obtain a valid JWT.
        /// 3. Split on `.`; take the signature segment (3rd part).
        /// 4. Use proptest to pick a byte index within the sig and a
        ///    replacement character from the base64url alphabet that differs
        ///    from the original.  If the sig is empty (cannot happen for
        ///    HMAC-SHA256) or every character in the alphabet equals the
        ///    original at the chosen position, skip with `prop_assume!`.
        /// 5. Reconstruct the token and assert `verify_jwt_at` returns `Err`.
        #[test]
        fn prop_jwt_tampered_signature_returns_err(
            claims  in arb_staff_claims(),
            secret  in arb_secret(),
            // We pick the byte-index and a "shift" into the alphabet
            // independently so proptest can shrink both dimensions.
            byte_idx_raw in any::<usize>(),
            alt_char_idx in any::<usize>(),
        ) {
            let token = sign_jwt(&claims, &secret)
                .expect("sign_jwt must not fail for well-formed StaffClaims");

            // Split into exactly three parts.
            let parts: Vec<&str> = token.splitn(3, '.').collect();
            prop_assume!(parts.len() == 3);

            let sig = parts[2];
            prop_assume!(!sig.is_empty());

            // Clamp the byte index to the sig length.
            let idx = byte_idx_raw % sig.len();
            let original_byte = sig.as_bytes()[idx];

            // Pick a replacement character from the base64url alphabet that
            // is different from the original byte at this position.
            // We iterate through `alt_char_idx % 64` offsets until we find
            // a character that differs.
            let alphabet_len = B64URL_CHARS.len();
            let start = alt_char_idx % alphabet_len;
            let replacement = {
                let mut found = None;
                for offset in 0..alphabet_len {
                    let candidate = B64URL_CHARS[(start + offset) % alphabet_len];
                    if candidate != original_byte {
                        found = Some(candidate);
                        break;
                    }
                }
                found
            };

            // If every character in the alphabet is the same as the original
            // (impossible in practice for base64url), skip this case.
            prop_assume!(replacement.is_some());
            let replacement = replacement.unwrap();

            // Build the tampered signature segment.
            let mut tampered_sig_bytes = sig.as_bytes().to_vec();
            tampered_sig_bytes[idx] = replacement;
            let tampered_sig = String::from_utf8(tampered_sig_bytes)
                .expect("base64url characters are always valid UTF-8");

            let tampered_token = format!("{}.{}.{}", parts[0], parts[1], tampered_sig);

            let result = verify_jwt_at(&tampered_token, &secret, TEST_NOW);

            prop_assert!(
                result.is_err(),
                "verify_jwt_at must return Err for a JWT with a tampered signature segment, \
                 but got Ok; original sig byte at idx {}: {:?}, replaced with: {:?}",
                idx,
                original_byte as char,
                replacement as char,
            );
        }
    }

    /// Verify that an expired token (exp <= now) returns `Err(Expired)`.
    #[test]
    fn jwt_expired_token_returns_err() {
        let claims = StaffClaims {
            sub: "expired-user".into(),
            email: "old@ezo.io".into(),
            role: Role::Public,
            onboarded: false,
            exp: 1_000, // far in the past relative to TEST_NOW
        };
        let secret = b"expiry-test-secret-key-32-bytes!!";
        let token = sign_jwt(&claims, secret).unwrap();
        let result = verify_jwt_at(&token, secret, TEST_NOW);
        assert!(
            matches!(result, Err(WorkerError::Auth(AuthError::Expired))),
            "expired token must return Err(Auth(Expired)), got: {:?}",
            result
        );
    }
}
