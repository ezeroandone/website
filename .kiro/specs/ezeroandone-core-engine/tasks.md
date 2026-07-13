# Implementation Plan: eZeroAndOne Core Engine

## Overview

Full-stack Cloudflare-native corporate platform: SvelteKit frontend on Cloudflare Pages, Rust/WASM
Worker backend, D1 relational storage, KV for auth/cache, and R2 for media and application
documents. Tasks are ordered so every step compiles and is wired into the running system before
the next step begins. No orphaned code.

Language: **Rust** (backend Worker) + **TypeScript/Svelte** (SvelteKit frontend).

---

## Tasks

- [x] 1. Project Scaffolding
  - [x] 1.1 Initialise the Rust Worker crate
    - Run `cargo init --lib worker` with `wasm32-unknown-unknown` target
    - Add `worker = "0.4"`, `worker-macros = "0.4"`, `serde = { version = "1", features = ["derive"] }`, `serde_json = "1"`, `uuid = { version = "1", features = ["v4","js"] }`, `hmac = "0.12"`, `sha2 = "0.10"`, `base64 = "0.22"`, `proptest = "1"` to `Cargo.toml`
    - Set `[lib] crate-type = ["cdylib"]` and add the release profile: `opt-level="z"`, `lto=true`, `codegen-units=1`, `panic="abort"`, `strip=true`
    - Create the full module tree stub files: `lib.rs`, `router.rs`, `middleware/mod.rs`, `middleware/cors.rs`, `middleware/rate_limit.rs`, `middleware/auth.rs`, `handlers/mod.rs`, `models/mod.rs`, `db/mod.rs`, `crypto/mod.rs`, `storage/mod.rs`, `email/mod.rs`
    - `lib.rs` entry point fully implemented: CORS preflight fast-path, rate limiting bootstrap, session parse, router dispatch, CORS attach
    - _Requirements: 18.2_
  - [x] 1.2 Initialise the SvelteKit project
    - Run `npm create svelte@latest frontend` with TypeScript, `adapter-cloudflare`
    - Install `@sveltejs/adapter-cloudflare@^4`, `svelte@^5`, `@testing-library/svelte@^5`, `fast-check@^3`, `vitest@^2`
    - Configure `svelte.config.js` to use `adapter-cloudflare`
    - Create the full route directory skeleton matching the design route map (section 3.1): all `+page.svelte`, `+page.server.ts`, `+layout.svelte` stub files
    - _Requirements: 15.1_
  - [x] 1.3 Create `wrangler.toml` and CI/CD pipeline
    - Write `wrangler.toml` with all bindings: `DB` (D1), `EZO_AUTH` (KV), `EZO_CACHE` (KV), `EZO_MEDIA` (R2), `compatibility_date = "2025-01-01"`, `main = "build/worker/shim.mjs"`, `format = "modules"`
    - Create `.github/workflows/deploy.yml` with jobs: `build-worker` (cargo build → wrangler deploy), `build-pages` (npm build → pages deploy)
    - Add `JWT_SECRET` and `QR_SECRET` as GitHub Actions secrets references in the workflow file
    - _Requirements: 2.6, 18.1_

- [x] 2. D1 Schema Migrations
  - [x] 2.1 Write migration SQL file and D1 migration runner
    - Create `worker/migrations/0001_initial.sql` containing the full DDL from design section 5.2: `staff`, `staff_lifecycle`, `post`, `team_profile`, `career`, `application`, `application_document`, `media_asset` tables with all CHECK constraints, UNIQUE constraints, and foreign keys with `ON DELETE CASCADE`
    - The `team_profile` table has: `id TEXT PRIMARY KEY REFERENCES staff(id) ON DELETE CASCADE`, `linkedin TEXT`, `github TEXT`, `twitter TEXT`, `skills TEXT`, `order_rank INTEGER NOT NULL DEFAULT 0`
    - Create all six performance indices: `idx_post_type_slug`, `idx_post_published`, `idx_application_career`, `idx_lifecycle_status`, `idx_app_doc_application`, `idx_media_context`
    - Implement `db/migrations.rs`: a `run_migrations` async fn that checks a `_migrations` tracking table and applies unapplied SQL files in order
    - _Requirements: 14.1, 14.2, 14.3, 14.4_
  - [x]* 2.2 Write integration tests for schema constraints
    - Test that `staff.role` check constraint rejects invalid values
    - Test that `staff_lifecycle.staff_id` UNIQUE + foreign key is enforced
    - Test that `application.status` transitions are constrained at DB level
    - Test index existence by querying `sqlite_master` for all six indices
    - Test that `team_profile.id` foreign key cascades correctly on `staff` delete
    - _Requirements: 14.1, 14.2, 14.3, 14.4_

- [x] 3. Rust Data Models
  - [x] 3.1 Implement core Rust model structs with serde derives
    - `models/staff.rs`: `Staff`, `StaffLifecycle`, `StaffPublicProfile`, `StaffAdmin`, `SessionContext`, `Role` enum (`SuperAdmin=4`, `Admin=3`, `Staff=2`, `Public=1`), `LifecycleStatus` enum, `TeamProfile` struct (`linkedin`, `github`, `twitter`, `skills`, `order_rank`)
    - `models/content.rs`: `Post`, `PostSummary`, `PostCreate`, `PostUpdate`, `PostType` enum
    - `models/career.rs`: `Career`, `CareerCreate`, `CareerUpdate`, `Application`, `ApplicationStatus` enum, `ApplicationDocument`, `DocumentMeta`, `MediaAsset`
    - `models/session.rs`: `StaffClaims` (JWT payload with `sub`, `email`, `role`, `onboarded`, `exp`); `QrClaims` (`sub`, `exp`, `iss`)
    - All structs derive `Serialize`, `Deserialize`; timestamps as `i64` (Unix seconds, not strings)
    - _Requirements: 14.3, 20.1, 20.3_
  - [x]* 3.2 Write property test for JSON round-trip fidelity (Property 15)
    - Use `proptest` to generate arbitrary `Staff`, `Post`, `Career`, `Application`, `ApplicationDocument`, `MediaAsset` instances
    - Assert `serde_json::from_str(&serde_json::to_string(&val).unwrap()) == val` for each type
    - Assert timestamp fields round-trip as JSON numbers (not strings) per Requirement 20.3
    - **Property 15: JSON serialisation round-trip for domain types**
    - **Validates: Requirements 20.2, 20.3**

- [x] 4. Cryptographic Primitives
  - [x] 4.1 Implement magic-link token generation (`crypto/token.rs`)
    - `generate_secure_token() -> String`: call `web_sys::window()?.crypto()?.get_random_values()` to fill 32 bytes, encode as 64-char lowercase hex
    - _Requirements: 1.3_
  - [x]* 4.2 Write property test for token uniqueness (Property 2 partial)
    - Generate 1000 tokens in a loop; assert all are distinct using a `HashSet`
    - Assert each token is exactly 64 characters and matches `^[0-9a-f]{64}$`
    - **Validates: Requirements 1.3**
  - [x] 4.3 Implement HS256 JWT sign and verify (`crypto/jwt.rs`)
    - `sign_jwt(claims: &StaffClaims, secret: &[u8]) -> Result<String, WorkerError>`: base64url-encode header + payload, compute HMAC-SHA256, return three-part JWT string
    - `verify_jwt(token: &str, secret: &[u8]) -> Result<StaffClaims, WorkerError>`: split on `.`, constant-time HMAC compare, parse claims, check `exp > now`
    - `generate_identity_jwt(staff_id: &str, qr_secret: &[u8]) -> String`: claims = `{ sub, exp: now+300, iss: "ezo-identity" }`, no PII beyond `sub`
    - `verify_identity_jwt(token: &str, qr_secret: &[u8]) -> Result<QrClaims, WorkerError>`: same verify logic, returns `QrClaims`
    - _Requirements: 1.9, 2.1, 2.2, 2.3, 2.4, 9.1, 9.2, 9.3_
  - [x]* 4.4 Write property test for JWT round-trip fidelity (Property 3)
    - For arbitrary `StaffClaims` with `exp = now+3600` and 32–64 byte secrets: `verify_jwt(sign_jwt(claims, secret), secret) == Ok(claims)`
    - **Property 3: JWT round-trip fidelity**
    - **Validates: Requirements 1.9, 2.1, 2.2, 2.3**
  - [x]* 4.5 Write property test for JWT tamper detection (Property 12)
    - For any valid JWT string, mutate a single byte in the signature segment; assert `verify_jwt` returns `Err(InvalidSignature)`
    - **Property 12: JWT signature tamper-detection**
    - **Validates: Requirements 2.2, 19.2**
  - [x] 4.6 Implement staff signing key provisioning (`crypto/signing_key.rs`)
    - `is_valid_public_key_pem(pem: &str) -> bool`: validate PEM header/footer and that key is Ed25519 or ECDSA P-256
    - `provision_signing_key(staff_id: &str, public_key_pem: &str, db: &D1Database) -> Result<(), WorkerError>`: validate PEM, run `UPDATE staff SET signing_public_key = ?1, updated_at = unixepoch() WHERE id = ?2`
    - _Requirements: 4.5, 4.6, 4.7_

- [x] 5. CORS, Rate Limiting, and Auth Middleware
  - [x] 5.1 Implement CORS middleware (`middleware/cors.rs`)
    - `cors_preflight_response() -> Response`: return HTTP 200 with `Access-Control-Allow-Origin`, `Access-Control-Allow-Methods`, `Access-Control-Allow-Headers`, `Access-Control-Allow-Credentials` headers
    - `attach_cors_headers(response: Response) -> Response`: append CORS headers to any outgoing response
    - Wire the OPTIONS fast-path in `lib.rs` entry point so preflight returns before any other middleware runs
    - _Requirements: 17.7, 17.8_
  - [x] 5.2 Implement KV rate limiter (`middleware/rate_limit.rs`)
    - `rate_limit_check(kv: &KvNamespace, ip: &str) -> Result<(), WorkerError>`: read `rl:{ip}`, parse `{count, window_start}`, increment or reset counter, write back with TTL 60 s
    - Return `Err(WorkerError::RateLimited)` when count > 5 within the same 60-second window
    - _Requirements: 1.12, 19.10_
  - [x]* 5.3 Write property test for rate limiter threshold (Property 14)
    - For any IP, simulate exactly 5 requests → all allowed; 6th request in same window → `Err(RateLimited)`
    - Simulate window expiry → counter resets and requests are allowed again
    - **Property 14: Rate-limiter enforces per-IP threshold**
    - **Validates: Requirements 1.12, 19.10**
  - [x] 5.4 Implement auth middleware (`middleware/auth.rs`)
    - `auth_middleware(request: &Request, env: &Env, required_role: Role) -> Result<SessionContext, Response>`: extract `session` cookie, call `verify_jwt`, compare `role_level(claims.role) >= role_level(required_role)`, return `Ok(ctx)` or `Err(401/403)`
    - `onboarding_guard(session_ctx: Option<&SessionContext>, path: &str) -> Option<Response>`: return `Some(302 /onboarding)` when `!ctx.onboarded` and path ∉ `BYPASS_PATHS`
    - _Requirements: 2.2, 2.3, 2.4, 2.5, 3.1, 3.2, 3.3, 4.1, 4.2_
  - [x]* 5.5 Write property test for RBAC monotonicity (Property 4)
    - For any role `r` and any route, if `has_access(r, route)` is true, then for all roles `r'` where `role_level(r') > role_level(r)`, `has_access(r', route)` is also true
    - **Property 4: RBAC is monotone**
    - **Validates: Requirements 3.2, 3.8**
  - [x]* 5.6 Write property test for onboarding guard completeness (Property 6)
    - For any authenticated session where `onboarded = false` and path ∉ `BYPASS_PATHS`, guard returns `Some(302 /onboarding)`
    - For any path ∈ `BYPASS_PATHS`, guard always returns `None` regardless of onboarding state
    - **Property 6: Onboarding guard completeness**
    - **Validates: Requirements 4.1, 4.2**

- [x] 6. Router and Error Handling
  - [x] 6.1 Implement the request router (`router.rs`)
    - `router_dispatch(request: &Request, session_ctx: Option<SessionContext>, env: &Env) -> Response`: match on `(method, path)` using prefix/segment matching and dispatch to the correct handler module
    - Implement the route protection table from design section 11.3: call `auth_middleware` with the correct `required_role` before dispatching to each protected handler
    - Return `404 { "error": "Not Found" }` for unmatched routes
    - _Requirements: 3.4, 3.5, 3.6, 3.7_
  - [x] 6.2 Implement `WorkerError` type and `error_to_response` (`router.rs`)
    - Define `WorkerError` enum with variants `Auth(AuthError)`, `Db(DbError)`, `Validation(ValidationError)`, `NotFound`, `Forbidden`, `RateLimited`, `Internal(String)` as specified in design section 15.1
    - Implement `error_to_response(err: WorkerError) -> Response` mapping per design section 15.2; internal errors log full detail and return generic `500` body
    - Wire `error_to_response` at the top-level entry so all `?`-propagated errors are converted uniformly
    - _Requirements: 17.1, 17.2, 17.3, 17.4, 17.5, 17.6_
  - [ ]* 6.3 Write property test for error-to-HTTP-status mapping (Property 20)
    - For every `WorkerError` variant, assert `error_to_response(err).status()` matches the defined mapping: `Auth(_)` → 401, `NotFound` → 404, `Forbidden` → 403, `RateLimited` → 429, `Validation(_)` → 400, others → 500
    - Assert no `WorkerError` variant produces a 200 or out-of-range status code
    - **Property 20: Error-to-HTTP-status mapping is total**
    - **Validates: Requirements 17.1, 17.2, 17.3, 17.4, 17.5, 17.6**

- [x] 7. Checkpoint — Core infrastructure complete
  - Ensure all Rust modules compile (`cargo check --target wasm32-unknown-unknown`)
  - Ensure all property tests pass (`cargo test`)
  - Ask the user if any questions arise before proceeding to handler implementation

- [x] 8. Magic-Link Authentication Handlers
  - [x] 8.1 Implement `POST /api/auth/request` handler (`handlers/auth.rs`)
    - Parse `{ email }` from JSON body; call `validate_corporate_email` (allowlist check, return HTTP 403 on failure without revealing allowed domains)
    - Call `rate_limit_check`; generate token via `generate_secure_token()`; store `ml:{token}` in `EZO_AUTH` with TTL 900 s
    - Call `email::provider::send_magic_link(email, token, env)` to dispatch the callback email
    - Return HTTP 200 `{ "message": "Magic link sent" }`
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5, 19.3_
  - [x] 8.2 Implement `GET /api/auth/callback` handler (`handlers/auth.rs`)
    - Extract `token` query param; fetch `ml:{token}` from `EZO_AUTH` then immediately delete it (single-use); return HTTP 401 if absent
    - Load or create staff row matching the email; sign `StaffClaims` JWT; set `session` cookie as `HttpOnly; Secure; SameSite=Strict`
    - Redirect to `/onboarding` if `onboarding_completed = false`, else to `/dashboard`
    - _Requirements: 1.6, 1.7, 1.8, 1.9, 1.10, 2.1, 19.1, 19.9_
  - [x] 8.3 Implement `POST /api/auth/logout` handler (`handlers/auth.rs`)
    - Clear the `session` cookie by setting it with `Max-Age=0` and the same security attributes
    - Return HTTP 200
    - _Requirements: 1.11_
  - [ ]* 8.4 Write property test for domain validation soundness (Property 1)
    - For any email string where `validate_corporate_email(email) = Ok(())`, assert `extract_domain(email) ∈ CORPORATE_DOMAINS`
    - For any email where the domain is not in `CORPORATE_DOMAINS`, assert the result is `Err`
    - **Property 1: Domain restriction is total**
    - **Validates: Requirements 1.1, 1.2**
  - [ ]* 8.5 Write property test for magic-link single-use semantics (Property 2)
    - After a successful callback exchange, assert `EZO_AUTH.get("ml:{token}")` returns `None`
    - Assert a second callback call with the same token returns HTTP 401
    - **Property 2: Magic-link tokens are single-use**
    - **Validates: Requirements 1.7, 19.1**

- [x] 9. Email Provider
  - [x] 9.1 Implement email provider HTTP client (`email/provider.rs`)
    - `send_magic_link(to: &str, token: &str, env: &Env) -> Result<(), WorkerError>`: build the callback URL `https://ezeroandone.io/auth/callback?token={token}` and POST the email via Mailgun or Resend REST API using the `MAIL_API_KEY` secret binding
    - `send_onboarding_email(to: &str, staff_id: &str, env: &Env) -> Result<(), WorkerError>`: dispatch the welcome / onboarding instructions email
    - Handle non-2xx responses from the email provider as `WorkerError::Internal`
    - _Requirements: 1.5, 7.6_

- [x] 10. Staff Onboarding Handlers
  - [x] 10.1 Implement `GET /api/onboarding/status` handler (`handlers/onboarding.rs`)
    - Require Staff role; derive current step from `staff.signing_public_key` (nil → step 1 or 2) and `staff.onboarding_completed` flag
    - Return `{ step: 1|2|3, completed: bool }`
    - _Requirements: 4.10_
  - [x] 10.2 Implement `PATCH /api/onboarding/profile` handler (`handlers/onboarding.rs`)
    - Require Staff role; parse `{ name, job_title, bio, avatar_url }` from body; run `UPDATE staff SET name=?1, job_title=?2, bio=?3, avatar_url=?4, updated_at=unixepoch() WHERE id=?5`
    - Return HTTP 200
    - _Requirements: 4.4_
  - [x] 10.3 Implement `POST /api/onboarding/signing-key` handler (`handlers/onboarding.rs`)
    - Require Staff role; parse `{ public_key_pem }`; call `provision_signing_key` from `crypto/signing_key.rs`
    - Return HTTP 400 with descriptive error on invalid PEM; HTTP 200 on success
    - _Requirements: 4.5, 4.6, 4.7_
  - [x] 10.4 Implement `POST /api/onboarding/complete` handler (`handlers/onboarding.rs`)
    - Require Staff role; set `onboarding_completed = 1` on the staff row; return HTTP 200
    - _Requirements: 4.8, 4.9_

- [x] 11. HR Pipeline — Career Listings and Applications
  - [x] 11.1 Implement public career listing handlers (`handlers/careers.rs`)
    - `GET /api/careers`: KV_CACHE lookup `careers:active` (TTL 60 s); on miss, `SELECT * FROM career WHERE active=1`; populate cache; return array
    - `GET /api/careers/:slug`: return career row or HTTP 404; no caching required for individual career pages
    - _Requirements: 5.1, 5.2, 5.3, 5.4, 13.3, 13.4, 13.5, 13.6_
  - [x] 11.2 Implement `POST /api/careers/:slug/apply` handler (`handlers/careers.rs`)
    - Verify career exists and `active = true`; parse `{ applicantName, applicantEmail, coverLetter }`; insert application row with `status = 'Applied'`; return HTTP 201 with created resource
    - _Requirements: 6.1, 6.2_
  - [x] 11.3 Implement admin career management handlers (`handlers/admin.rs`)
    - `GET /api/admin/careers`: require Admin; return all careers regardless of active status
    - `POST /api/admin/careers`: require Admin; validate `type` ∈ `{Full-Time, Part-Time, Contract, Internship}`; insert; return HTTP 201
    - `PATCH /api/admin/careers/:id`: require Admin; update allowed fields; return HTTP 200
    - _Requirements: 5.5, 5.6, 5.7_
  - [x] 11.4 Implement `GET /api/admin/applications` handler (`handlers/admin.rs`)
    - Require Admin; query all applications from D1 ordered by `applied_at DESC`; return `Application[]`
    - _Requirements: 3.6, 7.3_
  - [ ]* 11.5 Write property test for active-careers filter (Property 18)
    - For any response from `GET /api/careers`, assert every career in the array has `active = true`
    - **Property 18: Active-careers filter is total**
    - **Validates: Requirements 5.3**
  - [ ]* 11.6 Write property test for status transition validity — application (Property 5)
    - For every `(current, next)` pair not in the allowed transitions set, assert `apply_transition(current, next)` returns `Err(InvalidTransition)`
    - Assert no sequence of allowed transitions revisits a previously visited status
    - **Property 5: Application status transitions are acyclic**
    - **Validates: Requirements 7.1, 7.2**

- [x] 12. HR Pipeline — Document Upload and Applicant Lifecycle
  - [x] 12.1 Implement `POST /api/careers/:slug/apply/documents` handler (`handlers/careers.rs`)
    - Parse `multipart/form-data`; call `storage/validator.rs` to check MIME type (magic bytes) and file size (≤ 10 MB); reject with HTTP 400 if invalid
    - Check existing document count < 3 for the application; sanitise filename (strip `/` and `..`); put file into R2 at `applications/{application_id}/{sanitised_filename}`; insert `application_document` row
    - _Requirements: 6.3, 6.4, 6.5, 6.6, 6.7, 6.8, 6.9, 6.10_
  - [ ]* 12.2 Write property test for file size and MIME invariants (Property 10)
    - For any upload where MIME ∉ allowed set OR size > limit, assert the handler returns HTTP 400 before any R2 write
    - **Property 10: File size and MIME invariants**
    - **Validates: Requirements 6.3, 6.4, 6.5, 6.6, 12.2, 12.3, 12.4, 12.5**
  - [ ]* 12.3 Write property test for document count cap (Property 11)
    - For any `application_id` already holding 3 documents, assert a 4th upload returns HTTP 400 and no new R2 or D1 writes occur
    - **Property 11: Document count cap**
    - **Validates: Requirements 6.8**
  - [x] 12.4 Implement applicant lifecycle transition handler (`handlers/admin.rs`)
    - `PATCH /api/admin/applications/:id/status`: require Admin; validate target status is in the allowed transition set; update `status` and `updated_at`; return HTTP 200 or HTTP 400 on invalid transition
    - _Requirements: 7.1, 7.2, 7.3_
  - [x] 12.5 Implement hire applicant handler (`handlers/admin.rs`)
    - `POST /api/admin/applications/:id/hire`: require Admin; validate `probation_months ∈ [3,6]`; execute the atomic hire transaction (design section 9.3): update application, insert staff, insert staff_lifecycle; call `send_onboarding_email`; return HTTP 201 with `{ staff_id }` or HTTP 500 on rollback
    - _Requirements: 7.4, 7.5, 7.6, 7.7, 7.8_
  - [ ]* 12.6 Write property test for status transition validity — lifecycle (Property 5)
    - For every `(current, next)` lifecycle pair not in the allowed set, assert `apply_transition` returns `Err(InvalidTransition)`
    - **Property 5: Staff lifecycle transitions are acyclic**
    - **Validates: Requirements 8.2**

- [x] 13. Staff Lifecycle and Admin Staff Handlers
  - [x] 13.1 Implement staff lifecycle management handlers (`handlers/admin.rs`)
    - `POST /api/admin/staff/:id/confirm`: require Admin; call `confirm_staff` algorithm (design section 9.4); enforce `Probation → Confirmed` only; return HTTP 200 or HTTP 400 on invalid transition
    - `PATCH /api/admin/staff/:id/role`: require Admin (SuperAdmin required for assigning SuperAdmin role); validate role ∈ `{Staff, Admin, SuperAdmin}`; update staff row; return HTTP 200
    - `GET /api/admin/staff`: require Admin; return `StaffAdmin[]` with all fields except `signing_public_key`
    - _Requirements: 8.1, 8.2, 8.3, 8.4, 8.5, 8.6, 8.7_

- [x] 14. QR Identity Verification Handler
  - [x] 14.1 Implement `GET /api/verify` handler (`handlers/verify.rs`)
    - Extract `token` query param; call `verify_identity_jwt(token, env.QR_SECRET)`; on error return HTTP 401 with generic `{ "error": "Unauthorized" }` — do not distinguish expired vs invalid vs missing staff
    - On success, query staff + lifecycle; call `map_role_to_clearance`; return `IdentityResponse { name, photo_url, identity_status, clearance_level, verified_at }` — never include raw JWT claims
    - _Requirements: 9.4, 9.5, 9.6, 9.7, 19.4_
  - [ ]* 14.2 Write property test for QR token PII exclusion (Property 7)
    - For any staff member, decode the payload of `generate_identity_jwt(staff.id, secret)` and assert the object contains exactly the fields `sub`, `exp`, `iss` — no additional fields
    - **Property 7: QR tokens carry no PII**
    - **Validates: Requirements 9.1, 9.7**
  - [ ]* 14.3 Write property test for clearance level mapping (Property 13)
    - For every `(role, lifecycle_status)` combination, assert `map_role_to_clearance` returns a value ∈ `{1, 2, 3, 4}` matching the defined mapping
    - **Property 13: Clearance level mapping is total and correct**
    - **Validates: Requirements 9.5**

- [x] 15. Public Content and KV Cache Handlers
  - [x] 15.1 Implement KV cache-aside helper (`db/queries.rs`)
    - `get_cached_or_fetch(kv, cache_key, ttl_secs, fetch_fn)`: KV GET → on hit deserialise and return; on miss call `fetch_fn`, write to KV (non-fatal on KV write failure), return value
    - _Requirements: 13.1, 13.2, 13.3, 13.4, 13.5, 13.6_
  - [x] 15.2 Implement public content handlers (`handlers/content.rs`)
    - `GET /api/insights`, `GET /api/work`, `GET /api/capabilities`: return published posts (ordered by `published_at DESC`) using the `get_cached_or_fetch` helper with appropriate cache keys and TTLs
    - `GET /api/insights/:slug`, `GET /api/work/:slug`, `GET /api/capabilities/:slug`: KV_CACHE lookup `post:{slug}` (TTL 300 s); on miss query D1 with author join; return post + author public profile or HTTP 404
    - _Requirements: 10.1, 10.2, 10.3, 10.4, 13.1, 13.4, 13.5, 13.6_
  - [ ]* 15.3 Write property test for published-content filter (Property 17)
    - For any response from `GET /api/insights`, `GET /api/work`, or `GET /api/capabilities`, assert every post in the array has `published = true`
    - **Property 17: Published-content filter is total**
    - **Validates: Requirements 10.2**
  - [x] 15.4 Implement admin content management handlers (`handlers/admin.rs`)
    - `POST /api/admin/content`: require Admin; generate unique slug; insert post with `published = false`; return HTTP 201
    - `PATCH /api/admin/content/:id`: require Admin; update fields, refresh `updated_at`; validate slug uniqueness per type (return HTTP 409 on conflict); return HTTP 200
    - `DELETE /api/admin/content/:id`: require SuperAdmin; delete post; return HTTP 204
    - _Requirements: 10.5, 10.6, 10.7, 10.8_
  - [ ]* 15.5 Write property test for cache-aside consistency (Property 8)
    - For any cache key whose TTL has expired or been invalidated, assert the next `get_cached_or_fetch` call invokes `fetch_fn` (D1 path) rather than returning a stale value
    - Assert that a cache hit never invokes `fetch_fn`
    - **Property 8: Cache-aside consistency**
    - **Validates: Requirements 13.4, 13.5**

- [x] 16. Team Profiles Handler
  - [x] 16.1 Implement team profile handlers (`handlers/team.rs`)
    - `GET /api/team`: KV_CACHE lookup `team:list` (TTL 120 s); on miss query `SELECT username, name, job_title, bio, avatar_url FROM staff WHERE onboarding_completed=1`; populate cache; return `StaffPublicProfile[]`
    - `GET /api/team/:username`: query staff by username; return `StaffPublicProfile` or HTTP 404; ensure response never includes `email`, `role`, `id`, or `signing_public_key`
    - _Requirements: 11.1, 11.2, 11.3, 11.4, 13.2_
  - [ ]* 16.2 Write property test for public profile field exclusion (Property 16)
    - For any staff member response from `GET /api/team` or `GET /api/team/:username`, assert the JSON object contains none of `email`, `role`, `id`, `signing_public_key`
    - **Property 16: Public team profiles exclude sensitive fields**
    - **Validates: Requirements 11.4**

- [x] 17. R2 Storage, MIME Validation, and Media Upload
  - [x] 17.1 Implement R2 helpers and MIME validator (`storage/r2.rs`, `storage/validator.rs`)
    - `detect_mime_from_magic_bytes(bytes: &[u8]) -> Option<&'static str>`: inspect first 12 bytes for JPEG (`FF D8 FF`), PNG (`89 50 4E 47`), WebP (`52 49 46 46 … 57 45 42 50`), AVIF, PDF, DOCX/DOC magic signatures; return MIME or `None`
    - `sanitise_filename(name: &str) -> String`: strip all `/` and `..` sequences
    - `r2_put_public(bucket, key, bytes, mime, cache_control) -> Result<String, WorkerError>`: put object with `Cache-Control: public, max-age=31536000, immutable`; return `https://media.ezeroandone.com/{key}`
    - `generate_presigned_url(bucket, key, ttl_secs) -> Result<String, WorkerError>`: validate key starts with `applications/`; call `bucket.create_presigned_url(key, ttl_secs)`
    - _Requirements: 12.2, 12.6, 12.7, 12.9, 12.11, 19.5, 19.6_
  - [x] 17.2 Implement image upload handlers (`handlers/admin.rs`, `handlers/onboarding.rs`)
    - `POST /api/upload/avatar`: require Staff; validate MIME via magic bytes and size ≤ 4 MB; store at `avatars/{staff_id}.{ext}`; return `{ url }`
    - `POST /api/upload/post/:id/cover`, `POST /api/upload/post/:id/media`: require Admin; same validation; store at `posts/{id}/cover.{ext}` or `posts/{id}/media/{filename}`
    - `POST /api/upload/career/:id/hero`: require Admin; store at `careers/{id}/hero.{ext}`; return `{ url }`
    - _Requirements: 12.1, 12.2, 12.3, 12.4, 12.5, 12.6, 12.7, 12.8_
  - [x] 17.3 Implement document presigned URL handlers (`handlers/admin.rs`)
    - `GET /api/admin/applications/:id/documents`: require Admin; query `application_document` for `application_id`; return `DocumentMeta[]`
    - `GET /api/admin/applications/:id/documents/:doc_id/url`: require Admin; fetch `r2_key` from D1; validate key starts with `applications/`; call `generate_presigned_url(key, 300)`; return `{ url, expires_at }`
    - _Requirements: 12.9, 12.10, 12.11, 12.12_
  - [ ]* 17.4 Write property test for application document access restriction (Property 9)
    - For any caller with role < Admin, assert presigned URL generation returns `Err(Forbidden)`
    - For any R2 key not starting with `applications/`, assert presigned URL generation returns `Err(Forbidden)` regardless of caller role
    - **Property 9: Application document access is restricted**
    - **Validates: Requirements 6.10, 12.9, 12.10, 12.11**
  - [ ]* 17.5 Write property test for filename sanitisation (Property 19)
    - For any filename string, assert `sanitise_filename(name)` contains neither `/` nor `..`
    - **Property 19: Filename sanitisation removes path traversal sequences**
    - **Validates: Requirements 6.9, 19.6**

- [x] 18. Checkpoint — Backend complete
  - Ensure `cargo check --target wasm32-unknown-unknown` passes with zero errors
  - Ensure all `proptest` property tests pass with `cargo test`
  - Ask the user if any questions arise before proceeding to the frontend

- [x] 19. SvelteKit Frontend — Shared Utilities and Theme System
  - [x] 19.1 Implement the theme store (`src/lib/stores/theme.ts`)
    - Create `writable<Theme>` store initialised from `localStorage.getItem('ezo-theme') ?? 'dark'` on the browser
    - Subscribe to store changes: write to `localStorage` under key `ezo-theme` and set `document.documentElement.setAttribute('data-theme', val)`
    - Export `toggleTheme()` helper
    - _Requirements: 16.1, 16.2, 16.3, 16.6_
  - [x] 19.2 Implement global CSS custom properties and GlassCard component
    - Write `src/app.css` with `:root[data-theme="dark"]` and `:root[data-theme="light"]` blocks containing all CSS variables from design section 13.1 (glass morphism, neon accents, surface colours)
    - Create `src/lib/components/GlassCard.svelte` with `accentColor`, `blur` props; apply `.glass-card` styles with hover scale + neon shadow transition from design section 13.2
    - Create `src/lib/components/FloatingHeader.svelte` with `floatingHeader` Svelte action for scroll-driven compact state (design section 13.3)
    - _Requirements: 16.4, 16.5_
  - [x] 19.3 Implement shared TypeScript API client utilities (`src/lib/api.ts`)
    - Define all shared response interfaces from design section 12.7: `Post`, `StaffPublicProfile`, `Career`, `ApplicationSubmission`, `IdentityResponse`
    - Implement `apiFetch(path, options)`: wrapper that forwards the `session` cookie and throws the SvelteKit `error(status)` on non-OK responses
    - _Requirements: 15.2, 15.3, 20.4_

- [x] 20. SvelteKit Frontend — Public Routes
  - [x] 20.1 Implement public content page server load functions
    - `src/routes/insights/+page.server.ts`, `src/routes/work/+page.server.ts`, `src/routes/capabilities/+page.server.ts`: fetch list from Worker API; throw on non-OK
    - `src/routes/insights/[slug]/+page.server.ts`, `src/routes/work/[slug]/+page.server.ts`, `src/routes/capabilities/[slug]/+page.server.ts`: fetch single post including author; return `{ post }`
    - Render each `+page.svelte` with SSR using returned data
    - _Requirements: 15.1, 15.2, 15.3_
  - [x] 20.2 Implement team directory and profile pages
    - `src/routes/team/+page.server.ts`: fetch `GET /api/team`; return `{ staff }`
    - `src/routes/team/[username]/+page.server.ts`: fetch `GET /api/team/:username`; return `{ profile }` or throw 404
    - In `src/routes/team/[username]/+page.svelte`: render staff profile and a QR code widget that encodes the identity JWT fetched from the server load function (call `GET /api/verify` endpoint indirectly via a QR display library showing the JWT URL)
    - _Requirements: 11.1, 11.2, 11.3, 11.5, 15.1_
  - [x] 20.3 Implement careers listing and application form pages
    - `src/routes/careers/+page.server.ts`: fetch `GET /api/careers`; return `{ careers }`
    - `src/routes/careers/[slug]/+page.server.ts`: fetch `GET /api/careers/:slug`; return `{ career }` or throw 404
    - In `src/routes/careers/[slug]/+page.svelte`: render job detail and an application form that POSTs to `POST /api/careers/:slug/apply` then optionally uploads documents via `POST /api/careers/:slug/apply/documents`
    - _Requirements: 15.1, 15.2, 15.3_

- [x] 21. SvelteKit Frontend — Auth and Onboarding Routes
  - [x] 21.1 Implement auth login and callback pages
    - `src/routes/auth/login/+page.svelte`: render magic-link email entry form; POST to `POST /api/auth/request`; display success/error feedback
    - `src/routes/auth/callback/+page.server.ts`: on page load forward the `?token` query param to `GET /api/auth/callback`; the Worker sets the session cookie and returns the redirect; handle 401 by rendering an error page
    - _Requirements: 1.6, 15.1_
  - [x] 21.2 Implement onboarding wizard page
    - `src/routes/onboarding/+page.server.ts`: require authenticated session; fetch `GET /api/onboarding/status`; return `{ step, completed }`
    - `src/routes/onboarding/+page.svelte`: render a three-step wizard component; Step 1 submits profile fields via `PATCH /api/onboarding/profile`; Step 2 submits PEM key via `POST /api/onboarding/signing-key`; Step 3 calls `POST /api/onboarding/complete` then redirects to `/dashboard`
    - _Requirements: 4.3, 4.4, 4.5, 4.8, 4.9, 15.6_

- [x] 22. SvelteKit Frontend — Admin Portal Routes
  - [x] 22.1 Implement admin layout guard and dashboard
    - `src/routes/(admin)/+layout.server.ts`: require authenticated session with role ≥ Admin; redirect to `/auth/login` if session absent or insufficient role
    - `src/routes/(admin)/dashboard/+page.svelte`: render admin dashboard shell with sidebar navigation (staff, careers, content links)
    - _Requirements: 15.4, 15.5_
  - [x] 22.2 Implement admin staff management page
    - `src/routes/(admin)/staff/+page.server.ts`: fetch `GET /api/admin/staff`; return `{ staff }`
    - `src/routes/(admin)/staff/+page.svelte`: render staff table with confirm-probation and update-role actions; POST to appropriate Worker endpoints on action
    - _Requirements: 8.3, 8.5, 15.4_
  - [x] 22.3 Implement admin careers and applications pages
    - `src/routes/(admin)/careers/+page.server.ts`: fetch `GET /api/admin/careers` and render career management UI with create/edit forms
    - Render application list per career with status transition dropdowns calling `PATCH /api/admin/applications/:id/status`; add hire action calling `POST /api/admin/applications/:id/hire`
    - _Requirements: 5.5, 5.6, 5.7, 7.3, 7.6, 15.4_
  - [x] 22.4 Implement admin content management page
    - `src/routes/(admin)/content/+page.svelte`: render post list with create, edit, and (SuperAdmin) delete actions; forms call `POST /api/admin/content`, `PATCH /api/admin/content/:id`, `DELETE /api/admin/content/:id`
    - _Requirements: 10.5, 10.6, 10.7, 15.4_
  - [ ]* 22.5 Write Svelte component tests for theme toggle
    - Use `@testing-library/svelte` to render the theme toggle; simulate click; assert `data-theme` attribute changes on `<html>` and `localStorage` is updated
    - _Requirements: 16.2, 16.3_
  - [ ]* 22.6 Write fast-check property tests for frontend API deserialisation
    - Use `fast-check` to generate arbitrary `Post`, `StaffPublicProfile`, `Career`, `IdentityResponse` objects; serialise to JSON and parse through the TypeScript interfaces; assert no fields are dropped or type-coerced
    - _Requirements: 20.4_

- [x] 23. Security Hardening and XSS Prevention
  - [x] 23.1 Audit all SvelteKit templates for `{@html}` directive usage
    - Search the entire `src/` directory for `{@html}` directives on user-supplied content (bios, post body_md rendered HTML, applicant cover letters); replace with safe text rendering or a sanitising Svelte action
    - Ensure all Markdown rendering uses a sanitised renderer that strips script tags and event handlers before output
    - _Requirements: 19.8_
  - [x] 23.2 Verify session cookie security attributes in Worker response headers
    - Add an integration test that calls `GET /api/auth/callback` with a valid token and asserts the `Set-Cookie` header contains `HttpOnly`, `Secure`, and `SameSite=Strict`
    - _Requirements: 2.5, 19.9_

- [x] 24. Final Checkpoint — End-to-end wiring and test suite
  - Ensure `cargo build --target wasm32-unknown-unknown --release` produces a `.wasm` binary under 10 MB
  - Ensure `npm run build` in the SvelteKit project completes without TypeScript errors
  - Run the full Rust test suite (`cargo test`) and all Vitest frontend tests (`vitest --run`)
  - Ask the user if any questions arise before considering the implementation complete

---

## Notes

- Tasks marked with `*` are optional and can be skipped for a faster MVP delivery. All property and unit test sub-tasks carry `*`.
- Each task references specific requirements for full traceability back to the requirements document.
- Checkpoints at tasks 7, 18, and 24 gate progress and must pass before the next phase begins.
- **Language split**: Rust handles all backend Worker logic; TypeScript/Svelte handles all frontend Pages logic. No language ambiguity.
- Property tests use `proptest` (Rust backend) and `fast-check` (TypeScript frontend) as specified in the design testing strategy (section 16.2).
- All D1 queries throughout the implementation MUST use positional parameters — SQL string interpolation is forbidden (Requirement 14.5).
- The WASM binary release profile (`opt-level="z"`, `lto=true`, `codegen-units=1`, `panic="abort"`, `strip=true`) is set in task 1.1 and must not be changed in later tasks.
- MIME type validation (tasks 12.1, 17.1, 17.2) MUST inspect magic bytes server-side; client-supplied `Content-Type` headers must not be trusted (Requirement 19.5).
- The `JWT_SECRET` and `QR_SECRET` bindings must never appear in source code or WASM binary — they are injected at runtime via Wrangler secret bindings (Requirement 2.6).
- KV write failures in the cache-aside pattern (task 15.1) are non-fatal: log internally and return the D1 value to the caller (Requirement 13.6).
- The `applications/` R2 prefix is private by design. No task should add a public CDN mapping for that prefix (Requirement 6.10, 19.7).
- Task 11.4 (`GET /api/admin/applications`) was added to cover the missing endpoint from design section 12.4 (API surface).
- Task 15.5 (`cache-aside consistency`) was added to cover Property 8 (design Correctness Properties section), which had no corresponding test task.

## Task Dependency Graph

```json
{
  "waves": [
    { "id": 0, "tasks": ["1.1", "1.2", "1.3"] },
    { "id": 1, "tasks": ["2.1", "3.1"] },
    { "id": 2, "tasks": ["2.2", "3.2", "4.1", "4.3", "4.6"] },
    { "id": 3, "tasks": ["4.2", "4.4", "4.5", "5.1", "5.2", "6.1", "6.2", "9.1"] },
    { "id": 4, "tasks": ["5.3", "5.4", "5.5", "5.6", "6.3", "8.1", "8.2", "8.3", "10.1", "10.2", "10.3", "10.4"] },
    { "id": 5, "tasks": ["8.4", "8.5", "11.1", "11.2", "11.3", "11.4", "13.1", "14.1", "15.1", "16.1", "17.1"] },
    { "id": 6, "tasks": ["11.5", "11.6", "12.1", "12.4", "12.5", "13.2", "14.2", "14.3", "15.2", "15.4", "15.5", "16.2", "17.2", "17.3"] },
    { "id": 7, "tasks": ["12.2", "12.3", "12.6", "15.3", "17.4", "17.5"] },
    { "id": 8, "tasks": ["19.1", "19.2", "19.3"] },
    { "id": 9, "tasks": ["20.1", "20.2", "20.3", "21.1", "21.2"] },
    { "id": 10, "tasks": ["22.1"] },
    { "id": 11, "tasks": ["22.2", "22.3", "22.4"] },
    { "id": 12, "tasks": ["22.5", "22.6", "23.1", "23.2"] },
    { "id": 13, "tasks": ["24"] }
  ]
}
```
