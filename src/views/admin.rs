//! Admin login + feedback dashboard views.
//!
//! Visually consistent with the public marketing pages — same nav,
//! footer, and Hero primitive. The only departure is the dashboard,
//! which uses a denser table layout that wouldn't fit in the marketing
//! grid.

use loom_components::hero::{Hero, HeroBackground};
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
        section class="relative min-h-[80vh] flex items-center justify-center overflow-hidden bg-slate-50 py-16 md:py-24" {
            div class="absolute inset-0 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:24px_24px]" {}
            div class="relative z-10 w-full max-w-md mx-auto px-4 md:px-6" {
                div class="bg-white rounded-2xl border border-slate-200 shadow-sm p-6 md:p-8" {
                    div class="mb-6 text-center" {
                        span class="inline-block px-3 py-1 rounded-full bg-primary/10 text-primary font-semibold text-xs border border-primary/20 mb-4" { "Admin" }
                        h1 class="font-display text-2xl md:text-3xl font-bold text-slate-900 leading-tight mb-2" { "Sign in" }
                        p class="text-sm text-slate-600 leading-relaxed" {
                            "Enter your email — we'll send a one-time sign-in link."
                        }
                    }
                    @if let Some(msg) = error {
                        div class="mb-4 rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-800" {
                            (msg)
                        }
                    }
                    form method="post" action="/admin/login" class="space-y-4" {
                        label class="block" {
                            span class="block text-sm font-semibold text-slate-700 mb-2" { "Email" }
                            input
                                type="email"
                                name="email"
                                required
                                autocomplete="email"
                                inputmode="email"
                                placeholder="you@example.com"
                                class="block w-full rounded-md border border-slate-300 px-3 py-2 text-slate-900 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary";
                        }
                        button
                            type="submit"
                            class="w-full inline-flex items-center justify-center rounded-md bg-primary px-4 py-2.5 text-white font-semibold hover:bg-primary/90 focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2 transition-colors" {
                            "Send sign-in link"
                        }
                    }
                    p class="mt-6 text-xs text-slate-500 text-center" {
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
        a href="/" class="text-primary font-semibold" { "← Back home" }
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
        section class="relative pt-32 pb-8 md:pt-40 md:pb-12 overflow-hidden bg-slate-50" {
            div class="absolute inset-0 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:24px_24px]" {}
            div class="container relative mx-auto px-4 md:px-6 z-10 max-w-6xl" {
                div class="flex flex-col md:flex-row md:items-center md:justify-between gap-3" {
                    div {
                        span class="inline-block px-3 py-1 rounded-full bg-primary/10 text-primary font-semibold text-xs border border-primary/20 mb-3" { "Admin · Feedback" }
                        h1 class="font-display text-3xl md:text-4xl font-bold text-slate-900 leading-[1.1]" {
                            "Submitted feedback"
                        }
                        p class="text-sm text-slate-600 mt-2" {
                            "Signed in as " span class="font-semibold text-slate-800" { (email) } "."
                        }
                    }
                    div class="flex items-center gap-3" {
                        a href="/feedback/export?format=json" class="text-sm text-primary font-semibold underline" { "Export JSON" }
                        a href="/feedback/export?format=csv" class="text-sm text-primary font-semibold underline" { "Export CSV" }
                        form method="post" action="/admin/logout" class="inline" {
                            button type="submit" class="text-sm text-slate-500 underline hover:text-slate-700" { "Sign out" }
                        }
                    }
                }
            }
        }
        section class="py-8 md:py-12 bg-white" {
            div class="container mx-auto px-4 md:px-6 max-w-6xl" {
                @if rows.is_empty() {
                    div class="rounded-lg border border-slate-200 bg-slate-50 p-8 text-center" {
                        p class="text-slate-600" { "No feedback submissions yet." }
                    }
                } @else {
                    p class="text-sm text-slate-500 mb-6" { (rows.len()) " submission(s)" }
                    div class="space-y-6" {
                        @for row in rows.iter().rev() {
                            (feedback_card(row))
                        }
                    }
                }
            }
        }
    };
    page(
        "Admin · Feedback — PlausiDen",
        "/admin/feedback",
        body,
    )
}

fn feedback_card(row: &FeedbackRow) -> Markup {
    let received = row.received_at.format("%Y-%m-%d %H:%M UTC").to_string();
    html! {
        article class="rounded-xl border border-slate-200 bg-white shadow-sm overflow-hidden" {
            header class="border-b border-slate-100 bg-slate-50 px-5 py-3 flex flex-col sm:flex-row sm:items-center sm:justify-between gap-1" {
                div {
                    span class="font-semibold text-slate-900" { (row.name) }
                    @if !row.company.is_empty() {
                        span class="text-slate-400" { " · " }
                        span class="text-slate-700" { (row.company) }
                    }
                    @if !row.email.is_empty() {
                        span class="text-slate-400" { " · " }
                        a href={"mailto:" (row.email)} class="text-primary underline" { (row.email) }
                    }
                }
                div class="text-xs text-slate-500" {
                    "#" (row.id) " · " (received) " · consent: " span class="font-mono" { (display_consent(&row.consent)) }
                }
            }
            div class="p-5 grid gap-4 md:grid-cols-2" {
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
            div class="text-xs font-semibold text-slate-500 uppercase tracking-wide mb-1" { (label) }
            p class="text-slate-700 whitespace-pre-line text-sm leading-relaxed" { (value) }
        }
    }
}

fn display_consent(c: &str) -> &str {
    match c {
        "" => "(none)",
        s => s,
    }
}
