//! Field note: federated rule learning, without ever reading your mail.
//!
//! Promotes the capability without exposing repo internals. Talks about
//! what it does and why we built it; does not link to source, does not
//! describe specific data structures, does not name file paths.

use maud::{Markup, html};

/// Render the post body. The `view::blog::post` wrapper provides the
/// page chrome, eyebrow, title, and date — this fn returns the prose only.
#[must_use]
pub fn render() -> Markup {
    html! {

        p class="text-lg text-slate-600 leading-relaxed mb-8" {
            "Most email clients sort your inbox using static rules — \"if From contains @newsletter.com, file to Promotions.\" Those rules don't get smarter. The big providers solve that with machine learning trained on your messages: their algorithm reads your mail, learns from millions of users' behavior, and gets better. The trade you make is reading rights. We don't think that trade is necessary. Here's a different way."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "The premise: privacy and intelligence aren't opposed"
        }

        p class="mb-6" {
            "When a junk newsletter shows up in everyone's inbox and everyone moves it to Promotions, that pattern is a useful signal — and it's a pattern about the "
            em { "sender" }
            ", not about any individual recipient. The sender is shouting their identity in headers like "
            code class="text-sm bg-slate-100 px-1.5 py-0.5 rounded" { "List-Unsubscribe" }
            " and a domain everyone can see. Nothing that looks at "
            em { "those" }
            " signals is reading your mail."
        }

        p class="mb-6" {
            "The trick, then, is to learn from collective behavior over public-by-construction signals — and to make it structurally impossible for the system to ever look at private signals. Not \"trust us we won't,\" but \"the type system literally does not have a way to.\""
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "What \"public-by-construction\" means in practice"
        }

        p class="mb-6" {
            "When you flag a message as Promotions, the only things our client ever derives from that action are a small set of "
            em { "header features" }
            ":"
        }

        ul class="list-disc list-inside space-y-2 mb-6 text-slate-700" {
            li { "What domain it came from (e.g., " code class="text-sm bg-slate-100 px-1.5 py-0.5 rounded" { "@mailchimp.com" } ") — already on the envelope, visible to every relay it touched." }
            li { "Whether the message advertised a List-Unsubscribe header — a public IETF-standard signal the sender chose to publish." }
            li { "Whether it carried a high-priority hint." }
            li { "A handful of subject keywords the sender themselves wrote into the header." }
        }

        p class="mb-6" {
            "That's the entire feature surface. Bodies, addresses, names, the recipient list — none of those exist in the structure that gets shared. Not as a policy. As a property of the design: there is no way to put them in. Anyone who reviews the code (auditors, regulators, you) can verify that in a few minutes."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "How the collective gets smarter"
        }

        p class="mb-6" {
            "Each install observes its user's flags privately and derives candidate sorting rules — \"this user moves @mailchimp.com to Promotions every time.\" Those rules apply locally immediately. If the user opts in, a copy is signed with a per-device key and submitted to a public corroboration ledger."
        }

        p class="mb-6" {
            "The ledger only counts. It does not store who you are, when exactly you submitted, what your IP was, or what's in any message. It tracks, per rule pattern, how many independent installs have proposed it. When enough installs converge on the same pattern — and we mean really converge, not just one user with multiple devices — the rule surfaces in everyone else's client as a suggestion. One click to accept; one click to reject. Local rules always beat platform suggestions; suggestions can never promote a sender into your important inbox."
        }

        p class="mb-6" {
            "The result: Gmail-quality sorting that gets smarter from the network without anyone — including us — reading anyone's mail."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "What this is not"
        }

        p class="mb-6" {
            "It's not magic and it's not machine learning. There's no model. The patterns are simple typed rules — exactly the kind of \"if header X matches Y, file to Z\" that mail filters have used since the 1990s. The novelty is the signing-and-corroboration layer, the careful refusal to ship anything else, and the public verifiability of both."
        }

        p class="mb-6" {
            "It also isn't a silver bullet for every classification problem. Spam filtering, for instance, "
            em { "has" }
            " to look at content because spam evades static rules; that's a different problem and we don't claim to solve it this way. What we do claim is that "
            em { "ordinary" }
            " sorting — the difference between a Mailchimp blast and a real customer email — does not need anyone reading your mail."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "Why we think this matters"
        }

        p class="mb-6" {
            "There's a category of company that builds infrastructure on the premise that privacy is a niche concern — that ordinary users will trade reading rights for better features. We don't think that's true; we think those features can be delivered another way, and the only reason they aren't is that the alternative is harder to design and harder to monetize."
        }

        p class="mb-6" {
            "PlausiDen builds infrastructure for organizations that already know they're in the other category — law firms, healthcare practices, journalists, financial advisors, advocacy groups — where \"don't read our data\" is the requirement, not the upsell. The federated rule loop is one of several pieces in a larger stack we use to deliver the experience their users expect without compromising the structural guarantees they need."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "The shape of \"compose, don't compromise\""
        }

        p class="mb-6" {
            "If you've worked with us, you've heard us use the phrase \"compose, don't compromise.\" Federated learning over public signals is the canonical example of what that phrase means. Two things that "
            em { "look" }
            " mutually exclusive — privacy-as-a-property vs. continuous improvement — turn out to be compatible if you accept a constraint and design around it. The constraint is \"never look at body content for routing.\" The design that survives that constraint produces a system that's actually "
            em { "more" }
            " auditable than the alternative, because the privacy guarantee lives in the type system rather than in a policy."
        }

        p class="mb-6" {
            "Most of what we build for clients shares this structure. We don't promise privacy as a courtesy and ask you to trust us; we design pipelines where the promise is provable from the code. That makes audits faster, compliance reviews shorter, and incident response cheaper, because the answer to \"could this system have leaked X?\" is sometimes \"no — by construction\" instead of \"let me check the logs.\""
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "Where this lives"
        }

        p class="mb-6" {
            "Today the federated rule loop runs inside our internal tooling stack. It's the kind of work we do when we're solving a problem for a client that wants smarter classification but cannot allow content access — and we've found that articulating it as a general primitive, rather than a one-off, makes it useful across multiple engagements. Over time, pieces of this stack will become open source; we're being patient about timing because building them properly is a real investment, and we'd rather get sustainable revenue first than ship half a system into the world for free."
        }

        p class="mb-6" {
            "If you're working on something where the privacy / capability trade-off feels false — where you've been told you have to choose between sovereignty and good UX — we can probably help. The conversation usually starts with a specific bottleneck and ends with a different design choice you didn't know was on the table."
        }
    }
}
