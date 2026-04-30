//! `/feedback` — combined client-feedback + testimonial collection form.
//!
//! Two sections in one form:
//!   1. General feedback — what worked, what didn't, what to change.
//!      Required fields are intentionally minimal so a busy client
//!      can drop a one-paragraph reply.
//!   2. Testimonial — the four-question framework (alternative,
//!      reason for choosing PlausiDen, what changed, recommendation).
//!      Entirely optional; respondents who only want to give general
//!      feedback skip it.
//!
//! Submissions land at `team@plausiden.com` via local Postfix AND
//! persist to a local SQLite store at `/var/lib/plausiden-site/
//! feedback.db`. The `/feedback/export` endpoint dumps the store
//! as JSON / CSV / TSV (token-gated). When PlausiDen-CMS lands,
//! the same SQLite shape will back the CMS substrate; the schema
//! is intentionally CMS-shaped (typed fields per submission).
//!
//! Form chrome composed entirely from typed Loom primitives —
//! Hero for the top band, TextInput / TextArea / Select for fields,
//! Section for the form band, Heading + HelperText for in-form
//! prose, Button(Submit) for the submit CTA.

use loom_components::{
    Button, ButtonSize, ButtonType, ButtonVariant, Decoration, Heading, HeadingLevel, HeadingTone,
    HeadingVariant, HelperSize, HelperText, Hero, HeroBackground, InputType, Section,
    SectionPadding, SectionTheme, SectionWidth, Select, SelectOption, TextArea, TextInput,
};
use maud::{Markup, html};

use crate::views::layout::page;

const ATTRIBUTION_OPTIONS: &[SelectOption<'static>] = &[
    SelectOption {
        value: "",
        label: "Pick one",
    },
    SelectOption {
        value: "full",
        label: "Use my name and company",
    },
    SelectOption {
        value: "name_only",
        label: "Use my name, no company",
    },
    SelectOption {
        value: "role_only",
        label: "Use my role + industry, no name",
    },
    SelectOption {
        value: "anonymous",
        label: "Anonymous (\"client in the legal sector\")",
    },
    SelectOption {
        value: "private",
        label: "Don't publish — feedback only",
    },
];

/// Render the `/feedback` page.
#[must_use]
pub fn render() -> Markup {
    let form_body = html! {
        form method="post" action="/feedback" class="space-y-10" {

            // --- Identity ---
            div class="space-y-4" {
                (Heading {
                    text: "Who you are",
                    level: HeadingLevel::H2,
                    variant: HeadingVariant::Sub,
                    tone: HeadingTone::Ink,
                }.render())
                (TextInput {
                            id: "fb-name",
                            name: "name",
                            label: "Your name",
                            input_type: InputType::Text,
                            placeholder: Some("Your name"),
                            max_length: Some(100),
                            required: true,
                        }.render())
                        (TextInput {
                            id: "fb-company",
                            name: "company",
                            label: "Company / organization",
                            input_type: InputType::Text,
                            placeholder: Some("Optional"),
                            max_length: Some(200),
                            required: false,
                        }.render())
                        (TextInput {
                            id: "fb-email",
                            name: "email",
                            label: "Email (for follow-up)",
                            input_type: InputType::Email,
                            placeholder: Some("Optional"),
                            max_length: Some(200),
                            required: false,
                        }.render())
                    }

            // --- General feedback ---
            div class="space-y-4" {
                (Heading {
                    text: "General feedback",
                    level: HeadingLevel::H2,
                    variant: HeadingVariant::Sub,
                    tone: HeadingTone::Ink,
                }.render())
                (HelperText {
                    text: "Anything you want us to know. Keep it short, keep it honest. One sentence is fine.",
                    size: HelperSize::Default,
                    tone: HeadingTone::Ink,
                }.render())
                (TextArea {
                            id: "fb-worked",
                            name: "worked_well",
                            label: "1. What worked well?",
                            placeholder: None,
                            max_length: Some(2000),
                            required: false,
                        }.render())
                        (TextArea {
                            id: "fb-didnt",
                            name: "didnt_work",
                            label: "2. What didn't?",
                            placeholder: None,
                            max_length: Some(2000),
                            required: false,
                        }.render())
                    }

            // --- Testimonial ---
            div class="space-y-4" {
                (Heading {
                    text: "Testimonial (optional)",
                    level: HeadingLevel::H2,
                    variant: HeadingVariant::Sub,
                    tone: HeadingTone::Ink,
                }.render())
                (HelperText {
                    text: "If you're willing to be quoted on our site, answer as many of these as you'd like. Short and honest beats long and tidy. We'll edit for length, never for voice — and we won't publish anything without your sign-off.",
                    size: HelperSize::Default,
                    tone: HeadingTone::Ink,
                }.render())
                (Select {
                            id: "fb-consent",
                            name: "consent",
                            label: "3. How can we attribute you?",
                            options: ATTRIBUTION_OPTIONS,
                        }.render())
                        (TextArea {
                            id: "fb-alt",
                            name: "alternative",
                            label: "4. Before working with PlausiDen, what was the alternative? What would you have done?",
                            placeholder: None,
                            max_length: Some(2000),
                            required: false,
                        }.render())
                        (TextArea {
                            id: "fb-why",
                            name: "why_chose",
                            label: "5. What made you decide to work with us instead?",
                            placeholder: None,
                            max_length: Some(2000),
                            required: false,
                        }.render())
                        (TextArea {
                            id: "fb-changed",
                            name: "whats_changed",
                            label: "6. What does the working system let you do now that you couldn't before?",
                            placeholder: None,
                            max_length: Some(2000),
                            required: false,
                        }.render())
                        (TextArea {
                            id: "fb-rec",
                            name: "recommend",
                            label: "7. If a peer asked you whether to engage PlausiDen for similar work, what would you tell them?",
                            placeholder: None,
                            max_length: Some(2000),
                            required: false,
                        }.render())
                        (TextArea {
                            id: "fb-extra",
                            name: "anything_else",
                            label: "8. Anything else?",
                            placeholder: None,
                            max_length: Some(2000),
                            required: false,
                        }.render())
                    }

            div class="pt-4" {
                (Button {
                    label: "Send feedback",
                    variant: ButtonVariant::Primary,
                    size: ButtonSize::Lg,
                    aria_label: None,
                    icon: None,
                    decoration: Decoration::SoftShadow,
                    button_type: ButtonType::Submit,
                }.render())
                div class="mt-3" {
                    (HelperText {
                        text: "We'll read it. If you flagged a quote we can use, we'll email you the proposed wording before anything goes live.",
                        size: HelperSize::Tiny,
                        tone: HeadingTone::Ink,
                    }.render())
                }
            }
        }
    };

    let form_section = Section {
        body: &form_body,
        theme: SectionTheme::Light,
        width: SectionWidth::Narrow,
        padding: SectionPadding::Tight,
    }
    .render();

    let body = html! {
        (Hero {
            eyebrow: Some("For clients + collaborators"),
            headline_lead: "Feedback +",
            headline_accent: Some("testimonial."),
            subheadline: "If you've worked with PlausiDen, your feedback shapes what we ship next. The first section is for any feedback; the second section is the optional testimonial questions if you're willing to be quoted. We read every response by hand. Nothing here is shared without your consent.",
            cta: None,
            background: HeroBackground::GridLight,
        }.render())
        (form_section)
    };
    page("Feedback + testimonial — PlausiDen", "/feedback", body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_all_eight_questions() {
        let s = render().into_string();
        for q in &["1.", "2.", "3.", "4.", "5.", "6.", "7.", "8."] {
            assert!(s.contains(q), "feedback page missing question marker {q}");
        }
        assert!(s.contains(r#"name="worked_well""#));
        assert!(s.contains(r#"name="alternative""#));
        assert!(s.contains(r#"name="recommend""#));
        assert!(s.contains(r#"name="consent""#));
    }

    #[test]
    fn posts_to_self() {
        let s = render().into_string();
        assert!(s.contains(r#"action="/feedback""#));
        assert!(s.contains(r#"method="post""#));
    }

    #[test]
    fn consent_options_cover_attribution_modes() {
        let s = render().into_string();
        for opt in &["full", "name_only", "role_only", "anonymous", "private"] {
            assert!(s.contains(opt), "missing consent option: {opt}");
        }
    }

    #[test]
    fn uses_loom_form_primitives() {
        // Spot-check that the input chrome came from loom-components,
        // not raw class strings. Loom inputs all carry h-12 + bg-slate-50.
        let s = render().into_string();
        assert!(s.contains("h-12 bg-slate-50")); // loom-allow: test-assertion literal of expected loom-components output
    }
}
