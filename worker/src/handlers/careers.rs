#![allow(dead_code, unused_imports)]

//! Career listing and job application handlers.
//!
//! GET  /api/careers                       — list open roles (KV-cached, TTL 60 s)
//! GET  /api/careers/:slug                 — get single role (no cache)
//! POST /api/careers/:slug/apply           — submit an application
//! POST /api/careers/:slug/apply/documents — upload a supporting document

use worker::*;

use crate::models::career::{ApplicationDocument, Career, Application, ApplicationStatus};
use crate::router::{DbError, ValidationError, WorkerError, error_to_response};
use crate::storage::validator::{
    detect_mime_from_magic_bytes, sanitise_filename, ALLOWED_DOC_MIMES, MAX_DOC_SIZE,
    MAX_DOCS_PER_APPLICATION,
};

// ---------------------------------------------------------------------------
// Allowed employment types
// ---------------------------------------------------------------------------

const ALLOWED_CAREER_TYPES: &[&str] = &["Full-Time", "Part-Time", "Contract", "Internship"];

// ---------------------------------------------------------------------------
// GET /api/careers
// ---------------------------------------------------------------------------

/// List all active career listings.
///
/// Cache-aside pattern (Req 13.3, 13.4, 13.5, 13.6):
/// 1. Check KV_CACHE for `careers:active` (TTL 60 s).
/// 2. On cache hit: deserialise and return JSON array.
/// 3. On cache miss: `SELECT * FROM career WHERE active=1`, populate KV_CACHE
///    with TTL 60 s (write failure is non-fatal), return array.
///
/// Requirements: 5.1, 5.2, 5.3, 5.4, 13.3, 13.4, 13.5, 13.6
pub async fn list_careers(_req: &Request, env: &Env) -> Result<Response> {
    let cache_key = "careers:active";

    // ── 1. Try KV cache ──────────────────────────────────────────────────────
    let kv = env
        .kv("EZO_CACHE")
        .map_err(|e| WorkerError::Internal(e.to_string()))?;

    if let Ok(Some(cached)) = kv.get(cache_key).text().await {
        // Cache hit — return directly without querying D1 (Req 13.4)
        return Response::from_json(&serde_json::from_str::<serde_json::Value>(&cached)
            .unwrap_or(serde_json::Value::Array(vec![])))
            .map(|r| r.with_status(200))
            .map_err(|e| WorkerError::Internal(e.to_string()).into());
    }

    // ── 2. Cache miss — query D1 ─────────────────────────────────────────────
    let db = env
        .d1("DB")
        .map_err(|e| WorkerError::Internal(e.to_string()))?;

    let results = db
        .prepare("SELECT id, slug, title, description_md, department, type, active, created_at FROM career WHERE active=1")
        .all()
        .await
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?;

    let careers: Vec<Career> = results
        .results::<Career>()
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?;

    // ── 3. Populate KV cache (non-fatal on failure) ──────────────────────────
    let json_str = serde_json::to_string(&careers)
        .unwrap_or_else(|_| "[]".to_string());

    match kv.put(cache_key, &json_str) {
        Ok(builder) => {
            if let Err(e) = builder.expiration_ttl(60).execute().await {
                // Req 13.6: KV write failure must NOT be surfaced to caller
                console_log!("KV cache write failed for {}: {}", cache_key, e);
            }
        }
        Err(e) => {
            console_log!("KV cache put builder failed for {}: {}", cache_key, e);
        }
    }

    // ── 4. Return array ──────────────────────────────────────────────────────
    Response::from_json(&careers)
        .map(|r| r.with_status(200))
        .map_err(|e| WorkerError::Internal(e.to_string()).into())
}

// ---------------------------------------------------------------------------
// GET /api/careers/:slug
// ---------------------------------------------------------------------------

/// Return a single career listing by slug, or HTTP 404.
///
/// No caching required for individual career pages (per task spec).
///
/// Requirements: 5.1, 5.4
pub async fn get_career(_req: &Request, env: &Env, slug: &str) -> Result<Response> {
    let db = env
        .d1("DB")
        .map_err(|e| WorkerError::Internal(e.to_string()))?;

    let career = db
        .prepare(
            "SELECT id, slug, title, description_md, department, type, active, created_at \
             FROM career WHERE slug = ?1",
        )
        .bind(&[slug.into()])
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?
        .first::<Career>(None)
        .await
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?;

    match career {
        Some(c) => Response::from_json(&c)
            .map(|r| r.with_status(200))
            .map_err(|e| WorkerError::Internal(e.to_string()).into()),
        None => error_to_response(WorkerError::NotFound),
    }
}

// ---------------------------------------------------------------------------
// POST /api/careers/:slug/apply
// ---------------------------------------------------------------------------

/// Submit a job application for an active career listing.
///
/// 1. Verify the career exists and `active = true`; return HTTP 404 otherwise.
/// 2. Parse JSON body `{ applicantName, applicantEmail, coverLetter }`.
/// 3. Insert application row with `status = 'Applied'`, `applied_at = unixepoch()`.
/// 4. Return HTTP 201 with the created `Application` resource.
///
/// Requirements: 6.1, 6.2
pub async fn apply(mut req: Request, env: &Env, slug: &str) -> Result<Response> {
    let db = env
        .d1("DB")
        .map_err(|e| WorkerError::Internal(e.to_string()))?;

    // ── 1. Verify career exists and is active ────────────────────────────────
    let career = db
        .prepare(
            "SELECT id, slug, title, description_md, department, type, active, created_at \
             FROM career WHERE slug = ?1",
        )
        .bind(&[slug.into()])
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?
        .first::<Career>(None)
        .await
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?;

    let career = match career {
        Some(c) if c.active => c,
        // Not found or inactive → HTTP 404 (Req 6.2)
        _ => return error_to_response(WorkerError::NotFound),
    };

    // ── 2. Parse request body ────────────────────────────────────────────────
    let body: serde_json::Value = req
        .json()
        .await
        .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));

    let applicant_name = body
        .get("applicantName")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();

    let applicant_email = body
        .get("applicantEmail")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();

    let cover_letter = body
        .get("coverLetter")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    if applicant_name.is_empty() || applicant_email.is_empty() {
        return error_to_response(WorkerError::Validation(
            ValidationError::InvalidInput("applicantName and applicantEmail are required".into()),
        ));
    }

    // ── 3. Insert application row ────────────────────────────────────────────
    // D1 auto-generates the id via DEFAULT (lower(hex(randomblob(16))))
    db.prepare(
        "INSERT INTO application (career_id, applicant_name, applicant_email, cover_letter, \
         status, applied_at, updated_at) \
         VALUES (?1, ?2, ?3, ?4, 'Applied', unixepoch(), unixepoch())",
    )
    .bind(&[
        career.id.clone().into(),
        applicant_name.clone().into(),
        applicant_email.clone().into(),
        cover_letter.clone().into(),
    ])
    .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?
    .run()
    .await
    .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?;

    // ── 4. Re-query the inserted row to return the full resource ─────────────
    let application = db
        .prepare(
            "SELECT id, career_id, applicant_name, applicant_email, cover_letter, \
             status, applied_at, updated_at \
             FROM application \
             WHERE career_id = ?1 AND applicant_email = ?2 \
             ORDER BY applied_at DESC LIMIT 1",
        )
        .bind(&[career.id.clone().into(), applicant_email.clone().into()])
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?
        .first::<Application>(None)
        .await
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?
        .ok_or_else(|| WorkerError::Internal("Application row missing after insert".into()))?;

    Response::from_json(&application)
        .map(|r| r.with_status(201))
        .map_err(|e| WorkerError::Internal(e.to_string()).into())
}

// ---------------------------------------------------------------------------
// POST /api/careers/:slug/apply/documents
// ---------------------------------------------------------------------------

/// Upload a supporting document (CV, cover letter, portfolio) for an existing
/// application.
///
/// The endpoint is public (no session required). The client must supply the
/// `application_id` query parameter to identify the target application.
///
/// Processing pipeline:
/// 1. Look up the career by slug; return HTTP 404 if not found or inactive.
/// 2. Parse `application_id` from the query string; return HTTP 400 if absent.
/// 3. Verify the application exists and belongs to this career; HTTP 404 otherwise.
/// 4. Parse the multipart form body and extract the `file` field.
/// 5. Detect MIME from magic bytes; reject if not in ALLOWED_DOC_MIMES.
/// 6. Reject if file size exceeds MAX_DOC_SIZE (10 MB).
/// 7. Reject if the application already has MAX_DOCS_PER_APPLICATION (3) documents.
/// 8. Sanitise the original filename.
/// 9. Store the file in R2 at `applications/{application_id}/{sanitised_filename}`.
/// 10. Insert an `application_document` row and return HTTP 201.
///
/// Requirements: 6.3, 6.4, 6.5, 6.6, 6.7, 6.8, 6.9, 6.10
pub async fn upload_document(mut req: Request, env: &Env, slug: &str) -> Result<Response> {
    let db = env
        .d1("DB")
        .map_err(|e| WorkerError::Internal(e.to_string()))?;

    // ── 1. Verify career exists and is active ────────────────────────────────
    let career = db
        .prepare(
            "SELECT id, slug, title, description_md, department, type, active, created_at \
             FROM career WHERE slug = ?1",
        )
        .bind(&[slug.into()])
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?
        .first::<Career>(None)
        .await
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?;

    let career = match career {
        Some(c) if c.active => c,
        _ => return error_to_response(WorkerError::NotFound),
    };

    // ── 2. Parse application_id from query string ────────────────────────────
    let url = req.url().map_err(|e| WorkerError::Internal(e.to_string()))?;
    let application_id: Option<String> = url
        .query_pairs()
        .find(|(k, _)| k == "application_id")
        .map(|(_, v)| v.into_owned());

    let application_id = match application_id {
        Some(id) if !id.is_empty() => id,
        _ => {
            return error_to_response(WorkerError::Validation(ValidationError::InvalidInput(
                "application_id query parameter is required".into(),
            )));
        }
    };

    // ── 3. Verify application exists and belongs to this career ──────────────
    let application = db
        .prepare(
            "SELECT id, career_id, applicant_name, applicant_email, cover_letter, \
             status, applied_at, updated_at \
             FROM application WHERE id = ?1",
        )
        .bind(&[application_id.as_str().into()])
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?
        .first::<Application>(None)
        .await
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?;

    let application = match application {
        Some(a) if a.career_id == career.id => a,
        _ => return error_to_response(WorkerError::NotFound),
    };

    // Keep `application` bound but acknowledge it's only used for validation.
    let _ = &application;

    // ── 4. Parse multipart form and extract the file field ───────────────────
    let form = req
        .form_data()
        .await
        .map_err(|e| WorkerError::Internal(format!("Failed to parse form data: {}", e)))?;

    let file_entry = form.get("file");
    let (file_bytes, original_filename) = match file_entry {
        Some(FormEntry::File(file)) => {
            let name = file.name();
            let bytes = file
                .bytes()
                .await
                .map_err(|e| WorkerError::Internal(format!("Failed to read file bytes: {}", e)))?;
            (bytes, name)
        }
        _ => {
            return Response::from_json(&serde_json::json!({ "error": "file field is required" }))
                .map(|r| r.with_status(400))
                .map_err(|e| WorkerError::Internal(e.to_string()).into());
        }
    };

    // ── 5. Detect MIME from magic bytes ─────────────────────────────────────
    let mime = match detect_mime_from_magic_bytes(&file_bytes) {
        Some(m) if ALLOWED_DOC_MIMES.contains(&m) => m,
        _ => {
            return Response::from_json(&serde_json::json!({ "error": "Invalid file type" }))
                .map(|r| r.with_status(400))
                .map_err(|e| WorkerError::Internal(e.to_string()).into());
        }
    };

    // ── 6. Check file size ≤ MAX_DOC_SIZE (10 MB) ────────────────────────────
    if file_bytes.len() > MAX_DOC_SIZE {
        return Response::from_json(&serde_json::json!({ "error": "File too large" }))
            .map(|r| r.with_status(400))
            .map_err(|e| WorkerError::Internal(e.to_string()).into());
    }

    // ── 7. Check document count ≤ MAX_DOCS_PER_APPLICATION (3) ───────────────
    let count_row = db
        .prepare(
            "SELECT COUNT(*) as count FROM application_document WHERE application_id = ?1",
        )
        .bind(&[application_id.as_str().into()])
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?
        .first::<serde_json::Value>(None)
        .await
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?;

    let doc_count = count_row
        .as_ref()
        .and_then(|row| row.get("count"))
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    if doc_count >= MAX_DOCS_PER_APPLICATION {
        return Response::from_json(&serde_json::json!({ "error": "Maximum document count reached" }))
            .map(|r| r.with_status(400))
            .map_err(|e| WorkerError::Internal(e.to_string()).into());
    }

    // ── 8. Sanitise original filename ────────────────────────────────────────
    let sanitised = sanitise_filename(&original_filename);

    // ── 9. Store in R2 ───────────────────────────────────────────────────────
    let bucket = env
        .bucket("EZO_MEDIA")
        .map_err(|e| WorkerError::Internal(format!("Failed to get R2 bucket: {}", e)))?;

    let r2_key = format!("applications/{}/{}", application_id, sanitised);

    let http_metadata = HttpMetadata {
        content_type: Some(mime.to_string()),
        ..HttpMetadata::default()
    };

    bucket
        .put(&r2_key, Data::Bytes(file_bytes))
        .http_metadata(http_metadata)
        .execute()
        .await
        .map_err(|e| WorkerError::Internal(format!("R2 put failed: {}", e)))?;

    // ── 10. Insert application_document row in D1 ────────────────────────────
    db.prepare(
        "INSERT INTO application_document (application_id, r2_key, original_filename, mime_type) \
         VALUES (?1, ?2, ?3, ?4)",
    )
    .bind(&[
        application_id.as_str().into(),
        r2_key.as_str().into(),
        sanitised.as_str().into(),
        mime.into(),
    ])
    .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?
    .run()
    .await
    .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?;

    // ── Re-query inserted row and return HTTP 201 ────────────────────────────
    // Use the application_id + r2_key to uniquely identify the just-inserted row.
    let doc = db
        .prepare(
            "SELECT id, application_id, r2_key, original_filename, mime_type, uploaded_at \
             FROM application_document \
             WHERE application_id = ?1 AND r2_key = ?2 \
             ORDER BY uploaded_at DESC LIMIT 1",
        )
        .bind(&[application_id.as_str().into(), r2_key.as_str().into()])
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?
        .first::<ApplicationDocument>(None)
        .await
        .map_err(|e| WorkerError::Db(DbError::Query(e.to_string())))?
        .ok_or_else(|| WorkerError::Internal("Document row missing after insert".into()))?;

    Response::from_json(&doc)
        .map(|r| r.with_status(201))
        .map_err(|e| WorkerError::Internal(e.to_string()).into())
}
