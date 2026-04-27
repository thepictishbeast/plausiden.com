//! Branded HTML email templates.
//!
//! Email clients are notoriously unforgiving — Gmail strips `<style>`
//! blocks; Outlook ignores half of CSS3; dark-mode renderers invert
//! colors unpredictably. So every template here is hand-written
//! table-based HTML with inline styles. No external assets, no remote
//! fonts, no JS. The fallback for any client that can't render is the
//! `text/plain` part the caller pairs with the HTML.
//!
//! SECURITY: All caller-supplied strings are escaped via `escape_html`
//! before being interpolated. No raw `PreEscaped` here — these run
//! over user-supplied content (feedback bodies, magic-link URLs).

const BRAND_NAVY: &str = "#0a2c52";
const BRAND_PRIMARY: &str = "#0d488a";
const BRAND_ACCENT: &str = "#3b82f6";
const TEXT_BODY: &str = "#334155";
const TEXT_MUTED: &str = "#64748b";
const SURFACE: &str = "#f8fafc";

/// HTML-escape arbitrary text for inclusion in an email body. Conservative —
/// covers the five XML metacharacters.
fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

/// Common shell that wraps an inner content fragment in the brand
/// chrome (header bar, padded card, footer signature).
fn shell(title: &str, preheader: &str, inner: &str) -> String {
    let title_safe = escape_html(title);
    let preheader_safe = escape_html(preheader);
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<meta name="color-scheme" content="light only">
<meta name="supported-color-schemes" content="light only">
<title>{title_safe}</title>
</head>
<body style="margin:0;padding:0;background:{SURFACE};font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif;color:{TEXT_BODY};-webkit-font-smoothing:antialiased;">
<div style="display:none;max-height:0;overflow:hidden;mso-hide:all;">{preheader_safe}</div>
<table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" style="background:{SURFACE};">
  <tr><td align="center" style="padding:32px 16px;">
    <table role="presentation" width="600" cellpadding="0" cellspacing="0" border="0" style="max-width:600px;background:#ffffff;border:1px solid #e2e8f0;border-radius:12px;overflow:hidden;">
      <tr><td style="background:linear-gradient(135deg,{BRAND_NAVY} 0%,{BRAND_PRIMARY} 100%);padding:24px 32px;">
        <table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0">
          <tr>
            <td align="left" style="font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif;color:#ffffff;font-size:18px;font-weight:700;letter-spacing:-0.01em;">
              <span style="display:inline-block;width:28px;height:28px;background:{BRAND_ACCENT};border-radius:6px;vertical-align:middle;margin-right:10px;"></span>PlausiDen <span style="color:{BRAND_ACCENT};font-weight:700;">LLC</span>
            </td>
          </tr>
        </table>
      </td></tr>
      <tr><td style="padding:32px;">
        {inner}
      </td></tr>
      <tr><td style="background:{SURFACE};padding:20px 32px;border-top:1px solid #e2e8f0;">
        <p style="margin:0;font-size:12px;line-height:1.6;color:{TEXT_MUTED};">
          PlausiDen LLC · Massachusetts, USA<br>
          <a href="https://plausiden.com" style="color:{BRAND_PRIMARY};text-decoration:none;">plausiden.com</a> · <a href="mailto:team@plausiden.com" style="color:{BRAND_PRIMARY};text-decoration:none;">team@plausiden.com</a>
        </p>
      </td></tr>
    </table>
  </td></tr>
</table>
</body>
</html>
"#
    )
}

/// HTML body for the "sign in to admin" magic-link email.
#[must_use]
pub fn magic_link_html(link: &str) -> String {
    let link_safe = escape_html(link);
    let inner = format!(
        r#"<h1 style="margin:0 0 16px;font-size:22px;line-height:1.3;color:{BRAND_NAVY};font-weight:700;">Sign in to PlausiDen admin</h1>
<p style="margin:0 0 24px;font-size:15px;line-height:1.6;color:{TEXT_BODY};">
  Click the button below within <strong style="color:{BRAND_NAVY};">15 minutes</strong> to sign in. The link is single-use — once you click it, it can't be reused.
</p>
<table role="presentation" cellpadding="0" cellspacing="0" border="0" style="margin:0 0 24px;">
  <tr><td>
    <a href="{link_safe}" style="display:inline-block;padding:14px 28px;background:{BRAND_PRIMARY};color:#ffffff;font-size:15px;font-weight:600;text-decoration:none;border-radius:8px;">Sign in</a>
  </td></tr>
</table>
<p style="margin:0 0 8px;font-size:13px;line-height:1.6;color:{TEXT_MUTED};">If the button doesn't work, paste this URL into your browser:</p>
<p style="margin:0 0 24px;font-size:12px;line-height:1.5;color:{TEXT_MUTED};word-break:break-all;font-family:ui-monospace,SFMono-Regular,Menlo,Consolas,monospace;background:{SURFACE};padding:10px 12px;border-radius:6px;border:1px solid #e2e8f0;">
  {link_safe}
</p>
<p style="margin:0;font-size:13px;line-height:1.6;color:{TEXT_MUTED};">
  Didn't request this? You can safely ignore this email — the link will expire on its own.
</p>"#
    );
    shell(
        "Sign in to PlausiDen admin",
        "Single-use sign-in link, valid for 15 minutes.",
        &inner,
    )
}

/// HTML body for the "new feedback received" notification to team@.
#[must_use]
pub fn feedback_notification_html(
    row_id: i64,
    name: &str,
    company: &str,
    email: &str,
    consent: &str,
    sections: &[(&str, &str)],
) -> String {
    use std::fmt::Write as _;
    let name_safe = escape_html(name);
    let company_safe = escape_html(company);
    let email_safe = escape_html(email);
    let consent_safe = escape_html(if consent.is_empty() {
        "(none)"
    } else {
        consent
    });

    let mut sections_html = String::new();
    for (label, value) in sections {
        if value.trim().is_empty() {
            continue;
        }
        let _ = write!(
            sections_html,
            r#"<div style="margin:0 0 18px;"><div style="font-size:11px;font-weight:600;letter-spacing:0.08em;text-transform:uppercase;color:{TEXT_MUTED};margin-bottom:6px;">{label}</div><div style="font-size:14px;line-height:1.6;color:{TEXT_BODY};white-space:pre-line;">{value}</div></div>"#,
            label = escape_html(label),
            value = escape_html(value),
        );
    }
    if sections_html.is_empty() {
        let _ = write!(
            sections_html,
            r#"<p style="font-size:14px;color:{TEXT_MUTED};font-style:italic;">No long-form answers provided.</p>"#
        );
    }

    let inner = format!(
        r#"<h1 style="margin:0 0 8px;font-size:22px;line-height:1.3;color:{BRAND_NAVY};font-weight:700;">New feedback · #{row_id}</h1>
<p style="margin:0 0 24px;font-size:14px;color:{TEXT_MUTED};">
  Submitted via the public form on plausiden.com/feedback.
</p>
<table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" style="background:{SURFACE};border-radius:8px;border:1px solid #e2e8f0;margin:0 0 24px;">
  <tr><td style="padding:14px 16px;">
    <table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0">
      <tr><td style="font-size:13px;color:{TEXT_MUTED};padding-bottom:4px;">From</td><td style="font-size:13px;color:{TEXT_BODY};font-weight:600;padding-bottom:4px;text-align:right;">{name_safe}</td></tr>
      <tr><td style="font-size:13px;color:{TEXT_MUTED};padding-bottom:4px;">Company</td><td style="font-size:13px;color:{TEXT_BODY};padding-bottom:4px;text-align:right;">{company_safe}</td></tr>
      <tr><td style="font-size:13px;color:{TEXT_MUTED};padding-bottom:4px;">Email</td><td style="font-size:13px;color:{TEXT_BODY};padding-bottom:4px;text-align:right;">{email_safe}</td></tr>
      <tr><td style="font-size:13px;color:{TEXT_MUTED};">Consent</td><td style="font-size:13px;color:{TEXT_BODY};font-family:ui-monospace,SFMono-Regular,Menlo,Consolas,monospace;text-align:right;">{consent_safe}</td></tr>
    </table>
  </td></tr>
</table>
{sections_html}
<p style="margin:24px 0 0;font-size:13px;color:{TEXT_MUTED};">
  <a href="https://plausiden.com/admin/feedback" style="color:{BRAND_PRIMARY};text-decoration:none;font-weight:600;">View in admin →</a>
</p>"#
    );

    shell(
        "New feedback received",
        &format!("New feedback from {name} · #{row_id}"),
        &inner,
    )
}

/// HTML body for the "new contact inquiry" notification to team@.
#[must_use]
pub fn inquiry_notification_html(
    name: &str,
    reply_to: &str,
    phone: &str,
    company: &str,
    service: &str,
    message: &str,
) -> String {
    let name_safe = escape_html(or_omitted(name));
    let reply_safe = escape_html(or_omitted(reply_to));
    let phone_safe = escape_html(or_omitted(phone));
    let company_safe = escape_html(or_omitted(company));
    let service_safe = escape_html(or_omitted(service));
    let message_safe = escape_html(message);

    let inner = format!(
        r#"<h1 style="margin:0 0 8px;font-size:22px;line-height:1.3;color:{BRAND_NAVY};font-weight:700;">New encrypted inquiry</h1>
<p style="margin:0 0 24px;font-size:14px;color:{TEXT_MUTED};">
  Submitted via the public form on plausiden.com/contact.
</p>
<table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" style="background:{SURFACE};border-radius:8px;border:1px solid #e2e8f0;margin:0 0 24px;">
  <tr><td style="padding:14px 16px;">
    <table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0">
      <tr><td style="font-size:13px;color:{TEXT_MUTED};padding-bottom:4px;">Name</td><td style="font-size:13px;color:{TEXT_BODY};font-weight:600;padding-bottom:4px;text-align:right;">{name_safe}</td></tr>
      <tr><td style="font-size:13px;color:{TEXT_MUTED};padding-bottom:4px;">Reply-to</td><td style="font-size:13px;color:{TEXT_BODY};padding-bottom:4px;text-align:right;">{reply_safe}</td></tr>
      <tr><td style="font-size:13px;color:{TEXT_MUTED};padding-bottom:4px;">Phone</td><td style="font-size:13px;color:{TEXT_BODY};padding-bottom:4px;text-align:right;">{phone_safe}</td></tr>
      <tr><td style="font-size:13px;color:{TEXT_MUTED};padding-bottom:4px;">Company</td><td style="font-size:13px;color:{TEXT_BODY};padding-bottom:4px;text-align:right;">{company_safe}</td></tr>
      <tr><td style="font-size:13px;color:{TEXT_MUTED};">Service</td><td style="font-size:13px;color:{TEXT_BODY};text-align:right;">{service_safe}</td></tr>
    </table>
  </td></tr>
</table>
<div style="margin:0 0 8px;font-size:11px;font-weight:600;letter-spacing:0.08em;text-transform:uppercase;color:{TEXT_MUTED};">Message</div>
<div style="font-size:14px;line-height:1.6;color:{TEXT_BODY};white-space:pre-line;border-left:3px solid {BRAND_ACCENT};padding:8px 0 8px 14px;">{message_safe}</div>"#
    );

    shell(
        "New encrypted inquiry",
        &format!("Inquiry from {name}"),
        &inner,
    )
}

const fn or_omitted(s: &str) -> &str {
    if s.is_empty() { "(omitted)" } else { s }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magic_link_html_contains_link_and_brand() {
        let h = magic_link_html("https://plausiden.com/admin/login/verify?token=abc.def");
        assert!(h.contains("PlausiDen"));
        assert!(h.contains("Sign in"));
        assert!(h.contains("https://plausiden.com/admin/login/verify?token=abc.def"));
        // Inline styles only — no <style> blocks (some clients strip them)
        assert!(!h.contains("<style"));
    }

    #[test]
    fn magic_link_html_escapes_link() {
        // A pathological link with HTML metacharacters mustn't break out of
        // the href attribute or the visible code block.
        let h = magic_link_html("https://example.com/?x=<script>");
        assert!(!h.contains("<script>"));
        assert!(h.contains("&lt;script&gt;"));
    }

    #[test]
    fn feedback_notification_html_renders_sections() {
        let h = feedback_notification_html(
            42,
            "Tim",
            "Sacred.Vote",
            "tim@example.com",
            "full",
            &[
                ("What worked well", "the explainer is the killer feature"),
                ("What didn't", ""),
                ("Why chose PlausiDen", "the audit trail"),
            ],
        );
        assert!(h.contains("#42"));
        assert!(h.contains("Tim"));
        assert!(h.contains("Sacred.Vote"));
        assert!(h.contains("the explainer is the killer feature"));
        assert!(h.contains("the audit trail"));
        // Empty section was suppressed
        assert!(!h.contains("What didn't"));
    }

    #[test]
    fn inquiry_notification_html_escapes_message_body() {
        let h = inquiry_notification_html(
            "Mallory",
            "m@x.com",
            "",
            "",
            "",
            "<img src=x onerror=alert(1)>",
        );
        assert!(!h.contains("<img src=x"));
        assert!(h.contains("&lt;img src=x onerror=alert(1)&gt;"));
    }
}
