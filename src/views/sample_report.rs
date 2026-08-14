//! `/sample-report` — the deliverable, shown before anyone has to buy it.
//!
//! Buyers comparing security quotes cannot compare method from a price,
//! and "penetration test" covers everything from a person working
//! through an application to a scan with a cover page. The one artefact
//! that settles it is the report, and almost nobody publishes one. So
//! this page shows the anatomy of ours and walks a single finding all
//! the way through: plain-English summary, technical detail,
//! reproduction, evidence, impact, remediation, retest.
//!
//! HARD CONSTRAINT: nothing here describes a real client. The worked
//! example is a fictional practice, and the page says so twice — once
//! near the top where a skimmer sees it, once beside the evidence block
//! where a careful reader looks. Publishing a real client's findings to
//! win new work would be exactly the judgment failure this firm sells
//! against, and a sanitised-but-real report is still a real report.

use loom_components::hero::{Hero, HeroBackground};
use loom_components::{
    Button, ButtonShape, ButtonSize, ButtonType, ButtonVariant, Decoration, Heading, HeadingLevel,
    HeadingTone, HeadingVariant, Lede, Section, SectionPadding, SectionTheme, SectionWidth,
    TextLink, TextLinkSize, TextLinkVariant,
};
use maud::{Markup, html};

use super::layout::page_with_description;

/// Every section that appears in a PlausiDen report, in order.
///
/// Published because the order is itself a claim: the executive summary
/// comes first and is written for the person who signs, not as a
/// preamble to the technical section.
const REPORT_ANATOMY: &[(&str, &str)] = &[
    (
        "Executive summary",
        "One page. What we tested, what we found, what it means commercially, and what we would fix first. Written for the partner or practice manager who has to decide what to fund, and readable by someone who will never open the rest.",
    ),
    (
        "Scope and rules of engagement",
        "The exact hosts, applications and accounts in scope, the testing window, what we agreed not to touch, and the escalation path if we found something critical mid-engagement. Copied verbatim from the signed scope so there is no drift between what you bought and what we did.",
    ),
    (
        "Method",
        "Which standards the work ran against and to what level, so a reader can tell a tested requirement from an untested one. An assessment that does not state its coverage is not reproducible.",
    ),
    (
        "Findings",
        "One entry per issue, each with reproduction steps, evidence, business impact and remediation. Ordered by what we think you should fix first, which is not always the same as the highest score.",
    ),
    (
        "What we could not test",
        "The parts of the surface we did not reach, and why — out of scope, unavailable during the window, or blocked. A report that implies total coverage it did not have is worse than no report.",
    ),
    (
        "Retest record",
        "What was fixed, what was verified, and what remains open, with dates. Issued after the 30-day retest at no extra cost.",
    ),
];

/// Metadata header for the worked example, rendered as hairline
/// definition rows — the same pattern as the standards list on
/// /services, reused rather than reinvented.
const FINDING_META: &[(&str, &str)] = &[
    ("Finding", "PD-2026-014"),
    (
        "Title",
        "Any authenticated user can read another practice's matter documents",
    ),
    (
        "CVSS 3.1",
        "6.5 Medium — AV:N/AC:L/PR:L/UI:N/S:U/C:H/I:N/A:N",
    ),
    ("Business impact", "High — see note below"),
    ("Mapping", "OWASP ASVS V4.2.1 (IDOR) · MITRE ATT&CK T1213"),
    ("Status", "Fixed; verified at retest on day 12"),
];

/// The reproduction steps, numbered exactly as they appear in a real
/// report. Concrete enough that the client's own engineer can confirm
/// the finding without asking us what we meant.
const REPRO_STEPS: &[&str] = &[
    "Authenticate to the application as a standard user belonging to Practice A.",
    "Open any matter owned by that practice and note the numeric matter ID in the URL.",
    "Call the document listing endpoint directly, substituting a matter ID belonging to Practice B.",
    "Observe a 200 response containing document metadata and signed download URLs for the other practice's matter.",
    "Fetch one of the returned URLs. The document downloads with no further authorisation check.",
];

/// A redacted request/response pair. Deliberately boring: real evidence
/// is a transcript, not a screenshot with a red circle on it.
const EVIDENCE: &str = r#"GET /api/v1/matters/8814/documents HTTP/1.1
Host: app.example-practice.test
Cookie: session=<redacted>

HTTP/1.1 200 OK
Content-Type: application/json

{
  "matter_id": 8814,
  "practice_id": 27,          <-- session belongs to practice_id 14
  "documents": [
    { "id": 41902, "name": "Engagement letter (executed).pdf",
      "url": "https://.../41902?sig=<redacted>" },
    { "id": 41913, "name": "Deposition transcript - draft.docx",
      "url": "https://.../41913?sig=<redacted>" }
  ]
}"#;

/// Things a PlausiDen report will not contain. Stated as commitments
/// because each one is a practice the sales brief positions against,
/// and naming the practice is fair where naming a firm is not.
const NOT_IN_THE_REPORT: &[&str] = &[
    "Raw scanner output pasted in as findings. Tool results are triage input, not a deliverable.",
    "Severity inflated to justify the invoice. An informational finding is labelled informational.",
    "Padding — no forty pages of generic advice about password policy wrapped around six real issues.",
    "Findings without reproduction steps. If your engineer cannot reproduce it, we have not finished writing it.",
    "A separate charge to check whether the fix worked.",
];

/// Render `/sample-report`.
///
/// Ordering is the argument: the illustrative notice comes before any
/// finding text, so nobody can skim the worked example and take it for
/// a real client engagement.
pub fn render() -> Markup {
    let body = html! {
        (Hero {
            eyebrow: Some("Sample deliverable"),
            // Kept short deliberately: "before you buy one." wrapped to leave
            // "one." orphaned on its own line at 1440px.
            headline_lead: "Read the report",
            headline_accent: Some("before you buy."),
            subheadline: "Security testing is sold on trust because the buyer rarely sees the product before paying for it. This is the format we deliver, and a worked finding taken all the way through — summary, reproduction, evidence, impact, fix, retest.",
            cta: None,
            background: HeroBackground::GridLight,
        }.render())

        (illustrative_notice())
        (anatomy_band())
        (worked_finding_band())
        (exclusions_band())
        (final_cta())
    };
    page_with_description(
        "Sample Penetration Test Report — PlausiDen",
        "/sample-report",
        "See the report format before you buy: executive summary, scope, method, findings with reproduction steps and evidence, and the included 30-day retest record.",
        body,
    )
}

/// The "this is not a real client" notice.
///
/// Deliberately styled in the site palette rather than as a yellow
/// warning box: it is a statement of fact, not an error state, and a
/// caution-coloured banner would be the loudest element on a page whose
/// whole argument is restraint.
fn illustrative_notice() -> Markup {
    html! {
        section class="py-10 bg-white" { // loom-allow: notice strip — lighter cadence than a content band
            div class="container mx-auto px-4 md:px-6 max-w-4xl" { // loom-allow: article-width container
                div class="pd-proof" {
                    p class="text-slate-600 text-[15px] leading-relaxed" {
                        span class="font-semibold text-slate-900" { "This example is illustrative. " }
                        "The practice, the hostname, the matter IDs and the finding are invented for this page. "
                        "We do not publish client findings, sanitised or otherwise — a redacted real report is still a real report, "
                        "and the industry it describes is small enough that details identify people."
                    }
                }
            }
        }
    }
}

/// What every report contains, in the order it appears.
fn anatomy_band() -> Markup {
    html! {
        section class="py-24 bg-white" { // loom-allow: content band — py-24 primary cadence
            div class="container mx-auto px-4 md:px-6 max-w-4xl pd-reveal" { // loom-allow: article container with scroll-reveal hook
                span class="text-[10px] uppercase tracking-[0.2em] font-semibold text-slate-400" { "Anatomy" }
                div class="mt-3 mb-6" { // loom-allow: eyebrow-to-heading spacer
                    (Heading {
                        text: "What is in the document.",
                        level: HeadingLevel::H2,
                        variant: HeadingVariant::Section,
                        tone: HeadingTone::Ink,
                    }.render())
                }
                (Lede {
                    text: "Six sections, always in this order. The executive summary is first because the person who signs the invoice should not have to read a findings table to learn whether the engagement went well.",
                    tone: HeadingTone::Ink,
                }.render())
                dl class="border-t border-slate-200 mt-12" { // loom-allow: hairline definition list — same pattern as the /services standards table
                    @for (name, detail) in REPORT_ANATOMY {
                        div class="grid grid-cols-1 md:grid-cols-3 gap-2 md:gap-7 py-5 border-b border-slate-200" { // loom-allow: definition row
                            dt class="font-semibold text-slate-900" { (name) }
                            dd class="md:col-span-2 text-slate-600 text-[15px] md:text-base leading-relaxed font-light" { (detail) }
                        }
                    }
                }
            }
        }
    }
}

/// One finding, start to finish.
fn worked_finding_band() -> Markup {
    html! {
        section class="py-24 bg-slate-50" { // loom-allow: worked-example band — tinted to separate it from the anatomy band above
            div class="container mx-auto px-4 md:px-6 max-w-4xl pd-reveal" { // loom-allow: article container with scroll-reveal hook
                span class="text-[10px] uppercase tracking-[0.2em] font-semibold text-slate-400" { "Worked example" }
                div class="mt-3 mb-6" { // loom-allow: eyebrow-to-heading spacer
                    (Heading {
                        text: "One finding, all the way through.",
                        level: HeadingLevel::H2,
                        variant: HeadingVariant::Section,
                        tone: HeadingTone::Ink,
                    }.render())
                }
                (Lede {
                    text: "An authorization gap, chosen because it is the kind of flaw no scanner reports: every request is well-formed, the endpoint behaves exactly as written, and nothing in the response looks like an error.",
                    tone: HeadingTone::Ink,
                }.render())

                dl class="border-t border-slate-200 mt-12" { // loom-allow: finding metadata as hairline rows — matches the anatomy table
                    @for (label, value) in FINDING_META {
                        div class="grid grid-cols-1 md:grid-cols-3 gap-2 md:gap-7 py-5 border-b border-slate-200" { // loom-allow: definition row
                            dt class="font-semibold text-slate-900" { (label) }
                            dd class="md:col-span-2 text-slate-600 text-[15px] md:text-base leading-relaxed font-light" { (value) }
                        }
                    }
                }

                div class="mt-12" { // loom-allow: sub-section rhythm inside the worked example
                    h3 class="text-[11px] uppercase tracking-[0.18em] font-semibold text-slate-500 mb-6" { "Summary, as the client reads it" }
                    p class="text-slate-600 text-[15px] md:text-base leading-relaxed font-light" {
                        "A logged-in user at one practice can retrieve documents belonging to a different practice by changing a number in a web address. "
                        "No password is needed beyond the user's own, no software is required, and the application records the access as normal activity. "
                        "We reached two documents on a matter that did not belong to the account we were using, including an executed engagement letter."
                    }
                }

                div class="mt-12" { // loom-allow: sub-section rhythm
                    h3 class="text-[11px] uppercase tracking-[0.18em] font-semibold text-slate-500 mb-6" { "Reproduction" }
                    ol class="list-decimal list-inside space-y-3 text-slate-600 text-[15px] md:text-base leading-relaxed font-light" {
                        @for step in REPRO_STEPS {
                            li { (step) }
                        }
                    }
                }

                div class="mt-12" { // loom-allow: sub-section rhythm
                    h3 class="text-[11px] uppercase tracking-[0.18em] font-semibold text-slate-500 mb-6" { "Evidence" }
                    // Wide content scrolls inside its own container so the page
                    // body never scrolls sideways on a phone.
                    div class="overflow-x-auto rounded-xl border border-slate-200 bg-white" { // loom-allow: evidence transcript container
                        pre class="p-5 md:p-6 text-xs md:text-sm font-mono text-slate-700 leading-relaxed" { (EVIDENCE) }
                    }
                    p class="text-slate-500 text-sm mt-3" {
                        "Redacted for publication. In a real report the session token and signed URLs are replaced the same way, and the full transcript is supplied separately."
                    }
                }

                div class="mt-12" { // loom-allow: sub-section rhythm
                    h3 class="text-[11px] uppercase tracking-[0.18em] font-semibold text-slate-500 mb-6" { "Why the two ratings differ" }
                    p class="text-slate-600 text-[15px] md:text-base leading-relaxed font-light" {
                        "CVSS scores this 6.5, Medium. We record the business impact as High, and we say so in the same row rather than quietly overriding the number. "
                        "CVSS measures the technical characteristics of the flaw; it has no way to know that the records are client-confidential, that the practice owes a notification duty if they were disclosed, "
                        "or that an opposing party obtaining a draft deposition transcript is a different category of problem from a data-protection ticket. "
                        "A report that hands you only the score has done the arithmetic and left you the judgment."
                    }
                }

                div class="mt-12" { // loom-allow: sub-section rhythm
                    h3 class="text-[11px] uppercase tracking-[0.18em] font-semibold text-slate-500 mb-6" { "Remediation, and what happened next" }
                    p class="text-slate-600 text-[15px] md:text-base leading-relaxed font-light" {
                        "Enforce the authorisation check server-side, on the practice ID carried by the session rather than the one supplied in the request, "
                        "and apply it to the document-fetch endpoint as well as the listing endpoint — fixing only the listing leaves the signed URLs reachable. "
                        "Add a regression test that asserts a cross-practice request returns 403."
                    }
                    p class="text-slate-600 text-[15px] md:text-base leading-relaxed font-light mt-6" {
                        "This one was reported the afternoon we found it rather than held for the report, because a live cross-tenant disclosure is not a finding to save for a meeting. "
                        "It was fixed in nine days and verified at retest on day twelve, inside the 30-day window included in the price."
                    }
                }
            }
        }
    }
}

/// What the report deliberately omits.
fn exclusions_band() -> Markup {
    html! {
        section class="py-24 bg-white" { // loom-allow: content band — py-24 primary cadence
            div class="container mx-auto px-4 md:px-6 max-w-4xl pd-reveal" { // loom-allow: article container with scroll-reveal hook
                span class="text-[10px] uppercase tracking-[0.2em] font-semibold text-slate-400" { "Exclusions" }
                div class="mt-3 mb-6" { // loom-allow: eyebrow-to-heading spacer
                    (Heading {
                        text: "What you will not find in it.",
                        level: HeadingLevel::H2,
                        variant: HeadingVariant::Section,
                        tone: HeadingTone::Ink,
                    }.render())
                }
                ul class="space-y-4 mt-8" { // loom-allow: commitment list using the shared proof-rule treatment
                    @for item in NOT_IN_THE_REPORT {
                        li class="pd-proof text-slate-600 text-[15px] md:text-base leading-relaxed font-light" { (item) }
                    }
                }
                p class="text-slate-500 text-sm mt-12" { // loom-allow: prose with inline links
                    "The method behind it is on the "
                    (TextLink { label: "services page", href: "/services", variant: TextLinkVariant::Underlined, size: TextLinkSize::Default }.render())
                    ", and the engagement model is on "
                    (TextLink { label: "how we work", href: "/how-we-work", variant: TextLinkVariant::Underlined, size: TextLinkSize::Default }.render())
                    "."
                }
            }
        }
    }
}

fn final_cta() -> Markup {
    let cta_button = Button {
        label: "Book a scoping call",
        variant: ButtonVariant::Primary,
        size: ButtonSize::Lg,
        aria_label: None,
        icon: None,
        decoration: Decoration::SoftShadow,
        button_type: ButtonType::Button,
        shape: ButtonShape::default(),
    }
    .render();
    let body = html! {
        div class="text-center pd-reveal" {
            div class="mb-6" {
                (Heading {
                    text: "Want one of these about your own systems?",
                    level: HeadingLevel::H2,
                    variant: HeadingVariant::Section,
                    tone: HeadingTone::Ink,
                }.render())
            }
            div class="mb-8" {
                (Lede {
                    text: "The scoping call is 45 minutes and costs nothing. We agree the surface, the window and the price in writing before any testing starts, and you will know inside that call whether we are the right firm for the work.",
                    tone: HeadingTone::Ink,
                }.render())
            }
            a href="/contact" { (cta_button) }
        }
    };
    Section {
        body: &body,
        theme: SectionTheme::Tinted,
        width: SectionWidth::Article,
        padding: SectionPadding::Loose,
    }
    .render()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The page must say, unmissably, that the example is not a real
    /// client. This is the one assertion on this page that is not about
    /// sales — publishing a real engagement to win new work would be a
    /// confidentiality failure, and the disclaimer is what keeps a
    /// future edit honest.
    #[test]
    fn the_example_is_labelled_illustrative() {
        let page = render().into_string();
        assert!(
            page.contains("This example is illustrative"),
            "the illustrative-example notice has been removed or reworded; \
             a sample report without it reads as a published client engagement"
        );
        assert!(
            page.contains("We do not publish client findings"),
            "the page no longer states that client findings are never published"
        );
    }

    /// A worked finding without reproduction steps is a claim, not a
    /// demonstration — which is precisely the practice this page argues
    /// against.
    #[test]
    fn the_worked_finding_can_actually_be_reproduced() {
        assert!(
            REPRO_STEPS.len() >= 4,
            "a reproduction section this short does not demonstrate anything"
        );
        let page = render().into_string();
        for step in REPRO_STEPS {
            let head: String = step.chars().take(30).collect();
            assert!(
                page.contains(&head),
                "reproduction step missing from page: {head:?}"
            );
        }
    }

    /// The retest window is a commercial promise encoded in
    /// `salesman-quote` as well. Both must agree, or a prospect reads
    /// one number here and receives another in the quote.
    #[test]
    fn retest_window_matches_the_rate_card() {
        let page = render().into_string();
        assert!(
            page.contains("30-day") || page.contains("30 days"),
            "the sample report no longer states the 30-day retest window"
        );
    }

    /// No real host, client or person may appear. `example-practice.test`
    /// is deliberate: `.test` is reserved by RFC 2606 and can never be
    /// registered, so the transcript cannot resolve to somebody's site.
    #[test]
    fn the_worked_example_uses_a_reserved_domain() {
        assert!(
            EVIDENCE.contains(".test"),
            "the evidence transcript should use an RFC 2606 reserved domain so it \
             can never resolve to a real host"
        );
        for real in ["plausiden.com", "gmail", "@", "192.168.", "10.0."] {
            if real == "@" {
                continue;
            }
            assert!(
                !EVIDENCE.contains(real),
                "the evidence transcript contains {real:?}, which reads as a real target"
            );
        }
    }
}
