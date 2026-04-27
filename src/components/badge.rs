//! Badge primitive — re-exports `loom_components::badge` types.
//!
//! The hand-rolled local Badge was retired in favor of
//! `loom_components::Badge`, which produces the same DOM via typed
//! `BadgeTone` + `BadgeSize` enums. Callers using the old API
//! continue to work via these re-exports.

#[allow(unused_imports)]
pub(crate) use loom_components::badge::{Badge, BadgeSize, BadgeTone};
