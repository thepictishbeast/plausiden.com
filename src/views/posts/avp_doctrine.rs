//! Field note: how the AVP Doctrine shapes everything we ship.
//!
//! Sanitized public version of the operating doctrine that governs
//! `PlausiDen` repos. Talks about the principles + the discipline they
//! enforce; does not reproduce the full internal doctrine document.

use maud::{Markup, html};

/// Render the post body. Wrapper supplies chrome, eyebrow, title, date.
#[must_use]
pub fn render() -> Markup {
    html! {

        p class="text-lg text-slate-600 leading-relaxed mb-8" {
            "Most consultancies have informal taste. They have engineers whose code reads cleaner than the team average, reviewers who catch the same five mistakes everyone else makes, and a tribal sense of what shipping looks like when it's done well. That tribal sense is fragile. It walks out the door with the senior engineer, it bends under deadline pressure, and it can't be transmitted to a new collaborator quickly. We took a different approach: we wrote our taste down."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "What a doctrine is"
        }

        p class="mb-6" {
            "Internally we call it the AVP Doctrine — \"audit, verify, prove.\" It's a single document that governs how every PlausiDen repository is structured, how every public function is annotated, what gets reviewed at what stage, and what \"done\" means before we ship to a client environment. It is not aspirational; it is enforced. A pull request that violates the doctrine fails CI before a human reads it."
        }

        p class="mb-6" {
            "The doctrine is not a style guide. Style guides are conventions. The doctrine is a contract: \"if a piece of code lives in this repo, then these properties hold of it.\" Some examples of the contracts we enforce:"
        }

        ul class="list-disc list-inside space-y-2 mb-6 text-slate-700" {
            li { "Every public function is annotated with what we call a " strong { "BUG ASSUMPTION" } " — a one-paragraph explanation of what would have to be true for the function to break." }
            li { "Every defense-in-depth measure carries a " strong { "SECURITY" } " annotation explaining what threat it mitigates and why we believe the mitigation holds." }
            li { "The Rust crates we ship forbid " code class="text-sm bg-slate-100 px-1.5 py-0.5 rounded" { "unsafe_code" } " at the crate level and require " code class="text-sm bg-slate-100 px-1.5 py-0.5 rounded" { "missing_docs = deny" } " — a compile error if any public item lacks documentation." }
            li { "Ship decisions — \"why this and not the alternative?\" — are recorded in code comments at the call site, not in a wiki nobody reads." }
            li { "Annotations exist as markers a code-search can find. There are no slogans; if the convention can't be enforced by a CI grep, it can't be relied on." }
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "Why we do this"
        }

        p class="mb-6" {
            "Consulting work compounds. We start a project, ship the first version, hand it off to the client's team, and revisit it eighteen months later when something needs to change. The cost of that revisit is determined almost entirely by what we were disciplined about at the start. Specifically: by whether the next person reading the code — possibly future-us, possibly the client's new engineer — can answer two questions cheaply."
        }

        p class="mb-4" {
            "The questions are:"
        }

        ul class="list-disc list-inside space-y-2 mb-6 text-slate-700" {
            li { strong { "Why is this here?" } " (Why this design, not the obvious alternative? What problem does this solve that we wouldn't notice if we deleted it?)" }
            li { strong { "What would have to be true to break it?" } " (Under what assumptions does this code work? Are those assumptions still true today?)" }
        }

        p class="mb-6" {
            "The AVP Doctrine forces both questions to have answers in the code itself. Not in a separate doc. Not in tribal memory. Right there, in the function body, where the next reader will look."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "What this changes for clients"
        }

        p class="mb-6" {
            "Three things, in our experience, are different about working on a system we built versus a system built without this discipline."
        }

        p class="mb-6" {
            strong { "Audits get faster. " }
            "When a malpractice carrier, a state regulator, or your own counsel shows up with a security questionnaire, the answers are not buried in three different engineers' inboxes — they are inline annotations in the relevant code. We can produce evidence for each control by running a search, not by reconstructing memory."
        }

        p class="mb-6" {
            strong { "Onboarding gets faster. " }
            "When you eventually want your own staff to take over the system — and you should — they read the code with the doctrine annotations and have a working understanding in days, not months. The system explains itself."
        }

        p class="mb-6" {
            strong { "Incident response gets shorter. " }
            "When something goes wrong at 3am, the right question is not \"why did this happen?\" but \"what assumptions broke?\" The BUG ASSUMPTION annotations are the literal answer to that question. The relevant code can usually be found in one search."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "What's the catch"
        }

        p class="mb-6" {
            "Writing this much down is slow. A function that another shop would ship in twenty lines takes us thirty after the annotations. A pull request that another shop would land in an hour takes us a half-day after the BUG ASSUMPTION review and the SECURITY justification. We are deliberately slower at the first version."
        }

        p class="mb-6" {
            "The bet — and we think it's the right one — is that the second through tenth versions are dramatically faster, because nobody is rebuilding context that was lost. And the version where something breaks is dramatically faster, because the assumption that broke is right next to the code that depends on it."
        }

        p class="mb-6" {
            "It's the same bet a senior engineer makes every time they take twenty minutes to write a tighter commit message: a small upfront cost in service of a much larger downstream payoff. We just made it the rule rather than a habit some engineers have."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "Where this connects to everything else we say"
        }

        p class="mb-6" {
            "The phrase we use elsewhere is \"compose, don't compromise.\" The AVP Doctrine is what makes that phrase load-bearing. When we tell a law firm that their privacy guarantees are properties of the architecture rather than promises, we're saying: those guarantees are written down as BUG ASSUMPTION annotations on the relevant functions, and a reviewer can verify them in a code search. When we tell a healthcare practice that their ePHI handling is BAA-ready, we're saying: the controls are annotated, the audit trail is reproducible, and a regulator can be handed a tour of the system that doesn't require an interpreter."
        }

        p class="mb-6" {
            "It's not magic. It's just a level of discipline that turns out to compound across projects. We've watched it pay off in our own work; we wrote it down so it could pay off across more projects than any one engineer can hold in their head."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "What we're doing with it next"
        }

        p class="mb-6" {
            "The doctrine is currently an internal document. We are slowly preparing a sanitized public version — partly because we think other small consultancies could benefit from the pattern, partly because publishing it makes our own commitments more durable. Like everything else we build, we'd rather get sustainable revenue first and ship the open version once, properly, than ship something half-baked into the world for free."
        }

        p class="mb-6" {
            "If your team is dealing with the second-version problem — where every change to a system you shipped a year ago feels disproportionately expensive — the conversation usually starts there. The fix is rarely \"refactor everything.\" Most of the time it's \"start writing down the assumptions, function by function, as you touch them.\" We can help that be more than a New Year's resolution."
        }
    }
}
