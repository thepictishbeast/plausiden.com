//! Contact page — proper Maud markup. Visual styling matches the
//! production design (same Tailwind classes, same card layout, same
//! lucide icons), but the form is real HTML with stable IDs, accessible
//! labels, and a proper `action="/contact" method="post"`.
//!
//! BUG ASSUMPTION: Form field `name` attributes must match
//! [`crate::inquiry::InquiryForm`] exactly — `name`, `email`, `phone`,
//! `company`, `service`, `message`. Renaming a field here without
//! updating the handler breaks submissions silently.
//!
//! SECURITY: Input field lengths are enforced server-side; client-side
//! `maxlength` attributes are convenience-only (they match the server
//! limits to keep UX feedback honest).

use maud::{Markup, PreEscaped, html};

use super::layout::page;

const SVG_PHONE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-phone w-6 h-6 text-primary"><path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path></svg>"#;

const SVG_MAIL: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-mail w-6 h-6 text-primary"><rect width="20" height="16" x="2" y="4" rx="2"></rect><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path></svg>"#;

const SVG_PIN: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-map-pin w-6 h-6 text-primary"><path d="M20 10c0 4.993-5.539 10.193-7.399 11.799a1 1 0 0 1-1.202 0C9.539 20.193 4 14.993 4 10a8 8 0 0 1 16 0"></path><circle cx="12" cy="10" r="3"></circle></svg>"#;

/// Standard `<input>` class string — matches the production shadcn-styled
/// input visual exactly.
const INPUT_CLASSES: &str = "flex w-full rounded-md border border-input px-3 py-2 text-base ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 md:text-sm h-12 bg-slate-50";

const TEXTAREA_CLASSES: &str = "flex w-full rounded-md border border-input px-3 py-2 text-base ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 md:text-sm min-h-[150px] bg-slate-50 resize-none";

const SELECT_CLASSES: &str = "flex w-full rounded-md border border-input px-3 py-2 text-base ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 md:text-sm h-12 bg-slate-50";

const LABEL_CLASSES: &str = "text-sm font-medium leading-none";

/// Render the contact page.
#[must_use]
pub fn render() -> Markup {
    let body = html! {
        div class="pt-20 min-h-screen bg-slate-50" {
            // Header band
            div class="bg-slate-900 text-white py-16" {
                div class="container mx-auto px-4 md:px-6 text-center" {
                    h1 class="font-display text-4xl md:text-5xl font-bold mb-4" { "Contact Us" }
                    p class="text-xl text-slate-300" { "Reach out for a quote and free consultation today." }
                }
            }

            div class="container mx-auto px-4 md:px-6 py-16 -mt-10" {
                div class="grid grid-cols-1 lg:grid-cols-3 gap-8" {

                    // Left card — contact details
                    div class="lg:col-span-1 space-y-6 reveal" {
                        div class="shadcn-card rounded-xl border border-card-border text-card-foreground border-none shadow-xl bg-white overflow-hidden" {
                            div class="bg-primary p-6" {
                                h3 class="text-white font-display text-xl font-bold" { "Get in touch" }
                                p class="text-primary-foreground/80 mt-2 text-sm" {
                                    "We're here to answer any questions about our IT solutions."
                                }
                            }
                            div class="p-8 space-y-8" {
                                (contact_row(SVG_PHONE, "Phone", "tel:9783516495", "978-351-6495"))
                                (contact_row(SVG_MAIL, "Email", "mailto:team@plausiden.com", "team@plausiden.com"))
                                (contact_row_text(SVG_PIN, "Location", "Massachusetts, USA"))
                            }
                        }
                    }

                    // Right card — the actual form
                    div class="lg:col-span-2 reveal" {
                        div class="shadcn-card rounded-xl border bg-card border-card-border text-card-foreground border-none shadow-xl" {
                            div class="p-8 md:p-10" {
                                h2 class="font-display text-2xl font-bold text-slate-900 mb-6" { "Send us a message" }

                                form action="/contact" method="post" class="space-y-6" {

                                    div class="grid grid-cols-1 md:grid-cols-2 gap-6" {
                                        div class="space-y-2" {
                                            label class=(LABEL_CLASSES) for="contact-name" { "Full Name" }
                                            input type="text" id="contact-name" name="name"
                                                class=(INPUT_CLASSES) placeholder="John Doe" maxlength="100";
                                        }
                                        div class="space-y-2" {
                                            label class=(LABEL_CLASSES) for="contact-email" { "Email Address" }
                                            input type="email" id="contact-email" name="email"
                                                class=(INPUT_CLASSES) placeholder="john@company.com" maxlength="200" required;
                                        }
                                    }

                                    div class="grid grid-cols-1 md:grid-cols-2 gap-6" {
                                        div class="space-y-2" {
                                            label class=(LABEL_CLASSES) for="contact-phone" { "Phone Number" }
                                            input type="tel" id="contact-phone" name="phone"
                                                class=(INPUT_CLASSES) placeholder="(555) 000-0000" maxlength="50";
                                        }
                                        div class="space-y-2" {
                                            label class=(LABEL_CLASSES) for="contact-company" { "Company Name" }
                                            input type="text" id="contact-company" name="company"
                                                class=(INPUT_CLASSES) placeholder="Your Business LLC" maxlength="200";
                                        }
                                    }

                                    div class="space-y-2" {
                                        label class=(LABEL_CLASSES) for="contact-service" { "Service Interest" }
                                        select id="contact-service" name="service" class=(SELECT_CLASSES) {
                                            option value="" { "Select a service" }
                                            option value="IT Operations" { "IT Operations" }
                                            option value="Cyber Security" { "Cyber Security" }
                                            option value="Artificial Intelligence" { "Artificial Intelligence" }
                                            option value="Industrial Automation" { "Industrial Automation" }
                                            option value="Software Development" { "Software Development" }
                                            option value="Hardware Solutions" { "Hardware Solutions" }
                                            option value="Networking" { "Networking" }
                                            option value="Other" { "Other / General Inquiry" }
                                        }
                                    }

                                    div class="space-y-2" {
                                        label class=(LABEL_CLASSES) for="contact-message" { "Message" }
                                        textarea id="contact-message" name="message"
                                            class=(TEXTAREA_CLASSES)
                                            placeholder="Tell us about your project or requirements..."
                                            maxlength="5000" required {}
                                    }

                                    button type="submit"
                                        class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring bg-primary text-primary-foreground border border-primary-border min-h-9 px-4 py-2 w-full h-12 text-lg font-semibold shadow-lg shadow-primary/20 transition-colors hover:bg-primary/90" {
                                        "Send Message"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
    page("Contact — PlausiDen", "/contact", body)
}

fn contact_row(svg: &str, label: &str, href: &str, text: &str) -> Markup {
    html! {
        div class="flex items-start gap-4" {
            div class="bg-primary/10 p-3 rounded-lg" { (PreEscaped(svg)) }
            div {
                p class="font-semibold text-slate-900" { (label) }
                a href=(href) class="text-slate-600 hover:text-primary transition-colors" { (text) }
            }
        }
    }
}

fn contact_row_text(svg: &str, label: &str, text: &str) -> Markup {
    html! {
        div class="flex items-start gap-4" {
            div class="bg-primary/10 p-3 rounded-lg" { (PreEscaped(svg)) }
            div {
                p class="font-semibold text-slate-900" { (label) }
                p class="text-slate-600" { (text) }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contact_renders_nonempty() {
        let s = render().into_string();
        assert!(s.len() > 4000, "contact page unexpectedly short: {} bytes", s.len());
    }

    #[test]
    fn contact_form_posts_to_self() {
        let s = render().into_string();
        assert!(s.contains(r#"action="/contact""#));
        assert!(s.contains(r#"method="post""#));
    }

    /// Every input/textarea/select has a stable `id` whose label
    /// references it via `for=` — fixes the prior axe a11y violations.
    #[test]
    fn every_field_has_label_binding() {
        let s = render().into_string();
        for field in &[
            "contact-name",
            "contact-email",
            "contact-phone",
            "contact-company",
            "contact-service",
            "contact-message",
        ] {
            assert!(
                s.contains(&format!(r#"for="{field}""#)),
                "missing label for=\"{field}\""
            );
            assert!(
                s.contains(&format!(r#"id="{field}""#)),
                "missing id=\"{field}\""
            );
        }
    }

    #[test]
    fn field_names_match_handler_struct() {
        // These must match crate::inquiry::InquiryForm field names
        // (or aliases). Renaming requires both sides to change.
        let s = render().into_string();
        for name in &["name", "email", "phone", "company", "service", "message"] {
            assert!(
                s.contains(&format!(r#"name="{name}""#)),
                "missing form field name=\"{name}\""
            );
        }
    }

    #[test]
    fn no_unsplash_origin() {
        assert!(!render().into_string().contains("images.unsplash.com"));
    }

    #[test]
    fn no_secure_drop_label() {
        assert!(!render().into_string().contains(">Secure Drop<"));
    }

    /// The legacy React `:r0:-form-item` IDs (which broke a11y) must
    /// not return as a regression.
    #[test]
    fn no_react_use_id_artifacts() {
        let s = render().into_string();
        assert!(!s.contains(":r0:"));
        assert!(!s.contains(":-form-item"));
    }
}
