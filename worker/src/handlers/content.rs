#![allow(dead_code, unused_imports)]

//! Public content handlers — no authentication required.
//!
//! GET /api/insights             — list published insight posts
//! GET /api/insights/:slug       — get single insight post
//! GET /api/work                 — list published work posts
//! GET /api/work/:slug           — get single work post
//! GET /api/capabilities         — list published capability posts
//! GET /api/capabilities/:slug   — get single capability post
//!
//! All list endpoints use `get_cached_or_fetch` with a 300-second TTL.
//! All single-post endpoints cache under `post:{slug}` with 300-second TTL.
//!
//! Requirements: 10.1–10.4, 13.1, 13.4–13.6

use std::result::Result as StdResult;

use serde::{Deserialize, Serialize};
use worker::*;

use crate::db::queries::get_cached_or_fetch;
use crate::router::{DbError, WorkerError, error_to_response};

// ---------------------------------------------------------------------------
// D1 result types
// ---------------------------------------------------------------------------

/// Row returned by the list query (no body_md to keep payload small).
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PostListRow {
    pub id: String,
    #[serde(rename = "type")]
    pub post_type: String,
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub published_at: Option<i64>,
    pub updated_at: i64,
    pub author_id: Option<String>,
}

/// Row returned by the single-post query (includes author join).
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PostDetailRow {
    pub id: String,
    #[serde(rename = "type")]
    pub post_type: String,
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub body_md: String,
    pub author_id: Option<String>,
    pub published_at: Option<i64>,
    pub updated_at: i64,
    pub published: i64,
    // Author fields joined from staff
    pub username: Option<String>,
    pub name: Option<String>,
    pub job_title: Option<String>,
    pub avatar_url: Option<String>,
}

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Execute the published-posts list query for a given `post_type` string.
///
/// Returns a `Vec<PostListRow>` ordered by `published_at DESC`.
async fn fetch_post_list(env: &Env, post_type: &str) -> StdResult<Vec<PostListRow>, WorkerError> {
    let db = env
        .d1("DB")
        .map_err(|e| WorkerError::Internal(e.to_string()))?;

    let results = db
        .prepare(
            "SELECT id, type, slug, title, summary, published_at, updated_at, author_id \
             FROM post \
             WHERE type = ?1 AND published = 1 \
             ORDER BY published_at DESC",
        )
        .bind(&[post_type.into()])
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?
        .all()
        .await
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?;

    results
        .results::<PostListRow>()
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))
}

/// Execute the single-post query for a given `slug` and `post_type`.
///
/// Returns `None` if no matching published row exists.
async fn fetch_post_detail(
    env: &Env,
    slug: &str,
    post_type: &str,
) -> StdResult<Option<PostDetailRow>, WorkerError> {
    let db = env
        .d1("DB")
        .map_err(|e| WorkerError::Internal(e.to_string()))?;

    db.prepare(
        "SELECT p.id, p.type, p.slug, p.title, p.summary, p.body_md, \
                p.author_id, p.published_at, p.updated_at, p.published, \
                s.username, s.name, s.job_title, s.avatar_url \
         FROM post p \
         LEFT JOIN staff s ON p.author_id = s.id \
         WHERE p.slug = ?1 AND p.type = ?2 AND p.published = 1",
    )
    .bind(&[slug.into(), post_type.into()])
    .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?
    .first::<PostDetailRow>(None)
    .await
    .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))
}

/// Shared list handler — queries the cache then D1, returns JSON array.
async fn list_posts(_req: &Request, env: &Env, post_type: &str, cache_key: &str) -> Result<Response> {
    let kv = match env.kv("EZO_CACHE") {
        Ok(k) => k,
        Err(e) => return error_to_response(WorkerError::Internal(e.to_string())),
    };

    let post_type_owned = post_type.to_string();

    let posts = get_cached_or_fetch(&kv, cache_key, 300, || async move {
        fetch_post_list(env, &post_type_owned).await
    })
    .await;

    match posts {
        Ok(list) => Response::from_json(&list)
            .map(|r| r.with_status(200))
            .map_err(|e| WorkerError::Internal(e.to_string()).into()),
        Err(e) => error_to_response(e),
    }
}

/// Shared single-post handler — queries the cache then D1, returns JSON or 404.
async fn get_post(
    _req: &Request,
    env: &Env,
    slug: &str,
    post_type: &str,
) -> Result<Response> {
    let cache_key = format!("post:{}", slug);
    let kv = match env.kv("EZO_CACHE") {
        Ok(k) => k,
        Err(e) => return error_to_response(WorkerError::Internal(e.to_string())),
    };

    let slug_owned = slug.to_string();
    let post_type_owned = post_type.to_string();

    // The cached value is Option<PostDetailRow>; on a miss fetch_fn returns the Option.
    let result = get_cached_or_fetch::<Option<PostDetailRow>, _, _>(
        &kv,
        &cache_key,
        300,
        || async move { fetch_post_detail(env, &slug_owned, &post_type_owned).await },
    )
    .await;

    match result {
        Ok(Some(post)) => Response::from_json(&post)
            .map(|r| r.with_status(200))
            .map_err(|e| WorkerError::Internal(e.to_string()).into()),
        Ok(None) => error_to_response(WorkerError::NotFound),
        Err(e) => error_to_response(e),
    }
}

// ---------------------------------------------------------------------------
// GET /api/insights
// ---------------------------------------------------------------------------

/// List all published insight posts ordered by `published_at DESC`.
///
/// Cache key: `insights:list`, TTL: 300 s.
///
/// Requirements: 10.1, 10.2, 13.1, 13.4, 13.5, 13.6
pub async fn list_insights(req: &Request, env: &Env) -> Result<Response> {
    list_posts(req, env, "insight", "insights:list").await
}

// ---------------------------------------------------------------------------
// GET /api/insights/:slug
// ---------------------------------------------------------------------------

/// Return a single published insight post including author public profile,
/// or HTTP 404 if not found.
///
/// Cache key: `post:{slug}`, TTL: 300 s.
///
/// Requirements: 10.1, 10.3, 10.4, 13.1, 13.4, 13.5, 13.6
pub async fn get_insight(req: &Request, env: &Env, slug: &str) -> Result<Response> {
    get_post(req, env, slug, "insight").await
}

// ---------------------------------------------------------------------------
// GET /api/work
// ---------------------------------------------------------------------------

/// List all published work posts ordered by `published_at DESC`.
///
/// Cache key: `work:list`, TTL: 300 s.
///
/// Requirements: 10.1, 10.2, 13.1, 13.4, 13.5, 13.6
pub async fn list_work(req: &Request, env: &Env) -> Result<Response> {
    list_posts(req, env, "work", "work:list").await
}

// ---------------------------------------------------------------------------
// GET /api/work/:slug
// ---------------------------------------------------------------------------

/// Return a single published work post including author public profile,
/// or HTTP 404 if not found.
///
/// Cache key: `post:{slug}`, TTL: 300 s.
///
/// Requirements: 10.1, 10.3, 10.4, 13.1, 13.4, 13.5, 13.6
pub async fn get_work(req: &Request, env: &Env, slug: &str) -> Result<Response> {
    get_post(req, env, slug, "work").await
}

// ---------------------------------------------------------------------------
// GET /api/capabilities
// ---------------------------------------------------------------------------

/// List all published capability posts ordered by `published_at DESC`.
///
/// Cache key: `capabilities:list`, TTL: 300 s.
///
/// Requirements: 10.1, 10.2, 13.1, 13.4, 13.5, 13.6
pub async fn list_capabilities(req: &Request, env: &Env) -> Result<Response> {
    list_posts(req, env, "capability", "capabilities:list").await
}

// ---------------------------------------------------------------------------
// GET /api/capabilities/:slug
// ---------------------------------------------------------------------------

/// Return a single published capability post including author public profile,
/// or HTTP 404 if not found.
///
/// Cache key: `post:{slug}`, TTL: 300 s.
///
/// Requirements: 10.1, 10.3, 10.4, 13.1, 13.4, 13.5, 13.6
pub async fn get_capability(req: &Request, env: &Env, slug: &str) -> Result<Response> {
    get_post(req, env, slug, "capability").await
}
