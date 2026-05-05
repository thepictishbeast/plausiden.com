//! `/solutions/legal` — vertical landing page for law-firm IT.
//!
//! Audience: a managing partner, IT director, or office administrator
//! at a small-to-mid-sized firm who clicked through from an outbound
//! email. Already pre-qualified by the email; the page's job is to
//! confirm we're the right fit and produce a contact-form submit.
//!
//! BUG ASSUMPTION: We never make legal-compliance claims that imply we
//! provide legal advice. Wording is "designed around" / "built for" —
//! never "compliant with" without a specific, verified scope.
//!
//! Page structure is shared with `healthcare` + `journalism` via
//! [`super::template::render_vertical_landing`] — only the per-vertical
//! copy lives here.

use loom_icons as icons;
use maud::Markup;

use super::template::{Capability, EngagementStep, VerticalLanding, render_vertical_landing};

const DESCRIPTION: &str = "IT infrastructure designed around a law firm's duty of confidentiality. Self-hosted email, matter-aware document handling, audit-ready compliance posture. We design pipelines where the privacy guarantee is provable, not promised.";

/// Render the law-firm vertical landing page.
#[must_use]
pub fn render() -> Markup {
    let svg_lock = icons::LOCK.render();
    let svg_file = icons::FILE_TEXT.render();
    let svg_audit = icons::CLIPBOARD_CHECK.render();
    let svg_users = icons::USERS.render();
    let svg_shield = icons::SHIELD.render();

    render_vertical_landing(&VerticalLanding {
        title: "Legal IT — PlausiDen",
        path: "/solutions/legal",
        description: DESCRIPTION,

        hero_eyebrow: "For law firms",
        hero_lead: "IT infrastructure your duty of confidentiality",
        hero_accent: "can rest on.",
        hero_subheadline: "Your obligations under ABA Model Rule 1.6 don't pause at the firewall. We design and operate the systems behind a modern practice — email, document handling, client communication, audit trails — for firms whose work demands a posture stronger than \"trust us.\"",
        primary_cta_label: "Schedule a confidentiality review",

        pain_heading: "What \"good IT for a law firm\" actually means",
        pain_paragraphs: &[
            "Most managed-service providers treat law firms like any other small business: a few seats, a Microsoft 365 tenant, a backup script, a help desk. That works until it doesn't — until the day a client asks who has access to their matter file, until an opposing counsel issues a discovery preservation letter, until your malpractice carrier asks specific questions on the renewal questionnaire.",
            "We start somewhere different: with the legal-ethics constraints that actually shape your practice, then design the technical posture that satisfies them. The result tends to look unfamiliar to general-purpose IT shops — and like a relief to firm administrators who've had to translate between counsel and a vendor for years.",
        ],

        capabilities_heading: "What we cover",
        capabilities_lede: "Concrete capability areas where firms most often need help. Engagements typically start with one and expand.",
        capabilities: &[
            Capability {
                icon_svg: &svg_lock,
                title: "Confidential email + file sharing",
                description: "Self-hosted mail with TLS-required transport, DKIM/SPF/DMARC enforced, encrypted-at-rest storage, and routing that never sends client data through third-party content scanners. No \"smart features\" that require reading messages.",
            },
            Capability {
                icon_svg: &svg_file,
                title: "Matter-aware document handling",
                description: "Document management with per-matter access control, retention policies that honor preservation obligations, and audit trails that survive the inevitable \"who saw what, when?\" question. Designed to make e-discovery production faster, not slower.",
            },
            Capability {
                icon_svg: &svg_audit,
                title: "Compliance-ready audit posture",
                description: "Logs, access reviews, and control documentation organized for state-bar inquiries, cyber-liability questionnaires, and client-side vendor reviews. We've answered these questions before; we know which evidence each reviewer actually wants.",
            },
            Capability {
                icon_svg: &svg_users,
                title: "Conflicts and access discipline",
                description: "User and group structure that mirrors how matters actually run — partner / associate / paralegal / outside counsel — so an ethical wall is enforced by the file system, not by an attorney remembering not to look. Onboarding and offboarding scripts that don't leave a former associate with stale access.",
            },
            Capability {
                icon_svg: &svg_shield,
                title: "Threat-modeled defense",
                description: "Phishing resistance tuned for the lures that actually target lawyers (wire-fraud impersonations, false subpoenas, malicious filing-portal lookalikes). Endpoint and network defenses sized to the firm — no enterprise theater you can't operate.",
            },
            Capability {
                icon_svg: &svg_file,
                title: "Continuity and recovery",
                description: "Backups that survive ransomware (immutable, tested, restorable to a known-good point). Documented runbooks for the scenarios most likely to take a small firm offline. Retainer-grade response if the worst day happens.",
            },
        ],

        posture_eyebrow: "Why firms come to us",
        posture_heading: "We design infrastructure where the privacy guarantee is provable, not promised.",
        posture_paragraphs: &[
            "Most vendors say they take privacy seriously and ask you to take their word for it. We design pipelines where the promise is verifiable from the architecture — where the answer to \"could this system have leaked X?\" is sometimes \"no, by construction\" instead of \"let me check the logs.\"",
            "That posture matters in two places. First, when a client or opposing counsel asks pointed questions about your data handling, you have answers your IT vendor can defend in writing. Second, when something goes wrong — and infrastructure eventually does — your incident response is shorter because the blast radius was bounded by design.",
        ],
        posture_check_lines: &[
            "Engagements scope-limited to what we agreed to touch — no roving access to your file server.",
            "Configuration changes proposed in writing before they ship; nothing changes silently.",
            "Documentation that survives a personnel change at our shop or yours.",
            "Plain-English explanations of every choice, so your malpractice carrier and outside auditor can each understand what's in place.",
        ],

        engagement_heading: "How an engagement starts",
        engagement_intro: "There's no template. Every firm we work with starts with a different bottleneck — a flagged renewal questionnaire, a botched cloud migration, a client demand we read on their behalf, an associate who left with the wrong things on a laptop. The intake conversation is short; the proposal that follows is specific.",
        engagement_steps: &[
            EngagementStep {
                title: "Confidentiality review (no commitment)",
                description: "A 45-minute call about your current setup, current pain, and the specific obligations that shape your decisions. We sign an NDA before; we leave with enough to write a real proposal.",
            },
            EngagementStep {
                title: "Written proposal",
                description: "Specific scope, specific deliverables, specific price. Includes which of your existing tools stay, which we replace, and what the 90-day picture looks like.",
            },
            EngagementStep {
                title: "Implementation + handoff",
                description: "We do the work; you get documentation that lets your next vendor (or in-house IT person) run it without us. Optional ongoing retainer for monitoring and incident response.",
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

    /// Hero must speak to law firms and surface ABA Rule 1.6, the
    /// foundational confidentiality obligation. If a marketing pass
    /// strips this, the page loses its hook.
    #[test]
    fn hero_references_aba_rule() {
        let s = render().into_string();
        assert!(s.contains("Rule 1.6"));
        assert!(s.contains("law firms"));
    }

    #[test]
    fn capability_grid_present() {
        let s = render().into_string();
        for cap in &[
            "Confidential email",
            "Matter-aware document handling",
            "Compliance-ready audit posture",
            "Threat-modeled defense",
        ] {
            assert!(s.contains(cap), "missing capability card: {cap}");
        }
    }

    #[test]
    fn engagement_steps_numbered() {
        let s = render().into_string();
        assert!(s.contains("Confidentiality review"));
        assert!(s.contains("Written proposal"));
        assert!(s.contains("Implementation + handoff"));
    }

    #[test]
    fn final_cta_points_to_contact() {
        let s = render().into_string();
        assert!(s.contains(r#"href="/contact""#));
        assert!(s.contains("team@plausiden.com"));
    }

    /// REGRESSION-GUARD: we must not accidentally claim the page
    /// itself constitutes legal advice.
    #[test]
    fn no_legal_advice_claim() {
        let s = render().into_string();
        // Forbidden phrases that imply we're giving legal advice
        for forbidden in &["legal advice", "this advice", "we advise"] {
            assert!(
                !s.to_lowercase().contains(forbidden),
                "page implies legal advice: '{forbidden}'"
            );
        }
    }
}
