//! Field note: provable-by-construction privacy.
//!
//! Argues for designing systems where the privacy guarantee is a
//! property of the architecture, not a runtime promise. Sanitized;
//! does not name clients, repos, or specific files.

use maud::{Markup, html};

/// Render the post body. Wrapper supplies chrome, eyebrow, title, date.
#[must_use]
pub fn render() -> Markup {
    html! {

        p class="text-lg text-slate-600 leading-relaxed mb-8" {
            "There are two ways a system can promise to protect your data. The first is a policy — a sentence in a privacy notice or a clause in a contract. The second is a property — a fact about the architecture that would have to stop being true for the promise to be broken. Most software in the world picks the first. We try to pick the second whenever the engineering allows."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "What \"provable by construction\" means"
        }

        p class="mb-6" {
            "Take a concrete example. Imagine a system that auto-categorizes incoming email — sorting newsletters into one folder, transactional alerts into another, social-platform notifications into a third. Most implementations of this read the message body. They have to: that's how you tell a marketing email from an order receipt."
        }

        p class="mb-6" {
            "Then they add a privacy policy: \"we don't store the body content, we just look at it during classification.\" That's a policy. It's enforceable by audit, by trust, by reputation, by lawsuit. It's not enforceable by the structure of the code. A bug, a feature request, a future engineer who didn't read the comments — any of these can quietly break the promise."
        }

        p class="mb-6" {
            "A different design: the classifier is given a typed value that contains "
            em { "only" }
            " a small set of header features (sender domain, presence of certain headers, a few subject tokens). The body is never passed in. The function that receives the typed value cannot read the body, because the body isn't reachable from its parameters. The compiler proves this. The privacy promise is now a property: the body cannot leak through this code path because the code path has no syntactic access to it."
        }

        p class="mb-6" {
            "That's what \"provable by construction\" means in practice. Not \"we promise we won't.\" Instead: \"we wrote the function so that it can't.\""
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "What this changes about audits"
        }

        p class="mb-6" {
            "Most security audits we've watched run the same way: a reviewer with a checklist asks questions about controls, the IT team produces logs and screenshots, the auditor satisfies themselves that the controls were in place at some point. The conversation is fundamentally about evidence of behavior."
        }

        p class="mb-6" {
            "Provable-by-construction shifts the conversation. Instead of \"show me the logs that prove this didn't leak,\" the auditor reads the function signature. The parameters don't include the body. The function returns a value of a type that cannot encode the body. The compiler refuses to build the project if any code path tries to violate the constraint. There are no runtime logs to forensically reconstruct because the violation isn't expressible."
        }

        p class="mb-6" {
            "This makes some classes of audit dramatically faster. Compliance reviewers, malpractice carriers, downstream vendors who want a SOC 2 Type 2 report — when the answer to their question is \"no, by construction,\" you don't need a paper trail. You need a code review, and the answer is durable across releases."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "What this changes about incident response"
        }

        p class="mb-6" {
            "When something goes wrong — when a service gets compromised, when a credential leaks, when a disk image walks out of the data center — the first question is always: what could this have exposed?"
        }

        p class="mb-6" {
            "If your system has policies, that question is hard. You need to reconstruct what the breached component had access to, what it actually did with that access, what was logged, what wasn't, what data flowed through which paths during the relevant window. This takes days or weeks. Sometimes the honest answer is \"we don't know.\""
        }

        p class="mb-6" {
            "If your system has properties, the question is much faster. You read the type signature of the breached component. You see what it could have touched, structurally. You can be definitive about the upper bound of exposure within an hour. Often the answer is \"the worst case is bounded to this small set of fields, because no other field is reachable from this code.\""
        }

        p class="mb-6" {
            "This is why we design pipelines this way for the clients who actually have to live with regulators. \"By construction\" is a much shorter incident response than \"by policy.\""
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "Where it doesn't apply"
        }

        p class="mb-6" {
            "Not every privacy concern can be expressed as a type-system property. Spam filters need to read content because spam evades static signals. Search indexes have to look at the data they index. End-to-end encryption requires the decryption key to live somewhere a user has access to, which means a compromised user is a compromised key."
        }

        p class="mb-6" {
            "We don't claim to convert every promise into a property. We do claim that the share of promises that "
            em { "can" }
            " become properties is much higher than most teams assume — and that the engineering effort to convert them pays back over years of audits, incident responses, and conversations with regulators."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "What this looks like in our work"
        }

        p class="mb-6" {
            "When we design a mail pipeline for a law firm, the categorization code only sees headers, never bodies. When we design a case-management system for a nonprofit, the donor and beneficiary records live in separate access scopes that the application code can't cross-query — the type system enforces it. When we ship a federated rule-learning loop, the rule patterns are constrained at the AST level so they can't reference message content at all."
        }

        p class="mb-6" {
            "These are not policies that we wrote down and hope to enforce. They're code-level facts that can be verified by reading the function signatures. The reviewer gets to leave with confidence that does not depend on our reputation."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "The cost"
        }

        p class="mb-6" {
            "It's slower upfront. A function that takes the entire message and decides what to do with it is shorter than a function that takes a typed feature struct that another function had to extract. Splitting the data flow across boundaries adds wiring; making those boundaries enforceable adds types."
        }

        p class="mb-6" {
            "We accept that cost because our clients accept it. Law firms accept it because the alternative is a malpractice premium. Healthcare practices accept it because the alternative is an OCR letter. Newsrooms accept it because the alternative is a subpoena that would have been unanswerable in any other architecture. The cost is paid once, at design time. The audits, the responses, the regulator conversations — those payments would otherwise recur for the lifetime of the system."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "Where this connects to everything else"
        }

        p class="mb-6" {
            "If you've worked with us, you've heard the phrase \"compose, don't compromise.\" The phrase is shorthand for: when two requirements look mutually exclusive, design until you find an architecture where both hold by construction. Privacy and continuous improvement, audit-friendliness and developer velocity, sovereignty and modern UX — these are the trade-offs people accept because they don't see the third option. We spend a lot of our engineering time looking for the third option."
        }

        p class="mb-6" {
            "Not always successfully. Sometimes the trade-off is real and we have to pick a side. But often enough — more often than the industry default — there's a design where both promises can be properties. When we find one, that's the design we ship."
        }

        p class="mb-6" {
            "If your team is currently maintaining promises that ought to be properties, the conversation usually starts there. Look at the function signatures. Look at the data flow. Where you find a parameter that "
            em { "could" }
            " carry sensitive data through a code path that doesn't need it, you have a candidate for promotion to a structural guarantee. The first one is the hardest. After three or four, the muscle is built and the rest of the architecture follows."
        }
    }
}
