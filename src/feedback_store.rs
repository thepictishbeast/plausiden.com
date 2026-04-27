//! SQLite-backed store for feedback + testimonial submissions.
//!
//! Schema is intentionally CMS-shaped — every typed form field lives
//! in a column, plus a `received_at` timestamp and a `consent` enum.
//! The `/feedback/export` endpoint dumps the table as JSON / CSV /
//! TSV. When PlausiDen-CMS lands, the same SQLite shape is the
//! migration source.
//!
//! BUG ASSUMPTION: The store is single-process. plausiden-site runs
//! as one binary; concurrent writes are serialized by SQLite's
//! per-connection write lock. WAL mode allows concurrent readers.
//!
//! SECURITY: The DB lives at `/var/lib/plausiden-site/feedback.db`,
//! owned by the `plausiden` system user (mode 0600 effectively via
//! the directory's 0750). The export endpoint is gated by an
//! environment-variable token (`PLAUSIDEN_ADMIN_TOKEN`); without
//! the token set, the endpoint refuses every request, even from
//! the loopback. Read responses are PII; logging is suppressed.

use std::path::Path;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use rusqlite::{Connection, params};
use serde::Serialize;

/// One submission row, mirroring the form fields plus metadata.
#[derive(Debug, Clone, Serialize)]
pub struct FeedbackRow {
    /// Auto-incrementing row id.
    pub id: i64,
    /// Server-side timestamp of receipt.
    pub received_at: DateTime<Utc>,
    /// Submitter name (required field on the form).
    pub name: String,
    /// Submitter company / org (optional).
    pub company: String,
    /// Submitter email (optional).
    pub email: String,
    /// "What worked well?" (general feedback).
    pub worked_well: String,
    /// "What didn't?" (general feedback).
    pub didnt_work: String,
    /// Attribution consent (`full` / `name_only` / `role_only` /
    /// `anonymous` / `private`). Empty string when not selected.
    pub consent: String,
    /// Testimonial: alternative considered.
    pub alternative: String,
    /// Testimonial: why chose PlausiDen.
    pub why_chose: String,
    /// Testimonial: what changed.
    pub whats_changed: String,
    /// Testimonial: would recommend.
    pub recommend: String,
    /// Anything else.
    pub anything_else: String,
}

/// Shared store handle. Wrapped in `Arc<Mutex<>>` by the caller
/// (axum State) so multiple handler threads can serialize on the
/// SQLite connection.
pub struct FeedbackStore {
    conn: tokio::sync::Mutex<Connection>,
}

impl std::fmt::Debug for FeedbackStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FeedbackStore").finish_non_exhaustive()
    }
}

impl FeedbackStore {
    /// Open or create the store at `path`. Runs the initial migration
    /// idempotently.
    ///
    /// # Errors
    /// Returns the underlying rusqlite error if the file cannot be
    /// opened or the migration fails.
    pub fn open(path: &Path) -> rusqlite::Result<Arc<Self>> {
        let conn = Connection::open(path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        conn.execute_batch(
            r"
            CREATE TABLE IF NOT EXISTS feedback (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                received_at     TEXT    NOT NULL,
                name            TEXT    NOT NULL,
                company         TEXT    NOT NULL,
                email           TEXT    NOT NULL,
                worked_well     TEXT    NOT NULL,
                didnt_work      TEXT    NOT NULL,
                consent         TEXT    NOT NULL,
                alternative     TEXT    NOT NULL,
                why_chose       TEXT    NOT NULL,
                whats_changed   TEXT    NOT NULL,
                recommend       TEXT    NOT NULL,
                anything_else   TEXT    NOT NULL
            );
            CREATE INDEX IF NOT EXISTS feedback_received_at
                ON feedback(received_at);
            ",
        )?;
        Ok(Arc::new(Self {
            conn: tokio::sync::Mutex::new(conn),
        }))
    }

    /// In-memory store for tests. Same schema; no on-disk side effects.
    ///
    /// # Errors
    /// Returns the underlying rusqlite error on schema-creation failure.
    pub fn open_in_memory() -> rusqlite::Result<Arc<Self>> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(
            r"
            CREATE TABLE IF NOT EXISTS feedback (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                received_at     TEXT    NOT NULL,
                name            TEXT    NOT NULL,
                company         TEXT    NOT NULL,
                email           TEXT    NOT NULL,
                worked_well     TEXT    NOT NULL,
                didnt_work      TEXT    NOT NULL,
                consent         TEXT    NOT NULL,
                alternative     TEXT    NOT NULL,
                why_chose       TEXT    NOT NULL,
                whats_changed   TEXT    NOT NULL,
                recommend       TEXT    NOT NULL,
                anything_else   TEXT    NOT NULL
            );
            ",
        )?;
        Ok(Arc::new(Self {
            conn: tokio::sync::Mutex::new(conn),
        }))
    }

    /// Insert a new row. `received_at` is set server-side to `now()`.
    ///
    /// # Errors
    /// Returns the underlying rusqlite error on insert failure.
    pub async fn insert(&self, row: &FeedbackInsert<'_>) -> rusqlite::Result<i64> {
        let conn = self.conn.lock().await;
        let received_at = Utc::now().to_rfc3339();
        conn.execute(
            r"
            INSERT INTO feedback (
                received_at, name, company, email,
                worked_well, didnt_work,
                consent, alternative, why_chose, whats_changed,
                recommend, anything_else
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
            ",
            params![
                received_at,
                row.name,
                row.company,
                row.email,
                row.worked_well,
                row.didnt_work,
                row.consent,
                row.alternative,
                row.why_chose,
                row.whats_changed,
                row.recommend,
                row.anything_else,
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// Read every row. Used by the export endpoint.
    ///
    /// # Errors
    /// Returns the underlying rusqlite error on query failure.
    #[allow(clippy::significant_drop_tightening)]
    pub async fn list_all(&self) -> rusqlite::Result<Vec<FeedbackRow>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            r"
            SELECT id, received_at, name, company, email,
                   worked_well, didnt_work, consent, alternative,
                   why_chose, whats_changed, recommend, anything_else
            FROM feedback
            ORDER BY id ASC
            ",
        )?;
        let rows = stmt
            .query_map([], |r| {
                let received_str: String = r.get(1)?;
                let received_at = DateTime::parse_from_rfc3339(&received_str)
                    .map_or_else(|_| Utc::now(), |dt| dt.with_timezone(&Utc));
                Ok(FeedbackRow {
                    id: r.get(0)?,
                    received_at,
                    name: r.get(2)?,
                    company: r.get(3)?,
                    email: r.get(4)?,
                    worked_well: r.get(5)?,
                    didnt_work: r.get(6)?,
                    consent: r.get(7)?,
                    alternative: r.get(8)?,
                    why_chose: r.get(9)?,
                    whats_changed: r.get(10)?,
                    recommend: r.get(11)?,
                    anything_else: r.get(12)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }
}

/// Borrowed view of an insertion payload.
#[derive(Debug, Clone)]
pub struct FeedbackInsert<'a> {
    /// Submitter name.
    pub name: &'a str,
    /// Submitter company.
    pub company: &'a str,
    /// Submitter email.
    pub email: &'a str,
    /// "What worked well?".
    pub worked_well: &'a str,
    /// "What didn't?".
    pub didnt_work: &'a str,
    /// Attribution consent.
    pub consent: &'a str,
    /// Testimonial: alternative.
    pub alternative: &'a str,
    /// Testimonial: why chose.
    pub why_chose: &'a str,
    /// Testimonial: what changed.
    pub whats_changed: &'a str,
    /// Testimonial: recommend.
    pub recommend: &'a str,
    /// Anything else.
    pub anything_else: &'a str,
}

/// Render `rows` as JSON.
#[must_use]
pub fn export_json(rows: &[FeedbackRow]) -> String {
    serde_json::to_string_pretty(rows).unwrap_or_else(|_| "[]".into())
}

/// Render `rows` as a delimiter-separated table. Pass `,` for CSV
/// or `\t` for TSV. Quotes any cell containing the delimiter or a
/// newline using RFC 4180 conventions for CSV.
#[must_use]
pub fn export_dsv(rows: &[FeedbackRow], delim: char) -> String {
    use std::fmt::Write as _;
    let cols = [
        "id",
        "received_at",
        "name",
        "company",
        "email",
        "worked_well",
        "didnt_work",
        "consent",
        "alternative",
        "why_chose",
        "whats_changed",
        "recommend",
        "anything_else",
    ];
    let mut out = String::with_capacity(2048);
    for (i, c) in cols.iter().enumerate() {
        if i > 0 {
            out.push(delim);
        }
        out.push_str(c);
    }
    out.push('\n');
    for r in rows {
        let cells = [
            r.id.to_string(),
            r.received_at.to_rfc3339(),
            r.name.clone(),
            r.company.clone(),
            r.email.clone(),
            r.worked_well.clone(),
            r.didnt_work.clone(),
            r.consent.clone(),
            r.alternative.clone(),
            r.why_chose.clone(),
            r.whats_changed.clone(),
            r.recommend.clone(),
            r.anything_else.clone(),
        ];
        for (i, c) in cells.iter().enumerate() {
            if i > 0 {
                out.push(delim);
            }
            let needs_quote =
                c.contains(delim) || c.contains('\n') || c.contains('"') || c.contains('\r');
            if needs_quote {
                out.push('"');
                for ch in c.chars() {
                    if ch == '"' {
                        out.push_str("\"\"");
                    } else {
                        out.push(ch);
                    }
                }
                out.push('"');
            } else {
                let _ = write!(out, "{c}");
            }
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture<'a>() -> FeedbackInsert<'a> {
        FeedbackInsert {
            name: "Tim",
            company: "Sacred.Vote",
            email: "tim@example.com",
            worked_well: "the audit explainer is the killer feature",
            didnt_work: "the GUI is still a scaffold",
            consent: "full",
            alternative: "build it in-house, then give up",
            why_chose: "you let me see the rules",
            whats_changed: "I can audit a vote in one paragraph",
            recommend: "yes, especially for civic infra",
            anything_else: "",
        }
    }

    #[tokio::test]
    async fn insert_and_list_roundtrip() {
        let store = FeedbackStore::open_in_memory().unwrap();
        let id = store.insert(&fixture()).await.unwrap();
        assert!(id > 0);
        let rows = store.list_all().await.unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name, "Tim");
        assert_eq!(rows[0].consent, "full");
    }

    #[tokio::test]
    async fn export_json_round_trips() {
        let store = FeedbackStore::open_in_memory().unwrap();
        store.insert(&fixture()).await.unwrap();
        let json = export_json(&store.list_all().await.unwrap());
        assert!(json.contains("\"name\""));
        assert!(json.contains("Tim"));
        assert!(json.contains("\"consent\""));
        // Round-trip via serde_json::Value
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(v.is_array());
    }

    #[tokio::test]
    async fn export_csv_quotes_cells_with_commas_and_newlines() {
        let store = FeedbackStore::open_in_memory().unwrap();
        let mut payload = fixture();
        payload.recommend = "yes, with caveats";
        payload.anything_else = "line1\nline2";
        store.insert(&payload).await.unwrap();
        let csv = export_dsv(&store.list_all().await.unwrap(), ',');
        assert!(csv.contains("\"yes, with caveats\""));
        assert!(csv.contains("\"line1\nline2\""));
    }

    #[tokio::test]
    async fn export_tsv_uses_tabs() {
        let store = FeedbackStore::open_in_memory().unwrap();
        store.insert(&fixture()).await.unwrap();
        let tsv = export_dsv(&store.list_all().await.unwrap(), '\t');
        assert!(tsv.contains("\tname\t"));
        assert!(tsv.contains("\tTim\t"));
    }
}
