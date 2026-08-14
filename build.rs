//! Emits a build stamp used to cache-bust static assets whose filenames carry
//! no content hash (motion.css, animations.css, menu.js, ...).
//!
//! The static handler serves them `immutable` with a week-long max-age, which
//! is right for the content-hashed Tailwind bundle but would otherwise pin
//! returning visitors to a stale copy of every hand-maintained asset. Changing
//! the query string on each build is what makes a deploy actually reach them.
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    println!("cargo:rustc-env=PLAUSIDEN_BUILD_STAMP={stamp}");
    // Re-run when any hand-maintained asset changes, so a CSS-only edit still
    // produces a fresh stamp.
    println!("cargo:rerun-if-changed=static");
    println!("cargo:rerun-if-changed=build.rs");
}
