//! Card primitive — re-exports `loom_components::card` types so the
//! visual contract for cards lives in exactly one place across the
//! ecosystem.
//!
//! The old hand-rolled `ServiceCard` + `service_card` helpers were
//! retired in favor of `FeatureCard` with `FeatureCardStyle::Bold`,
//! which produces the same DOM. Callers that imported the old types
//! transparently continue to work via these re-exports.

pub(crate) use loom_components::card::{
    Card, CardElevation, CardHover, CardPadding, FeatureCard, FeatureCardStyle, LinkCard,
};

/// Backwards-compatible alias for the Bold-styled feature card.
/// Prefer [`FeatureCard`] directly in new code.
pub(crate) type ServiceCard<'a> = FeatureCard<'a>;
