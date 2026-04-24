//! Shared page chrome. Every page is wrapped in this function so the head, nav,
//! and footer stay consistent.

use maud::{DOCTYPE, Markup, html};

pub(crate) fn page(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="color-scheme" content="dark light";
                meta name="robots" content="index, follow";
                title { (title) " — PlausiDen" }
                link rel="icon" type="image/svg+xml" href="/static/favicon.svg";
                link rel="icon" type="image/png" sizes="96x96" href="/static/favicon-96x96.png";
                link rel="apple-touch-icon" sizes="180x180" href="/static/apple-touch-icon.png";
                link rel="manifest" href="/static/site.webmanifest";
                link rel="stylesheet" href="/static/style.css";
            }
            body {
                header class="site-header" {
                    a href="/" class="brand" { "PlausiDen" }
                    nav {
                        a href="/services" { "Services" }
                        a href="/contact" { "Encrypted Inquiry" }
                    }
                }
                main { (body) }
                footer class="site-footer" {
                    p {
                        "© PlausiDen. No cookies, no tracking, no logs. "
                        a href="/contact" { "Encrypted Inquiry" }
                        "."
                    }
                }
            }
        }
    }
}
