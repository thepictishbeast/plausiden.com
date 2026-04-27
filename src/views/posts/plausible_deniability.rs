//! Field note: what plausible deniability means in our architecture.
//!
//! Argues that the suite's contribution is a publication-stage defense
//! — not just hiding data, but actively making the data that escapes
//! unreliable. Names the bright lines (no fraud, no perjury, no
//! interference with regulated records) and the compliance separation
//! problem we still have to solve.

use maud::{Markup, html};

/// Render the post body. Wrapper supplies chrome, eyebrow, title, date.
#[must_use]
pub fn render() -> Markup {
    html! {

        p class="text-lg text-slate-600 leading-relaxed mb-8" {
            "We named the company after a phrase. Most privacy products are sold around hiding — we encrypt your data, we tunnel your traffic, we anonymize your identity. The phrase we picked is older and more interesting: "
            em { "plausible deniability" }
            ". The category move it implies is different from hiding. It's the difference between an empty room and a room so full of identical objects that nobody can tell which one is yours."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "Hiding is brittle. Unreliability is recoverable."
        }

        p class="mb-6" {
            "Every hiding-based privacy guarantee has a single point of failure. Encryption can be broken by key theft, traffic analysis, or a court order. VPNs can be compromised at the exit node. Tor can be deanonymized by correlation attacks given enough patience. The privacy property holds up to that one failure, then collapses entirely."
        }

        p class="mb-6" {
            "Unreliability has a different failure mode. If an adversary recovers your encrypted data and your traffic logs and your full search history, they have something — but what they have doesn't yield a reliable inference. The signal is buried in noise that was deliberately added, methodically, over years. Recovering the haystack does not, by itself, find the needle. The defense is robust to the kind of catastrophic compromise the hiding model can't survive."
        }

        p class="mb-6" {
            "This is what we mean when we say we sell unreliability. Not as a marketing flourish — as the actual architectural property the products are designed to produce."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "What \"publication-stage defense\" means"
        }

        p class="mb-6" {
            "Existing privacy stacks defend two stages of the data lifecycle. They protect data "
            em { "in transit" }
            " (TLS, VPN, Tor) and they protect data "
            em { "at rest" }
            " (full-disk encryption, end-to-end encryption, hardware tokens). What almost no consumer-facing product protects is data "
            em { "after publication" }
            " — the data that has already escaped into search histories, ad-graph profiles, social-graph adjacencies, location logs, broker databases, and AI training corpora. Most teams treat published data as lost: there's nothing left to protect once it's out."
        }

        p class="mb-6" {
            "We don't think it's lost. It's just been ceded to the wrong assumption — that any leakage is total. The pollution model rejects that assumption. If you can keep adding plausible-but-false records to the same channels at the same rate, the published substrate stays noisy. The data exists; it just doesn't carry the weight it would if it were the only record."
        }

        p class="mb-6" {
            "Concretely: a search history that includes a single query for \"how to dispose of a body\" is one thing. A search history that includes that query alongside forty-seven other equally-improbable queries — entered systematically by software the user installed, with disclosed methodology — is a different artifact entirely. The first looks like evidence. The second looks like an unreliable substrate."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "Why this changes the burden of proof"
        }

        p class="mb-6" {
            "Plausible deniability has a traditional sense (intelligence-community usage: a principal can claim with some basis not to have known about a deniable operation). Our sense is more structural: a record is consistent with multiple competing inferences, and no one inference is supported strongly enough to carry weight. The two senses converge in practice. If your data record is genuinely ambiguous between many explanations, the principal can credibly say \"that's not me\" because the evidence does not in fact pick out them."
        }

        p class="mb-6" {
            "This shifts the burden in every adversarial setting where data is exhibited:"
        }

        p class="mb-6" {
            strong { "Criminal proceedings." }
            " A search-history exhibit is worth less when the defense can show systematic noise injection on the same device over years, with the methodology published in an open-source tool the defendant disclosed using. That isn't perjury — every record is genuine, the user really did initiate it. It's a stipulated unreliability of the substrate. Prosecutors are forced to carry their burden some other way."
        }

        p class="mb-6" {
            strong { "Civil discovery." }
            " Opposing counsel running keyword searches against produced data returns more chaff than wheat. Forensic experts are billable hours that get spent eliminating red herrings. The cost of an adversarial extraction goes up; the value of what's extracted goes down. We don't want litigation to be cheaper to attack than to defend."
        }

        p class="mb-6" {
            strong { "Data brokers." }
            " The industry's product is segmentation accuracy. Brokers price on confidence, not just on volume. A profile contaminated with high-volume, high-plausibility false records degrades the broker's segment-quality scores. The economics of mass behavioral targeting depend on the underlying data being approximately true; pollution attacks the assumption."
        }

        p class="mb-6" {
            strong { "AI training." }
            " The largest models are trained on scraped behavioral substrates. Pollution doesn't make a single model fail — it makes the substrate poisoned in aggregate. The cost is paid by the buyer: bad data, mismatched inferences, harder evaluation. We can't stop the scraping; we can shrink the value of what's scraped."
        }

        p class="mb-6" {
            strong { "Reputation." }
            " The same shift applies to the substrate that strangers use to judge a person — social-graph adjacencies, public profile data, scraped activity. When everyone knows the substrate is noisy, no individual data point is reputation-bearing. A workplace investigation, a custody hearing, a journalist's pre-publication review — all become less reliant on \"the data says\" and more reliant on real evidence."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "Where this doesn't apply, and why we say so out loud"
        }

        p class="mb-6" {
            "Pollution is not a license to falsify records you have a regulated duty to keep accurate. We will not build, and will not ship, tools that do any of the following:"
        }

        p class="mb-6" {
            strong { "No fraudulent records of regulated transactions. " }
            "Tax filings, financial statements, healthcare records, court submissions, KYC submissions — these are records the law requires you to keep accurate. A pollution tool that touches them is fraud, not privacy. The line is bright, the line is short, and we are on the side of the line that keeps real records real."
        }

        p class="mb-6" {
            strong { "No interference with judicial process. " }
            "Polluting the publicly-scraped substrate is not the same as polluting evidence under preservation order. If a court has compelled production of specific data, that data is preserved unmodified. The pollution tools have an off switch tied to legal-hold workflows. A law firm using us cannot accidentally have its self-protection pollution leak into discoverable case records."
        }

        p class="mb-6" {
            strong { "No perjury support. " }
            "We design for ambiguity in the substrate, not lying under oath. Plausible deniability is a property of records; testimony is a separate question, governed by separate rules, and our products do not extend their methodology there."
        }

        p class="mb-6" {
            "These bright lines are not a marketing concession. They are the load-bearing wall that lets the rest of the architecture work. A pollution stack that doesn't respect them is — correctly — an obstruction tool, and obstruction tools collapse under the first serious legal scrutiny they encounter. The whole point is to ship something that survives that scrutiny."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "The compliance separation problem"
        }

        p class="mb-6" {
            "Our hardest design problem isn't building the pollution. It's keeping the pollution out of the substrates that are supposed to be authoritative. A law firm using a search-noise tool to protect attorney browsing history cannot have that noise contaminate the firm's own case-management records. A healthcare practice cannot have its self-protection pollution land in the patient chart. A journalist cannot have decoy contacts visible in the documentation that proves a source's testimony."
        }

        p class="mb-6" {
            "The answer is architectural separation: the pollution surface and the regulated-record surface are different storage scopes, with different I/O paths, and a typed boundary between them that the application code cannot cross. This is the same composability discipline we use for other privacy properties (see our note on "
            em { "provable-by-construction privacy" }
            "). When we design a pollution tool for a regulated context, we design the boundary first and the noise generator second."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "What this means for the products we ship"
        }

        p class="mb-6" {
            "The PlausiDen suite is not finished. The pieces that exist today (the rule-based mail client, the audit framework, the design system, the supersociety doctrine) are the substrate; the pollution layer is what makes the doctrine ship. We are deliberate about staging this. The hiding-stage products go first because they're easier to evaluate and harder to misuse. The publication-stage products go second, because they require the legal and ethical scaffolding to be in place before they're useful."
        }

        p class="mb-6" {
            "The pollution tools we will eventually ship include: a search-noise generator that injects plausible queries on the user's behalf, with disclosed cadence and source; an ad-trap that engages with served ads to systematically degrade profile-building; a decoy-publication stream for social platforms that cannot be opted out of; a location-noise injector for the operating-system-level location services; and an email-decoy generator for the relationship-graph layer. Each of these is small. None of them is novel in isolation. The novelty is the suite, the typed boundary between pollution and regulated record, and the audit machinery that proves the boundary holds."
        }

        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-12 mb-4" {
            "Why this is the right time"
        }

        p class="mb-6" {
            "The threat models that pollution defends against — mass behavioral targeting, AI training scrape, broker segmentation, reputation inference — were not load-bearing fifteen years ago. They are now. The legal frameworks have not caught up; the market has not caught up. People who care about their privacy still default to hiding-stage tools because that's what the market sells. We think the architecture has matured to the point where the publication-stage layer is shippable, and we think the people most exposed to the asymmetric inference economy — clients of the legal, healthcare, journalism, and advocacy practices we serve — have the most to gain from a stack that explicitly defends it."
        }

        p class="mb-6" {
            "We named the company after the property. The products will eventually exhibit it. This note is here to document what the property actually is, before the marketing gets ahead of the engineering."
        }
    }
}
