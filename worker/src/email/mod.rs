#![allow(dead_code, unused_imports)]

//! Email delivery — thin HTTP client over Resend.

mod provider;

pub use provider::{send_magic_link, send_onboarding_email};
