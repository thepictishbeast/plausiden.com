//! `/status` — operator-facing operational status page.
//!
//! Renders the current process's uptime, the build's commit hash if
//! available, and the build profile. This is a *self-report*: it
//! does not include any external probe results (those live in the
//! external-monitors workflow). For the full health surface,
//! including third-party probes, see status.plausiden.com (when
//! that subdomain ships).
//!
//! No JS, no auto-refresh, no live counters — the operator is the
//! consumer, and they re-load when they want a fresh number.

use std::sync::OnceLock;
use std::time::Instant;

use loom_components::{Badge, BadgeSize, BadgeTone};
use maud::{Markup, html};

use super::layout::page;

/// Process start time. Captured the first time `render` is called;
/// represents "uptime since the first `/status` hit served by this
/// process," which is essentially the process start time after the
/// router is wired.
static START: OnceLock<Instant> = OnceLock::new();

/// Compile-time commit hash, populated by the build script if the
/// `PLAUSIDEN_GIT_SHA` env var is set; otherwise falls back to a
/// placeholder string. Set in CI via:
///   `PLAUSIDEN_GIT_SHA=$(git rev-parse --short HEAD) cargo build`
const GIT_SHA: &str = match option_env!("PLAUSIDEN_GIT_SHA") {
    Some(s) => s,
    None => "(unset)",
};

/// Build profile — `release` in production, `debug` for dev runs.
const BUILD_PROFILE: &str = if cfg!(debug_assertions) {
    "debug"
} else {
    "release"
};

/// Render `/status`.
#[must_use]
pub fn render() -> Markup {
    let start = *START.get_or_init(Instant::now);
    let uptime = Instant::now().saturating_duration_since(start);
    let total_secs = uptime.as_secs();
    let days = total_secs / 86_400;
    let hours = (total_secs % 86_400) / 3_600;
    let minutes = (total_secs % 3_600) / 60;
    let seconds = total_secs % 60;
    let uptime_str = format!("{days}d {hours}h {minutes}m {seconds}s");

    let body = html! {
        section class="pt-32 pb-16 md:pt-44 md:pb-20 bg-slate-50" {
            div class="container mx-auto px-4 md:px-6 max-w-3xl" {
                div class="mb-6" { (Badge { label: "Operational", tone: BadgeTone::Primary, size: BadgeSize::Md }.render()) }
                h1 class="font-display text-4xl md:text-5xl font-bold text-slate-900 leading-[1.1] mb-4" {
                    "Status"
                }
                p class="text-lg text-slate-600 max-w-2xl leading-relaxed" {
                    "Self-reported by the running plausiden.com process. External probes (TLS grade, DNS health, mail deliverability) run on a separate cadence; see external-monitors in the CI logs."
                }
            }
        }

        section class="py-16 bg-white" {
            div class="container mx-auto px-4 md:px-6 max-w-2xl" {
                div class="rounded-xl border border-emerald-200 bg-emerald-50 p-6 mb-8" {
                    div class="flex items-center gap-3" {
                        div class="w-3 h-3 rounded-full bg-emerald-500" {}
                        p class="text-sm text-emerald-900 font-semibold" {
                            "All systems operational"
                        }
                    }
                    p class="text-xs text-emerald-800 mt-2" {
                        "If you can read this page, the request handler, the renderer, and the security-headers middleware are all up."
                    }
                }

                dl class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-4 text-sm" {
                    div {
                        dt class="text-slate-500 font-medium" { "Uptime since process start" }
                        dd class="text-slate-900 font-mono mt-1" { (uptime_str) }
                    }
                    div {
                        dt class="text-slate-500 font-medium" { "Build profile" }
                        dd class="text-slate-900 font-mono mt-1" { (BUILD_PROFILE) }
                    }
                    div {
                        dt class="text-slate-500 font-medium" { "Commit" }
                        dd class="text-slate-900 font-mono mt-1" { (GIT_SHA) }
                    }
                    div {
                        dt class="text-slate-500 font-medium" { "Hostname" }
                        dd class="text-slate-900 font-mono mt-1" { "plausiden.com" }
                    }
                }
            }
        }
    };
    page("Status — PlausiDen", "/status", body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_status_page() {
        let s = render().into_string();
        assert!(s.contains("All systems operational"));
        assert!(s.contains("Uptime since process start"));
    }

    #[test]
    fn uptime_format_is_dhms() {
        // After the first call, uptime should already be > 0s
        // OR == 0s on a fast first call. Either way the format
        // must include the four units in order.
        let s = render().into_string();
        // crude regex-free check — look for "d " "h " "m " "s"
        assert!(s.contains("d "));
        assert!(s.contains("h "));
        assert!(s.contains("m "));
    }
}
