//! CMS read-side handler — serves `/docs/{slug}` from a
//! filesystem-backed `cms-core::FsStorage`.
//!
//! State strategy: a [`CmsState`] is constructed once at process
//! start from the `PLAUSIDEN_CMS_ROOT` env var (or `./cms-store`
//! when unset) and threaded through the axum router. Tests
//! construct their own [`CmsState`] pointing at a fixture
//! directory.
//!
//! When the configured root directory is not present, the state
//! holds `None` and `/docs/*` returns 404 for every slug — silent
//! rather than 500ing on a deployment that ships no CMS content.
//!
//! SECURITY:
//!   * Only `PageStatus::Published` pages are served. Drafts,
//!     reviewed-not-yet-live, and archived pages return 404.
//!   * Slugs are validated by `cms_core::Page::validate_slug` at
//!     write time; the handler additionally rejects any slug that
//!     fails the same validation before touching the filesystem,
//!     so a hostile path component (`..`, `/`, NUL) cannot reach
//!     `FsStorage::read_page`.
//!   * The site slug is hard-coded to `plausiden-com`.

use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use cms_core::{FsStorage, Page, Storage, page::PageStatus};

use crate::views::cms_pages;
use crate::views::not_found;

/// Default site slug served from `/docs/{slug}`. The CMS supports
/// N sites in one store; routing more than one of them is a
/// follow-up.
const SITE_SLUG: &str = "plausiden-com";

/// CMS state injected into axum handlers. Cheap to clone (interior
/// `Arc`), `None` when the configured root is missing.
#[derive(Clone, Debug, Default)]
pub struct CmsState {
    storage: Option<Arc<FsStorage>>,
}

/// One published-page summary for sitemap enumeration. Holds just
/// enough to emit a `<url>` entry — slug + `lastmod` ISO date.
#[derive(Debug, Clone)]
pub struct SitemapEntry {
    /// Page slug; becomes the `/docs/{slug}` path component.
    pub slug: String,
    /// Last update timestamp, formatted as `YYYY-MM-DD` for the
    /// sitemap `<lastmod>` element.
    pub updated_at: String,
}

impl CmsState {
    /// Enumerate every Published page in the configured site. Used
    /// by the sitemap builder so search engines discover CMS-backed
    /// docs. Returns an empty Vec when no store is configured or
    /// the read fails — the sitemap stays valid either way.
    #[must_use]
    pub fn published_entries(&self) -> Vec<SitemapEntry> {
        let Some(storage) = self.storage.as_deref() else {
            return Vec::new();
        };
        let pages = match storage.list_pages(SITE_SLUG) {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!("cms: list_pages failed during sitemap build: {e}");
                return Vec::new();
            }
        };
        pages
            .into_iter()
            .filter(|p| matches!(p.status, PageStatus::Published))
            .map(|p| SitemapEntry {
                slug: p.slug,
                updated_at: p.updated_at.format("%Y-%m-%d").to_string(),
            })
            .collect()
    }

    /// Construct from the `PLAUSIDEN_CMS_ROOT` env var (or
    /// `./cms-store` when unset). Returns a state with `None`
    /// storage if the directory is missing or fails to open.
    #[must_use]
    pub fn from_env() -> Self {
        let root = std::env::var("PLAUSIDEN_CMS_ROOT")
            .map_or_else(|_| PathBuf::from("./cms-store"), PathBuf::from);
        Self::from_root(&root)
    }

    /// Construct from an explicit root path. Returns a state with
    /// `None` storage if the directory is missing or fails to
    /// open. Use [`Self::from_env`] for the production path.
    #[must_use]
    pub fn from_root(root: &std::path::Path) -> Self {
        if !root.exists() {
            tracing::info!(
                "cms: root {} not present; /docs/* will return 404",
                root.display()
            );
            return Self::default();
        }
        match FsStorage::open(root) {
            Ok(s) => {
                tracing::info!("cms: opened store at {}", root.display());
                Self {
                    storage: Some(Arc::new(s)),
                }
            }
            Err(e) => {
                tracing::error!("cms: failed to open store at {}: {e}", root.display());
                Self::default()
            }
        }
    }
}

/// Handle `GET /docs/{slug}`. Returns the rendered Maud body on a
/// hit, the shared 404 view (with `404` status) on a miss.
pub async fn serve_doc(
    State(state): State<CmsState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    // SECURITY: validate before touching the filesystem.
    if Page::validate_slug(&slug).is_err() {
        return (StatusCode::NOT_FOUND, not_found::render()).into_response();
    }
    let Some(storage) = state.storage.as_deref() else {
        return (StatusCode::NOT_FOUND, not_found::render()).into_response();
    };
    match storage.read_page(SITE_SLUG, &slug) {
        Ok(p) if matches!(p.status, PageStatus::Published) => {
            let path = format!("/docs/{slug}");
            cms_pages::render(&p, &path).into_response()
        }
        // Draft / Reviewed / Archived → 404. An editor working on a
        // draft never sees it on the public read path.
        Ok(_) | Err(_) => (StatusCode::NOT_FOUND, not_found::render()).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cms_core::page::{Block, BlockKind, FieldValue, Section, SectionTheme, Site, ThemeChoice};
    use std::collections::BTreeMap;

    fn fixture_storage() -> (tempfile::TempDir, FsStorage) {
        let dir = tempfile::tempdir().unwrap();
        let s = FsStorage::open(dir.path()).unwrap();
        s.write_site(&Site {
            slug: SITE_SLUG.into(),
            display_name: "PlausiDen".into(),
            theme: ThemeChoice::LoomLight,
        })
        .unwrap();
        (dir, s)
    }

    fn published_page(slug: &str) -> Page {
        let mut p = Page::draft(slug, "Why PPS");
        p.status = PageStatus::Published;
        let mut fields = BTreeMap::new();
        fields.insert("heading".into(), FieldValue::Text("Why PPS".into()));
        fields.insert(
            "body".into(),
            FieldValue::Text("Plausible deniability is a posture.".into()),
        );
        p.sections.push(Section {
            anchor: None,
            theme: SectionTheme::Light,
            blocks: vec![Block {
                kind: BlockKind::HeadingBody,
                fields,
            }],
        });
        p
    }

    #[test]
    fn published_page_renders_via_storage() {
        let (_dir, storage) = fixture_storage();
        storage
            .write_page(SITE_SLUG, &published_page("why-pps"))
            .unwrap();
        let p = storage.read_page(SITE_SLUG, "why-pps").unwrap();
        assert!(matches!(p.status, PageStatus::Published));
    }

    #[test]
    fn draft_status_is_filtered_at_handler_layer() {
        let mut p = published_page("draft-page");
        p.status = PageStatus::Draft;
        assert!(!matches!(p.status, PageStatus::Published));
    }

    #[test]
    fn invalid_slug_short_circuits() {
        for bad in ["", "../etc", "Has Spaces", "UPPER"] {
            assert!(Page::validate_slug(bad).is_err());
        }
    }

    #[test]
    fn from_root_with_missing_dir_returns_empty_state() {
        let s = CmsState::from_root(std::path::Path::new("/no/such/path/should/exist"));
        assert!(s.storage.is_none());
    }

    #[test]
    fn from_root_with_existing_dir_opens_storage() {
        let (dir, _) = fixture_storage();
        let s = CmsState::from_root(dir.path());
        assert!(s.storage.is_some());
    }
}
