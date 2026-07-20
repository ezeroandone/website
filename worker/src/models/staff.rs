// Staff domain models.

use serde::{Deserialize, Deserializer, Serialize};

/// Deserialise a D1 INTEGER/FLOAT column into a Rust `bool`.
///
/// D1 returns SQLite booleans as floating-point (0.0 / 1.0) rather than
/// true/false. Using `#[serde(untagged)]` is unreliable here because serde
/// tries `bool` first and surfaces its error before reaching `f64`. Instead
/// we deserialise into a `serde_json::Value` and match on the actual variant.
fn deserialize_bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    match serde_json::Value::deserialize(deserializer)? {
        serde_json::Value::Bool(b)   => Ok(b),
        serde_json::Value::Number(n) => {
            // Covers both integer (0/1) and float (0.0/1.0) cases.
            Ok(n.as_f64().map(|f| f != 0.0).unwrap_or(false))
        }
        other => Err(D::Error::custom(format!(
            "expected bool or number for boolean column, got: {other}"
        ))),
    }
}

/// Access role granted to a staff member.
///
/// Uses `#[repr(u8)]` so discriminant values are stable and orderable.
/// Serialises/deserialises as PascalCase strings (e.g. `"SuperAdmin"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
#[serde(rename_all = "PascalCase")]
pub enum Role {
    /// Public / unauthenticated access level.
    Public = 1,
    /// Regular staff member.
    Staff = 2,
    /// Administrator with elevated permissions.
    Admin = 3,
    /// Highest privilege level.
    SuperAdmin = 4,
}

impl PartialOrd for Role {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Role {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

/// Lifecycle state of a staff record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LifecycleStatus {
    /// Staff is in their probationary period.
    Probation,
    /// Staff has been confirmed as a permanent employee.
    Confirmed,
    /// Staff is no longer active.
    Inactive,
}

/// Full staff record (internal use — never serialised directly to public API).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Staff {
    /// Unique identifier (UUID hex).
    pub id: String,
    /// Corporate email address.
    pub email: String,
    /// Unique username / URL slug for public profile.
    pub username: String,
    /// Display name.
    pub name: String,
    /// Job title / position.
    pub job_title: String,
    /// Short biography.
    pub bio: String,
    /// Public URL of the staff avatar stored in R2.
    pub avatar_url: String,
    /// RBAC role.
    pub role: Role,
    /// Whether the staff member has completed the onboarding wizard.
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub onboarding_completed: bool,
    /// PEM-encoded Ed25519/ECDSA public key for QR identity signing.
    pub signing_public_key: Option<String>,
    /// Record creation timestamp (Unix seconds).
    pub created_at: i64,
    /// Last update timestamp (Unix seconds).
    pub updated_at: i64,
}

/// Lifecycle record for a staff member.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StaffLifecycle {
    /// Unique identifier.
    pub id: String,
    /// Foreign key reference to `staff.id`.
    pub staff_id: String,
    /// Current lifecycle status.
    pub status: LifecycleStatus,
    /// Probation period start timestamp (Unix seconds).
    pub probation_start: i64,
    /// Probation period end timestamp (Unix seconds); `None` if still in probation.
    pub probation_end: Option<i64>,
    /// Timestamp when staff was confirmed (Unix seconds); `None` until confirmed.
    pub confirmed_at: Option<i64>,
}

/// Read-only public profile returned by `GET /api/team/:username`.
///
/// Intentionally omits: `email`, `role`, `id`, and `signing_public_key`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffPublicProfile {
    /// URL-safe username.
    pub username: String,
    /// Display name.
    pub name: String,
    /// Job title shown on the team page.
    pub job_title: String,
    /// Short biography.
    pub bio: String,
    /// Public CDN URL of the avatar image.
    pub avatar_url: String,
}

/// Full staff record as visible to admins.
///
/// Includes all `Staff` fields except `signing_public_key` (sensitive cryptographic material).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffAdmin {
    pub id: String,
    pub email: String,
    pub username: String,
    pub name: String,
    pub job_title: String,
    pub bio: String,
    pub avatar_url: String,
    pub role: Role,
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub onboarding_completed: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Social / professional profile links and ordering metadata for the team page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamProfile {
    /// Foreign key reference to `staff.id`.
    pub id: String,
    /// LinkedIn profile URL.
    pub linkedin: Option<String>,
    /// GitHub profile URL.
    pub github: Option<String>,
    /// Twitter/X handle or URL.
    pub twitter: Option<String>,
    /// Comma-separated skill tags (stored as TEXT in D1).
    pub skills: Option<String>,
    /// Ascending sort order for the team directory page.
    pub order_rank: i64,
}
