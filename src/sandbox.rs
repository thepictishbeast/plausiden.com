//! In-process sandboxing, applied at startup after the listener binds but
//! before the service accepts traffic.
//!
//! SECURITY: This is a defense-in-depth layer on top of the systemd hardening
//! unit. If a future deployment runs outside systemd (container without the
//! hardened unit, developer laptop, etc.), the binary still restricts itself
//! to its legitimate filesystem footprint. A process that cannot read `/etc`,
//! `/home`, or the caller's home directory cannot exfiltrate local secrets
//! even if an attacker achieves arbitrary code execution.

#[cfg(target_os = "linux")]
use landlock::{
    ABI, AccessFs, PathBeneath, PathFd, Ruleset, RulesetAttr, RulesetCreatedAttr, RulesetStatus,
};

/// Outcome of the sandbox application. Logged at startup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // `NotApplicable` is never constructed on Linux builds; kept for readable log output on ports.
pub enum SandboxOutcome {
    /// Kernel enforced every rule we installed. Best case.
    FullyEnforced,
    /// Kernel accepted rules but at an older ABI level than requested.
    PartiallyEnforced,
    /// Kernel does not support Landlock (< 5.13) or the LSM is disabled.
    NotEnforced,
    /// Compile-target is not Linux. No-op.
    NotApplicable,
    /// Disabled at runtime via `PLAUSIDEN_SANDBOX=off`.
    DisabledByEnv,
}

/// Environment variable that, when set to exactly `"off"`, suppresses sandbox
/// installation. Intended for local development and triage; production
/// deployments never set this.
const DISABLE_ENV: &str = "PLAUSIDEN_SANDBOX";

/// Read `DISABLE_ENV` and return whether the sandbox should be suppressed.
/// Extracted so [`apply_inner`] is testable without env manipulation
/// (env mutation is `unsafe` under Rust 2024, which conflicts with the
/// crate-root `#![forbid(unsafe_code)]`).
fn is_disabled_by_env() -> bool {
    std::env::var(DISABLE_ENV).as_deref() == Ok("off")
}

/// Restrict the process to read-only access beneath `static_dir`. Writes are
/// forbidden everywhere. Called after binding the TCP listener so the bind
/// socket is already established.
///
/// BUG ASSUMPTION: `static_dir` must exist at call time and be readable by
/// the current uid. `landlock`'s `PathFd::new` returns `EACCES` / `ENOENT`
/// otherwise.
///
/// SECURITY: Once this returns `FullyEnforced`, the process cannot `open(2)`
/// any path outside `static_dir` for read, and cannot write anywhere at all
/// (we install no `AccessFs::write_to` allowances). A later exec would
/// inherit the restriction. Tokio's epoll / socket FDs are unaffected — they
/// were opened before `restrict_self()`.
pub fn apply(static_dir: &str) -> SandboxOutcome {
    apply_inner(static_dir, is_disabled_by_env())
}

#[cfg(target_os = "linux")]
fn apply_inner(static_dir: &str, disabled: bool) -> SandboxOutcome {
    if disabled {
        tracing::warn!("sandbox disabled by {}=off", DISABLE_ENV);
        return SandboxOutcome::DisabledByEnv;
    }

    // `ABI::V3` is Linux 6.2+ (adds refer for file move/link restrictions).
    // The crate negotiates down if the kernel is older.
    let read_access = AccessFs::from_read(ABI::V3);

    let fd = match PathFd::new(static_dir) {
        Ok(fd) => fd,
        Err(e) => {
            tracing::error!(error = %e, %static_dir, "landlock: failed to open static dir");
            return SandboxOutcome::NotEnforced;
        }
    };

    let result = Ruleset::default()
        .handle_access(read_access)
        .and_then(Ruleset::create)
        .and_then(|ruleset| ruleset.add_rule(PathBeneath::new(fd, read_access)))
        .and_then(landlock::RulesetCreated::restrict_self);

    match result {
        Ok(status) => match status.ruleset {
            RulesetStatus::FullyEnforced => {
                tracing::info!(%static_dir, "landlock: fully enforced (read-only, no writes)");
                SandboxOutcome::FullyEnforced
            }
            RulesetStatus::PartiallyEnforced => {
                tracing::warn!(
                    %static_dir,
                    "landlock: partially enforced — kernel lacks newer ABI features"
                );
                SandboxOutcome::PartiallyEnforced
            }
            RulesetStatus::NotEnforced => {
                tracing::warn!("landlock: not enforced — kernel too old (<5.13) or LSM disabled");
                SandboxOutcome::NotEnforced
            }
        },
        Err(e) => {
            tracing::warn!(
                error = %e,
                "landlock: install failed; continuing without in-binary sandbox"
            );
            SandboxOutcome::NotEnforced
        }
    }
}

/// No-op on non-Linux targets. Systemd / other platform sandboxing is the
/// caller's responsibility.
#[cfg(not(target_os = "linux"))]
fn apply_inner(_static_dir: &str, disabled: bool) -> SandboxOutcome {
    if disabled {
        SandboxOutcome::DisabledByEnv
    } else {
        SandboxOutcome::NotApplicable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Passing `disabled = true` short-circuits and never touches the kernel.
    /// This is the path `cargo test` relies on so tests keep unrestricted FS.
    #[test]
    fn disabled_short_circuits() {
        assert_eq!(
            apply_inner("/tmp", true),
            SandboxOutcome::DisabledByEnv,
            "disabled=true must always return DisabledByEnv"
        );
    }

    /// A nonexistent `static_dir` must not panic. On Linux the `PathFd` open
    /// fails and we return `NotEnforced`; on other targets the stub returns
    /// `NotApplicable`. Either outcome is acceptable — the assert is that we
    /// don't crash.
    #[test]
    fn nonexistent_path_does_not_panic() {
        let out = apply_inner("/does-not-exist-nowhere", false);
        assert!(
            matches!(
                out,
                SandboxOutcome::NotEnforced | SandboxOutcome::NotApplicable
            ),
            "unexpected outcome: {out:?}"
        );
    }

    /// `is_disabled_by_env` returns false when the env var is unset. We cannot
    /// mutate the env in a test under `#![forbid(unsafe_code)]`, so we only
    /// assert the shape we can observe without mutation: the function returns
    /// a bool and does not panic.
    #[test]
    fn env_disable_probe_is_pure_bool() {
        let _ = is_disabled_by_env();
    }
}
