//! Button primitive — Rust mirror of the production shadcn `<Button>`.
//!
//! Variant + size enums close the space of legal configurations. Callers
//! build a `Button { … }` value and call `.render()`. Raw class strings never
//! leak out of this module.

use maud::{Markup, PreEscaped, html};

use super::Classed;

/// Button visual family. Maps 1:1 to the variant set shadcn/ui ships.
#[derive(Debug, Clone, Copy)]
pub(crate) enum ButtonVariant {
    /// `bg-primary text-primary-foreground` — the default filled button.
    /// Used for: Get a Quote, Get a Free Consultation, Start Your Journey.
    Primary,
    /// `border [border-color:var(--button-outline)]` + configurable fg tone.
    /// Used for: Explore Services, Learn About Our Mission.
    Outline,
    /// Emerald-tinted outline — used for the Encrypted Inquiry / Secure Drop
    /// nav pill in production.
    OutlineEmerald,
}

/// Button sizing presets. Captures the three heights production uses.
#[derive(Debug, Clone, Copy)]
pub(crate) enum ButtonSize {
    /// `min-h-8 px-3 text-xs` — compact nav pill.
    Sm,
    /// `min-h-9 ... text-lg px-8 py-6` — medium CTA (Learn About Our Mission).
    Md,
    /// `min-h-10 text-lg px-8 py-6 rounded-xl` — hero CTA (Free Consultation).
    Lg,
}

/// Where an icon sits relative to the text label.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum IconPosition {
    Before,
    After,
}

/// Assembled button config. Callers construct with explicit fields; this
/// struct intentionally has no builder pattern (too many dependencies for
/// an initial pass).
#[derive(Debug, Clone, Copy)]
pub(crate) struct Button<'a> {
    pub(crate) variant: ButtonVariant,
    pub(crate) size: ButtonSize,
    pub(crate) label: &'a str,
    /// Optional inline SVG markup. Passed through `PreEscaped`.
    pub(crate) icon: Option<&'a str>,
    pub(crate) icon_position: IconPosition,
    /// Extra classes appended after the variant/size classes. Used for
    /// one-off tweaks like the slate-200 hero outline or hover shadows.
    pub(crate) extra_classes: &'a str,
}

impl Classed for Button<'_> {
    fn classes(&self) -> String {
        let base = "inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium \
                    focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring \
                    disabled:pointer-events-none disabled:opacity-50 \
                    [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 \
                    hover-elevate active-elevate-2";

        let variant = match self.variant {
            ButtonVariant::Primary => {
                "bg-primary text-primary-foreground border border-primary-border"
            }
            ButtonVariant::Outline => {
                "border [border-color:var(--button-outline)] shadow-xs active:shadow-none"
            }
            ButtonVariant::OutlineEmerald => {
                "border [border-color:var(--button-outline)] shadow-xs active:shadow-none \
                 border-emerald-500/50 text-emerald-700 hover:bg-emerald-50 hover:text-emerald-800"
            }
        };

        let size = match self.size {
            ButtonSize::Sm => "min-h-8 rounded-md px-3 text-xs",
            ButtonSize::Md => "min-h-9 rounded-xl px-8 py-6 text-lg",
            ButtonSize::Lg => "min-h-10 rounded-xl px-8 py-6 text-lg",
        };

        let mut out = String::with_capacity(
            base.len() + variant.len() + size.len() + self.extra_classes.len() + 4,
        );
        out.push_str(base);
        out.push(' ');
        out.push_str(variant);
        out.push(' ');
        out.push_str(size);
        if !self.extra_classes.is_empty() {
            out.push(' ');
            out.push_str(self.extra_classes);
        }
        out
    }
}

impl Button<'_> {
    /// Render the button as a `<button>` element (no `<a>` wrapper).
    /// Callers wrap in `a href=(…)` themselves when the button should navigate.
    ///
    /// BUG ASSUMPTION: `label` is escaped by Maud automatically; `icon` is
    /// pre-escaped SVG and must be trusted.
    #[must_use]
    pub(crate) fn render(self) -> Markup {
        let classes = self.classes();
        html! {
            button class=(classes) {
                @if self.icon.is_some() && self.icon_position == IconPosition::Before {
                    @if let Some(svg) = self.icon { (PreEscaped(svg)) }
                }
                (self.label)
                @if self.icon.is_some() && self.icon_position == IconPosition::After {
                    @if let Some(svg) = self.icon { (PreEscaped(svg)) }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primary_lg_contains_required_production_classes() {
        let b = Button {
            variant: ButtonVariant::Primary,
            size: ButtonSize::Lg,
            label: "Get a Free Consultation",
            icon: None,
            icon_position: IconPosition::After,
            extra_classes: "",
        };
        let cls = b.classes();
        assert!(cls.contains("bg-primary"));
        assert!(cls.contains("[&_svg]:size-4"));
        assert!(cls.contains("[&_svg]:shrink-0"));
        assert!(cls.contains("min-h-10"));
        assert!(cls.contains("rounded-xl"));
    }

    #[test]
    fn outline_emerald_sm_contains_production_contract() {
        let b = Button {
            variant: ButtonVariant::OutlineEmerald,
            size: ButtonSize::Sm,
            label: "Encrypted Inquiry",
            icon: None,
            icon_position: IconPosition::Before,
            extra_classes: "group",
        };
        let cls = b.classes();
        assert!(cls.contains("[border-color:var(--button-outline)]"));
        assert!(cls.contains("border-emerald-500/50"));
        assert!(cls.contains("text-emerald-700"));
        assert!(cls.contains("min-h-8"));
        assert!(cls.contains("group"));
        assert!(cls.contains("[&_svg]:shrink-0"));
    }

    #[test]
    fn render_places_icon_before_label() {
        let m = Button {
            variant: ButtonVariant::Primary,
            size: ButtonSize::Sm,
            label: "Send",
            icon: Some("<svg data-test=\"1\"></svg>"),
            icon_position: IconPosition::Before,
            extra_classes: "",
        }
        .render()
        .into_string();
        let svg_pos = m.find("svg data-test=\"1\"").unwrap();
        let label_pos = m.find("Send").unwrap();
        assert!(svg_pos < label_pos);
    }

    #[test]
    fn render_places_icon_after_label() {
        let m = Button {
            variant: ButtonVariant::Primary,
            size: ButtonSize::Lg,
            label: "Next",
            icon: Some("<svg data-test=\"2\"></svg>"),
            icon_position: IconPosition::After,
            extra_classes: "",
        }
        .render()
        .into_string();
        let svg_pos = m.find("svg data-test=\"2\"").unwrap();
        let label_pos = m.find("Next").unwrap();
        assert!(label_pos < svg_pos);
    }
}
