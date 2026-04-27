// SUPERSOCIETY: this module is the typed-component scaffold. Some variants
// (Outline / Md / Lg / Badge / service_card) are deliberately defined ahead
// of the views that will adopt them — explicit `#[allow(dead_code)]` keeps
// the architecture visible without forcing every call site to migrate
// before parity is verified.
#![allow(dead_code)]

//! shadcn/ui-equivalent primitives in Rust.
//!
//! The production React site composes its UI from a small set of primitives
//! (`<Button>`, `<Card>`, `<Badge>`, `Input`, …) with typed variant props.
//! This module is the Rust mirror: every primitive returns `Markup`, takes
//! typed config (variant/size enums, not raw strings), and emits the exact
//! Tailwind class contract the production stylesheet expects.
//!
//! Benefits over inline class strings:
//! 1. Class drift is impossible — every button in the site routes through
//!    one function, so "missing `[&_svg]:shrink-0`" can't happen twice.
//! 2. Variants are closed sets — new variants require a code change that
//!    shows up in review, not a copy-paste of 15 Tailwind classes.
//! 3. The `Classed` trait (below) gives us a consistent "compose class
//!    fragments into a final class= string" discipline.

pub(crate) mod badge;
pub(crate) mod button;
pub(crate) mod card;

#[allow(unused_imports)] // re-exports kept for callers that adopt these later
pub(crate) use badge::{Badge, BadgeTone};
#[allow(unused_imports)]
pub(crate) use button::{Button, ButtonSize, ButtonVariant, Decoration, IconPosition};
#[allow(unused_imports)]
pub(crate) use card::{ServiceCard, service_card};

/// Every component that renders a `class="…"` string goes through this
/// trait. Forces "base classes + variant classes + size classes + extra"
/// to be assembled in one place per primitive.
pub(crate) trait Classed {
    /// Return the full Tailwind class string for this component configuration.
    fn classes(&self) -> String;
}
