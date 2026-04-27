# Security Policy

PlausiDen LLC takes the security of plausiden.com — and the broader
ecosystem of PlausiDen tools — seriously. If you believe you've found
a vulnerability, this document explains how to report it.

## Reporting a vulnerability

Email **security@plausiden.com**. PGP key fingerprint will be published
once the public key is signed.

What to include:

- A description of the issue.
- Reproduction steps (URL, payload, expected vs. actual behavior).
- The impact you believe the issue has.
- Your name and how you'd like to be credited (optional).

We commit to:

- Acknowledge your report within **3 business days**.
- Provide an initial assessment within **10 business days**.
- Keep you informed of remediation progress.
- Credit you in our public advisory (if you want credit) once the issue
  is resolved or you confirm you'd prefer not to wait.

## Scope

In scope:

- `plausiden.com` (production site).
- `next.plausiden.com` (staging).
- `mail.plausiden.com` (mail server: SMTP/IMAP/ManageSieve, ports 25,
  465, 587, 993, 4190).
- Code in any **public** repository under
  `https://github.com/thepictishbeast/PlausiDen-*`.
- The `crab-ledger` Tor hidden service, once announced.

Out of scope:

- Third-party services we don't operate (DNS registrars, certificate
  authorities, content delivery networks). Report those upstream.
- Social-engineering of PlausiDen staff.
- Physical access to equipment.
- Denial-of-service attacks on production infrastructure.

## Safe harbor

We will not pursue legal action against researchers who:

- Make a good-faith effort to comply with this policy.
- Do not access, modify, or destroy data that doesn't belong to you.
- Do not perform attacks that disrupt service for other users.
- Give us reasonable time to respond before public disclosure
  (typically 90 days, but we'll discuss in good faith).

## Recognition

We maintain a public Hall of Fame for researchers who report verified
vulnerabilities. We don't currently offer a bounty; we hope to in the
future once we are at a financial scale where that's responsible.

## What you should NOT do

- Submit reports through GitHub Issues, PRs, or any public channel.
  Use email.
- Attempt to access user data, including your own under another
  account, beyond the minimum needed to demonstrate the issue.
- Publicly disclose the issue before we've had a chance to remediate.
- Engage in social engineering or physical attacks.

If you're unsure whether something is in scope or whether your
research approach is acceptable, email us before testing.
