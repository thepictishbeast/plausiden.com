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

use loom_components::form::{InputType, Select, SelectOption, TextArea, TextInput};
use loom_icons as icons;
use maud::{Markup, PreEscaped, html};

use super::layout::page;

const SERVICE_OPTIONS: &[SelectOption<'static>] = &[
    SelectOption {
        value: "",
        label: "Select a service",
    },
    SelectOption {
        value: "IT Operations",
        label: "IT Operations",
    },
    SelectOption {
        value: "Cyber Security",
        label: "Cyber Security",
    },
    SelectOption {
        value: "Artificial Intelligence",
        label: "Artificial Intelligence",
    },
    SelectOption {
        value: "Industrial Automation",
        label: "Industrial Automation",
    },
    SelectOption {
        value: "Software Development",
        label: "Software Development",
    },
    SelectOption {
        value: "Hardware Solutions",
        label: "Hardware Solutions",
    },
    SelectOption {
        value: "Networking",
        label: "Networking",
    },
    SelectOption {
        value: "Other",
        label: "Other / General Inquiry",
    },
];

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
                                (contact_row(&icons::PHONE.render(), "Phone", "tel:9783516495", "978-351-6495"))
                                (contact_row(&icons::MAIL.render(), "Email", "mailto:team@plausiden.com", "team@plausiden.com"))
                                (contact_row_text(&icons::MAP_PIN.render(), "Location", "Massachusetts, USA"))
                            }
                        }
                    }

                    // Right card — the actual form
                    div class="lg:col-span-2 reveal" {
                        div class="shadcn-card rounded-xl border bg-card border-card-border text-card-foreground border-none shadow-xl" {
                            div class="p-8 md:p-10" {
                                h2 class="font-display text-2xl font-bold text-slate-900 mb-6" { "Send us a message" }

                                form action="/contact" method="post" class="space-y-6" {

                                    // Honeypot — visually hidden via the project's
                                    // `sr-only` utility (positioned off-screen, no
                                    // inline style attribute so CSP stays strict).
                                    // A naive contact-form-spam bot fills every input;
                                    // server reads `website` and silently drops if
                                    // non-empty. Field name is innocuous so the bot
                                    // won't notice. COUPLING-EXEMPT: not a UI element
                                    // for users; no nav link or backend route to audit.
                                    div class="sr-only" aria-hidden="true" {
                                        label for="contact-website" { "Leave this field empty" }
                                        input type="text" id="contact-website" name="website" tabindex="-1" autocomplete="off" value="";
                                    }

                                    div class="grid grid-cols-1 md:grid-cols-2 gap-6" {
                                        (TextInput {
                                            id: "contact-name",
                                            name: "name",
                                            label: "Full Name",
                                            input_type: InputType::Text,
                                            placeholder: Some("John Doe"),
                                            max_length: Some(100),
                                            required: false,
                                        }.render())
                                        (TextInput {
                                            id: "contact-email",
                                            name: "email",
                                            label: "Email Address",
                                            input_type: InputType::Email,
                                            placeholder: Some("john@company.com"),
                                            max_length: Some(200),
                                            required: true,
                                        }.render())
                                    }

                                    div class="grid grid-cols-1 md:grid-cols-2 gap-6" {
                                        (TextInput {
                                            id: "contact-phone",
                                            name: "phone",
                                            label: "Phone Number",
                                            input_type: InputType::Tel,
                                            placeholder: Some("(555) 000-0000"),
                                            max_length: Some(50),
                                            required: false,
                                        }.render())
                                        (TextInput {
                                            id: "contact-company",
                                            name: "company",
                                            label: "Company Name",
                                            input_type: InputType::Text,
                                            placeholder: Some("Your Business LLC"),
                                            max_length: Some(200),
                                            required: false,
                                        }.render())
                                    }

                                    (Select {
                                        id: "contact-service",
                                        name: "service",
                                        label: "Service Interest",
                                        options: SERVICE_OPTIONS,
                                    }.render())

                                    (TextArea {
                                        id: "contact-message",
                                        name: "message",
                                        label: "Message",
                                        placeholder: Some("Tell us about your project or requirements..."),
                                        max_length: Some(5000),
                                        required: true,
                                    }.render())

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
        assert!(
            s.len() > 4000,
            "contact page unexpectedly short: {} bytes",
            s.len()
        );
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
