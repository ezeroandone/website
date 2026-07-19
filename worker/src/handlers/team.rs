#![allow(dead_code, unused_imports)]

//! Team profile handlers.
//! GET /api/team               — list active public staff profiles (onboarding_completed = 1)
//! GET /api/team/:username     — get a single staff public profile
//!
//! The list endpoint uses get_cached_or_fetch with cache key "team:list" and TTL 120 s.
//! The single-profile endpoint queries D1 directly with no caching.
//!
//! Response type for both endpoints: StaffPublicProfile — which intentionally
//! excludes `email`, `role`, `id`, and `signing_public_key`.
//!
//! Requirements: 11.1, 11.2, 11.3, 11.4, 13.2

use worker::*;

use crate::db::queries::get_cached_or_fetch;
use crate::models::staff::StaffPublicProfile;
use crate::router::{DbError, WorkerError, error_to_response};

// ---------------------------------------------------------------------------
// GET /api/team
// ---------------------------------------------------------------------------

/// List all active staff public profiles (onboarding_completed = 1).
///
/// Cache key: `team:list`, TTL: 120 s.
///
/// Requirements: 11.1, 11.2, 13.2
pub async fn list_team(_req: &Request, env: &Env) -> Result<Response> {
    let kv = match env.kv("EZO_CACHE") {
        Ok(k) => k,
        Err(e) => return error_to_response(WorkerError::Internal(e.to_string())),
    };

    let result = get_cached_or_fetch(&kv, "team:list", 120, || async move {
        let db = env
            .d1("DB")
            .map_err(|e| WorkerError::Internal(e.to_string()))?;

        let results = db
            .prepare(
                "SELECT username, name, job_title, bio, avatar_url \
                 FROM staff \
                 WHERE onboarding_completed = 1",
            )
            .all()
            .await
            .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?;

        results
            .results::<StaffPublicProfile>()
            .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))
    })
    .await;

    match result {
        Ok(list) => Response::from_json(&list)
            .map(|r| r.with_status(200))
            .map_err(|e| WorkerError::Internal(e.to_string()).into()),
        Err(e) => error_to_response(e),
    }
}

// ---------------------------------------------------------------------------
// GET /api/team/:username
// ---------------------------------------------------------------------------

/// Return a single active staff public profile by username, or HTTP 404 if
/// not found or onboarding not completed.
///
/// Response intentionally excludes: `email`, `role`, `id`, `signing_public_key`.
///
/// Requirements: 11.3, 11.4
pub async fn get_team_member(_req: &Request, env: &Env, username: &str) -> Result<Response> {
    let db = match env.d1("DB") {
        Ok(d) => d,
        Err(e) => return error_to_response(WorkerError::Internal(e.to_string())),
    };

    let result = db
        .prepare(
            "SELECT username, name, job_title, bio, avatar_url \
             FROM staff \
             WHERE username = ?1 AND onboarding_completed = 1",
        )
        .bind(&[username.into()])
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))
        .and_then(|stmt| Ok(stmt));

    let stmt = match result {
        Ok(s) => s,
        Err(e) => return error_to_response(e),
    };

    match stmt
        .first::<StaffPublicProfile>(None)
        .await
    {
        Ok(Some(profile)) => Response::from_json(&profile)
            .map(|r| r.with_status(200))
            .map_err(|e| WorkerError::Internal(e.to_string()).into()),
        Ok(None) => error_to_response(WorkerError::NotFound),
        Err(e) => error_to_response(WorkerError::Db(DbError::Query(e.to_string()))),
    }
}
