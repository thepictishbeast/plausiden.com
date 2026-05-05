//! `/solutions/healthcare` — vertical landing page for small healthcare
//! practices and their administrators.
//!
//! Audience: practice manager, IT lead, or owner-operator at an
//! independent practice (1-25 providers). Already pre-qualified by an
//! outbound email; the page confirms fit and produces a contact-form
//! submit.
//!
//! BUG ASSUMPTION: We never claim "HIPAA compliant" — that's a
//! determination only a covered entity can make. We claim our systems
//! are "designed around HIPAA's Security Rule" or similar.
//!
//! Page structure is shared with `legal` + `journalism` via
//! [`super::template::render_vertical_landing`] — only the per-vertical
//! copy lives here.

use loom_icons as icons;
use maud::Markup;

use super::template::{Capability, EngagementStep, VerticalLanding, render_vertical_landing};

const DESCRIPTION: &str = "IT infrastructure designed around HIPAA's Security Rule for small healthcare practices. Self-hosted email, audit-ready ePHI handling, BAA-ready posture. Built for practices that take patient confidentiality as the floor, not the ceiling.";

/// Render the small-healthcare vertical landing page.
#[must_use]
pub fn render() -> Markup {
    let svg_lock = icons::LOCK.render();
    let svg_heart = icons::HEART.render();
    let svg_file = icons::FILE_TEXT.render();
    let svg_audit = icons::CLIPBOARD_CHECK.render();
    let svg_users = icons::USERS.render();
    let svg_shield = icons::SHIELD.render();

    render_vertical_landing(&VerticalLanding {
        title: "Healthcare IT — PlausiDen",
        path: "/solutions/healthcare",
        description: DESCRIPTION,

        hero_eyebrow: "For healthcare practices",
        hero_lead: "Patient confidentiality,",
        hero_accent: "designed in.",
        hero_subheadline: "HIPAA's Security Rule is the floor for every IT decision in your practice. We design infrastructure that satisfies the rule by construction — and gives you the documentation that BAAs, breach-notification timelines, and OCR audits will eventually ask for.",
        primary_cta_label: "Schedule a security review",

        pain_heading: "What \"good IT for a small practice\" actually means",
        pain_paragraphs: &[
            "Most managed-service providers treat a five-provider clinic the same as a real-estate office: a Microsoft 365 tenant, a backup script, a help desk number. That works until OCR sends a letter, until your malpractice carrier asks specific questions about ePHI handling, until a former employee's laptop walks out the door.",
            "We start somewhere different: with the Security Rule's actual administrative, physical, and technical safeguards, then design the technical posture that satisfies them. The result tends to look unfamiliar to general-purpose IT shops — and like a relief to administrators who've been translating between counsel, carrier, and vendor for years.",
        ],

        capabilities_heading: "What we cover",
        capabilities_lede: "Concrete capability areas where small practices most often need help.",
        capabilities: &[
            Capability {
                icon_svg: &svg_lock,
                title: "ePHI-aware email + messaging",
                description: "Self-hosted mail with TLS-required transport, DKIM/SPF/DMARC enforced, encrypted-at-rest storage. Patient communication routed without third-party content scanners. Bcc-to-self workflows that survive an OCR records request.",
            },
            Capability {
                icon_svg: &svg_heart,
                title: "EHR integration without the surface",
                description: "Sane integration with whichever EHR you actually run (Athena, eClinicalWorks, Cerner, OpenEMR). We harden the access surface around the EHR rather than replace it; we never recommend a switch unless the existing one is the actual bottleneck.",
            },
            Capability {
                icon_svg: &svg_file,
                title: "Document handling with retention discipline",
                description: "Document storage with HIPAA-aligned access control, retention policies that honor state record-keeping rules, and audit trails that survive the inevitable \"who saw this chart?\" question.",
            },
            Capability {
                icon_svg: &svg_audit,
                title: "BAA + audit-ready posture",
                description: "Logs, access reviews, control documentation, and risk assessments organized for OCR audits, malpractice questionnaires, and downstream BAA partner reviews. We've answered these questions before; we know which evidence each reviewer wants.",
            },
            Capability {
                icon_svg: &svg_users,
                title: "Workforce access discipline",
                description: "User and group structure mirroring how the practice actually runs — providers, nurses, billing, admin, contractors. Onboarding/offboarding scripts that don't leave a former employee with stale ePHI access. Termination is a one-command operation, not a 12-step checklist.",
            },
            Capability {
                icon_svg: &svg_shield,
                title: "Threat-modeled defense",
                description: "Phishing resistance tuned to the lures targeting clinics (fake referral attachments, billing-software impersonations, ransomware aimed at practices unable to operate offline). Endpoint and network defenses sized to the practice — no enterprise theater you can't operate.",
            },
        ],

        posture_eyebrow: "Why practices come to us",
        posture_heading: "We design infrastructure where the privacy guarantee is provable, not promised.",
        posture_paragraphs: &[
            "Most vendors say they take privacy seriously and ask you to take their word for it. We design pipelines where the promise is verifiable from the architecture — where the answer to \"could this system have leaked this chart?\" is sometimes \"no, by construction\" instead of \"let me check the logs.\"",
            "That posture matters in two places. First, when OCR or your malpractice carrier asks pointed questions, you have answers your IT vendor can defend in writing. Second, when something goes wrong — and infrastructure eventually does — your incident response is shorter because the blast radius was bounded by design.",
        ],
        posture_check_lines: &[
            "Engagements scope-limited to what we agreed to touch — no roving access to your EHR or chart store.",
            "Configuration changes proposed in writing before they ship; nothing changes silently.",
            "Documentation that survives a personnel change at our shop or yours.",
            "Plain-English explanations of every choice, so your malpractice carrier and outside auditor can each understand what's in place.",
        ],

        engagement_heading: "How an engagement starts",
        engagement_intro: "There's no template. Every practice we work with starts with a different bottleneck — a flagged carrier questionnaire, a botched cloud migration, a ransomware near-miss, an employee who left with the wrong things on a laptop. The intake conversation is short; the proposal that follows is specific.",
        engagement_steps: &[
            EngagementStep {
                title: "Security review (no commitment)",
                description: "A 45-minute call about your current setup, current pain, and the specific obligations that shape your decisions. We sign a mutual NDA before; we leave with enough to write a real proposal.",
            },
            EngagementStep {
                title: "Written proposal + BAA scope",
                description: "Specific scope, specific deliverables, specific price. Includes a draft BAA scope and clarity on which of your existing tools stay vs. change.",
            },
            EngagementStep {
                title: "Implementation + handoff",
                description: "We do the work; you get documentation that lets your next vendor (or in-house IT person) run it without us. Optional ongoing retainer for monitoring + incident response.",
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
    fn hero_references_security_rule_and_audience() {
        let s = render().into_string();
        assert!(s.contains("Security Rule"));
        assert!(s.contains("healthcare practices"));
    }

    #[test]
    fn capability_grid_present() {
        let s = render().into_string();
        for cap in &[
            "ePHI-aware email",
            "EHR integration",
            "Document handling",
            "BAA + audit-ready",
            "Workforce access",
            "Threat-modeled defense",
        ] {
            assert!(s.contains(cap), "missing capability card: {cap}");
        }
    }

    /// REGRESSION-GUARD: the page must not claim "HIPAA compliant"
    /// outright — only a covered entity can determine that.
    #[test]
    fn no_unconditional_hipaa_compliant_claim() {
        let s = render().into_string().to_lowercase();
        assert!(
            !s.contains("hipaa compliant"),
            "page implies HIPAA compliance"
        );
        assert!(!s.contains("we are compliant with hipaa"));
    }

    #[test]
    fn no_legal_or_medical_advice_claim() {
        let s = render().into_string().to_lowercase();
        for forbidden in &["legal advice", "medical advice", "we advise"] {
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
