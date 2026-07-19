// JWT claims models for session and QR verification tokens.

use serde::{Deserialize, Serialize};

use super::staff::Role;

/// Claims embedded in the staff session JWT (stored in `session` HttpOnly cookie).
///
/// Field names match the standard JWT claim names where applicable (`sub`, `exp`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StaffClaims {
    /// Subject — the staff member's unique identifier (`staff.id`).
    pub sub: String,
    /// Corporate email address.
    pub email: String,
    /// RBAC role (serialised as PascalCase string via `Role`'s serde impl).
    pub role: Role,
    /// Whether the staff member has completed the onboarding wizard.
    pub onboarded: bool,
    /// Expiry timestamp (Unix seconds).
    pub exp: i64,
}

/// Claims embedded in the short-lived QR corporate identity token.
///
/// Deliberately carries **no PII** beyond the opaque `sub` identifier.
/// Verified by `GET /api/verify` to display public identity information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QrClaims {
    /// Subject — the staff member's unique identifier (`staff.id`).
    pub sub: String,
    /// Expiry timestamp (Unix seconds); tokens are valid for 300 seconds.
    pub exp: i64,
    /// Issuer identifier — must equal `"ezo-identity"`.
    pub iss: String,
}
