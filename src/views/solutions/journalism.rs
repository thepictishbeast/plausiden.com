//! `/solutions/journalism` — vertical landing page for newsroom IT.
//!
//! Audience: editor, IT lead, or technologist at a small-to-mid-sized
//! newsroom (or freelancer collective). Already pre-qualified by an
//! outbound email; the page confirms fit and produces a contact-form
//! submit.
//!
//! BUG ASSUMPTION: We never imply we are providing legal advice
//! around journalist privilege; only that we operate the
//! infrastructure that supports it.
//!
//! Page structure is shared with `legal` + `healthcare` via
//! [`super::template::render_vertical_landing`] — only the per-vertical
//! copy lives here.

use loom_icons as icons;
use maud::Markup;

use super::template::{Capability, EngagementStep, VerticalLanding, render_vertical_landing};

const DESCRIPTION: &str = "IT infrastructure designed around source confidentiality. Encrypted source channels, threat-modeled endpoints, Tor onion publication, subpoena-ready records discipline. Built for newsrooms whose adversaries include state actors and well-resourced corporate counsel.";

/// Render the journalism vertical landing page.
#[must_use]
pub fn render() -> Markup {
    let svg_lock = icons::LOCK.render();
    let svg_users = icons::USERS.render();
    let svg_file = icons::FILE_TEXT.render();
    let svg_shield = icons::SHIELD.render();
    let svg_globe = icons::GLOBE.render();
    let svg_audit = icons::CLIPBOARD_CHECK.render();

    render_vertical_landing(&VerticalLanding {
        title: "Journalism IT — PlausiDen",
        path: "/solutions/journalism",
        description: DESCRIPTION,

        hero_eyebrow: "For journalists + newsrooms",
        hero_lead: "Sources should be able to trust you.",
        hero_accent: "Your tools should too.",
        hero_subheadline: "Source confidentiality is your most binding obligation, and the threat model is real. We build infrastructure for newsrooms whose adversaries include state-level actors, well-resourced corporate counsel, and routine credential phishing. Same posture, sized to your team.",
        primary_cta_label: "Schedule a threat-model conversation",

        pain_heading: "What good infrastructure looks like for a newsroom",
        pain_paragraphs: &[
            "Most IT vendors treat newsrooms like any other small business: a Microsoft 365 tenant, a help desk, occasional patching. That's not enough. Your threat model includes targeted phishing tuned to specific bylines, supply-chain attacks on commodity collaboration tools, lawful and unlawful demands for source identification, and the very real risk of a former staffer's laptop becoming a source-list disclosure.",
            "We start somewhere different. We design assuming compromise is possible and bound the blast radius accordingly. The result tends to look unfamiliar to general-purpose IT shops — and like a relief to editors who've been doing this work in their head.",
        ],

        capabilities_heading: "What we cover",
        capabilities_lede: "Concrete capabilities where small newsrooms most often need help.",
        capabilities: &[
            Capability {
                icon_svg: &svg_lock,
                title: "Encrypted source channels",
                description: "Self-hosted secure dropbox alternative — without the SecureDrop operational overhead. End-to-end encrypted intake routing to specific journalists, key rotation discipline, anonymous-tip workflows that don't depend on Tor literacy from the source.",
            },
            Capability {
                icon_svg: &svg_users,
                title: "Newsroom-aware access discipline",
                description: "Per-story / per-investigation access scopes. An investigative team's working files are visible to that team only — not to the metro desk, not to legal until it's time. Onboarding/offboarding scripts that don't leave a former freelancer with stale source access.",
            },
            Capability {
                icon_svg: &svg_file,
                title: "Document handling with metadata discipline",
                description: "Workflow that strips metadata before publication, preserves it for verification audit trails, and treats document-level access as a first-class concept. Designed for the moment a leaked PDF needs to be cleaned without losing the chain-of-custody record.",
            },
            Capability {
                icon_svg: &svg_shield,
                title: "Endpoint hardening sized to your staff",
                description: "Threat-modeled laptop + phone configurations: full-disk encryption, application allowlists, USB policy, DLP. Tuned to your reporters' actual workflows — not enterprise theater that gets disabled the first time it blocks a deadline.",
            },
            Capability {
                icon_svg: &svg_globe,
                title: "Tor + onion publishing",
                description: "Secondary publication channel via Tor onion service. Submitted as part of standard infrastructure, not a side project. Your readers in countries where the clearnet site is blocked still reach you.",
            },
            Capability {
                icon_svg: &svg_audit,
                title: "Subpoena-ready records discipline",
                description: "Logs, retention policies, and access trails organized for the moment legal calls. Documentation that survives a subpoena response, a 230-c-2 takedown demand, or a Pulitzer-side records request. We know which evidence each reviewer wants because we have prepared this kind of dossier before.",
            },
        ],

        posture_eyebrow: "Why newsrooms come to us",
        posture_heading: "Source confidentiality is a property of the architecture, not a promise.",
        posture_paragraphs: &[
            "Most vendors say they take source confidentiality seriously and ask you to take their word for it. We design pipelines where the promise is verifiable from the architecture — where the answer to \"could this system have leaked the source list?\" is sometimes \"no, by construction\" instead of \"let me check the logs.\"",
            "When something goes wrong — and infrastructure eventually does — your incident response is shorter because the blast radius was bounded by design. When legal calls, you have answers your IT vendor can defend in writing.",
        ],
        posture_check_lines: &[
            "Engagements scope-limited to what we agreed to touch — no roving access to your source list or working files.",
            "Configuration changes proposed in writing before they ship; nothing changes silently.",
            "Documentation that survives a personnel change at our shop or yours.",
            "Plain-English explanations of every choice, so editorial leadership understands what's in place and why.",
        ],

        engagement_heading: "How an engagement starts",
        engagement_intro: "There's no template. Every newsroom we work with starts with a different bottleneck — a recent phishing campaign, a pending subpoena, a beat that suddenly needs onion publication, a Slack workspace that became a de-facto source list. The intake conversation is short; the proposal that follows is specific.",
        engagement_steps: &[
            EngagementStep {
                title: "Threat-model conversation (no commitment)",
                description: "A 45-minute call about your beat, your sources' adversaries, your current pain. We sign a mutual NDA before; we leave with enough to write a real proposal.",
            },
            EngagementStep {
                title: "Written proposal",
                description: "Specific scope, specific deliverables, specific price. We adjust to grant cycles where applicable.",
            },
            EngagementStep {
                title: "Implementation + handoff",
                description: "We do the work; you get documentation that lets your next vendor (or in-house technologist) run it without us. Optional ongoing retainer for monitoring + incident response.",
            },
        ],

        cta_heading: "Ready to talk?",
        cta_subline: "Tell us what's on your plate — even if you're not sure whether it's an IT problem yet. The first conversation is free, the NDA is mutual, and we'll tell you if we're not the right fit.",
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_nonempty() {
        let s = render().into_string();
        assert!(s.len() > 5000);
    }

    #[test]
    fn hero_references_sources_and_audience() {
        let s = render().into_string();
        assert!(s.contains("Source"));
        assert!(s.contains("journalists") || s.contains("newsrooms"));
    }

    #[test]
    fn capability_grid_present() {
        let s = render().into_string();
        for cap in &[
            "Encrypted source channels",
            "Newsroom-aware access",
            "Document handling",
            "Endpoint hardening",
            "Tor + onion publishing",
            "Subpoena-ready records",
        ] {
            assert!(s.contains(cap), "missing capability card: {cap}");
        }
    }

    #[test]
    fn no_legal_advice_claim() {
        let s = render().into_string().to_lowercase();
        for forbidden in &["legal advice", "we advise"] {
            assert!(!s.contains(forbidden), "forbidden phrase: {forbidden}");
        }
    }

    #[test]
    fn final_cta_points_to_contact() {
        let s = render().into_string();
        assert!(s.contains(r#"href="/contact""#));
        assert!(s.contains("team@plausiden.com"));
    }
}
