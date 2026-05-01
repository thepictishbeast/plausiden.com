//! `/solutions/nonprofit` — vertical landing page for small nonprofits
//! handling donor and beneficiary data.
//!
//! Page structure shared with the other vertical landings via
//! [`super::template::render_vertical_landing`]. This variant runs the
//! posture band as paragraphs only and skips the engagement-steps band.

use loom_icons as icons;
use maud::Markup;

use super::template::{render_vertical_landing, Capability, VerticalLanding};

const DESCRIPTION: &str = "IT infrastructure for small nonprofits. Donor data hardened against breach, beneficiary confidentiality preserved, audit trails ready for grant reviews + state charity examiners. Sized to a 5–50 person mission, not an enterprise.";

/// Render `/solutions/nonprofit`.
#[must_use]
pub fn render() -> Markup {
    let svg_lock = icons::LOCK.render();
    let svg_users = icons::USERS.render();
    let svg_file = icons::FILE_TEXT.render();
    let svg_shield = icons::SHIELD.render();
    let svg_audit = icons::CLIPBOARD_CHECK.render();
    let svg_heart = icons::HEART.render();

    render_vertical_landing(VerticalLanding {
        title: "Nonprofit IT — PlausiDen",
        path: "/solutions/nonprofit",
        description: DESCRIPTION,

        hero_eyebrow: "For nonprofits + advocacy orgs",
        hero_lead: "Donor trust + beneficiary confidentiality,",
        hero_accent: "designed in.",
        hero_subheadline: "Donors trust you with money; beneficiaries trust you with their stories. Both deserve infrastructure that takes \"don't share this\" as a structural property, not a courtesy. We design it for the funding scale you actually operate at.",
        primary_cta_label: "Schedule a privacy review",

        pain_heading: "Nonprofit IT, sized to a real mission",
        pain_paragraphs: &[
            "Most IT vendors price nonprofits like they price for-profits, then offer a 10% discount. We don't. We size engagements to actual nonprofit budgets, default to recommending free + open-source where it serves you, and produce documentation grant funders can read without an interpreter.",
            "Mission-aligned organizations also tend to handle some of the most sensitive data in the country: domestic-violence survivor records, asylum-seeker case files, abortion-fund applications, immigrant-services intake. We design the technical posture that protects both your donors and the people you serve.",
        ],

        capabilities_heading: "What we cover",
        capabilities_lede: "Capability areas where small nonprofits most often need help.",
        capabilities: &[
            Capability {
                icon_svg: &svg_lock,
                title: "Donor + beneficiary data isolation",
                description: "Per-program / per-fund / per-cohort access scopes. The development team sees donors; case-management sees beneficiaries; auditors see what they need without seeing each other. Compromise of one role doesn't expose the other.",
            },
            Capability {
                icon_svg: &svg_heart,
                title: "Threat-modeled by program type",
                description: "Different programs have different adversaries. Domestic-violence services design for stalkerware threat models; immigrant-services design for state-level surveillance; advocacy orgs design against subpoena-driven discovery. We tailor by program.",
            },
            Capability {
                icon_svg: &svg_file,
                title: "Grant + funder reporting",
                description: "Reporting infrastructure that produces the right answer per funder, on schedule, without re-keying spreadsheets. Audit trails that satisfy state-charity-bureau examination + IRS Form 990 supporting docs.",
            },
            Capability {
                icon_svg: &svg_users,
                title: "Volunteer + contractor onboarding",
                description: "Onboarding flow for volunteers, interns, contractors, and case workers that scopes access to the program they actually work on. Termination is a one-command operation, not a 12-step checklist.",
            },
            Capability {
                icon_svg: &svg_shield,
                title: "Phishing + impersonation defenses",
                description: "Tuned to the lures that target nonprofits: fake grant-portal login pages, donor-impersonation fund redirects, board-member-spoofing wire requests. Endpoint defenses sized to a small staff.",
            },
            Capability {
                icon_svg: &svg_audit,
                title: "Audit-ready documentation",
                description: "Logs, access reviews, control documentation organized for grant-funder vendor reviews, state charity registrations, and 501(c)(3) governance audits. Often the single biggest gap a small nonprofit has.",
            },
        ],

        posture_eyebrow: "Why nonprofits come to us",
        posture_heading: "Mission alignment in the technical posture, not just the marketing.",
        posture_paragraphs: &[
            "Most IT vendors say they support nonprofits by offering a discount. We support them by designing infrastructure that reflects mission alignment in the architecture: donors are not products, beneficiaries are not data points, and access scopes mirror the trust your mission requires.",
            "We also know nonprofit budgets are real budgets. Engagements are scoped tightly, recommendations default to free + open-source where it serves you, and we have a sliding-scale option for organizations under $500k annual budget.",
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
    fn hero_references_donors_and_beneficiaries() {
        let s = render().into_string();
        assert!(s.contains("Donor"));
        assert!(s.contains("eneficiar"));
    }

    #[test]
    fn no_advice_claim() {
        let s = render().into_string().to_lowercase();
        for forbidden in &["legal advice", "we advise", "tax advice"] {
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
