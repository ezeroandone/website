// Content (posts) domain models.

use serde::{Deserialize, Serialize};

/// Discriminator for the type of content post.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PostType {
    Insight,
    Work,
    Capability,
}

/// Full post record as stored in D1 (includes new metadata columns).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    #[serde(rename = "type")]
    pub post_type: PostType,
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub body_md: String,
    pub author_id: Option<String>,
    pub published_at: Option<i64>,
    pub updated_at: i64,
    pub published: bool,
    // Rich metadata (0002_post_meta)
    #[serde(default)]
    pub featured_image_url: String,
    #[serde(default)]
    pub category: String,
    /// Comma-separated tags
    #[serde(default)]
    pub tags: String,
    // Work-specific
    #[serde(default)]
    pub project_type: String,
    /// Comma-separated technology names
    #[serde(default)]
    pub technologies: String,
    // Capability-specific
    #[serde(default)]
    pub material_icon: String,
}

/// Lightweight summary returned in list endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostSummary {
    pub slug: String,
    #[serde(rename = "type")]
    pub post_type: PostType,
    pub title: String,
    pub summary: String,
    pub author_username: Option<String>,
    pub published_at: Option<i64>,
    pub featured_image_url: String,
    pub category: String,
    pub tags: String,
    pub material_icon: String,
}

/// Request body for creating a new post (admin only).
#[derive(Debug, Clone, Deserialize)]
pub struct PostCreate {
    #[serde(rename = "type")]
    pub post_type: PostType,
    pub title: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub body_md: String,
    pub slug: Option<String>,
    #[serde(default)]
    pub featured_image_url: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub tags: String,
    #[serde(default)]
    pub project_type: String,
    #[serde(default)]
    pub technologies: String,
    #[serde(default)]
    pub material_icon: String,
}

/// Request body for updating an existing post (admin only).
#[derive(Debug, Clone, Deserialize)]
pub struct PostUpdate {
    pub title: Option<String>,
    pub summary: Option<String>,
    pub body_md: Option<String>,
    pub slug: Option<String>,
    pub published: Option<bool>,
    pub featured_image_url: Option<String>,
    pub category: Option<String>,
    pub tags: Option<String>,
    pub project_type: Option<String>,
    pub technologies: Option<String>,
    pub material_icon: Option<String>,
}

/// A team member linked to a post (staff or external contractor).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTeamMember {
    pub id: String,
    pub post_id: String,
    pub staff_id: Option<String>,
    // Populated via JOIN when staff_id is set
    pub staff_name: Option<String>,
    pub staff_username: Option<String>,
    pub staff_avatar_url: Option<String>,
    // External contractor fields
    #[serde(default)]
    pub ext_name: String,
    #[serde(default)]
    pub ext_role: String,
    #[serde(default)]
    pub ext_url: String,
    pub sort_order: i64,
}

/// A client logo / brand worked with.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientLogo {
    pub id: String,
    pub name: String,
    pub logo_url: String,
    #[serde(default)]
    pub website_url: String,
    pub sort_order: i64,
    pub active: bool,
    pub created_at: i64,
}

/// Request body for creating/updating a client logo.
#[derive(Debug, Clone, Deserialize)]
pub struct ClientLogoUpsert {
    pub name: String,
    pub logo_url: String,
    #[serde(default)]
    pub website_url: String,
    #[serde(default)]
    pub sort_order: i64,
    #[serde(default = "default_true")]
    pub active: bool,
}

fn default_true() -> bool { true }
