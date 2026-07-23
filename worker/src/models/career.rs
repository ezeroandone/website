// Career listing and application domain models.

use serde::{Deserialize, Serialize};

/// Status of a submitted job application.
///
/// Reflects the state machine defined in design section 9.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ApplicationStatus {
    /// Initial state when a candidate submits an application.
    Applied,
    /// Application is being actively reviewed / candidate is interviewing.
    Interviewing,
    /// An offer has been extended to the candidate.
    Offered,
    /// Candidate has accepted the offer and been hired.
    Hired,
    /// Application has been rejected at any stage.
    Rejected,
}

/// A published job opening.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Career {
    /// Unique identifier (UUID hex).
    pub id: String,
    /// URL-safe slug for the career page.
    pub slug: String,
    /// Job title.
    pub title: String,
    /// Full job description in Markdown.
    pub description_md: String,
    /// Department or team name.
    pub department: String,
    /// Employment type: `Full-Time`, `Part-Time`, `Contract`, or `Internship`.
    #[serde(rename = "type")]
    pub career_type: String,
    /// Whether this listing is accepting applications.
    pub active: bool,
    /// Record creation timestamp (Unix seconds).
    pub created_at: i64,
}

/// Request body for creating a career listing (admin only).
#[derive(Debug, Clone, Deserialize)]
pub struct CareerCreate {
    /// URL-safe slug (must be unique).
    pub slug: String,
    /// Job title.
    pub title: String,
    /// Full Markdown job description.
    #[serde(default)]
    pub description_md: String,
    /// Department or team.
    #[serde(default)]
    pub department: String,
    /// Employment type.
    #[serde(rename = "type")]
    pub career_type: String,
    /// Whether the listing is initially active.
    #[serde(default = "default_true")]
    pub active: bool,
}

/// Request body for updating a career listing (admin only).
///
/// All fields optional — only present fields are updated.
#[derive(Debug, Clone, Deserialize)]
pub struct CareerUpdate {
    pub title: Option<String>,
    pub description_md: Option<String>,
    pub department: Option<String>,
    #[serde(rename = "type")]
    pub career_type: Option<String>,
    pub active: Option<bool>,
}

/// A job application submitted by a candidate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Application {
    /// Unique identifier (UUID hex).
    pub id: String,
    /// FK reference to `career.id`.
    pub career_id: String,
    /// Candidate's full name.
    pub applicant_name: String,
    /// Candidate's email address.
    pub applicant_email: String,
    /// Optional cover letter text.
    pub cover_letter: String,
    /// Current application status.
    pub status: ApplicationStatus,
    /// Application submission timestamp (Unix seconds).
    pub applied_at: i64,
    /// Last status update timestamp (Unix seconds).
    pub updated_at: i64,
}

/// A single document uploaded as part of an application (tracked in D1).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDocument {
    /// Unique identifier.
    pub id: String,
    /// FK reference to `application.id`.
    pub application_id: String,
    /// R2 object key (path) where the file is stored.
    pub r2_key: String,
    /// Original filename provided by the uploader.
    pub original_filename: String,
    /// Detected MIME type.
    pub mime_type: String,
    /// Upload timestamp (Unix seconds).
    pub uploaded_at: i64,
}

/// Metadata returned to admins when listing application documents.
///
/// Omits the raw `r2_key` to avoid leaking internal storage paths.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMeta {
    /// Document row identifier.
    pub id: String,
    /// FK reference to `application.id`.
    pub application_id: String,
    /// Original filename provided by the uploader.
    pub original_filename: String,
    /// Detected MIME type.
    pub mime_type: String,
    /// Upload timestamp (Unix seconds).
    pub uploaded_at: i64,
}

/// A media asset (image) stored in R2 and tracked in D1.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaAsset {
    /// Unique identifier.
    pub id: String,
    /// R2 object key (path).
    pub r2_key: String,
    /// Context type: `avatar`, `post_cover`, `post_media`, or `career_hero`.
    pub context_type: String,
    /// ID of the entity this asset belongs to (staff ID, post ID, etc.).
    pub context_id: String,
    /// Public CDN URL for the asset.
    pub public_url: String,
    /// FK reference to `staff.id` of the uploader.
    pub uploaded_by: Option<String>,
    /// Upload timestamp (Unix seconds).
    pub uploaded_at: i64,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Default value helper for serde — returns `true`.
fn default_true() -> bool {
    true
}
