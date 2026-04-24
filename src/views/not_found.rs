//! 404 view.

use maud::{Markup, html};

use super::layout::page;

pub(crate) fn render() -> Markup {
    let body = html! {
        section class="notfound" {
            h1 { "Nothing here." }
            p { "No redirect. No tracking pixel. Just nothing." }
            p { a href="/" { "Return home" } }
        }
    };
    page("Not Found", body)
}
