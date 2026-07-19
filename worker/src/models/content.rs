// Content (posts) domain models.

use serde::{Deserialize, Serialize};

/// Discriminator for the type of content post.
///
/// Serialises as lowercase strings to match the D1 CHECK constraint:
/// `('insight','work','capability')`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PostType {
    Insight,
    Work,
    Capability,
}

/// Full post record as stored in D1.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Post {
    /// Unique identifier (UUID hex).
    pub id: String,
    /// Content type discriminator.
    #[serde(rename = "type")]
    pub post_type: PostType,
    /// URL-safe slug (unique per type).
    pub slug: String,
    /// Post title.
    pub title: String,
    /// Short summary / excerpt for list views.
    pub summary: String,
    /// Full Markdown body.
    pub body_md: String,
    /// FK reference to `staff.id` of the post author.
    pub author_id: Option<String>,
    /// Publication timestamp (Unix seconds); `None` if unpublished.
    pub published_at: Option<i64>,
    /// Last update timestamp (Unix seconds).
    pub updated_at: i64,
    /// Whether the post is publicly visible.
    pub published: bool,
}

/// Lightweight summary returned in list endpoints (e.g. `GET /api/insights`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostSummary {
    /// URL-safe slug.
    pub slug: String,
    /// Content type.
    #[serde(rename = "type")]
    pub post_type: PostType,
    /// Post title.
    pub title: String,
    /// Short summary / excerpt.
    pub summary: String,
    /// Username of the author (joined from `staff`).
    pub author_username: Option<String>,
    /// Publication timestamp (Unix seconds).
    pub published_at: Option<i64>,
}

/// Request body for creating a new post (admin only).
#[derive(Debug, Clone, Deserialize)]
pub struct PostCreate {
    /// Content type.
    #[serde(rename = "type")]
    pub post_type: PostType,
    /// Post title (required).
    pub title: String,
    /// Short summary.
    #[serde(default)]
    pub summary: String,
    /// Full Markdown body.
    #[serde(default)]
    pub body_md: String,
    /// URL-safe slug; auto-derived from `title` if absent.
    pub slug: Option<String>,
}

/// Request body for updating an existing post (admin only).
///
/// All fields are optional — only present fields are updated.
#[derive(Debug, Clone, Deserialize)]
pub struct PostUpdate {
    /// Updated title.
    pub title: Option<String>,
    /// Updated summary.
    pub summary: Option<String>,
    /// Updated Markdown body.
    pub body_md: Option<String>,
    /// Updated slug (must remain unique within the same post type).
    pub slug: Option<String>,
    /// Toggle publication state.
    pub published: Option<bool>,
}
