//! 404 view.

use maud::{Markup, html};

use super::layout::page;

/// Render the not-found page.
///
/// BUG ASSUMPTION: Called by the axum fallback handler and by a direct test
/// in `main.rs`. Must return a non-empty body, because an empty body on 404
/// confuses users more than a short message.
#[must_use]
pub fn render() -> Markup {
    let body = html! {
        section class="notfound" {
            div class="inner" {
                h1 { "Nothing here." }
                p { "No redirect. No tracking pixel. Just nothing." }
                p { a href="/" class="btn btn-primary" { "Return home" } }
            }
        }
    };
    page("Not Found", body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_found_body_mentions_nothing_here() {
        let s = render().into_string();
        assert!(s.contains("Nothing here"));
    }

    #[test]
    fn not_found_provides_escape_link_home() {
        let s = render().into_string();
        assert!(s.contains("href=\"/\""));
    }
}
