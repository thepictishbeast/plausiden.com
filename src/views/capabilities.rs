//! `/capabilities` — the in-house stack PlausiDen operates.
//!
//! Distinct from `/services`, which is *what we do for clients*.
//! `/capabilities` is *what we built for ourselves first* — the mail
//! infrastructure, CMS, design system, AI surface, outbound system,
//! mail client, etc. — and which now back the services we sell.
//!
//! Copy stays at the **capability layer**. No version claims, no
//! feature lists, no concrete vendor / tool counts. Tools mature
//! continuously and copy goes stale faster than the publishing
//! cadence.

use maud::{Markup, html};

use crate::views::layout::page;

/// One capability category in the in-house stack.
struct Capability {
    /// Eyebrow (short noun phrase).
    eyebrow: &'static str,
    /// Heading (the category itself).
    heading: &'static str,
    /// Two-paragraph description, intentionally general.
    blurb_1: &'static str,
    blurb_2: &'static str,
}

const CAPABILITIES: &[Capability] = &[
    Capability {
        eyebrow: "Mail infrastructure",
        heading: "Our own email server.",
        blurb_1: "We run our own mail stack end to end — the server, the rules layer, the client. Clients we onboard inherit the same posture: their mail is held on infrastructure we operate, configured to a security profile we audit ourselves, and never depends on a third-party SaaS we don't control.",
        blurb_2: "Mail is one of the most subpoena-prone surfaces in any business. Owning the stack is the difference between answering a discovery request in a paragraph and answering it through a vendor's legal team.",
    },
    Capability {
        eyebrow: "Outbound + CRM",
        heading: "Our own prospecting and relationship layer.",
        blurb_1: "We operate our own outbound and customer-relationship system rather than license a SaaS that learns about our clients in the process of helping us track them. The data stays on infrastructure we control, in formats we own, with audit trails we can produce on request.",
        blurb_2: "The same system is offered to clients as a product when their threat model says the same thing about their pipeline data.",
    },
    Capability {
        eyebrow: "Content + CMS",
        heading: "Our own content infrastructure.",
        blurb_1: "Our publishing surface — including this site — is content-managed by tooling we operate ourselves. No third-party CMS holds the content, no theming layer needs to phone home, and the entire publishing pipeline is reviewable by us before a page lands public.",
        blurb_2: "When clients need a content surface that meets the same bar — privacy-respecting, auditable, no analytics-by-default — the tooling is already operational.",
    },
    Capability {
        eyebrow: "Design system",
        heading: "Our own typed design system.",
        blurb_1: "Visual consistency across our products is enforced by a design system we built, not by a Figma library or a third-party component kit. Every component is composed from typed tokens; every page composes those components; lint refuses raw class strings outside the system.",
        blurb_2: "The downstream consequence is that visual changes ship as one-line token edits across every surface, instead of redesigns handed to a stranger who can't see the production code.",
    },
    Capability {
        eyebrow: "AI surface",
        heading: "Our own AI tooling.",
        blurb_1: "We operate our own AI surface for the categories where AI helps — internal tooling, content workflows, suggestion systems — without piping client data through third-party model providers we don't audit. Where a hosted model is genuinely the right tool, we treat it as one input, not the system of record.",
        blurb_2: "This is the difference between a consultancy that recommends \"go pay a model provider\" and one that builds the workflow around constraints the client can verify.",
    },
    Capability {
        eyebrow: "Mail client",
        heading: "Our own desktop mail client.",
        blurb_1: "We're building a local-first mail client whose filtering rules are transparent, editable, and explainable per message. The client stores nothing in the cloud, runs the rules layer the user can read, and surfaces a \"why is this in this folder?\" affordance backed by the rule that matched.",
        blurb_2: "It's the consumer-facing piece of the same architecture we deploy for clients — same Sieve rules, same audit posture, same guarantee that no opaque ML decided where their mail went.",
    },
    Capability {
        eyebrow: "Audit + observability",
        heading: "Our own audit machinery.",
        blurb_1: "Every PlausiDen repo runs through a catalog of audits we wrote — backend↔frontend coupling, version-control hygiene, supersociety conformance, doctrine adherence, and more. Each audit lives as a checklist plus an automated check; CI gates merges on the automated half and operators run the manual half on a cadence.",
        blurb_2: "Clients get a sanitized version of the same machinery if they want — the audit catalog and checklist generator are designed to be portable across the stacks we engage with.",
    },
];

/// Render `/capabilities` wrapped in the shared site chrome.
#[must_use]
pub fn render() -> Markup {
    let body = html! {
        section class="relative pt-32 pb-16 md:pt-44 md:pb-24 bg-slate-50 overflow-hidden" {
            div class="container relative mx-auto px-4 md:px-6 z-10 max-w-4xl" {
                span class="inline-block px-4 py-1.5 rounded-full bg-primary/10 text-primary font-semibold text-sm mb-6 border border-primary/20" {
                    "What we run, not just what we sell"
                }
                h1 class="font-display text-4xl md:text-5xl lg:text-6xl font-bold text-slate-900 leading-[1.1] mb-6" {
                    "Capabilities."
                }
                p class="text-lg md:text-xl text-slate-600 max-w-2xl leading-relaxed" {
                    "Most consultancies depend on a stack of third-party SaaS to run their own operations. We don't. We build and operate the tools we recommend — mail, outbound, CMS, design system, AI surface, mail client, audit machinery — and the engagements we sell are backed by the same infrastructure we trust ourselves with."
                }
                p class="text-base text-slate-500 mt-4 max-w-2xl leading-relaxed" {
                    "The list below is intentionally general. Tools mature continuously, and we'd rather show what we cover than commit copy to a feature list that's wrong by the time the page is read."
                }
            }
        }

        @for (i, c) in CAPABILITIES.iter().enumerate() {
            section class=(if i % 2 == 0 { "py-16 bg-white" } else { "py-16 bg-slate-50" }) {
                div class="container mx-auto px-4 md:px-6 max-w-3xl" {
                    span class="inline-block px-3 py-1 rounded-full bg-primary/10 text-primary font-semibold text-xs mb-4 border border-primary/20" {
                        (c.eyebrow)
                    }
                    h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mb-6" {
                        (c.heading)
                    }
                    p class="text-slate-600 leading-relaxed mb-4" { (c.blurb_1) }
                    p class="text-slate-600 leading-relaxed" { (c.blurb_2) }
                }
            }
        }

        section class="py-20 bg-primary/5" {
            div class="container mx-auto px-4 md:px-6 text-center max-w-2xl" {
                h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-6" {
                    "Want any of this for your firm?"
                }
                p class="text-slate-600 text-lg mb-8" {
                    "The tools we operate ourselves are the tools we deploy for clients. The engagement model is in /services; the contact form is the start."
                }
                a href="/contact" {
                    button type="button" class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium bg-primary text-primary-foreground border border-primary-border min-h-10 px-8 py-6 rounded-xl text-lg shadow-xl shadow-primary/20 hover:-translate-y-0.5 transition-all" {
                        "Start the conversation"
                    }
                }
            }
        }
    };
    page("Capabilities — PlausiDen", "/capabilities", body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_every_capability() {
        let s = render().into_string();
        for c in CAPABILITIES {
            assert!(s.contains(c.eyebrow), "missing capability: {}", c.eyebrow);
            assert!(s.contains(c.heading), "missing heading: {}", c.heading);
        }
    }

    #[test]
    fn no_specific_versions_or_vendor_names_leaked() {
        // Reviewer guard: this page is the durable copy. It must not
        // bake in concrete vendor names, version numbers, or feature
        // lists that go stale faster than the publishing cadence.
        let s = render().into_string();
        for forbidden in &[
            "Postfix",
            "Dovecot",
            "Iced",
            "Maud",
            "Axum",
            "v1.",
            "v2.",
            "ChatGPT",
            "GPT-4",
            "OpenAI",
        ] {
            assert!(
                !s.contains(forbidden),
                "capabilities page leaks specific tool/vendor: {forbidden}"
            );
        }
    }
}
