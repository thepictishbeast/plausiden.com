//! Admin login + feedback dashboard views.
//!
//! Visually consistent with the public marketing pages — same nav,
//! footer, and Hero primitive. The only departure is the dashboard,
//! which uses a denser table layout that wouldn't fit in the marketing
//! grid.

use loom_components::hero::{Hero, HeroBackground};
use loom_components::{
    Badge, BadgeSize, BadgeTone, Heading, HeadingLevel, HeadingTone, HeadingVariant, TextLink,
    TextLinkSize, TextLinkVariant,
};
use maud::{Markup, html};

use super::layout::page;
use crate::feedback_store::FeedbackRow;

/// `GET /admin/login` view. `error` carries an optional banner message
/// surfaced after a bad email or expired link.
#[must_use]
pub fn login(error: Option<&str>) -> Markup {
    // The whole sign-in surface lives inside a single centered card on
    // desktop and a full-width column on mobile. The card is capped at
    // ~28rem so the input + button don't sprawl across a 1440px viewport.
    let body = html! {
        section class="relative min-h-[80vh] flex items-center justify-center overflow-hidden bg-slate-50 py-16 md:py-24" { // loom-allow: vertically-centred login surface — min-h-[80vh] + flex centering not in Loom Section vocabulary
            div class="absolute inset-0 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:24px_24px]" {} // loom-allow: SVG grid fleck — same pattern as blog hero
            div class="relative z-10 w-full max-w-md mx-auto px-4 md:px-6" { // loom-allow: max-w-md login container with z-10 fleck stacking
                div class="bg-white rounded-2xl border border-slate-200 shadow-sm p-6 md:p-8" { // loom-allow: inset login card chrome — rounded-2xl + shadow-sm
                    div class="mb-6 text-center" { // loom-allow: centred login-card header
                        div class="mb-4" {
                            (Badge { label: "Admin", tone: BadgeTone::Primary, size: BadgeSize::Sm }.render())
                        }
                        div class="mb-2" {
                            (Heading {
                                text: "Sign in",
                                level: HeadingLevel::H1,
                                variant: HeadingVariant::Sub,
                                tone: HeadingTone::Ink,
                            }.render())
                        }
                        p class="text-sm text-slate-600 leading-relaxed" { // loom-allow: login lede — smaller than Loom Lede on phone
                            "Enter your email — we'll send a one-time sign-in link."
                        }
                    }
                    @if let Some(msg) = error {
                        div class="mb-4 rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-800" { // loom-allow: red error banner — semantic colour, no Loom ErrorBanner primitive
                            (msg)
                        }
                    }
                    form method="post" action="/admin/login" class="space-y-4" { // loom-allow: login form vertical rhythm
                        label class="block" { // loom-allow: field label wrapper
                            span class="block text-sm font-semibold text-slate-700 mb-2" { "Email" } // loom-allow: field label text
                            input
                                type="email"
                                name="email"
                                required
                                autocomplete="email"
                                inputmode="email"
                                placeholder="you@example.com"
                                class="block w-full rounded-md border border-slate-300 px-3 py-2 text-slate-900 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary"; // loom-allow: bespoke email input — Loom TextInput composes a label-input-helper triple, doesn't fit a single-input login form
                        }
                        button
                            type="submit"
                            class="w-full inline-flex items-center justify-center rounded-md bg-primary px-4 py-2.5 text-white font-semibold hover:bg-primary/90 focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2 transition-colors" { // loom-allow: w-full submit button — Loom Button has no Block-width variant
                            "Send sign-in link"
                        }
                    }
                    p class="mt-6 text-xs text-slate-500 text-center" { // loom-allow: footer note — micro-meta line below form
                        "Links expire in 15 minutes and can be used once."
                    }
                }
            }
        }
    };
    page("Admin sign-in — PlausiDen", "/admin/login", body)
}

/// Convenience wrapper for an error banner without showing the form
/// twice. Used by verify-failure paths.
#[must_use]
pub fn login_error(message: &str) -> Markup {
    login(Some(message))
}

/// `POST /admin/login` ack page.
#[must_use]
pub fn magic_link_sent(email: &str) -> Markup {
    let cta = html! {
        (TextLink {
            label: "← Back home",
            href: "/",
            variant: TextLinkVariant::PrimaryBold,
            size: TextLinkSize::Default,
        }.render())
    };
    let subline = format!(
        "If {email} is authorized, a sign-in link is on its way. Check your inbox — and your spam folder, just in case."
    );
    let body = html! {
        (Hero {
            eyebrow: Some("Check your email"),
            headline_lead: "Link sent.",
            headline_accent: None,
            subheadline: &subline,
            cta: Some(&cta),
            background: HeroBackground::GridLight,
        }.render())
    };
    page("Check your email — PlausiDen", "/admin/login", body)
}

/// `GET /admin/feedback` dashboard.
#[must_use]
pub fn feedback_dashboard(email: &str, rows: &[FeedbackRow]) -> Markup {
    let body = html! {
        section class="relative pt-32 pb-8 md:pt-40 md:pb-12 overflow-hidden bg-slate-50" { // loom-allow: dashboard hero — pt-32/40 cadence + tighter pb-8/12 below Loom Section padding scale
            div class="absolute inset-0 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:24px_24px]" {} // loom-allow: SVG grid fleck — same pattern as blog hero
            div class="container relative mx-auto px-4 md:px-6 z-10 max-w-6xl" { // loom-allow: dashboard container max-w-6xl wider than Loom Wide
                div class="flex flex-col md:flex-row md:items-center md:justify-between gap-3" { // loom-allow: header row — title left, exports right; responsive flex
                    div {
                        div class="mb-3" {
                            (Badge { label: "Admin · Feedback", tone: BadgeTone::Primary, size: BadgeSize::Sm }.render())
                        }
                        (Heading {
                            text: "Submitted feedback",
                            level: HeadingLevel::H1,
                            variant: HeadingVariant::Section,
                            tone: HeadingTone::Ink,
                        }.render())
                        p class="text-sm text-slate-600 mt-2" { // loom-allow: signed-in-as line with inline span — TextLink pattern doesn't compose
                            "Signed in as " span class="font-semibold text-slate-800" { (email) } "." // loom-allow: bolded inline email span inside a sentence
                        }
                    }
                    div class="flex items-center gap-3" { // loom-allow: header right — exports + logout
                        (TextLink {
                            label: "Export JSON",
                            href: "/feedback/export?format=json",
                            variant: TextLinkVariant::Underlined,
                            size: TextLinkSize::Small,
                        }.render())
                        (TextLink {
                            label: "Export CSV",
                            href: "/feedback/export?format=csv",
                            variant: TextLinkVariant::Underlined,
                            size: TextLinkSize::Small,
                        }.render())
                        form method="post" action="/admin/logout" class="inline" { // loom-allow: inline POST form for logout
                            button type="submit" class="text-sm text-slate-500 underline hover:text-slate-700" { "Sign out" } // loom-allow: textual logout button — looks like a TextLink, posts a form
                        }
                    }
                }
            }
        }
        section class="py-8 md:py-12 bg-white" { // loom-allow: dashboard body band — tighter py-8/12 cadence
            div class="container mx-auto px-4 md:px-6 max-w-6xl" { // loom-allow: dashboard container max-w-6xl
                @if rows.is_empty() {
                    div class="rounded-lg border border-slate-200 bg-slate-50 p-8 text-center" { // loom-allow: empty-state panel — centred grey box
                        p class="text-slate-600" { "No feedback submissions yet." } // loom-allow: empty-state prose
                    }
                } @else {
                    p class="text-sm text-slate-500 mb-6" { (rows.len()) " submission(s)" } // loom-allow: submission counter line
                    div class="space-y-6" { // loom-allow: vertical rhythm between feedback cards
                        @for row in rows.iter().rev() {
                            (feedback_card(row))
                        }
                    }
                }
            }
        }
    };
    page("Admin · Feedback — PlausiDen", "/admin/feedback", body)
}

fn feedback_card(row: &FeedbackRow) -> Markup {
    let received = row.received_at.format("%Y-%m-%d %H:%M UTC").to_string();
    html! {
        article class="rounded-xl border border-slate-200 bg-white shadow-sm overflow-hidden" { // loom-allow: feedback-card shell — admin-only chrome
            header class="border-b border-slate-100 bg-slate-50 px-5 py-3 flex flex-col sm:flex-row sm:items-center sm:justify-between gap-1" { // loom-allow: card header bar — name + meta on a tinted strip
                div {
                    span class="font-semibold text-slate-900" { (row.name) } // loom-allow: contributor name — bolded ink
                    @if !row.company.is_empty() {
                        span class="text-slate-400" { " · " } // loom-allow: dot separator
                        span class="text-slate-700" { (row.company) } // loom-allow: company name — slightly lighter ink
                    }
                    @if !row.email.is_empty() {
                        span class="text-slate-400" { " · " } // loom-allow: dot separator
                        @let mailto = format!("mailto:{}", row.email);
                        (TextLink {
                            label: &row.email,
                            href: &mailto,
                            variant: TextLinkVariant::PrimaryUnderlined,
                            size: TextLinkSize::Default,
                        }.render())
                    }
                }
                div class="text-xs text-slate-500" { // loom-allow: meta line — id + timestamp + consent
                    "#" (row.id) " · " (received) " · consent: " span class="font-mono" { (display_consent(&row.consent)) } // loom-allow: mono consent value
                }
            }
            div class="p-5 grid gap-4 md:grid-cols-2" { // loom-allow: 2-up field-block grid inside card body
                (field_block("What worked well", &row.worked_well))
                (field_block("What didn't", &row.didnt_work))
                (field_block("Alternative considered", &row.alternative))
                (field_block("Why chose PlausiDen", &row.why_chose))
                (field_block("What's changed", &row.whats_changed))
                (field_block("Would recommend", &row.recommend))
                (field_block("Anything else", &row.anything_else))
            }
        }
    }
}

fn field_block(label: &str, value: &str) -> Markup {
    if value.trim().is_empty() {
        return html! {};
    }
    html! {
        div {
            div class="text-xs font-semibold text-slate-500 uppercase tracking-wide mb-1" { (label) } // loom-allow: field label — uppercase-tracked micro-label
            p class="text-slate-700 whitespace-pre-line text-sm leading-relaxed" { (value) } // loom-allow: field value — preserves authored line breaks
        }
    }
}

fn display_consent(c: &str) -> &str {
    match c {
        "" => "(none)",
        s => s,
    }
}
