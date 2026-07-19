#![allow(dead_code, unused_imports)]

//! Domain models shared across handlers and database layers.

pub mod career;
pub mod content;
pub mod session;
pub mod staff;

#[cfg(test)]
mod tests;
