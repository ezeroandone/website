#![allow(dead_code, unused_imports)]

//! Database access layer — typed D1 queries and schema migrations.

pub mod migrations;
pub mod queries;

#[cfg(all(test, feature = "schema-tests"))]
mod schema_tests;
