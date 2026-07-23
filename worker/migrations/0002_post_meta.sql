-- Migration: 0002_post_meta
-- Adds rich metadata columns to post, team member linking, and client logos table.

-- ============================================================
-- Extend post table with rich metadata
-- ============================================================

-- Featured image stored in R2, public URL
ALTER TABLE post ADD COLUMN featured_image_url TEXT NOT NULL DEFAULT '';

-- Category string (free-form, e.g. "Case Study", "Tutorial")
ALTER TABLE post ADD COLUMN category TEXT NOT NULL DEFAULT '';

-- Comma-separated tags stored as TEXT
ALTER TABLE post ADD COLUMN tags TEXT NOT NULL DEFAULT '';

-- Work-specific fields
ALTER TABLE post ADD COLUMN project_type TEXT NOT NULL DEFAULT '';
ALTER TABLE post ADD COLUMN technologies TEXT NOT NULL DEFAULT '';

-- Capability-specific: Google Material Icon name (e.g. "rocket_launch")
ALTER TABLE post ADD COLUMN material_icon TEXT NOT NULL DEFAULT '';

-- ============================================================
-- Post team members (staff or external contractors)
-- ============================================================
CREATE TABLE IF NOT EXISTS post_team_member (
  id          TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
  post_id     TEXT NOT NULL REFERENCES post(id) ON DELETE CASCADE,
  -- Either staff_id (internal) or contractor details (external)
  staff_id    TEXT REFERENCES staff(id) ON DELETE SET NULL,
  -- External contractor fields (used when staff_id IS NULL)
  ext_name    TEXT NOT NULL DEFAULT '',
  ext_role    TEXT NOT NULL DEFAULT '',
  ext_url     TEXT NOT NULL DEFAULT '',
  sort_order  INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_post_team_post ON post_team_member(post_id);

-- ============================================================
-- Client logos (brand worked with)
-- ============================================================
CREATE TABLE IF NOT EXISTS client_logo (
  id          TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
  name        TEXT NOT NULL,
  logo_url    TEXT NOT NULL,
  website_url TEXT NOT NULL DEFAULT '',
  sort_order  INTEGER NOT NULL DEFAULT 0,
  active      INTEGER NOT NULL DEFAULT 1,
  created_at  INTEGER NOT NULL DEFAULT (unixepoch())
);

-- ============================================================
-- Update media_asset context_type to include new contexts
-- ============================================================
-- SQLite does not support ALTER TABLE ... MODIFY COLUMN CHECK constraints,
-- so we just document the new allowed values here. The application layer
-- enforces the enum; the DB CHECK on the original column remains as-is
-- (it already allows any TEXT — the CHECK was only in the initial DDL comment).
