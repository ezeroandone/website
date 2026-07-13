# Requirements Document

## Introduction

eZeroAndOne.io is a Cloudflare-native corporate platform built on SvelteKit (Cloudflare Pages),
a Rust/WASM Cloudflare Worker, Cloudflare D1 (SQLite), Cloudflare KV, and Cloudflare R2. The
platform combines a public-facing website (insights, work, capabilities, team profiles, careers)
with a secured staff portal featuring passwordless magic-link authentication, role-based access
control (RBAC), a staff onboarding wizard, cryptographic identity QR codes, and a full HR
applicant lifecycle pipeline — all operated within Cloudflare Free Tier CPU and size budgets.

---

## Glossary

- **Worker**: The Cloudflare Worker running the Rust/WASM binary that handles all API business logic.
- **Pages**: The Cloudflare Pages project serving the SvelteKit frontend.
- **D1**: Cloudflare D1 relational SQLite database (`ezeroandone-db`).
- **KV_AUTH**: Cloudflare KV namespace `EZO_AUTH` — stores magic-link tokens and rate-limit counters.
- **KV_CACHE**: Cloudflare KV namespace `EZO_CACHE` — stores serialised response cache entries.
- **R2**: Cloudflare R2 bucket `EZO_MEDIA` — stores binary media assets and application documents.
- **Magic_Link_System**: The passwordless authentication subsystem (token generation, email dispatch, callback exchange).
- **Auth_Middleware**: The Worker middleware that validates session cookies and enforces RBAC.
- **Onboarding_Wizard**: The three-step onboarding flow a newly hired staff member must complete.
- **HR_Pipeline**: The applicant lifecycle management subsystem (career listings, applications, status transitions).
- **QR_Identity_System**: The cryptographic staff identity verification subsystem using short-lived JWTs.
- **RBAC**: Role-Based Access Control with roles Public (1), Staff (2), Admin (3), SuperAdmin (4).
- **Staff**: An authenticated platform user with role Staff, Admin, or SuperAdmin.
- **Applicant**: A public user who submits a job application.
- **Content_System**: The post management subsystem covering insights, work, and capabilities post types.
- **Cache_Aside**: The KV cache-aside pattern used to reduce D1 read load.
- **Presigned_URL**: A time-limited R2 URL granting temporary read access to a private object.
- **Session_Cookie**: An HttpOnly, Secure, SameSite=Strict cookie carrying a signed HS256 JWT.
- **JWT_Secret**: A 32-byte minimum secret stored in the Worker's secret binding, never in code.
- **QR_Secret**: A separate secret from JWT_Secret, used exclusively to sign identity QR tokens.
- **Corporate_Domain**: An email domain in the server-side allowlist (e.g., `@ezeroandone.com`).
- **ApplicationStatus**: One of `Applied`, `Interviewing`, `Offered`, `Hired`, `Rejected`.
- **StaffLifecycleStatus**: One of `Probation`, `Confirmed`, `Inactive`.
- **ClearanceLevel**: Integer 1–4 derived from role and lifecycle status for QR verification responses.
- **WASM_Binary**: The compiled Rust `.wasm` file deployed to the Worker.
- **Rate_Limiter**: The per-IP request throttle implemented via KV_AUTH counters.
- **Pretty_Printer**: Not applicable — no user-defined grammar; JSON serialisation/deserialisation covers round-trip requirements.

---

## Requirements

### Requirement 1: Passwordless Magic-Link Authentication

**User Story:** As a staff member, I want to log in using a magic link sent to my corporate email,
so that I can access the platform without managing a password.

#### Acceptance Criteria

1. WHEN a staff member submits an email address to `POST /api/auth/request`, THE Magic_Link_System SHALL validate that the email belongs to a Corporate_Domain before proceeding.
2. IF the submitted email does not belong to a Corporate_Domain, THEN THE Magic_Link_System SHALL return HTTP 403 without revealing which domains are accepted.
3. WHEN a valid corporate email is received, THE Magic_Link_System SHALL generate a cryptographically secure 64-character lowercase hexadecimal token using the Web Crypto CSPRNG.
4. WHEN a token is generated, THE Magic_Link_System SHALL store the token in KV_AUTH under key `ml:{token}` with a TTL of exactly 900 seconds (15 minutes).
5. WHEN a token is stored, THE Magic_Link_System SHALL dispatch a magic-link email to the staff member containing the callback URL `https://ezeroandone.io/auth/callback?token={token}`.
6. WHEN a staff member visits the callback URL, THE Magic_Link_System SHALL exchange the token by calling `GET /api/auth/callback?token={token}`.
7. WHEN the Worker receives a callback request, THE Magic_Link_System SHALL retrieve the token from KV_AUTH and immediately delete it, ensuring the token is single-use.
8. IF the token is not found in KV_AUTH or has expired, THEN THE Magic_Link_System SHALL return HTTP 401.
9. WHEN a valid token is exchanged, THE Magic_Link_System SHALL load or create a staff record matching the email, sign an HS256 JWT containing `{ sub, email, role, onboarded, exp }`, and set a `session` cookie that is HttpOnly, Secure, and SameSite=Strict.
10. WHEN a successful session cookie is issued, THE Magic_Link_System SHALL redirect the user to `/onboarding` if `onboarding_completed = false`, otherwise to `/dashboard`.
11. WHEN a staff member calls `POST /api/auth/logout`, THE Magic_Link_System SHALL clear the session cookie and return HTTP 200.
12. WHEN an IP address exceeds 5 auth requests within a 60-second window, THE Rate_Limiter SHALL return HTTP 429 for subsequent requests within that window.

---

### Requirement 2: JWT Session Management

**User Story:** As the platform, I want all session tokens to be verifiable and tamper-proof,
so that authenticated requests are trustworthy.

#### Acceptance Criteria

1. THE Worker SHALL sign all session JWTs using HS256 with a secret of at least 32 bytes stored in the Worker's secret binding.
2. WHEN verifying a JWT, THE Auth_Middleware SHALL reject tokens with an invalid HMAC signature and return HTTP 401.
3. WHEN verifying a JWT, THE Auth_Middleware SHALL reject tokens whose `exp` claim is less than or equal to the current Unix timestamp and return HTTP 401.
4. THE Auth_Middleware SHALL use constant-time comparison when verifying HMAC signatures to prevent timing attacks.
5. IF a session cookie is absent on a protected route, THEN THE Auth_Middleware SHALL return HTTP 401.
6. THE Worker SHALL never embed the JWT_Secret or QR_Secret in the WASM binary or source code.

---

### Requirement 3: Role-Based Access Control (RBAC)

**User Story:** As a platform administrator, I want all API routes to enforce minimum role requirements,
so that staff cannot access resources beyond their permission level.

#### Acceptance Criteria

1. THE RBAC system SHALL define four role levels: Public (1), Staff (2), Admin (3), SuperAdmin (4).
2. WHEN a request arrives at a protected route, THE Auth_Middleware SHALL verify that the authenticated user's role level is greater than or equal to the route's required role level.
3. IF the user's role level is below the required level, THEN THE Auth_Middleware SHALL return HTTP 403.
4. THE Worker SHALL require at minimum Public role (no authentication) for all `GET /api/insights/*`, `GET /api/work/*`, `GET /api/capabilities/*`, `GET /api/team/*`, `GET /api/careers/*`, `POST /api/careers/*/apply`, `POST /api/auth/*`, `GET /api/auth/callback`, `GET /api/verify`, and `POST /api/careers/*/apply/documents` routes.
5. THE Worker SHALL require at minimum Staff role for `GET /api/onboarding/*`, `PATCH /api/onboarding/*`, and `POST /api/upload/avatar` routes.
6. THE Worker SHALL require at minimum Admin role for all `GET /api/admin/*`, `POST /api/admin/*`, `PATCH /api/admin/*`, `POST /api/upload/post/*`, `POST /api/upload/career/*`, and `GET /api/admin/applications/*/documents*` routes.
7. THE Worker SHALL require SuperAdmin role for all `DELETE /api/admin/*` routes.
8. WHERE a role is higher in the hierarchy, THE Auth_Middleware SHALL grant all permissions held by lower roles (monotone access).

---

### Requirement 4: Staff Onboarding Wizard

**User Story:** As a newly hired staff member, I want a guided onboarding wizard,
so that I can complete my profile, provision my signing key, and acknowledge the company briefing before accessing the platform.

#### Acceptance Criteria

1. WHEN a staff member is authenticated and `onboarding_completed = false`, THE Onboarding_Wizard SHALL redirect all non-bypass requests to `/onboarding` with HTTP 302.
2. THE Onboarding_Wizard SHALL treat the following paths as bypass paths exempt from the onboarding redirect: all `/api/auth/*`, all `/api/onboarding/*`, and all `/auth/*` routes.
3. THE Onboarding_Wizard SHALL present three sequential steps: Step 1 (profile details), Step 2 (signing key provisioning), Step 3 (company briefing acknowledgement).
4. WHEN a staff member submits Step 1 via `PATCH /api/onboarding/profile`, THE Onboarding_Wizard SHALL update the staff record's `name`, `job_title`, `bio`, and `avatar_url` fields and return HTTP 200.
5. WHEN a staff member submits Step 2 via `POST /api/onboarding/signing-key`, THE Onboarding_Wizard SHALL validate that the submitted `public_key_pem` is a valid PEM-encoded Ed25519 or ECDSA P-256 public key.
6. IF the submitted public key PEM is invalid, THEN THE Onboarding_Wizard SHALL return HTTP 400 with a descriptive validation error.
7. WHEN a valid public key PEM is submitted, THE Onboarding_Wizard SHALL store it in the `staff.signing_public_key` column and refresh `updated_at`.
8. WHEN a staff member submits Step 3 via `POST /api/onboarding/complete`, THE Onboarding_Wizard SHALL set `onboarding_completed = true` on the staff record and return HTTP 200.
9. WHEN onboarding is completed, THE Worker SHALL redirect the user to `/dashboard`.
10. WHEN `GET /api/onboarding/status` is called, THE Onboarding_Wizard SHALL return the current step number (1, 2, or 3) and whether onboarding is completed.


---

### Requirement 5: HR Pipeline — Career Listings

**User Story:** As an administrator, I want to manage job listings,
so that the careers page always reflects open positions and can be updated without code changes.

#### Acceptance Criteria

1. THE Content_System SHALL store career listings in D1 with fields: `id`, `slug` (unique), `title`, `description_md`, `department`, `type`, `active`, `created_at`.
2. THE Worker SHALL accept `type` values of `Full-Time`, `Part-Time`, `Contract`, and `Internship` only; all other values SHALL be rejected with HTTP 400.
3. WHEN `GET /api/careers` is called, THE Worker SHALL return only career rows where `active = true`.
4. WHEN `GET /api/careers/:slug` is called and the slug does not match any career, THE Worker SHALL return HTTP 404.
5. WHEN an Admin submits `POST /api/admin/careers`, THE Worker SHALL create a new career listing and return HTTP 201 with the created resource.
6. WHEN an Admin submits `PATCH /api/admin/careers/:id`, THE Worker SHALL update the specified career listing and return HTTP 200.
7. WHEN `GET /api/admin/careers` is called by an Admin, THE Worker SHALL return all career listings regardless of active status.

---

### Requirement 6: HR Pipeline — Job Applications

**User Story:** As a public visitor, I want to apply for open positions,
so that I can submit my interest and supporting documents to the company.

#### Acceptance Criteria

1. WHEN a visitor submits `POST /api/careers/:slug/apply` with `{ applicantName, applicantEmail, coverLetter }`, THE HR_Pipeline SHALL create an application record with status `Applied` and return HTTP 201.
2. IF the career referenced by `:slug` does not exist or has `active = false`, THEN THE HR_Pipeline SHALL return HTTP 404.
3. WHEN `POST /api/careers/:slug/apply/documents` is called with a multipart file, THE HR_Pipeline SHALL validate that the MIME type is one of `application/pdf`, `application/msword`, or `application/vnd.openxmlformats-officedocument.wordprocessingml.document`.
4. IF the uploaded document MIME type is not in the allowed list, THEN THE HR_Pipeline SHALL return HTTP 400 with an error indicating an invalid file type.
5. WHEN a document upload is received, THE HR_Pipeline SHALL validate the file size is at most 10 MB (10,485,760 bytes).
6. IF the uploaded document exceeds 10 MB, THEN THE HR_Pipeline SHALL return HTTP 400 with a file-too-large error.
7. WHEN a document upload passes validation, THE HR_Pipeline SHALL store the file in R2 at key `applications/{application_id}/{sanitised_filename}` and insert an `application_document` row in D1.
8. IF an application already has 3 documents attached, THEN THE HR_Pipeline SHALL reject any further document uploads with HTTP 400.
9. THE HR_Pipeline SHALL sanitise filenames before storage to remove path traversal sequences (`/`, `..`).
10. WHEN documents are stored, THE HR_Pipeline SHALL never make the R2 path `applications/` publicly accessible.

---

### Requirement 7: HR Pipeline — Applicant Lifecycle Transitions

**User Story:** As an administrator, I want to move applicants through the hiring pipeline,
so that I can track progress from application to hire or rejection.

#### Acceptance Criteria

1. THE HR_Pipeline SHALL enforce the following allowed status transitions: `Applied → Interviewing`, `Interviewing → Offered`, `Interviewing → Rejected`, `Offered → Hired`, `Offered → Rejected`.
2. IF an Admin attempts a status transition that is not in the allowed set, THEN THE HR_Pipeline SHALL return HTTP 400 with an invalid-transition error.
3. WHEN an Admin calls `PATCH /api/admin/applications/:id/status` with an allowed target status, THE HR_Pipeline SHALL update the application's `status` and `updated_at` fields and return HTTP 200.
4. WHEN an Admin calls `POST /api/admin/applications/:id/hire` with `{ probation_months }`, THE HR_Pipeline SHALL accept only values in the range 3–6 inclusive.
5. IF `probation_months` is outside the range 3–6, THEN THE HR_Pipeline SHALL return HTTP 400.
6. WHEN a valid hire request is processed, THE HR_Pipeline SHALL atomically: set `application.status = 'Hired'`, create a `staff` row with `role = 'Staff'` and `onboarding_completed = false`, create a `staff_lifecycle` row with `status = 'Probation'` and `probation_end` computed as `now + probation_months × 30 × 86400 seconds`, and dispatch an onboarding email to the new corporate address.
7. WHEN a hire transaction succeeds, THE HR_Pipeline SHALL return HTTP 201 with the new `staff_id`.
8. IF any step of the hire atomic transaction fails, THEN THE HR_Pipeline SHALL roll back all changes and return HTTP 500.

---

### Requirement 8: Staff Lifecycle Management

**User Story:** As an administrator, I want to manage the staff lifecycle from probation to confirmed or inactive,
so that I can accurately reflect employment status across the platform.

#### Acceptance Criteria

1. THE Worker SHALL create a `staff_lifecycle` row with `status = 'Probation'` whenever a new staff member is created via the hire flow.
2. THE Worker SHALL enforce the following lifecycle transitions: `Probation → Confirmed`, `Probation → Inactive`, `Confirmed → Inactive`; all other transitions SHALL be rejected with HTTP 400.
3. WHEN an Admin calls `POST /api/admin/staff/:id/confirm` on a staff member whose lifecycle status is `Probation`, THE Worker SHALL set `staff_lifecycle.status = 'Confirmed'` and `confirmed_at = now()`.
4. IF the target staff member's lifecycle status is not `Probation`, THEN THE Worker SHALL return HTTP 400 with an invalid-transition error.
5. WHEN an Admin updates a staff member's role via `PATCH /api/admin/staff/:id/role`, THE Worker SHALL accept only `Staff`, `Admin`, or `SuperAdmin` as valid values.
6. IF an invalid role value is provided, THEN THE Worker SHALL return HTTP 400.
7. THE Worker SHALL require SuperAdmin role to assign the `SuperAdmin` role to another staff member.


---

### Requirement 9: Cryptographic Identity QR Verification

**User Story:** As an external verifier, I want to scan a staff member's QR code and receive a verified identity response,
so that I can confirm the person's employment status and clearance level.

#### Acceptance Criteria

1. THE QR_Identity_System SHALL generate a short-lived HS256 JWT for each staff profile page, containing only `{ sub: staff_id, exp, iss: "ezo-identity" }` — no personally identifiable information beyond the staff ID.
2. WHEN generating a QR token, THE QR_Identity_System SHALL set `exp` to `now + 300 seconds` (5 minutes).
3. THE QR_Identity_System SHALL sign identity tokens using QR_Secret, which is a separate secret from JWT_Secret.
4. WHEN `GET /api/verify?token=<jwt>` is called with a valid, non-expired token, THE QR_Identity_System SHALL return a JSON response containing `{ name, photo_url, identity_status, clearance_level, verified_at }`.
5. THE QR_Identity_System SHALL compute `clearance_level` as: 1 for any non-Confirmed staff member, 2 for Confirmed Staff, 3 for Confirmed Admin, 4 for Confirmed SuperAdmin.
6. IF the QR token is expired, has an invalid signature, or references a non-existent staff member, THEN THE QR_Identity_System SHALL return HTTP 401 with a generic error — without distinguishing between the three failure reasons.
7. WHEN the QR_Identity_System returns a verification response, THE Worker SHALL never include the raw JWT claims in the response body.

---

### Requirement 10: Public Content — Insights, Work, and Capabilities

**User Story:** As a public visitor, I want to browse the company's articles, portfolio, and services,
so that I can learn about the company's work and expertise.

#### Acceptance Criteria

1. THE Content_System SHALL store posts in D1 with a `type` field constrained to `insight`, `work`, or `capability`.
2. WHEN `GET /api/insights`, `GET /api/work`, or `GET /api/capabilities` is called, THE Content_System SHALL return only posts where `published = true`, ordered by `published_at` descending.
3. WHEN `GET /api/insights/:slug`, `GET /api/work/:slug`, or `GET /api/capabilities/:slug` is called, THE Content_System SHALL return the matching post including its author's public profile.
4. IF no post exists for the requested slug, THEN THE Content_System SHALL return HTTP 404.
5. WHEN an Admin creates a post via `POST /api/admin/content`, THE Content_System SHALL generate a unique slug, persist the post with `published = false` by default, and return HTTP 201.
6. WHEN an Admin updates a post via `PATCH /api/admin/content/:id`, THE Content_System SHALL update the specified fields and refresh `updated_at`.
7. WHEN a SuperAdmin deletes a post via `DELETE /api/admin/content/:id`, THE Content_System SHALL remove the post record and return HTTP 204.
8. THE Content_System SHALL reject post slugs that are not unique across the post type with HTTP 409.

---

### Requirement 11: Team Profiles

**User Story:** As a public visitor, I want to view the team directory and individual staff profiles,
so that I can learn about the people behind the company.

#### Acceptance Criteria

1. WHEN `GET /api/team` is called, THE Worker SHALL return all staff members who have `onboarding_completed = true`, including their public profile fields: `username`, `name`, `job_title`, `bio`, `avatarUrl`.
2. WHEN `GET /api/team/:username` is called, THE Worker SHALL return the public profile for the matching staff member.
3. IF no staff member exists with the requested username, THEN THE Worker SHALL return HTTP 404.
4. THE Worker SHALL not expose staff `email`, `role`, `id`, or `signing_public_key` in public team profile responses.
5. WHEN a staff member's profile page is rendered, THE Pages frontend SHALL display a QR code widget containing the identity JWT for that staff member.

---

### Requirement 12: Media and File Upload

**User Story:** As a staff member or administrator, I want to upload images and documents,
so that the platform can display rich content and process applications.

#### Acceptance Criteria

1. THE Worker SHALL accept image uploads at `POST /api/upload/avatar`, `POST /api/upload/post/:id/cover`, `POST /api/upload/post/:id/media`, and `POST /api/upload/career/:id/hero` as `multipart/form-data`.
2. WHEN an image upload is received, THE Worker SHALL validate the MIME type is one of `image/jpeg`, `image/png`, `image/webp`, or `image/avif` by inspecting the file's magic bytes, not the client-supplied Content-Type header.
3. IF the image MIME type is not in the allowed list, THEN THE Worker SHALL return HTTP 400.
4. WHEN an image upload is received, THE Worker SHALL validate the file size is at most 4 MB (4,194,304 bytes).
5. IF an image upload exceeds 4 MB, THEN THE Worker SHALL return HTTP 400 with a file-too-large error.
6. WHEN an image upload passes validation, THE Worker SHALL store the file in R2 at the following deterministic paths: `avatars/{staff_id}.{ext}` for avatars, `posts/{post_id}/cover.{ext}` for post covers, `posts/{post_id}/media/{sanitised_filename}` for inline post images, `careers/{career_id}/hero.{ext}` for career hero images.
7. WHEN a public image is stored in R2, THE Worker SHALL set `Cache-Control: public, max-age=31536000, immutable` on the R2 object metadata and return the public CDN URL (`https://media.ezeroandone.com/{r2_key}`).
8. THE Worker SHALL require at minimum Staff role for avatar uploads and at minimum Admin role for post and career image uploads.
9. WHEN an Admin requests a document presigned URL via `GET /api/admin/applications/:id/documents/:doc_id/url`, THE Worker SHALL generate a time-limited R2 presigned URL with a TTL of 300 seconds.
10. IF the requesting user's role is below Admin, THEN THE Worker SHALL return HTTP 403 for any presigned URL request.
11. THE Worker SHALL only generate presigned URLs for R2 keys that begin with `applications/`; any other key SHALL result in HTTP 403.
12. WHEN `GET /api/admin/applications/:id/documents` is called by an Admin, THE Worker SHALL return the document metadata list (`DocumentMeta[]`) for that application from D1.


---

### Requirement 13: KV Cache-Aside Pattern

**User Story:** As the platform operator, I want frequently read content to be served from KV cache,
so that D1 query costs and CPU time are minimised within Free Tier budgets.

#### Acceptance Criteria

1. WHEN `GET /api/insights/:slug`, `GET /api/work/:slug`, or `GET /api/capabilities/:slug` is called, THE Worker SHALL attempt a KV_CACHE lookup under key `post:{slug}` before querying D1, with a TTL of 300 seconds.
2. WHEN `GET /api/team` is called, THE Worker SHALL attempt a KV_CACHE lookup under key `team:list` before querying D1, with a TTL of 120 seconds.
3. WHEN `GET /api/careers` is called, THE Worker SHALL attempt a KV_CACHE lookup under key `careers:active` before querying D1, with a TTL of 60 seconds.
4. WHEN a cache entry is present, THE Worker SHALL return the cached value without issuing a D1 query.
5. WHEN a cache entry is absent, THE Worker SHALL fetch data from D1, populate the KV_CACHE entry with the appropriate TTL, and return the value.
6. IF a KV_CACHE write fails after a successful D1 fetch, THE Worker SHALL log the failure and return the fetched value — the KV write failure SHALL NOT be surfaced to the caller.

---

### Requirement 14: D1 Database Integrity

**User Story:** As the platform, I want the relational data schema to enforce referential integrity and index all frequent access patterns,
so that data is consistent and queries remain performant.

#### Acceptance Criteria

1. THE D1 database SHALL enforce the `staff_lifecycle` table's `staff_id` as a `UNIQUE` foreign key referencing `staff(id)` with `ON DELETE CASCADE`.
2. THE D1 database SHALL enforce `application_document.application_id` as a foreign key referencing `application(id)` with `ON DELETE CASCADE`.
3. THE D1 database SHALL enforce check constraints: `staff.role IN ('SuperAdmin','Admin','Staff')`, `staff_lifecycle.status IN ('Probation','Confirmed','Inactive')`, `application.status IN ('Applied','Interviewing','Offered','Rejected','Hired')`, `post.type IN ('insight','work','capability')`, `career.type IN ('Full-Time','Part-Time','Contract','Internship')`, `media_asset.context_type IN ('avatar','post_cover','post_media','career_hero')`.
4. THE D1 database SHALL maintain the following indices for query performance: `idx_post_type_slug ON post(type, slug)`, `idx_post_published ON post(published, published_at DESC)`, `idx_application_career ON application(career_id, status)`, `idx_lifecycle_status ON staff_lifecycle(status, probation_end)`, `idx_app_doc_application ON application_document(application_id)`, `idx_media_context ON media_asset(context_type, context_id)`.
5. THE Worker SHALL use positional parameters for all D1 queries; string interpolation in SQL is forbidden.
6. THE Worker SHALL not issue D1 queries that require full table scans on unindexed columns.

---

### Requirement 15: SvelteKit Frontend Routes and SSR

**User Story:** As a public visitor or authenticated staff member, I want the frontend to serve all platform pages
with server-side rendering where applicable, so that public pages are SEO-optimised and authenticated pages are secure.

#### Acceptance Criteria

1. THE Pages project SHALL serve SSR-rendered pages for all public routes: `/`, `/insights`, `/insights/[slug]`, `/work`, `/work/[slug]`, `/capabilities`, `/capabilities/[slug]`, `/team`, `/team/[username]`, `/careers`, `/careers/[slug]`.
2. WHEN a SvelteKit server load function fetches from the Worker API, THE Pages project SHALL forward the `session` cookie in the request headers.
3. IF the Worker API returns a non-OK HTTP status, THE Pages project SHALL throw the corresponding HTTP error to the SvelteKit error boundary.
4. THE Pages project SHALL serve the admin portal routes (`/admin/dashboard`, `/admin/staff`, `/admin/careers`, `/admin/content`) under a protected layout that requires an authenticated session.
5. WHEN a user navigates to an admin route without a valid session, THE Pages project SHALL redirect to `/auth/login`.
6. THE Pages project SHALL serve the onboarding wizard at `/onboarding` and render steps based on the current onboarding status fetched from `GET /api/onboarding/status`.

---

### Requirement 16: Theme System

**User Story:** As a visitor or staff member, I want to toggle between dark and light themes,
so that I can use the platform comfortably in any lighting environment.

#### Acceptance Criteria

1. THE Pages project SHALL default to dark theme when no stored preference exists.
2. WHEN a user selects a theme, THE Pages project SHALL persist the preference to `localStorage` under key `ezo-theme` and apply it immediately.
3. WHEN the theme changes, THE Pages project SHALL set the `data-theme` attribute on the `<html>` element to `dark` or `light`.
4. WHILE dark theme is active, THE Pages project SHALL apply the CSS custom properties defined for `[data-theme="dark"]` including the glass morphism, neon accent, and surface variables.
5. WHILE light theme is active, THE Pages project SHALL apply the CSS custom properties defined for `[data-theme="light"]`.
6. THE Pages project SHALL persist and restore the theme preference across page reloads and navigation.


---

### Requirement 17: Error Handling and HTTP Status Mapping

**User Story:** As an API consumer, I want consistent, predictable error responses,
so that I can handle errors programmatically without leaking internal implementation details.

#### Acceptance Criteria

1. WHEN the Worker encounters a validation error, THE Worker SHALL return HTTP 400 with a JSON body `{ "error": "<descriptive message>" }`.
2. WHEN the Worker encounters an authentication failure, THE Worker SHALL return HTTP 401 with a JSON body `{ "error": "Unauthorized" }`.
3. WHEN the Worker encounters an authorisation failure, THE Worker SHALL return HTTP 403 with a JSON body `{ "error": "Forbidden" }`.
4. WHEN the Worker cannot locate a requested resource, THE Worker SHALL return HTTP 404 with a JSON body `{ "error": "Not Found" }`.
5. WHEN the Worker encounters a rate-limit violation, THE Worker SHALL return HTTP 429 with a JSON body `{ "error": "Too Many Requests" }`.
6. IF the Worker encounters an internal error, THEN THE Worker SHALL log the full error internally and return HTTP 500 with a generic JSON body `{ "error": "Internal Server Error" }` — never leaking stack traces or internal error messages to the caller.
7. THE Worker SHALL add CORS headers to all responses including error responses.
8. WHEN the Worker receives an OPTIONS preflight request, THE Worker SHALL return HTTP 200 with the appropriate CORS headers without invoking any middleware or handler logic.

---

### Requirement 18: Cloudflare Free Tier Optimisation

**User Story:** As the platform operator, I want the Worker to stay within Cloudflare Free Tier CPU and size constraints,
so that the platform can operate without incurring paid plan costs.

#### Acceptance Criteria

1. THE WASM_Binary SHALL not exceed 10 MB in total compiled size.
2. THE Worker SHALL compile Rust with `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `panic = "abort"`, and `strip = true`.
3. THE Worker SHALL complete all request handling within 10 ms of CPU time per invocation.
4. THE Worker SHALL use `async/await` throughout and SHALL NOT use synchronous blocking loops.
5. THE Worker SHALL use `serde` with derived implementations for all JSON serialisation to avoid runtime reflection overhead.
6. WHEN hot or repeated data is needed, THE Worker SHALL prefer KV_CACHE reads over D1 SELECT queries.
7. THE Worker SHALL not issue D1 queries against unindexed columns.

---

### Requirement 19: Security Hardening

**User Story:** As the platform operator, I want the platform to be hardened against common web attack vectors,
so that staff data and application documents are protected.

#### Acceptance Criteria

1. THE Worker SHALL delete a magic-link token from KV_AUTH immediately upon first successful use, ensuring single-use semantics.
2. THE Worker SHALL use constant-time byte comparison for all HMAC signature verification operations.
3. WHEN the Worker validates an email domain for authentication, THE Worker SHALL return the same HTTP 403 response regardless of the specific rejection reason, preventing domain enumeration.
4. WHEN the Worker validates a QR token, THE Worker SHALL return the same HTTP 401 response regardless of whether the token is expired, has an invalid signature, or references a missing staff member.
5. THE Worker SHALL validate uploaded file MIME types by inspecting magic bytes server-side; client-supplied Content-Type headers SHALL NOT be trusted for type validation.
6. THE Worker SHALL sanitise all uploaded filenames to remove `/` and `..` sequences before constructing R2 keys.
7. THE Worker SHALL never expose application document R2 keys or direct object URLs to public callers.
8. THE Pages project SHALL not use `{@html}` directives on any user-supplied content to prevent XSS.
9. THE Session_Cookie SHALL always be set with `HttpOnly`, `Secure`, and `SameSite=Strict` attributes.
10. THE Worker SHALL enforce per-IP rate limiting on all authentication endpoints, permitting at most 5 requests per IP per 60-second window.

---

### Requirement 20: JSON Serialisation Round-Trip

**User Story:** As the platform, I want all data transferred between the Worker and the frontend to be correctly serialised and deserialisable,
so that no data is lost or corrupted across the API boundary.

#### Acceptance Criteria

1. THE Worker SHALL serialise all API responses as valid JSON using `serde_json`.
2. FOR ALL valid D1 row types (Staff, Post, Career, Application, ApplicationDocument, MediaAsset), serialising then deserialising a value SHALL produce an equivalent value.
3. THE Worker SHALL serialise D1 Unix timestamp integers as JSON numbers, not strings.
4. THE Pages project SHALL deserialise all Worker API responses using the TypeScript interfaces defined in the design (`Post`, `StaffPublicProfile`, `Career`, `ApplicationSubmission`, `IdentityResponse`).

