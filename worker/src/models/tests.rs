// Property-based tests for domain model JSON round-trip fidelity.
//
// **Validates: Requirements 20.2, 20.3**
//
// Property 15: JSON serialisation round-trip for domain types.
// For any value of a domain type `T` that implements `Serialize + DeserializeOwned + PartialEq`,
// `serde_json::from_str(&serde_json::to_string(&val).unwrap()) == val` must hold.
// Additionally, all timestamp fields (`created_at`, `updated_at`, `applied_at`, `uploaded_at`,
// `probation_start`, `probation_end`, `confirmed_at`, `published_at`, `exp`) must serialise as
// JSON numbers (not strings), per Requirement 20.3.

#![cfg(test)]

use proptest::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::models::career::{Application, ApplicationDocument, ApplicationStatus, Career, MediaAsset};
use crate::models::content::{Post, PostType};
use crate::models::staff::{LifecycleStatus, Role, Staff, StaffLifecycle};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Core round-trip assertion: serialise to JSON then deserialise and compare.
fn assert_json_roundtrip<T>(val: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let json = serde_json::to_string(val).expect("serialisation must succeed");
    let recovered: T = serde_json::from_str(&json).expect("deserialisation must succeed");
    assert_eq!(*val, recovered, "JSON round-trip failed for value");
}

/// Assert that the named `i64` field in a JSON object serialises as a JSON number
/// (not a string), per Requirement 20.3.
fn assert_timestamp_is_number(json: &str, field: &str) {
    let v: serde_json::Value =
        serde_json::from_str(json).expect("valid JSON for timestamp check");
    let field_value = &v[field];
    assert!(
        field_value.is_number(),
        "timestamp field `{}` must be a JSON number, but got: {:?}",
        field,
        field_value,
    );
}

/// Assert that, if the named optional `i64` field is present, it serialises as
/// a JSON number (not a string or null with a numeric string inside).
fn assert_optional_timestamp_is_number(json: &str, field: &str) {
    let v: serde_json::Value =
        serde_json::from_str(json).expect("valid JSON for timestamp check");
    let field_value = &v[field];
    if !field_value.is_null() {
        assert!(
            field_value.is_number(),
            "optional timestamp field `{}` must be a JSON number when present, but got: {:?}",
            field,
            field_value,
        );
    }
}

// ---------------------------------------------------------------------------
// Proptest strategies
// ---------------------------------------------------------------------------

/// Strategy producing a non-empty ASCII-printable string bounded to a
/// reasonable length so generated JSON stays compact.
fn arb_string() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9 _\\-\\.@/]{1,64}".prop_map(|s| s)
}

/// Strategy producing an optional string.
fn arb_opt_string() -> impl Strategy<Value = Option<String>> {
    prop_oneof![Just(None), arb_string().prop_map(Some)]
}

/// Strategy for `Role`.
fn arb_role() -> impl Strategy<Value = Role> {
    prop_oneof![
        Just(Role::Public),
        Just(Role::Staff),
        Just(Role::Admin),
        Just(Role::SuperAdmin),
    ]
}

/// Strategy for `LifecycleStatus`.
fn arb_lifecycle_status() -> impl Strategy<Value = LifecycleStatus> {
    prop_oneof![
        Just(LifecycleStatus::Probation),
        Just(LifecycleStatus::Confirmed),
        Just(LifecycleStatus::Inactive),
    ]
}

/// Strategy for `PostType`.
fn arb_post_type() -> impl Strategy<Value = PostType> {
    prop_oneof![
        Just(PostType::Insight),
        Just(PostType::Work),
        Just(PostType::Capability),
    ]
}

/// Strategy for `ApplicationStatus`.
fn arb_application_status() -> impl Strategy<Value = ApplicationStatus> {
    prop_oneof![
        Just(ApplicationStatus::Applied),
        Just(ApplicationStatus::Interviewing),
        Just(ApplicationStatus::Offered),
        Just(ApplicationStatus::Hired),
        Just(ApplicationStatus::Rejected),
    ]
}

/// Unix-seconds timestamp strategy (positive, plausible range).
fn arb_timestamp() -> impl Strategy<Value = i64> {
    // Range: 2000-01-01 to 2100-01-01 in Unix seconds
    946_684_800_i64..4_102_444_800_i64
}

/// Strategy for a full `Staff` instance.
fn arb_staff() -> impl Strategy<Value = Staff> {
    (
        arb_string(), // id
        arb_string(), // email
        arb_string(), // username
        arb_string(), // name
        arb_string(), // job_title
        arb_string(), // bio
        arb_string(), // avatar_url
        arb_role(),
        any::<bool>(), // onboarding_completed
        arb_opt_string(), // signing_public_key
        arb_timestamp(), // created_at
        arb_timestamp(), // updated_at
    )
        .prop_map(
            |(id, email, username, name, job_title, bio, avatar_url, role, onboarding_completed, signing_public_key, created_at, updated_at)| {
                Staff {
                    id,
                    email,
                    username,
                    name,
                    job_title,
                    bio,
                    avatar_url,
                    role,
                    onboarding_completed,
                    signing_public_key,
                    created_at,
                    updated_at,
                }
            },
        )
}

/// Strategy for a full `Post` instance.
fn arb_post() -> impl Strategy<Value = Post> {
    // proptest tuples cap at 12 — chain two prop_flat_map calls.
    (
        arb_string(),         // id
        arb_post_type(),
        arb_string(),         // slug
        arb_string(),         // title
        arb_string(),         // summary
        arb_string(),         // body_md
        arb_opt_string(),     // author_id
        prop_oneof![Just(None), arb_timestamp().prop_map(Some)], // published_at
        arb_timestamp(),      // updated_at
        any::<bool>(),        // published
    )
    .prop_flat_map(|(id, post_type, slug, title, summary, body_md, author_id,
                     published_at, updated_at, published)| {
        // First extension: 3 new string fields
        (
            Just(id), Just(post_type), Just(slug), Just(title), Just(summary),
            Just(body_md), Just(author_id), Just(published_at), Just(updated_at),
            Just(published),
            arb_string(), // featured_image_url
            arb_string(), // category
        )
    })
    .prop_flat_map(|(id, post_type, slug, title, summary, body_md, author_id,
                     published_at, updated_at, published,
                     featured_image_url, category)| {
        // Second extension: 4 remaining fields
        (
            Just(id), Just(post_type), Just(slug), Just(title), Just(summary),
            Just(body_md), Just(author_id), Just(published_at), Just(updated_at),
            Just(published), Just(featured_image_url), Just(category),
            arb_string(), // tags
            arb_string(), // project_type
            arb_string(), // technologies
            arb_string(), // material_icon
        )
    })
    .prop_map(|(id, post_type, slug, title, summary, body_md, author_id,
                published_at, updated_at, published,
                featured_image_url, category,
                tags, project_type, technologies, material_icon)| {
        Post {
            id, post_type, slug, title, summary, body_md, author_id,
            published_at, updated_at, published,
            featured_image_url, category, tags,
            project_type, technologies, material_icon,
        }
    })
}

/// Strategy for a `Career` instance.
fn arb_career() -> impl Strategy<Value = Career> {
    (
        arb_string(), // id
        arb_string(), // slug
        arb_string(), // title
        arb_string(), // description_md
        arb_string(), // department
        prop_oneof![
            Just("Full-Time".to_string()),
            Just("Part-Time".to_string()),
            Just("Contract".to_string()),
            Just("Internship".to_string()),
        ],
        any::<bool>(),   // active
        arb_timestamp(), // created_at
    )
        .prop_map(|(id, slug, title, description_md, department, career_type, active, created_at)| {
            Career {
                id,
                slug,
                title,
                description_md,
                department,
                career_type,
                active,
                created_at,
            }
        })
}

/// Strategy for an `Application` instance.
fn arb_application() -> impl Strategy<Value = Application> {
    (
        arb_string(),            // id
        arb_string(),            // career_id
        arb_string(),            // applicant_name
        arb_string(),            // applicant_email
        arb_string(),            // cover_letter
        arb_application_status(),
        arb_timestamp(),         // applied_at
        arb_timestamp(),         // updated_at
    )
        .prop_map(
            |(id, career_id, applicant_name, applicant_email, cover_letter, status, applied_at, updated_at)| {
                Application {
                    id,
                    career_id,
                    applicant_name,
                    applicant_email,
                    cover_letter,
                    status,
                    applied_at,
                    updated_at,
                }
            },
        )
}

/// Strategy for an `ApplicationDocument` instance.
fn arb_application_document() -> impl Strategy<Value = ApplicationDocument> {
    (
        arb_string(), // id
        arb_string(), // application_id
        arb_string(), // r2_key
        arb_string(), // original_filename
        arb_string(), // mime_type
        arb_timestamp(), // uploaded_at
    )
        .prop_map(|(id, application_id, r2_key, original_filename, mime_type, uploaded_at)| {
            ApplicationDocument {
                id,
                application_id,
                r2_key,
                original_filename,
                mime_type,
                uploaded_at,
            }
        })
}

/// Strategy for a `MediaAsset` instance.
fn arb_media_asset() -> impl Strategy<Value = MediaAsset> {
    (
        arb_string(), // id
        arb_string(), // r2_key
        prop_oneof![
            Just("avatar".to_string()),
            Just("post_cover".to_string()),
            Just("post_media".to_string()),
            Just("career_hero".to_string()),
        ],
        arb_string(),     // context_id
        arb_string(),     // public_url
        arb_opt_string(), // uploaded_by
        arb_timestamp(),  // uploaded_at
    )
        .prop_map(|(id, r2_key, context_type, context_id, public_url, uploaded_by, uploaded_at)| {
            MediaAsset {
                id,
                r2_key,
                context_type,
                context_id,
                public_url,
                uploaded_by,
                uploaded_at,
            }
        })
}

// ---------------------------------------------------------------------------
// Property 15 — JSON round-trip fidelity (Requirements 20.2, 20.3)
// ---------------------------------------------------------------------------

proptest! {
    /// **Property 15: JSON serialisation round-trip for domain types**
    ///
    /// **Validates: Requirements 20.2, 20.3**
    ///
    /// For any `Staff` instance, serialising to JSON and deserialising back
    /// produces an equal value, and all timestamp fields are JSON numbers.
    #[test]
    fn prop_staff_json_roundtrip(val in arb_staff()) {
        let json = serde_json::to_string(&val).expect("Staff serialisation must succeed");
        let recovered: Staff = serde_json::from_str(&json)
            .expect("Staff deserialisation must succeed");
        prop_assert_eq!(val, recovered, "Staff JSON round-trip failed");
    }

    /// **Property 15: JSON serialisation round-trip for domain types**
    ///
    /// **Validates: Requirements 20.2, 20.3**
    ///
    /// Timestamp fields on `Staff` must serialise as JSON numbers.
    #[test]
    fn prop_staff_timestamps_are_numbers(val in arb_staff()) {
        let json = serde_json::to_string(&val).expect("Staff serialisation must succeed");
        assert_timestamp_is_number(&json, "created_at");
        assert_timestamp_is_number(&json, "updated_at");
    }

    /// **Property 15: JSON serialisation round-trip for domain types**
    ///
    /// **Validates: Requirements 20.2, 20.3**
    ///
    /// For any `Post` instance, the JSON round-trip is lossless.
    #[test]
    fn prop_post_json_roundtrip(val in arb_post()) {
        assert_json_roundtrip(&val);
    }

    /// **Property 15: JSON serialisation round-trip for domain types**
    ///
    /// **Validates: Requirements 20.2, 20.3**
    ///
    /// Timestamp fields on `Post` must serialise as JSON numbers.
    #[test]
    fn prop_post_timestamps_are_numbers(val in arb_post()) {
        let json = serde_json::to_string(&val).expect("Post serialisation must succeed");
        assert_timestamp_is_number(&json, "updated_at");
        assert_optional_timestamp_is_number(&json, "published_at");
    }

    /// **Property 15: JSON serialisation round-trip for domain types**
    ///
    /// **Validates: Requirements 20.2, 20.3**
    ///
    /// For any `Career` instance, the JSON round-trip is lossless.
    #[test]
    fn prop_career_json_roundtrip(val in arb_career()) {
        assert_json_roundtrip(&val);
    }

    /// **Property 15: JSON serialisation round-trip for domain types**
    ///
    /// **Validates: Requirements 20.2, 20.3**
    ///
    /// The `created_at` timestamp on `Career` must serialise as a JSON number.
    #[test]
    fn prop_career_timestamps_are_numbers(val in arb_career()) {
        let json = serde_json::to_string(&val).expect("Career serialisation must succeed");
        assert_timestamp_is_number(&json, "created_at");
    }

    /// **Property 15: JSON serialisation round-trip for domain types**
    ///
    /// **Validates: Requirements 20.2, 20.3**
    ///
    /// For any `Application` instance, the JSON round-trip is lossless.
    #[test]
    fn prop_application_json_roundtrip(val in arb_application()) {
        assert_json_roundtrip(&val);
    }

    /// **Property 15: JSON serialisation round-trip for domain types**
    ///
    /// **Validates: Requirements 20.2, 20.3**
    ///
    /// Timestamp fields on `Application` must serialise as JSON numbers.
    #[test]
    fn prop_application_timestamps_are_numbers(val in arb_application()) {
        let json = serde_json::to_string(&val).expect("Application serialisation must succeed");
        assert_timestamp_is_number(&json, "applied_at");
        assert_timestamp_is_number(&json, "updated_at");
    }

    /// **Property 15: JSON serialisation round-trip for domain types**
    ///
    /// **Validates: Requirements 20.2, 20.3**
    ///
    /// For any `ApplicationDocument` instance, the JSON round-trip is lossless.
    #[test]
    fn prop_application_document_json_roundtrip(val in arb_application_document()) {
        assert_json_roundtrip(&val);
    }

    /// **Property 15: JSON serialisation round-trip for domain types**
    ///
    /// **Validates: Requirements 20.2, 20.3**
    ///
    /// The `uploaded_at` timestamp on `ApplicationDocument` must serialise as a JSON number.
    #[test]
    fn prop_application_document_timestamps_are_numbers(val in arb_application_document()) {
        let json = serde_json::to_string(&val)
            .expect("ApplicationDocument serialisation must succeed");
        assert_timestamp_is_number(&json, "uploaded_at");
    }

    /// **Property 15: JSON serialisation round-trip for domain types**
    ///
    /// **Validates: Requirements 20.2, 20.3**
    ///
    /// For any `MediaAsset` instance, the JSON round-trip is lossless.
    #[test]
    fn prop_media_asset_json_roundtrip(val in arb_media_asset()) {
        assert_json_roundtrip(&val);
    }

    /// **Property 15: JSON serialisation round-trip for domain types**
    ///
    /// **Validates: Requirements 20.2, 20.3**
    ///
    /// The `uploaded_at` timestamp on `MediaAsset` must serialise as a JSON number.
    #[test]
    fn prop_media_asset_timestamps_are_numbers(val in arb_media_asset()) {
        let json = serde_json::to_string(&val).expect("MediaAsset serialisation must succeed");
        assert_timestamp_is_number(&json, "uploaded_at");
    }
}
