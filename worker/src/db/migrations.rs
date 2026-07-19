#![allow(dead_code)]

//! Schema migration runner.
//!
//! Applies versioned SQL migration files against the D1 database in
//! alphabetical order, tracking which migrations have already been applied
//! in a `_migrations` bookkeeping table.

use worker::*;

/// A single pending migration: its name (filename stem) and the full SQL text.
struct Migration {
    name: &'static str,
    sql: &'static str,
}

/// All migration files, ordered alphabetically by name.
/// Each entry is (migration_name, sql_text).
///
/// `include_str!` is evaluated at compile time, so the SQL file must exist
/// when the worker crate is compiled.  The path is relative to this source
/// file (`src/db/migrations.rs`) and walks up two levels to reach
/// `worker/migrations/`.
static MIGRATIONS: &[Migration] = &[Migration {
    name: "0001_initial",
    sql: include_str!("../../migrations/0001_initial.sql"),
}];

/// Run all unapplied migrations against `db`.
///
/// # Steps
/// 1. Ensure the `_migrations` tracking table exists.
/// 2. Load the list of already-applied migration names.
/// 3. For each migration in alphabetical order, skip those already applied.
/// 4. Execute the migration SQL.
/// 5. Record the migration name in `_migrations`.
///
/// Returns `Ok(())` when all migrations are current, or a [`worker::Error`]
/// if any step fails.
pub async fn run_migrations(db: &D1Database) -> Result<()> {
    // Step 1 — create tracking table if it doesn't exist.
    db.exec(
        "CREATE TABLE IF NOT EXISTS _migrations (\
            name TEXT PRIMARY KEY, \
            applied_at INTEGER NOT NULL DEFAULT (unixepoch())\
        )",
    )
    .await?;

    // Step 2 — fetch names that are already applied.
    let applied: Vec<String> = db
        .prepare("SELECT name FROM _migrations")
        .all()
        .await?
        .results::<AppliedRow>()?
        .into_iter()
        .map(|r| r.name)
        .collect();

    // Step 3-5 — apply each unapplied migration in order.
    for migration in MIGRATIONS {
        if applied.iter().any(|a| a == migration.name) {
            // Already applied — skip.
            continue;
        }

        console_log!("[migrations] applying {}", migration.name);

        // Step 4 — execute the migration SQL.
        // D1's `exec` runs a multi-statement SQL string.
        db.exec(migration.sql).await?;

        // Step 5 — record the migration as applied.
        db.prepare("INSERT INTO _migrations (name) VALUES (?1)")
            .bind(&[migration.name.into()])?
            .run()
            .await?;

        console_log!("[migrations] applied  {}", migration.name);
    }

    Ok(())
}

/// Internal helper struct for deserialising rows from `_migrations`.
#[derive(serde::Deserialize)]
struct AppliedRow {
    name: String,
}
