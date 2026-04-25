//! Badge primitive — the small pill labels the production site uses for
//! "Professional IT Solutions" / "Our Mission" / "Excellence in Execution" etc.

use maud::{Markup, html};

use super::Classed;

/// Tonal variants. Production mostly uses `Primary` (blue tint) and `White`
/// (translucent white on dark backgrounds).
#[derive(Debug, Clone, Copy)]
pub(crate) enum BadgeTone {
    /// `bg-primary/10 text-primary border border-primary/20` — default.
    Primary,
    /// `bg-white/10 text-white border border-white/10 backdrop-blur-sm` — on dark.
    OnDark,
}

/// A small pill label.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Badge<'a> {
    pub(crate) tone: BadgeTone,
    pub(crate) label: &'a str,
}

impl Classed for Badge<'_> {
    fn classes(&self) -> String {
        match self.tone {
            BadgeTone::Primary => {
                "inline-block px-4 py-1.5 rounded-full bg-primary/10 text-primary \
                 font-semibold text-sm mb-6 border border-primary/20"
                    .to_owned()
            }
            BadgeTone::OnDark => {
                "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-white/10 \
                 text-white text-sm font-medium mb-6 backdrop-blur-sm border border-white/10"
                    .to_owned()
            }
        }
    }
}

impl Badge<'_> {
    #[must_use]
    pub(crate) fn render(self) -> Markup {
        let classes = self.classes();
        html! {
            span class=(classes) {
                (self.label)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primary_badge_has_primary_bg() {
        let cls = Badge {
            tone: BadgeTone::Primary,
            label: "x",
        }
        .classes();
        assert!(cls.contains("bg-primary/10"));
        assert!(cls.contains("border-primary/20"));
    }

    #[test]
    fn on_dark_badge_uses_white_tint() {
        let cls = Badge {
            tone: BadgeTone::OnDark,
            label: "x",
        }
        .classes();
        assert!(cls.contains("bg-white/10"));
        assert!(cls.contains("backdrop-blur-sm"));
    }
}
