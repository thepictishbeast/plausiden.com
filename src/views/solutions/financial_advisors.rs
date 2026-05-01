//! `/solutions/financial-advisors` — vertical landing page for small
//! RIAs and wealth-management practices.
//!
//! Page structure shared with `legal` / `healthcare` / `journalism` /
//! `nonprofit` via [`super::template::render_vertical_landing`]. This
//! variant omits the engagement-steps band (the audience self-selects
//! into a "let's talk" without needing a 3-step explainer) and runs
//! the posture band as paragraphs only.

use loom_icons as icons;
use maud::Markup;

use super::template::{render_vertical_landing, Capability, VerticalLanding};

const DESCRIPTION: &str = "IT infrastructure for small RIAs and wealth-management practices. SEC custody-rule-aware, SOC 2-friendly, BCP-prepared. We design the technical posture custodians and clients are starting to ask about — before they ask.";

/// Render `/solutions/financial-advisors`.
#[must_use]
pub fn render() -> Markup {
    let svg_lock = icons::LOCK.render();
    let svg_users = icons::USERS.render();
    let svg_file = icons::FILE_TEXT.render();
    let svg_shield = icons::SHIELD.render();
    let svg_audit = icons::CLIPBOARD_CHECK.render();

    render_vertical_landing(VerticalLanding {
        title: "Financial Advisor IT — PlausiDen",
        path: "/solutions/financial-advisors",
        description: DESCRIPTION,

        hero_eyebrow: "For RIAs + wealth management",
        hero_lead: "Custodians are asking. ",
        hero_accent: "Be ready before they do.",
        hero_subheadline: "Custodians and clients increasingly want technical evidence: SOC 2 attestation, written information security plans, vendor management documentation, BCP rehearsal logs. We design the posture that produces those answers — sized to a small advisory practice, not a wirehouse.",
        primary_cta_label: "Schedule a custodian-readiness review",

        pain_heading: "What \"good IT for an advisory practice\" means",
        pain_paragraphs: &[
            "Most small advisory practices outgrow their IT before they outgrow their compliance. The Microsoft 365 tenant works until a custodian's vendor questionnaire arrives, until a client demands evidence of how their data is protected, until the WISP that was \"on the list\" needs to be a real document with real controls.",
            "We design IT for the moment those questions stop being theoretical. The posture is sized to a one-to-twenty-advisor practice — no enterprise theater, no shelfware. You get documentation that survives a custodian review, an SEC examination preparation, and a malpractice carrier renewal.",
        ],

        capabilities_heading: "What we cover",
        capabilities_lede: "Capability areas where small advisory practices most often need help.",
        capabilities: &[
            Capability {
                icon_svg: &svg_lock,
                title: "Client-data isolation by advisor",
                description: "Per-advisor access scopes that mirror how books actually run. Departing advisor takes their book, not the firm's. New advisor onboards with a clean access surface, not the previous person's leftover Drive shares.",
            },
            Capability {
                icon_svg: &svg_audit,
                title: "SOC 2-aligned controls + WISP",
                description: "Written information security plan that reflects what the systems actually do, not boilerplate. Control documentation organized for SOC 2 auditor review, custodian vendor questionnaires, and SEC examination preparation.",
            },
            Capability {
                icon_svg: &svg_file,
                title: "Books + records retention discipline",
                description: "Email + document retention that satisfies Rule 204-2 of the Advisers Act — non-erasable, non-rewritable, indexed. Audit trails that survive a regulatory document request.",
            },
            Capability {
                icon_svg: &svg_users,
                title: "Vendor management discipline",
                description: "Inventory of every cloud service holding client data, a real BAA-equivalent agreement with each, periodic re-reviews. Custodians ask for this; we have the template.",
            },
            Capability {
                icon_svg: &svg_shield,
                title: "Wire-fraud + impersonation defenses",
                description: "Email authentication tuned for the specific lures targeting advisory practices: client-impersonation wires, fee-quarter spoofing, custodian-portal lookalikes. Endpoint defenses sized to a small practice.",
            },
            Capability {
                icon_svg: &svg_audit,
                title: "BCP that's rehearsed, not aspirational",
                description: "Business continuity plan documented + tested annually. Restoration runbooks for the realistic scenarios (laptop failure, ransomware, key-employee departure). The carrier renewal questionnaire becomes a five-minute fill-in instead of a fire drill.",
            },
        ],

        posture_eyebrow: "Why advisory practices come to us",
        posture_heading: "We design infrastructure that produces the evidence regulators ask for.",
        posture_paragraphs: &[
            "Most vendors say they understand SEC compliance and ask you to take their word for it. We design pipelines where the controls are documented inline, the audit trail is reproducible from logs, and the answer to \"can you show this?\" is \"yes, here\" instead of \"let me check.\"",
            "When the custodian-side vendor review arrives, you forward a packet. When the SEC examiner's letter arrives, you respond on time, in writing, with evidence. Both events go from existential threats to routine paperwork.",
        ],
        posture_check_lines: &[],

        engagement_heading: "",
        engagement_intro: "",
        engagement_steps: &[],

        cta_heading: "Ready to talk?",
        cta_subline: "Tell us what's on your plate — even if you're not sure whether it's an IT problem yet. The first conversation is free, the NDA is mutual, and we'll tell you if we're not the right fit.",
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_nonempty() {
        assert!(render().into_string().len() > 5000);
    }

    #[test]
    fn hero_references_custodians_and_audience() {
        let s = render().into_string();
        assert!(s.contains("Custodians"));
        assert!(s.to_lowercase().contains("rias") || s.to_lowercase().contains("advisor"));
    }

    /// REGRESSION-GUARD: must not give legal/financial/regulatory advice.
    #[test]
    fn no_advice_claim() {
        let s = render().into_string().to_lowercase();
        for forbidden in &[
            "legal advice",
            "financial advice",
            "investment advice",
            "we advise",
        ] {
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
