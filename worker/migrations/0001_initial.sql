-- Migration: 0001_initial
-- Full DDL for eZeroAndOne core schema (D1 / SQLite)
-- All CREATE TABLE and CREATE INDEX statements use IF NOT EXISTS for idempotency.

-- ============================================================
-- Core staff table
-- ============================================================
CREATE TABLE IF NOT EXISTS staff (
  id                    TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
  email                 TEXT NOT NULL UNIQUE,
  username              TEXT NOT NULL UNIQUE,
  name                  TEXT NOT NULL DEFAULT '',
  job_title             TEXT NOT NULL DEFAULT '',
  bio                   TEXT NOT NULL DEFAULT '',
  avatar_url            TEXT NOT NULL DEFAULT '',
  role                  TEXT NOT NULL DEFAULT 'Staff'
                          CHECK(role IN ('SuperAdmin','Admin','Staff')),
  onboarding_completed  INTEGER NOT NULL DEFAULT 0,
  signing_public_key    TEXT,
  created_at            INTEGER NOT NULL DEFAULT (unixepoch()),
  updated_at            INTEGER NOT NULL DEFAULT (unixepoch())
);

-- ============================================================
-- Staff lifecycle (probation / confirmed / inactive)
-- ============================================================
CREATE TABLE IF NOT EXISTS staff_lifecycle (
  id               TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
  staff_id         TEXT NOT NULL UNIQUE REFERENCES staff(id) ON DELETE CASCADE,
  status           TEXT NOT NULL DEFAULT 'Probation'
                     CHECK(status IN ('Probation','Confirmed','Inactive')),
  probation_start  INTEGER NOT NULL DEFAULT (unixepoch()),
  probation_end    INTEGER,
  confirmed_at     INTEGER
);

-- ============================================================
-- Content posts (insights / work / capabilities)
-- ============================================================
CREATE TABLE IF NOT EXISTS post (
  id            TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
  type          TEXT NOT NULL CHECK(type IN ('insight','work','capability')),
  slug          TEXT NOT NULL UNIQUE,
  title         TEXT NOT NULL,
  summary       TEXT NOT NULL DEFAULT '',
  body_md       TEXT NOT NULL DEFAULT '',
  author_id     TEXT REFERENCES staff(id),
  published_at  INTEGER,
  updated_at    INTEGER NOT NULL DEFAULT (unixepoch()),
  published     INTEGER NOT NULL DEFAULT 0
);

-- ============================================================
-- Public team profiles (extends staff, one-to-one)
-- ============================================================
CREATE TABLE IF NOT EXISTS team_profile (
  id          TEXT PRIMARY KEY REFERENCES staff(id) ON DELETE CASCADE,
  linkedin    TEXT,
  github      TEXT,
  twitter     TEXT,
  skills      TEXT,
  order_rank  INTEGER NOT NULL DEFAULT 0
);

-- ============================================================
-- Job career listings
-- ============================================================
CREATE TABLE IF NOT EXISTS career (
  id              TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
  slug            TEXT NOT NULL UNIQUE,
  title           TEXT NOT NULL,
  description_md  TEXT NOT NULL DEFAULT '',
  department      TEXT NOT NULL DEFAULT '',
  type            TEXT NOT NULL DEFAULT 'Full-Time'
                    CHECK(type IN ('Full-Time','Part-Time','Contract','Internship')),
  active          INTEGER NOT NULL DEFAULT 1,
  created_at      INTEGER NOT NULL DEFAULT (unixepoch())
);

-- ============================================================
-- Job applications
-- ============================================================
CREATE TABLE IF NOT EXISTS application (
  id               TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
  career_id        TEXT NOT NULL REFERENCES career(id),
  applicant_name   TEXT NOT NULL,
  applicant_email  TEXT NOT NULL,
  cover_letter     TEXT NOT NULL DEFAULT '',
  status           TEXT NOT NULL DEFAULT 'Applied'
                     CHECK(status IN ('Applied','Interviewing','Offered','Rejected','Hired')),
  applied_at       INTEGER NOT NULL DEFAULT (unixepoch()),
  updated_at       INTEGER NOT NULL DEFAULT (unixepoch())
);

-- ============================================================
-- Performance indices
-- ============================================================
CREATE INDEX IF NOT EXISTS idx_post_type_slug     ON post(type, slug);
CREATE INDEX IF NOT EXISTS idx_post_published     ON post(published, published_at DESC);
CREATE INDEX IF NOT EXISTS idx_application_career ON application(career_id, status);
CREATE INDEX IF NOT EXISTS idx_lifecycle_status   ON staff_lifecycle(status, probation_end);

-- ============================================================
-- Application documents (R2 tracking)
-- ============================================================
CREATE TABLE IF NOT EXISTS application_document (
  id                TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
  application_id    TEXT NOT NULL REFERENCES application(id) ON DELETE CASCADE,
  r2_key            TEXT NOT NULL UNIQUE,
  original_filename TEXT NOT NULL,
  mime_type         TEXT NOT NULL,
  uploaded_at       INTEGER NOT NULL DEFAULT (unixepoch())
);

-- ============================================================
-- Media assets (R2 tracking — avatars, post covers, etc.)
-- ============================================================
CREATE TABLE IF NOT EXISTS media_asset (
  id            TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
  r2_key        TEXT NOT NULL UNIQUE,
  context_type  TEXT NOT NULL CHECK(context_type IN ('avatar','post_cover','post_media','career_hero')),
  context_id    TEXT NOT NULL,
  public_url    TEXT NOT NULL,
  uploaded_by   TEXT REFERENCES staff(id),
  uploaded_at   INTEGER NOT NULL DEFAULT (unixepoch())
);

-- ============================================================
-- Additional performance indices
-- ============================================================
CREATE INDEX IF NOT EXISTS idx_app_doc_application ON application_document(application_id);
CREATE INDEX IF NOT EXISTS idx_media_context       ON media_asset(context_type, context_id);
