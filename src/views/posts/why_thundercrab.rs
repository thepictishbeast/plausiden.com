//! Field note: why we're building Thundercrab.
//!
//! Argues that consumer email clients have replaced an open, auditable
//! standard (Sieve) with opaque ML categorization that users can't see,
//! can't edit, and can't carry between providers. Thundercrab is the
//! local-first GUI for typed, transparent mail rules.

use maud::{Markup, html};

/// Render the post body. Wrapper supplies chrome, eyebrow, title, date.
#[must_use]
pub fn render() -> Markup {
    html! {

        p class="text-lg text-slate-600 leading-relaxed mb-8" {
            "If you use Gmail, your inbox has a built-in opinion about what matters. Some messages get a Promotions tab; some get Updates; the important ones, sometimes, get the Primary tab. The Apple Mail VIP system, Outlook's Focused Inbox, the Superhuman split — every modern consumer mail product makes the same bet: the user shouldn't have to write filtering rules, the software should do it for them. We think this trade was a bad deal, and we're building the alternative."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "What got lost"
        }

        p class="mb-6" {
            "Email has had a standard for filtering since 2008. It's called Sieve, and it's exactly what you'd expect: a small declarative language for matching headers and routing messages into folders. Server-side. Standards-based. Documented in an RFC. Every serious mail server (Dovecot, Cyrus, Stalwart) implements it. Sieve scripts are deterministic — the same message produces the same routing every time. They're auditable — you can read the script and know exactly why a message went where it went. They're portable — moving providers means copying the script over."
        }

        p class="mb-6" {
            "What replaced Sieve in consumer products is opaque categorization. Gmail's Promotions tab is not driven by a rule a user can read, edit, or export. It is the output of a machine-learning classifier whose features include, at minimum, the sender, recipient, headers, body content, click history of similar messages by other Gmail users, and unspecified other signals derived from a corpus the user has no access to. When the classifier is wrong, the user has no way to ask why. They have a button — \"move to inbox\" — and the model, eventually, learns. Maybe."
        }

        p class="mb-6" {
            "This is the trade. The user gets a smarter default, and the user gives up the ability to know what their software is doing on their behalf."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "Why nobody has fixed it"
        }

        p class="mb-6" {
            "There's no commercial pressure to fix this. The opacity is a feature, not a bug, for the incumbents. An opaque classifier creates lock-in: you can't take your model with you, and the moment you switch providers you start over with the inbox of a stranger. A transparent rule-set is portable, and portability is the opposite of lock-in. Of course Gmail isn't going to ship a Sieve editor. They're a public company. The product strategy that maximizes shareholder value is the one that maximizes switching costs."
        }

        p class="mb-6" {
            "The open-source mail clients haven't fixed it either, for a different reason: the rule-editing UI is genuinely hard to design well. A naive form for \"if header X contains Y, move to folder Z\" is barely better than a regex. A good UI has to handle rule ordering, rule conflicts, score-based ranking, header/body addressing, and a dozen edge cases that come up the first time someone tries to filter a real inbox. Most open-source clients punt on this and ship a minimum-viable filter form, which means most users never write filters, which means the entire ecosystem looks like Sieve doesn't work — even though, on the server, it does."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "What we're building"
        }

        p class="mb-6" {
            "Thundercrab is a desktop mail client that treats Sieve rules as the primary user surface, not a dropdown buried six levels into preferences. The rules live where they belong: on the user's IMAP server. The client is a typed editor + audit explainer for them."
        }

        p class="mb-6" {
            "The core experience is this: every message in the inbox carries a small \"why\" affordance. Click it, and the client tells you which rule routed this message to this folder, which header matched, what score the rule earned against the alternatives, and what would happen if the rule changed. The user is never asked to trust a black box. The categorization is a fact about the rules they wrote, not an inference from a model they can't see."
        }

        p class="mb-6" {
            "When the user wants to add or edit a rule, the editor exposes the structure of Sieve directly — but with constraints. You can't write a rule that references the message body when you only need a header. You can't accidentally drop a rule's score below the previous rule and silently reorder the chain. The compiler refuses to push a malformed script. Bad rules can't ship."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "What \"local-first\" means here"
        }

        p class="mb-6" {
            "Thundercrab runs on the user's machine. Mail flows from their IMAP server to their disk; rules are pushed back via ManageSieve, which is the standardized protocol for installing Sieve scripts on a mail server. There is no cloud component. There is no telemetry. The client doesn't know what the user wrote, never sends it anywhere, and couldn't if it wanted to — there's no service to send to."
        }

        p class="mb-6" {
            "This matters for two reasons. First, mail is one of the most sensitive data sets a person owns; the categorization rules themselves often disclose what the user cares about. (\"Always file messages from this person under family\" is a fact about the user's relationships.) Building a cloud-mediated rule editor would mean those facts cross a network. We don't want that responsibility, and the user shouldn't have to grant it. Second, the architecture has to survive the moment when the company that built the client goes away. A client whose rules are stored on the user's IMAP server, in an open standard, keeps working when we vanish. A client whose rules live on our servers does not."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "What it isn't"
        }

        p class="mb-6" {
            "Thundercrab is not a Gmail clone. It does not categorize incoming mail using machine learning. It does not summarize threads with a language model running on someone else's hardware. It does not auto-respond, auto-snooze, auto-anything that requires inferring intent from data the user can't see."
        }

        p class="mb-6" {
            "It does, however, "
            em { "suggest" }
            " rules. When the same kind of message has been moved to the same folder five times, the client offers — explicitly, with a dialog the user must accept — to convert the pattern into a Sieve rule. The suggestion is generated locally, the rule is shown in full before installation, and the user can edit the suggestion before pushing it. The categorization itself is always rule-based and always explainable; the AI part is the suggestion engine, and the user always has final say."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "Why we care enough to build it"
        }

        p class="mb-6" {
            "We're an IT consultancy. Our clients — law firms, journalists, healthcare practices, advocacy nonprofits — handle some of the most subpoena-prone, journalistically-targeted, and adversarial mail in the country. Telling them to use Gmail because the categorization is good is professional malpractice. Telling them to switch to a self-hosted server but giving them no UI to actually run it has been the standing recommendation for fifteen years and the reason most of them stay on Gmail."
        }

        p class="mb-6" {
            "We needed a tool we could give our clients that lets them actually use the standard. Nothing existed. We've started building it."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "What's shipping when"
        }

        p class="mb-6" {
            "The first usable Thundercrab build is targeted for later this year. It will be open-source, free for personal use, and run on Linux first; macOS and Windows follow once the GTK theming is portable. We are not raising venture capital for it; the project is funded out of consultancy revenue, which means we can ship slowly and not be wrong on purpose. We will write more about it as the milestones land — what surprised us, what we got wrong the first time, what we'd do differently if we started over."
        }

        p class="mb-6" {
            "If your firm has been waiting for a client that treats Sieve like the standard it is, that doesn't ship rules to the cloud, and that explains every routing decision in plain English — that's the one we're building. The waitlist is short, the engineering is interesting, and we're happy to talk about either."
        }
    }
}
