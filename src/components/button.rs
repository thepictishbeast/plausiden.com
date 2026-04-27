//! Button primitive — delegated to `loom_components::button`.
//!
//! This module previously contained a hand-rolled Button. It now re-exports
//! the typed `Button` from PlausiDen-Loom so the visual contract lives in
//! exactly one place across the ecosystem.
//!
//! Doctrine: see `~/PlausiDen-Loom/CLAUDE.md`. Never write a raw class
//! string here; if you need a button shape that isn't expressible in the
//! Loom API, extend the design system in PlausiDen-Loom (separate PR).

pub(crate) use loom_components::button::{
    Button, ButtonSize, ButtonVariant, Decoration, IconPosition,
};
