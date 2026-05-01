//! Compile-time HTML via Maud. No runtime template engine; all output is
//! XSS-safe by construction — the Maud macro escapes every string interpolated
//! via `{}` and requires `PreEscaped` for anything raw.
//!
//! SECURITY: Using Maud (compile-time HTML) eliminates template-injection as
//! a bug class. There is no runtime template compiler, no unsafe eval, no
//! second-order escaping story. `PreEscaped` occurrences are audited sites
//! (grep the crate for `PreEscaped(`).

pub mod about;
pub mod admin;
pub mod blog;
pub mod capabilities;
pub mod case_studies;
pub mod cms_pages;
pub mod contact;
pub mod email;
pub mod feedback;
pub mod home;
pub mod how_we_work;
pub mod layout;
pub mod legal;
pub mod not_found;
pub mod posts;
pub mod pricing;
pub mod services;
pub mod solutions;
pub mod status;
pub mod subscribe;
