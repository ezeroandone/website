/// Integration tests for the D1 schema DDL.
///
/// Each test spins up an in-memory SQLite database, applies the full
/// `0001_initial.sql` migration, and asserts the constraint / index
/// behaviour defined in Requirements 14.1–14.4.
///
/// These tests run on the native host target (`cargo test`) using
/// `rusqlite` with the bundled SQLite library.  They do **not** run
/// under `wasm32-unknown-unknown`.
///
/// Requires the `schema-tests` Cargo feature:
///   `cargo test --features schema-tests`
#[cfg(all(test, feature = "schema-tests"))]
mod tests {
    use rusqlite::{Connection, Result as RusqliteResult, params};

    /// Load the migration DDL.  `include_str!` resolves the path relative to
    /// this source file at compile-time, so the path is workspace-relative.
    const MIGRATION_SQL: &str =
        include_str!("../../migrations/0001_initial.sql");

    /// Helper: open a fresh in-memory connection and apply the migration.
    fn open_db() -> RusqliteResult<Connection> {
        let conn = Connection::open_in_memory()?;
        // Enable foreign-key enforcement (off by default in SQLite).
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        conn.execute_batch(MIGRATION_SQL)?;
        Ok(conn)
    }

    // ----------------------------------------------------------------
    // Requirement 14.1 — CHECK constraints on enumerated columns
    // ----------------------------------------------------------------

    /// `staff.role` CHECK constraint must reject values outside
    /// ('SuperAdmin', 'Admin', 'Staff').
    ///
    /// Validates: Requirements 14.1
    #[test]
    fn staff_role_check_rejects_invalid_value() {
        let conn = open_db().expect("schema migration should succeed");

        let result = conn.execute(
            "INSERT INTO staff (id, email, username, role)
             VALUES ('s1', 'a@example.com', 'alice', 'Hacker')",
            [],
        );

        assert!(
            result.is_err(),
            "inserting an invalid role should be rejected by the CHECK constraint"
        );
    }

    /// `staff.role` CHECK constraint must accept every valid role value.
    ///
    /// Validates: Requirements 14.1
    #[test]
    fn staff_role_check_accepts_valid_values() {
        let conn = open_db().expect("schema migration should succeed");

        for (idx, role) in ["SuperAdmin", "Admin", "Staff"].iter().enumerate() {
            let email = format!("user{}@example.com", idx);
            let username = format!("user{}", idx);
            let id = format!("id{}", idx);
            conn.execute(
                "INSERT INTO staff (id, email, username, role) VALUES (?1, ?2, ?3, ?4)",
                params![id, email, username, role],
            )
            .unwrap_or_else(|e| panic!("role '{}' should be valid but got: {}", role, e));
        }
    }

    /// `staff_lifecycle.status` CHECK constraint must reject invalid values.
    ///
    /// Validates: Requirements 14.1
    #[test]
    fn lifecycle_status_check_rejects_invalid_value() {
        let conn = open_db().expect("schema migration should succeed");

        // Insert a valid staff row first (FK dependency).
        conn.execute(
            "INSERT INTO staff (id, email, username) VALUES ('s1', 'a@e.com', 'alice')",
            [],
        )
        .unwrap();

        let result = conn.execute(
            "INSERT INTO staff_lifecycle (id, staff_id, status)
             VALUES ('lc1', 's1', 'Fired')",
            [],
        );

        assert!(
            result.is_err(),
            "invalid lifecycle status should be rejected by the CHECK constraint"
        );
    }

    /// `application.status` CHECK constraint must reject invalid values.
    ///
    /// Validates: Requirements 14.1, 14.3
    #[test]
    fn application_status_check_rejects_invalid_value() {
        let conn = open_db().expect("schema migration should succeed");

        // Need a career row for the FK.
        conn.execute(
            "INSERT INTO career (id, slug, title) VALUES ('c1', 'eng-role', 'Engineer')",
            [],
        )
        .unwrap();

        let result = conn.execute(
            "INSERT INTO application (id, career_id, applicant_name, applicant_email, status)
             VALUES ('a1', 'c1', 'Bob', 'bob@x.com', 'Pending')",
            [],
        );

        assert!(
            result.is_err(),
            "invalid application status should be rejected by the CHECK constraint"
        );
    }

    /// `application.status` CHECK constraint must accept all valid status values.
    ///
    /// Validates: Requirements 14.1, 14.3
    #[test]
    fn application_status_check_accepts_valid_values() {
        let conn = open_db().expect("schema migration should succeed");

        conn.execute(
            "INSERT INTO career (id, slug, title) VALUES ('c1', 'eng-role', 'Engineer')",
            [],
        )
        .unwrap();

        let statuses = ["Applied", "Interviewing", "Offered", "Rejected", "Hired"];
        for (idx, status) in statuses.iter().enumerate() {
            let id = format!("a{}", idx);
            let email = format!("applicant{}@x.com", idx);
            conn.execute(
                "INSERT INTO application (id, career_id, applicant_name, applicant_email, status)
                 VALUES (?1, 'c1', 'Applicant', ?2, ?3)",
                params![id, email, status],
            )
            .unwrap_or_else(|e| panic!("status '{}' should be valid but got: {}", status, e));
        }
    }

    // ----------------------------------------------------------------
    // Requirement 14.2 — UNIQUE + foreign-key constraints on staff_lifecycle
    // ----------------------------------------------------------------

    /// `staff_lifecycle.staff_id` must be UNIQUE — a second lifecycle row for
    /// the same staff member must be rejected.
    ///
    /// Validates: Requirements 14.2
    #[test]
    fn staff_lifecycle_staff_id_is_unique() {
        let conn = open_db().expect("schema migration should succeed");

        conn.execute(
            "INSERT INTO staff (id, email, username) VALUES ('s1', 'a@e.com', 'alice')",
            [],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO staff_lifecycle (id, staff_id) VALUES ('lc1', 's1')",
            [],
        )
        .unwrap();

        let result = conn.execute(
            "INSERT INTO staff_lifecycle (id, staff_id) VALUES ('lc2', 's1')",
            [],
        );

        assert!(
            result.is_err(),
            "a second staff_lifecycle row for the same staff_id should be rejected by UNIQUE"
        );
    }

    /// `staff_lifecycle.staff_id` is a foreign key — inserting a lifecycle row
    /// for a non-existent staff member must be rejected.
    ///
    /// Validates: Requirements 14.2
    #[test]
    fn staff_lifecycle_staff_id_foreign_key_enforced() {
        let conn = open_db().expect("schema migration should succeed");

        let result = conn.execute(
            "INSERT INTO staff_lifecycle (id, staff_id) VALUES ('lc1', 'ghost')",
            [],
        );

        assert!(
            result.is_err(),
            "lifecycle row referencing a non-existent staff_id should be rejected by FK"
        );
    }

    // ----------------------------------------------------------------
    // Requirement 14.4 — CASCADE delete from staff to team_profile
    // ----------------------------------------------------------------

    /// Deleting a `staff` row must cascade and remove the linked `team_profile`
    /// row automatically.
    ///
    /// Validates: Requirements 14.4
    #[test]
    fn team_profile_cascades_on_staff_delete() {
        let conn = open_db().expect("schema migration should succeed");

        conn.execute(
            "INSERT INTO staff (id, email, username) VALUES ('s1', 'a@e.com', 'alice')",
            [],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO team_profile (id) VALUES ('s1')",
            [],
        )
        .unwrap();

        // Verify the profile exists before deletion.
        let count_before: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM team_profile WHERE id = 's1'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count_before, 1, "team_profile row should exist before staff delete");

        // Delete the staff row — should cascade.
        conn.execute("DELETE FROM staff WHERE id = 's1'", []).unwrap();

        let count_after: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM team_profile WHERE id = 's1'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count_after, 0, "team_profile row should be removed by CASCADE");
    }

    // ----------------------------------------------------------------
    // Requirement 14.4 — All six performance indices must be created
    // ----------------------------------------------------------------

    /// Query `sqlite_master` to verify each of the six required indices exists.
    ///
    /// Validates: Requirements 14.4
    #[test]
    fn all_six_indices_exist() {
        let conn = open_db().expect("schema migration should succeed");

        let required_indices = [
            "idx_post_type_slug",
            "idx_post_published",
            "idx_application_career",
            "idx_lifecycle_status",
            "idx_app_doc_application",
            "idx_media_context",
        ];

        for index_name in &required_indices {
            let found: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND name=?1",
                    params![index_name],
                    |row| row.get(0),
                )
                .unwrap_or(0);

            assert_eq!(
                found, 1,
                "expected index '{}' to exist in sqlite_master but it was not found",
                index_name
            );
        }
    }
}
