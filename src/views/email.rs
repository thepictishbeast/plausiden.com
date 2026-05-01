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
const TEXT_HEADING: &str = "#0f172a";
const TEXT_BODY: &str = "#334155";
const TEXT_MUTED: &str = "#64748b";
const SURFACE: &str = "#f8fafc";
const SURFACE_RAISED: &str = "#ffffff";
const HAIRLINE: &str = "#e2e8f0";

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

/// Pull the first letter of `s` (uppercased), or `?` if empty / non-ASCII.
/// Used for the avatar circle in notification emails.
fn initial(s: &str) -> String {
    s.trim()
        .chars()
        .find(char::is_ascii_alphabetic)
        .map_or_else(|| "?".to_string(), |c| c.to_ascii_uppercase().to_string())
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
<body style="margin:0;padding:0;background:{SURFACE};font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif;color:{TEXT_BODY};-webkit-font-smoothing:antialiased;line-height:1.5;">
<div style="display:none;max-height:0;overflow:hidden;mso-hide:all;font-size:1px;color:{SURFACE};">{preheader_safe}</div>
<table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" style="background:{SURFACE};">
  <tr><td align="center" style="padding:40px 16px;">
    <table role="presentation" width="600" cellpadding="0" cellspacing="0" border="0" style="max-width:600px;background:{SURFACE_RAISED};border:1px solid {HAIRLINE};border-radius:14px;overflow:hidden;box-shadow:0 1px 3px rgba(15,23,42,0.04),0 8px 24px rgba(15,23,42,0.06);">
      <tr><td style="background:linear-gradient(135deg,{BRAND_NAVY} 0%,{BRAND_PRIMARY} 60%,{BRAND_ACCENT} 100%);padding:28px 36px;">
        <table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0">
          <tr>
            <td align="left" style="font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif;color:#ffffff;">
              <table role="presentation" cellpadding="0" cellspacing="0" border="0">
                <tr>
                  <td style="vertical-align:middle;padding-right:14px;">
                    <span style="display:inline-block;width:36px;height:36px;background:rgba(255,255,255,0.18);border:1px solid rgba(255,255,255,0.35);border-radius:9px;text-align:center;line-height:36px;font-size:16px;font-weight:800;color:#ffffff;letter-spacing:-0.02em;">P</span>
                  </td>
                  <td style="vertical-align:middle;">
                    <div style="font-size:18px;font-weight:700;letter-spacing:-0.01em;color:#ffffff;">PlausiDen <span style="color:rgba(255,255,255,0.78);font-weight:500;">LLC</span></div>
                    <div style="font-size:12px;font-weight:500;color:rgba(255,255,255,0.72);letter-spacing:0.04em;text-transform:uppercase;margin-top:2px;">Plausible. Defensible.</div>
                  </td>
                </tr>
              </table>
            </td>
          </tr>
        </table>
      </td></tr>
      <tr><td style="padding:36px 36px 32px;">
        {inner}
      </td></tr>
      <tr><td style="background:{SURFACE};padding:22px 36px;border-top:1px solid {HAIRLINE};">
        <table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0">
          <tr>
            <td style="font-size:12px;line-height:1.7;color:{TEXT_MUTED};">
              <strong style="color:{TEXT_BODY};font-weight:600;">PlausiDen LLC</strong> &nbsp;·&nbsp; Massachusetts, USA<br>
              <a href="https://plausiden.com" style="color:{BRAND_PRIMARY};text-decoration:none;font-weight:600;">plausiden.com</a> &nbsp;·&nbsp; <a href="mailto:team@plausiden.com" style="color:{BRAND_PRIMARY};text-decoration:none;font-weight:600;">team@plausiden.com</a>
            </td>
          </tr>
        </table>
      </td></tr>
    </table>
    <p style="margin:16px 0 0;font-size:11px;color:{TEXT_MUTED};text-align:center;letter-spacing:0.02em;">
      This message was sent by an automated system. Replies go to a real human.
    </p>
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
        r#"<div style="display:inline-block;font-size:11px;font-weight:600;letter-spacing:0.1em;text-transform:uppercase;color:{BRAND_PRIMARY};background:rgba(13,72,138,0.08);padding:5px 10px;border-radius:999px;margin:0 0 14px;">Sign-in link</div>
<h1 style="margin:0 0 14px;font-size:26px;line-height:1.25;color:{TEXT_HEADING};font-weight:700;letter-spacing:-0.02em;">Sign in to PlausiDen admin</h1>
<p style="margin:0 0 28px;font-size:15px;line-height:1.65;color:{TEXT_BODY};">
  Click the button below within <strong style="color:{TEXT_HEADING};font-weight:600;">15 minutes</strong> to sign in. The link is single-use — once you click it, it can't be reused.
</p>
<table role="presentation" cellpadding="0" cellspacing="0" border="0" style="margin:0 0 30px;">
  <tr><td>
    <a href="{link_safe}" style="display:inline-block;padding:14px 30px;background:{BRAND_PRIMARY};color:#ffffff;font-size:15px;font-weight:600;text-decoration:none;border-radius:10px;letter-spacing:0.01em;box-shadow:0 1px 2px rgba(13,72,138,0.25),0 4px 14px rgba(13,72,138,0.18);">Sign in →</a>
  </td></tr>
</table>
<p style="margin:0 0 8px;font-size:13px;line-height:1.6;color:{TEXT_MUTED};">If the button doesn't work, paste this URL into your browser:</p>
<p style="margin:0 0 28px;font-size:12px;line-height:1.55;color:{TEXT_BODY};word-break:break-all;font-family:ui-monospace,SFMono-Regular,Menlo,Consolas,monospace;background:{SURFACE};padding:12px 14px;border-radius:8px;border:1px solid {HAIRLINE};">
  {link_safe}
</p>
<div style="height:1px;background:{HAIRLINE};margin:0 0 24px;"></div>
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
    let avatar = escape_html(&initial(name));

    let mut sections_html = String::new();
    for (label, value) in sections {
        if value.trim().is_empty() {
            continue;
        }
        let _ = write!(
            sections_html,
            r#"<table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" style="margin:0 0 14px;background:{SURFACE_RAISED};border:1px solid {HAIRLINE};border-radius:10px;border-left:3px solid {BRAND_ACCENT};">
              <tr><td style="padding:14px 16px 16px;">
                <div style="font-size:11px;font-weight:700;letter-spacing:0.1em;text-transform:uppercase;color:{BRAND_PRIMARY};margin-bottom:8px;">{label}</div>
                <div style="font-size:14.5px;line-height:1.65;color:{TEXT_BODY};white-space:pre-line;">{value}</div>
              </td></tr>
            </table>"#,
            label = escape_html(label),
            value = escape_html(value),
        );
    }
    if sections_html.is_empty() {
        let _ = write!(
            sections_html,
            r#"<p style="font-size:14px;color:{TEXT_MUTED};font-style:italic;margin:0;">No long-form answers provided.</p>"#
        );
    }

    let inner = format!(
        r#"<div style="display:inline-block;font-size:11px;font-weight:600;letter-spacing:0.1em;text-transform:uppercase;color:{BRAND_PRIMARY};background:rgba(13,72,138,0.08);padding:5px 10px;border-radius:999px;margin:0 0 14px;">Feedback · #{row_id}</div>
<h1 style="margin:0 0 8px;font-size:26px;line-height:1.25;color:{TEXT_HEADING};font-weight:700;letter-spacing:-0.02em;">New feedback received</h1>
<p style="margin:0 0 26px;font-size:14px;line-height:1.6;color:{TEXT_MUTED};">
  Submitted via the public form at plausiden.com/feedback.
</p>

<table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" style="background:{SURFACE};border:1px solid {HAIRLINE};border-radius:12px;margin:0 0 28px;">
  <tr><td style="padding:18px 20px;">
    <table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0">
      <tr>
        <td style="vertical-align:middle;padding-right:14px;width:48px;">
          <div style="width:46px;height:46px;line-height:46px;text-align:center;border-radius:50%;background:linear-gradient(135deg,{BRAND_PRIMARY} 0%,{BRAND_ACCENT} 100%);color:#ffffff;font-size:18px;font-weight:700;letter-spacing:-0.01em;">{avatar}</div>
        </td>
        <td style="vertical-align:middle;">
          <div style="font-size:16px;font-weight:700;color:{TEXT_HEADING};letter-spacing:-0.01em;">{name_safe}</div>
          <div style="font-size:13px;color:{TEXT_MUTED};margin-top:2px;">{email_safe}{company_separator}{company_safe}</div>
        </td>
      </tr>
    </table>
    <table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" style="margin-top:14px;border-top:1px dashed {HAIRLINE};padding-top:14px;">
      <tr>
        <td style="font-size:12px;color:{TEXT_MUTED};letter-spacing:0.04em;text-transform:uppercase;font-weight:600;">Consent</td>
        <td style="font-size:13px;color:{TEXT_BODY};font-family:ui-monospace,SFMono-Regular,Menlo,Consolas,monospace;text-align:right;">{consent_safe}</td>
      </tr>
    </table>
  </td></tr>
</table>

{sections_html}

<table role="presentation" cellpadding="0" cellspacing="0" border="0" style="margin:28px 0 0;">
  <tr><td>
    <a href="https://plausiden.com/admin/feedback" style="display:inline-block;padding:12px 24px;background:{BRAND_PRIMARY};color:#ffffff;font-size:14px;font-weight:600;text-decoration:none;border-radius:10px;letter-spacing:0.01em;box-shadow:0 1px 2px rgba(13,72,138,0.25),0 4px 14px rgba(13,72,138,0.18);">View in admin →</a>
  </td></tr>
</table>"#,
        company_separator = if company.trim().is_empty() {
            ""
        } else {
            " · "
        },
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
    let avatar = escape_html(&initial(name));

    let inner = format!(
        r#"<div style="display:inline-block;font-size:11px;font-weight:600;letter-spacing:0.1em;text-transform:uppercase;color:{BRAND_PRIMARY};background:rgba(13,72,138,0.08);padding:5px 10px;border-radius:999px;margin:0 0 14px;">New inquiry</div>
<h1 style="margin:0 0 8px;font-size:26px;line-height:1.25;color:{TEXT_HEADING};font-weight:700;letter-spacing:-0.02em;">New encrypted inquiry</h1>
<p style="margin:0 0 26px;font-size:14px;line-height:1.6;color:{TEXT_MUTED};">
  Submitted via the public form at plausiden.com/contact.
</p>

<table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" style="background:{SURFACE};border:1px solid {HAIRLINE};border-radius:12px;margin:0 0 24px;">
  <tr><td style="padding:18px 20px;">
    <table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0">
      <tr>
        <td style="vertical-align:middle;padding-right:14px;width:48px;">
          <div style="width:46px;height:46px;line-height:46px;text-align:center;border-radius:50%;background:linear-gradient(135deg,{BRAND_PRIMARY} 0%,{BRAND_ACCENT} 100%);color:#ffffff;font-size:18px;font-weight:700;letter-spacing:-0.01em;">{avatar}</div>
        </td>
        <td style="vertical-align:middle;">
          <div style="font-size:16px;font-weight:700;color:{TEXT_HEADING};letter-spacing:-0.01em;">{name_safe}</div>
          <div style="font-size:13px;color:{TEXT_MUTED};margin-top:2px;">{reply_safe}</div>
        </td>
      </tr>
    </table>
    <table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" style="margin-top:16px;border-top:1px dashed {HAIRLINE};padding-top:14px;">
      <tr>
        <td width="33%" style="vertical-align:top;padding-right:8px;">
          <div style="font-size:11px;font-weight:700;letter-spacing:0.1em;text-transform:uppercase;color:{TEXT_MUTED};margin-bottom:4px;">Phone</div>
          <div style="font-size:13px;color:{TEXT_BODY};font-weight:500;">{phone_safe}</div>
        </td>
        <td width="33%" style="vertical-align:top;padding-right:8px;">
          <div style="font-size:11px;font-weight:700;letter-spacing:0.1em;text-transform:uppercase;color:{TEXT_MUTED};margin-bottom:4px;">Company</div>
          <div style="font-size:13px;color:{TEXT_BODY};font-weight:500;">{company_safe}</div>
        </td>
        <td width="33%" style="vertical-align:top;">
          <div style="font-size:11px;font-weight:700;letter-spacing:0.1em;text-transform:uppercase;color:{TEXT_MUTED};margin-bottom:4px;">Service</div>
          <div style="font-size:13px;color:{TEXT_BODY};font-weight:500;">{service_safe}</div>
        </td>
      </tr>
    </table>
  </td></tr>
</table>

<table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" style="background:{SURFACE_RAISED};border:1px solid {HAIRLINE};border-radius:12px;border-left:3px solid {BRAND_ACCENT};margin:0 0 28px;">
  <tr><td style="padding:18px 20px 20px;">
    <div style="font-size:11px;font-weight:700;letter-spacing:0.1em;text-transform:uppercase;color:{BRAND_PRIMARY};margin-bottom:10px;">Message</div>
    <div style="font-size:14.5px;line-height:1.65;color:{TEXT_BODY};white-space:pre-line;">{message_safe}</div>
  </td></tr>
</table>

<table role="presentation" cellpadding="0" cellspacing="0" border="0">
  <tr><td>
    <a href="mailto:{reply_safe}" style="display:inline-block;padding:12px 24px;background:{BRAND_PRIMARY};color:#ffffff;font-size:14px;font-weight:600;text-decoration:none;border-radius:10px;letter-spacing:0.01em;box-shadow:0 1px 2px rgba(13,72,138,0.25),0 4px 14px rgba(13,72,138,0.18);">Reply to {name_safe} →</a>
  </td></tr>
</table>"#,
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
    fn feedback_notification_html_renders_avatar_initial() {
        // Avatar circle uses the first ASCII letter, uppercased.
        let h = feedback_notification_html(1, "tim", "", "t@x.com", "", &[("note", "hi")]);
        // The avatar block contains the uppercase T as inner text.
        assert!(h.contains(">T</div>"), "avatar initial missing");
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

    #[test]
    fn inquiry_notification_html_has_reply_cta() {
        let h = inquiry_notification_html("Alice", "alice@example.org", "", "", "", "hi");
        assert!(h.contains("Reply to Alice"));
        assert!(h.contains("mailto:alice@example.org"));
    }

    #[test]
    fn shell_has_brand_tagline() {
        let h = magic_link_html("https://x");
        assert!(h.contains("Plausible. Defensible."));
    }
}
