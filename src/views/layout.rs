//! Shared page chrome. Matches the production React site's `<head>` and
//! nav / footer structure so visual parity is preserved across server-rendered
//! pages.

use maud::{DOCTYPE, Markup, PreEscaped, html};

/// Shared site <head>. Loads the production Tailwind/shadcn CSS and the two
/// Google Fonts the CSS actually references (`Plus Jakarta Sans`, `Outfit`).
///
/// SECURITY: `<link rel="preconnect" ...>` to the Google Fonts origins is
/// deliberate — paired with the relaxed CSP in `crate::security`. SHIP-DECISION
/// recorded there.
fn head_tag(title: &str) -> Markup {
    html! {
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1";
            meta name="color-scheme" content="light";
            meta name="robots" content="index, follow";
            meta name="apple-mobile-web-app-title" content="PlausiDen";
            title { (title) }
            link rel="icon" type="image/png" href="/static/favicon-96x96.png" sizes="96x96";
            link rel="icon" type="image/svg+xml" href="/static/favicon.svg";
            link rel="shortcut icon" href="/static/favicon.ico";
            link rel="apple-touch-icon" sizes="180x180" href="/static/apple-touch-icon.png";
            link rel="manifest" href="/static/site.webmanifest";
            link rel="preconnect" href="https://fonts.googleapis.com";
            link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
            link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Outfit:wght@100..900&family=Plus+Jakarta+Sans:ital,wght@0,200..800;1,200..800&display=swap";
            link rel="stylesheet" href="/static/index-CWVVhmVm.css";
        }
    }
}

/// Shared top nav. Matches the production DOM classes so shadcn/ui + Tailwind
/// styling applies unchanged.
fn nav() -> Markup {
    html! {
        nav class="fixed top-0 left-0 right-0 z-50 transition-all duration-300 border-b bg-white/90 backdrop-blur-md border-border/50 py-3 shadow-sm" {
            div class="container mx-auto px-4 md:px-6 flex items-center justify-between" {
                a href="/" {
                    div class="flex items-center gap-2 cursor-pointer group" {
                        div class="bg-primary p-1.5 rounded-lg group-hover:scale-105 transition-transform duration-300" {
                            (PreEscaped(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-shield w-6 h-6 text-white"><path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"></path></svg>"#))
                        }
                        span class="font-display font-bold text-xl tracking-tight transition-colors text-slate-900" {
                            "PlausiDen " span class="text-primary" { "LLC" }
                        }
                    }
                }
                div class="hidden md:flex items-center gap-6" {
                    a href="/" {
                        span class="text-sm font-medium transition-colors hover:text-primary cursor-pointer relative group text-slate-600" { "Home" }
                    }
                    a href="/services" {
                        span class="text-sm font-medium transition-colors hover:text-primary cursor-pointer relative group text-slate-600" { "Services" }
                    }
                    a href="/about" {
                        span class="text-sm font-medium transition-colors hover:text-primary cursor-pointer relative group text-slate-600" { "About" }
                    }
                    a href="/contact" {
                        span class="text-sm font-medium transition-colors hover:text-primary cursor-pointer relative group text-slate-600" { "Contact" }
                    }
                    a href="/contact" {
                        button class="inline-flex items-center justify-center whitespace-nowrap font-medium focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 hover-elevate active-elevate-2 border border-emerald-500/50 text-emerald-700 hover:bg-emerald-50 hover:text-emerald-800 min-h-8 rounded-md px-3 text-xs gap-2 group" {
                            (PreEscaped(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-shield w-4 h-4 text-emerald-600 group-hover:scale-110 transition-transform"><path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"></path></svg>"#))
                            "Encrypted Inquiry"
                        }
                    }
                    a href="/contact" {
                        button class="inline-flex items-center justify-center whitespace-nowrap font-medium focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 hover-elevate active-elevate-2 bg-primary text-primary-foreground border border-primary-border min-h-8 rounded-md px-3 text-xs gap-2 shadow-lg shadow-primary/20" {
                            (PreEscaped(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-phone w-4 h-4"><path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path></svg>"#))
                            "Get a Quote"
                        }
                    }
                }
                button class="md:hidden p-2 text-slate-600" aria-label="Open menu" {
                    (PreEscaped(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-menu w-6 h-6"><line x1="4" x2="20" y1="12" y2="12"></line><line x1="4" x2="20" y1="6" y2="6"></line><line x1="4" x2="20" y1="18" y2="18"></line></svg>"#))
                }
            }
        }
    }
}

/// Shared footer. Matches the production 4-column layout (brand / Company /
/// Solutions / Contact) and slate-900 background.
fn footer() -> Markup {
    html! {
        footer class="bg-slate-900 text-slate-300 py-16" {
            div class="container mx-auto px-4 md:px-6" {
                div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-12" {
                    div class="space-y-6" {
                        div class="flex items-center gap-2 text-white" {
                            div class="bg-primary p-1.5 rounded-lg" {
                                (PreEscaped(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-shield w-6 h-6 text-white"><path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"></path></svg>"#))
                            }
                            span class="font-display font-bold text-xl tracking-tight" {
                                "PlausiDen " span class="text-primary" { "LLC" }
                            }
                        }
                        p class="text-slate-400 text-sm leading-relaxed max-w-xs" {
                            "Providing comprehensive, high-quality IT solutions that empower modern enterprises. General yet specific excellence in technology."
                        }
                    }
                    div class="space-y-6" {
                        h3 class="text-white font-display font-semibold text-lg" { "Company" }
                        ul class="space-y-3" {
                            li { a href="/" { span class="text-sm hover:text-white transition-colors cursor-pointer" { "Home" } } }
                            li { a href="/about" { span class="text-sm hover:text-white transition-colors cursor-pointer" { "About Us" } } }
                            li { a href="/services" { span class="text-sm hover:text-white transition-colors cursor-pointer" { "Services" } } }
                            li { a href="/contact" { span class="text-sm hover:text-white transition-colors cursor-pointer" { "Contact" } } }
                        }
                    }
                    div class="space-y-6" {
                        h3 class="text-white font-display font-semibold text-lg" { "Solutions" }
                        ul class="space-y-3 text-sm" {
                            li { "IT Operations" }
                            li { "Cyber Security" }
                            li { "Artificial Intelligence" }
                            li { "Industrial Automation" }
                            li { "Software Development" }
                            li { "Network Architecture" }
                        }
                    }
                    div class="space-y-6" {
                        h3 class="text-white font-display font-semibold text-lg" { "Contact" }
                        ul class="space-y-4" {
                            li class="flex items-start gap-3" {
                                (PreEscaped(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-phone w-5 h-5 text-primary shrink-0 mt-0.5"><path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path></svg>"#))
                                span class="text-sm hover:text-white transition-colors" { "978-351-6495" }
                            }
                            li class="flex items-start gap-3" {
                                (PreEscaped(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-mail w-5 h-5 text-primary shrink-0 mt-0.5"><rect width="20" height="16" x="2" y="4" rx="2"></rect><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path></svg>"#))
                                span class="text-sm hover:text-white transition-colors" { "team@plausiden.com" }
                            }
                            li class="flex items-start gap-3" {
                                (PreEscaped(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-map-pin w-5 h-5 text-primary shrink-0 mt-0.5"><path d="M20 10c0 4.993-5.539 10.193-7.399 11.799a1 1 0 0 1-1.202 0C9.539 20.193 4 14.993 4 10a8 8 0 0 1 16 0"></path><circle cx="12" cy="10" r="3"></circle></svg>"#))
                                span class="text-sm" { "Massachusetts, USA" }
                            }
                        }
                    }
                }
                div class="mt-16 pt-8 border-t border-slate-800 flex flex-col md:flex-row justify-between items-center gap-4 text-xs text-slate-500" {
                    p { "© 2026 PlausiDen LLC. All rights reserved." }
                    div class="flex gap-6" {
                        a href="/privacy-directive" { span class="hover:text-white transition-colors cursor-pointer" { "Sovereign Privacy Directive" } }
                        a href="/terms-of-service" { span class="hover:text-white transition-colors cursor-pointer" { "Sovereign Terms of Service" } }
                    }
                }
            }
        }
    }
}

/// Render a page with the site-wide chrome. `title` appears in the tab;
/// `body` is the per-page `<main>` content.
///
/// BUG ASSUMPTION: The nav + footer here mirror the production React site's
/// rendered DOM. Classes are Tailwind + shadcn/ui; styling lives in
/// `/static/index-CWVVhmVm.css` (copy of the production bundle).
///
/// SECURITY: The page declares two external origins: `fonts.googleapis.com`
/// (stylesheet) and `fonts.gstatic.com` (font binary). Matches the production
/// site's font loading. Consider self-hosting both Plus Jakarta Sans and
/// Outfit to revert the CSP relaxation.
#[must_use]
#[allow(clippy::needless_pass_by_value)] // Markup is PreEscaped<String>; consuming is idiomatic for a composition helper.
pub fn page(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            (head_tag(title))
            body {
                div id="root" {
                    div class="flex flex-col min-h-screen font-body text-slate-900" {
                        (nav())
                        main class="flex-grow" {
                            (body)
                        }
                        (footer())
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_emits_doctype_and_lang() {
        let s = page("Test — PlausiDen", html! { p { "x" } }).into_string();
        assert!(s.starts_with("<!DOCTYPE html>"));
        assert!(s.contains("<html lang=\"en\">"));
    }

    #[test]
    fn page_title_passed_through() {
        let s = page("About — PlausiDen", html! {}).into_string();
        assert!(s.contains("<title>About — PlausiDen</title>"));
    }

    #[test]
    fn page_links_production_stylesheet() {
        let s = page("X", html! {}).into_string();
        assert!(s.contains("/static/index-CWVVhmVm.css"));
    }

    #[test]
    fn page_nav_uses_encrypted_inquiry_not_secure_drop() {
        // REGRESSION-GUARD: the renamed call-to-action must not silently
        // revert to the old 'Secure Drop' wording.
        let s = page("X", html! {}).into_string();
        assert!(s.contains("Encrypted Inquiry"));
        assert!(!s.contains("Secure Drop"));
    }

    #[test]
    fn page_footer_contains_contact_info() {
        let s = page("X", html! {}).into_string();
        assert!(s.contains("978-351-6495"));
        assert!(s.contains("team@plausiden.com"));
        assert!(s.contains("Massachusetts, USA"));
    }
}
