//! Services page — proper Maud composition with Loom primitives.
//!
//! Replaces the prior `include_str!` bake of the production React DOM
//! with deeper per-service content: each capability area gets a
//! description, a "what this looks like in practice" sub-paragraph,
//! 5–7 capability bullets, a "who we typically work with" line, and
//! its own contact CTA. Every service section links to /contact so a
//! prospect can act on the specific area without scrolling through
//! the rest.

use loom_components::hero::{Hero, HeroBackground};
use loom_components::{
    Button, ButtonSize, ButtonType, ButtonVariant, Decoration, Heading, HeadingLevel, HeadingTone,
    HeadingVariant, Lede, Section, SectionPadding, SectionTheme, SectionWidth, TextLink,
    TextLinkSize, TextLinkVariant,
};
use loom_icons as icons;
use maud::{Markup, PreEscaped, html};

use super::layout::page;

/// One service category as it appears on the services page.
struct Service<'a> {
    /// `loom_icons` constant rendered with the section-icon class.
    icon: &'a icons::Icon,
    /// Section title.
    title: &'a str,
    /// One-paragraph lede.
    lede: &'a str,
    /// "What this looks like in practice" sub-paragraph.
    practice: &'a str,
    /// Capability bullets — what we actually do under this heading.
    capabilities: &'a [&'a str],
    /// Who we typically work with — sets the audience expectation.
    audience: &'a str,
    /// Sample engagement callout — concrete example shape (not a
    /// case study, just a representative scenario).
    sample: &'a str,
}

const SERVICES: &[Service] = &[
    Service {
        icon: &icons::SERVER,
        title: "IT Operations",
        lede: "Complete infrastructure management designed to keep your business running smoothly. We handle monitoring, maintenance, patching, and support so your team can focus on the work that earns revenue.",
        practice: "We operate IT the way ops engineers do at companies that take uptime seriously: written runbooks, change management with rollback paths, monitoring you can interpret without a dashboard PhD, and tickets that close with documentation, not just \"fixed.\"",
        capabilities: &[
            "24/7 monitoring with alert routing tuned for your team's actual on-call hours",
            "Patching cadence + change windows documented in advance, never a surprise",
            "Backup + tested restore procedures (the test is the part most vendors skip)",
            "Disaster-recovery runbooks specific to your stack, rehearsed annually",
            "Cloud cost discipline — we'll flag the $400/mo orphaned NAT gateway",
            "Help desk that documents the answer once, not the same answer ten times",
        ],
        audience: "Practices and small businesses with 5–100 staff whose IT is currently \"the person who's good with computers\" plus a Microsoft 365 reseller.",
        sample: "A 15-person law firm handed off the IT function from a departing partner. We took over Microsoft 365 admin, rebuilt the patch cadence, wrote three runbooks (network down, M365 outage, ransomware), and migrated their backups to a tested off-site target. 90-day handoff to a half-time IT coordinator.",
    },
    Service {
        icon: &icons::CIRCLE_CHECK,
        title: "Disaster Recovery",
        lede: "Recovery posture engineered to be tested, not just documented. We design backup + restore + failover so the procedure on paper matches the procedure under pressure — and we rehearse it on a cadence that catches drift before an incident does.",
        practice: "DR plans that don't get rehearsed are theater. We design the recovery procedure first, the backup schedule second, and we run live restore rehearsals on a cadence that matches the regulatory and contractual posture of the business. The test is the deliverable; the documentation is the artifact.",
        capabilities: &[
            "Recovery-time + recovery-point objectives written down per workload, not assumed",
            "Backup strategy designed for the actual restore path — backups you can't restore from are a liability, not an asset",
            "Cross-region / off-site replication tuned to threat model, not a vendor's default tier",
            "Live restore rehearsals on a cadence the practice can sustain — typically twice yearly",
            "Failover runbooks rehearsed under the conditions on-call would actually face",
            "Compliance-grade evidence packets for carriers, regulators, and downstream client reviews",
            "Tabletop incident exercises with the people who'd actually be in the room",
        ],
        audience: "Practices and small businesses that have outgrown \"we have a backup somewhere\" and need a recovery posture that survives a malpractice carrier review, a state-bar audit, or a HIPAA OCR letter.",
        sample: "A specialty practice's annual DR rehearsal exposed that their last-known-good backup was 11 days stale and the documented restore procedure referenced an admin who'd left two years prior. We rebuilt the backup target, rewrote the restore runbook against the current stack, and ran a tested restore that completed inside the documented RTO.",
    },
    Service {
        icon: &icons::SHIELD,
        title: "Cyber Security",
        lede: "Defense-in-depth strategies sized to your actual threat model. From compliance audits to real-time threat detection, we secure your digital perimeter without the enterprise theater you can't operate.",
        practice: "Most small organizations don't need a SOC; they need correctly-configured defaults, a written incident-response plan, and someone whose phone rings when the canary trips. We design the posture that matches that reality — and produces evidence regulators, carriers, and clients are starting to ask for.",
        capabilities: &[
            "Threat modeling — what would actually hurt us, vs. checklist theater",
            "Endpoint defense (EDR + DNS + DLP) sized to staff count, not seat-license fantasies",
            "Email authentication: SPF, DKIM, DMARC, MTA-STS — all four, properly tuned",
            "Penetration testing on the surfaces that matter (web app, internal AD, BYOD)",
            "Compliance audits + evidence packets for SOC 2, HIPAA, state-bar, NIST CSF",
            "Phishing simulation tuned to the lures targeting your industry, not generic",
            "Incident-response retainer — we'll be the people you call at 3am",
        ],
        audience: "Organizations whose clients, regulators, or insurers are starting to ask pointed security questions. Law firms, healthcare practices, RIAs, journalism orgs, advocacy nonprofits.",
        sample: "A regional medical practice failed a malpractice carrier renewal questionnaire on three controls. We documented the existing controls, remediated the three gaps (MFA on M365 admin, encrypted backups, written WISP), and produced an evidence packet the carrier accepted. Next renewal sailed through.",
    },
    Service {
        icon: &icons::BRAIN_CIRCUIT,
        title: "Artificial Intelligence",
        lede: "Practical AI integration that solves a real bottleneck — not AI for AI's sake. We help you identify where machine learning earns its keep, build it cleanly, and avoid the failure modes that have made \"AI rollout\" a synonym for \"vendor lock-in\" for many organizations.",
        practice: "We start with the question \"what would a 30% improvement on this specific workflow be worth?\" and work backward. If the answer is \"a tool already exists that's good enough,\" we say so. If the answer is \"build a custom model,\" we scope it like any other software engagement: written requirements, milestones, acceptance criteria.",
        capabilities: &[
            "Custom ML models for narrow, well-bounded problems (classification, extraction, ranking)",
            "Document intake automation — OCR + structured extraction + human-in-the-loop review",
            "Natural language search over your own corpus (without sending it to a third party)",
            "Predictive analytics with auditable feature engineering, not opaque black boxes",
            "AI strategy review — which of the 47 vendor pitches you've heard are actually worth a pilot",
            "Privacy-preserving deployment patterns (on-prem inference, federated approaches, content-free pipelines)",
        ],
        audience: "Organizations with a specific high-volume manual workflow they suspect could be partially automated, or a strategy team that needs a reality check on a pending vendor decision.",
        sample: "A small-claims litigation practice was spending 6 hours per case extracting deadlines from court filings. We built a document-intake pipeline that flags every deadline mentioned in incoming PDFs, attorney reviews and confirms in 5 minutes per case. Throughput up 4x.",
    },
    Service {
        icon: &icons::SETTINGS,
        title: "Industrial Automation",
        lede: "Operational efficiency through automation systems designed for reliability and auditability. We bridge the gap between OT and IT — without exposing your control plane to the corporate network's threat model.",
        practice: "Industrial environments have safety, regulatory, and uptime requirements that consumer-IT vendors don't take seriously. We design with those constraints first: air-gapped where it matters, segmented where it doesn't, and documented at a level that survives the operator turnover that eventually happens.",
        capabilities: &[
            "Robotic process automation (RPA) for back-office workflows that crossed the line from \"manual\" to \"a person's full-time job\"",
            "PLC programming + SCADA integration with proper version control and rollback",
            "OT/IT segmentation — production network properly isolated from the corporate domain",
            "Predictive-maintenance instrumentation: sensors, time-series data, alert routing",
            "IoT integration with vendor-locked-down devices, where the only sane path is custom",
            "Workflow optimization audits — find the part of the process that's costing 4x what it should",
        ],
        audience: "Manufacturers, logistics operators, lab facilities, and field-service organizations whose ops floor still runs on a 12-year-old WindowsCE box that nobody wants to touch.",
        sample: "A 40-person specialty manufacturer was losing ~3 hours/day to a manual inventory reconciliation between the floor and ERP. We built a barcode-scan pipeline with offline tolerance + nightly reconciliation. Reconciliation now takes 15 minutes; gap-closing time fell from days to hours.",
    },
    Service {
        icon: &icons::CODE,
        title: "Software Development",
        lede: "Custom software for anything you can describe. If a workflow is yours, a tool is yours, a system is yours, we build it the way it should be built — typed, tested, documented, deployable, and structured so the next engineer (yours or ours) can pick it up without us.",
        practice: "We treat every engagement as if you'll one day hire your own team to take it over. The deliverable is documented code, not a black box. Tests are real (not vanity); the deployment story is one command, not a 9-page wiki page. \"Custom\" means custom — we don't have a 200-template SaaS framework you have to fit into. We have an opinionated stack and we apply it to whatever the problem actually is.",
        capabilities: &[
            "Anything custom — name a workflow, name a tool, name a system. Civic / governance tooling (e.g., Sacred.Vote), case management, intake automation, internal admin, public-facing apps",
            "Web applications, primarily in Rust + TypeScript stacks for type-safe correctness",
            "Mobile apps for the workflows that need them — never as a checkbox",
            "API design + integration with the systems you already pay for (M365, Salesforce, Stripe, Plaid, etc.)",
            "Legacy modernization with a documented migration path, not a forklift rewrite",
            "Database design that survives 10x growth without an emergency consulting engagement",
            "Self-hostable architectures — your software runs on your infrastructure if you want it to",
            "Open-source-by-default where it serves you, with a clear license posture",
        ],
        audience: "Organizations with a workflow that's outgrown the off-the-shelf tools, a regulated environment where an off-the-shelf SaaS would create unacceptable data-handling exposure, or a one-of-a-kind requirement (Sacred.Vote-class civic infrastructure, niche compliance pipelines, novel research instrumentation) that no vendor sells.",
        sample: "A nonprofit running case-management on a Google Sheets + email workflow needed real software but couldn't afford SaaS pricing tiers. We built a self-hosted case-management app on top of their existing infrastructure, documented for in-house handoff. Annual cost: $0 SaaS, ~3hr/mo maintenance.",
    },
    Service {
        icon: &icons::CPU,
        title: "Hardware Solutions",
        lede: "Strategic procurement and lifecycle management of enterprise hardware. We make sure your team has the right tools — and that the procurement process doesn't become an operational liability.",
        practice: "Hardware is where small-organization IT often hemorrhages money: refresh cycles nobody owns, BYOD chaos that breaks compliance, and surprise vendor markups on \"managed\" purchases. We standardize the fleet, write the lifecycle policy, and source competitively.",
        capabilities: &[
            "Procurement at pass-through pricing — no license-arbitrage markup",
            "Standardized device images (laptops, workstations, mobile) with proper MDM",
            "Asset management: who has what, when it was bought, when it depreciates out",
            "Lifecycle replacement planning — predictable budget, no surprises",
            "Secure decommissioning + data destruction with documented chain of custody",
            "Hardware repair coordination so a broken laptop is a 2-day inconvenience, not a 2-week one",
        ],
        audience: "Organizations whose laptop fleet is whatever the staff bought on Best Buy + a few legacy desktops nobody remembers buying. Anyone preparing for an audit that asks \"what hardware are you running?\"",
        sample: "A 25-person practice had 19 different laptop configurations, no asset register, and three machines running unsupported OS versions. We standardized on two refresh tiers, wrote a 3-year refresh budget, sourced competitively (saved 18% over their previous vendor), and inventoried everything that walked through the door.",
    },
    Service {
        icon: &icons::GLOBE,
        title: "Network Architecture",
        lede: "Robust network architecture that ensures high availability + speed without enterprise-tier complexity. We design connectivity that scales with you and stays operable by your eventual in-house IT person.",
        practice: "Most small-organization networks accumulate complexity over years until nobody understands them anymore. We start with a documented current-state diagram, then design a target state that's simpler, faster, and properly segmented. No mesh-of-tunnels architectures that nobody can debug at 2am.",
        capabilities: &[
            "Network design + documentation (the diagram + the rationale behind every decision)",
            "VPN solutions: WireGuard for new builds, OpenVPN where legacy compatibility demands it",
            "Wi-Fi architecture: coverage, capacity, segmentation (guest / staff / IoT properly separated)",
            "Connectivity audits — find the dual-uplink that became a single point of failure when one circuit was canceled",
            "Firewall ruleset cleanup with documented rationale for every rule that survives",
            "Tor + onion-service architecture for organizations whose threat model warrants it",
            "Zero-trust patterns sized to small teams, not Fortune 500 implementations",
        ],
        audience: "Organizations whose network is \"whatever the previous IT person set up\" plus a few hardware swaps over the years. Anyone preparing to expand to a second location or a hybrid-remote workforce.",
        sample: "A regional firm with three offices was running a hub-and-spoke VPN that fell over weekly. We replaced it with a mesh WireGuard setup, segmented per-office and per-role, documented the topology + every firewall rule. Outages dropped from weekly to none in the first 90 days.",
    },
];

/// Render `/services`.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn render() -> Markup {
    let body = html! {

        (Hero {
            eyebrow: Some("Our services"),
            headline_lead: "Comprehensive IT,",
            headline_accent: Some("specifically scoped."),
            subheadline: "We offer a full suite of IT solutions designed for organizations that want infrastructure they can audit, retain, and eventually run without us. General scope, specific expertise — pick the area that's biting you, or schedule an intake to figure out where to start.",
            cta: None,
            background: HeroBackground::GridLight,
        }.render())

        @for (i, svc) in SERVICES.iter().enumerate() {
            (service_section(svc, i % 2 == 0))
        }

        (posture_band())
        (final_cta())
    };
    page("Services — PlausiDen", "/services", body)
}

/// Cross-cutting "how we approach every engagement" band — same dark
/// shape used by the solutions/* template, replicated here so this
/// page doesn't depend on that module.
fn posture_band() -> Markup {
    html! {
        section class="py-20 bg-slate-900 text-white relative overflow-hidden" { // loom-allow: dark posture band — slate-900 with decorative blur exceeds Loom Section{Dark} skeleton
            div class="absolute top-0 right-0 w-96 h-96 bg-primary/20 rounded-full blur-3xl -translate-y-1/2 translate-x-1/2" {} // loom-allow: positioned decorative primary-blob blur
            div class="container relative mx-auto px-4 md:px-6 max-w-4xl reveal" { // loom-allow: container exceeds Loom Wide
                span class="inline-block px-3 py-1 rounded-full bg-white/10 text-white text-sm font-medium mb-6 backdrop-blur-sm border border-white/10" { // loom-allow: glass-morphism eyebrow badge — pending Badge::Eyebrow with Compact size
                    "How we approach every engagement"
                }
                (Heading {
                    text: "Different shapes, same posture.",
                    level: HeadingLevel::H2,
                    variant: HeadingVariant::Section,
                    tone: HeadingTone::OnDark,
                }.render())
                div class="mt-6 space-y-6" { // loom-allow: spacer + vertical rhythm between dark-band Lede paragraphs
                    (Lede {
                        text: "Whether the engagement is a $1,500 discovery, a $9,500/mo retainer, or a $60,000 fixed-scope project, the operating posture is identical: written proposals, scope-limited access, audit-ready documentation, and a real handoff path.",
                        tone: HeadingTone::OnDark,
                    }.render())
                    (Lede {
                        text: "We aim for engagements that produce documentation a competent successor can use to take over. The next vendor — yours or ours — should be able to read what we built, understand why, and run it without us. That's what \"compose, don't compromise\" means at the engagement level.",
                        tone: HeadingTone::OnDark,
                    }.render())
                }
                p class="text-slate-300 mt-8" { // loom-allow: prose with two inline links; standard BodyText doesn't take inline children
                    "Read more in "
                    (TextLink { label: "how we work", href: "/how-we-work", variant: TextLinkVariant::Underlined, size: TextLinkSize::Default }.render())
                    " or check our "
                    (TextLink { label: "pricing posture", href: "/pricing-transparency", variant: TextLinkVariant::Underlined, size: TextLinkSize::Default }.render())
                    "."
                }
            }
        }
    }
}

fn final_cta() -> Markup {
    let cta_button = Button {
        label: "Schedule an intake call",
        variant: ButtonVariant::Primary,
        size: ButtonSize::Lg,
        aria_label: None,
        icon: None,
        decoration: Decoration::SoftShadow,
        button_type: ButtonType::Button,
    }
    .render();
    let mailto_link = TextLink {
        label: "team@plausiden.com",
        href: "mailto:team@plausiden.com",
        variant: TextLinkVariant::PrimaryMedium,
        size: TextLinkSize::Default,
    }
    .render();
    let body = html! {
        div class="text-center reveal" {
            div class="mb-6" {
                (Heading {
                    text: "Not sure where to start?",
                    level: HeadingLevel::H2,
                    variant: HeadingVariant::Section,
                    tone: HeadingTone::Ink,
                }.render())
            }
            div class="mb-8" {
                (Lede {
                    text: "The intake conversation is free, the NDA is mutual, and we'll tell you if we're not the right fit. Tell us what's on your plate — even if you're not sure whether it's an IT problem yet.",
                    tone: HeadingTone::Ink,
                }.render())
            }
            a href="/contact" { (cta_button) }
            p class="text-slate-500 text-sm mt-6" { // loom-allow: prose with inline link
                "Or write to "
                (mailto_link)
                " · 978-351-6495"
            }
        }
    };
    Section {
        body: &body,
        theme: SectionTheme::Tinted,
        width: SectionWidth::Article,
        padding: SectionPadding::Loose,
    }
    .render()
}

/// Render one service as a condensed card with `<details>` for the
/// depth. The summary panel — icon + title + lede — is always
/// visible; clicking expands into practice + full capabilities +
/// audience + sample + the per-service CTA.
///
/// Native `<details>` was chosen over a JS toggle: it works without
/// JS, is keyboard accessible by default, and survives reduced-
/// motion / no-JS / RSS-reader / accessibility-tree consumption.
/// `light_band` toggles the background so adjacent cards alternate.
fn service_section(svc: &Service, light_band: bool) -> Markup {
    let bg = if light_band {
        "bg-white"
    } else {
        "bg-slate-50"
    };
    let icon_svg = svc.icon.render_with_class("w-7 h-7 text-primary"); // loom-allow: SVG class attribute, not Maud-emitted utility chain
    html! {
        section class=(format!("py-8 md:py-10 {bg}")) { // loom-allow: <details> card band — alternating zebra background controlled by light_band
            div class="container mx-auto px-4 md:px-6 max-w-4xl reveal" { // loom-allow: <details> container with scroll-reveal hook
                details class="rounded-xl border border-slate-200 bg-white shadow-sm hover:shadow-md transition-shadow" { // loom-allow: <details>/<summary> shell — pending Loom CollapsibleCard primitive (only consumer is services.rs)
                    summary class="flex items-start gap-4 p-5 md:p-6 cursor-pointer" { // loom-allow: collapsible header row chrome
                        div class="bg-primary/10 w-12 h-12 rounded-lg flex items-center justify-center shrink-0" { // loom-allow: tinted icon-tile, 48px square
                            (PreEscaped(icon_svg))
                        }
                        div class="flex-1 min-w-0" { // loom-allow: flex grow-fill column inside summary row
                            h2 class="font-display text-xl md:text-2xl font-bold text-slate-900" { // loom-allow: collapsible title — Heading{Sub} omits md:text-2xl scaling
                                (svc.title)
                            }
                            p class="text-slate-600 text-sm md:text-base leading-relaxed mt-2" { (svc.lede) } // loom-allow: collapsible lede — smaller than Loom Lede on phones
                            p class="text-xs text-primary mt-3 font-semibold" { "Read more →" } // loom-allow: collapse-affordance hint
                        }
                    }
                    div class="px-5 md:px-6 pb-6 pt-2 border-t border-slate-100" { // loom-allow: collapsible body chrome — top-bordered drawer
                        p class="text-slate-600 leading-relaxed mb-5" { (svc.practice) } // loom-allow: practice-statement prose

                        p class="font-semibold text-slate-900 mb-2" { "Capabilities" } // loom-allow: in-drawer subheading — smaller than Heading{Card}
                        ul class="list-disc list-inside space-y-1.5 mb-6 text-slate-700" { // loom-allow: bulleted capabilities list — no Loom BulletList primitive
                            @for cap in svc.capabilities {
                                li { (*cap) }
                            }
                        }

                        div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6" { // loom-allow: 2-up audience/sample sub-cards
                            div class="rounded-lg bg-slate-50 border border-slate-200 p-4" { // loom-allow: tinted sub-card chrome
                                p class="font-semibold text-slate-900 mb-1 text-sm" { "Who we typically work with" } // loom-allow: sub-card title
                                p class="text-slate-600 text-sm leading-relaxed" { (svc.audience) } // loom-allow: sub-card body
                            }
                            div class="rounded-lg bg-slate-50 border border-slate-200 p-4" { // loom-allow: tinted sub-card chrome
                                p class="font-semibold text-slate-900 mb-1 text-sm" { "Sample engagement" } // loom-allow: sub-card title
                                p class="text-slate-600 text-sm leading-relaxed" { (svc.sample) } // loom-allow: sub-card body
                            }
                        }

                        a href="/contact" class="inline-flex items-center gap-2 text-primary font-semibold" { // loom-allow: in-drawer CTA link — primary-coloured, no underline
                            "Talk to us about " (svc.title) " →"
                        }
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
    fn renders_nonempty() {
        assert!(render().into_string().len() > 12_000);
    }

    /// Every service section has a contact link — the user-facing
    /// requirement that prompted this rewrite.
    #[test]
    fn every_service_has_a_contact_cta() {
        let s = render().into_string();
        for svc in SERVICES {
            let needle = format!("Talk to us about {} →", svc.title);
            assert!(
                s.contains(&needle),
                "missing per-service contact CTA for {}",
                svc.title
            );
        }
    }

    /// All seven services are present.
    #[test]
    fn all_services_listed() {
        let s = render().into_string();
        for title in &[
            "IT Operations",
            "Cyber Security",
            "Artificial Intelligence",
            "Industrial Automation",
            "Software Development",
            "Hardware Solutions",
            "Network Architecture",
        ] {
            assert!(s.contains(title), "missing service: {title}");
        }
    }

    /// Each service surfaces at least 5 capabilities (deeper than the
    /// 4 the prior baked DOM had).
    #[test]
    fn every_service_has_5plus_capabilities() {
        for svc in SERVICES {
            assert!(
                svc.capabilities.len() >= 5,
                "{} has only {} capabilities; need >= 5",
                svc.title,
                svc.capabilities.len()
            );
        }
    }

    /// REGRESSION-GUARD: page must not silently revert to the legacy
    /// Secure Drop wording.
    #[test]
    fn no_secure_drop_text() {
        let s = render().into_string();
        assert!(!s.contains(">Secure Drop<"));
    }

    /// Cross-link to /how-we-work + /pricing — these are mid-funnel
    /// pages the services page should hand off to.
    #[test]
    fn cross_links_to_how_we_work_and_pricing() {
        let s = render().into_string();
        assert!(s.contains(r#"href="/how-we-work""#));
        assert!(s.contains(r#"href="/pricing-transparency""#));
    }
}
