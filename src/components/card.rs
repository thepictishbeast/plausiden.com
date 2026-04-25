//! Card primitive — specifically the service-card layout the production
//! homepage uses for its 3-column grid of practice areas.

use maud::{Markup, PreEscaped, html};

/// Config for a service card.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ServiceCard<'a> {
    /// Inline SVG (pre-escaped) for the Lucide icon.
    pub(crate) icon_svg: &'a str,
    pub(crate) title: &'a str,
    pub(crate) description: &'a str,
}

/// Render a service card matching the production shadcn `<Card>` layout:
/// rounded-xl border, hover-lift, primary/5 icon tile that flips to primary
/// on group-hover.
///
/// BUG ASSUMPTION: `icon_svg` is trusted SVG markup. `title` and
/// `description` are escaped by Maud.
#[must_use]
pub(crate) fn service_card(cfg: ServiceCard<'_>) -> Markup {
    html! {
        div {
            div class="shadcn-card rounded-xl bg-card text-card-foreground h-full border border-slate-100 shadow-lg hover:shadow-xl transition-all duration-300 hover:border-primary/20 group" {
                div class="p-8" {
                    div class="w-14 h-14 rounded-2xl bg-primary/5 flex items-center justify-center mb-6 group-hover:bg-primary group-hover:text-white transition-colors duration-300" {
                        (PreEscaped(cfg.icon_svg))
                    }
                    h3 class="font-display text-xl font-bold text-slate-900 mb-3" { (cfg.title) }
                    p class="text-slate-600 leading-relaxed" { (cfg.description) }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_card_wraps_icon_and_content() {
        let m = service_card(ServiceCard {
            icon_svg: "<svg data-test=\"c\"></svg>",
            title: "IT Operations",
            description: "Robust infrastructure.",
        })
        .into_string();
        assert!(m.contains("shadcn-card"));
        assert!(m.contains("svg data-test=\"c\""));
        assert!(m.contains(">IT Operations<"));
        assert!(m.contains("Robust infrastructure"));
    }
}
