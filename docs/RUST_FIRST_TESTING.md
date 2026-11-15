# Rust-first scripts and tests: ClipTown execution note

**Policy:** New first-party test harnesses, CI validation utilities, generated-source checks, scripts, maintenance tasks and migrations must be implemented in **Rust**, not Python. Use ordinary Rust `#[test]` suites and a local `cargo xtask` or `tools/<name>/Cargo.toml` where appropriate. Thin shell and YAML wrappers may invoke Rust but must not embed Python code. Existing Python product implementations are separate from this rule and need a documented exception.

## Current AttoLang rollout

- [cliptown-clients #24](https://github.com/cliptown/cliptown-clients/pull/24): AttoLang `.atto` SDK target and canonical contract receipts. Rust should validate the source, manifests and negative canaries.
- [cliptown-interfaces #44](https://github.com/cliptown/cliptown-interfaces/pull/44): closed local pre-encryption `.atto` JSON Schema. Rust should test valid, invalid, unexpected-field and size boundary cases.
- [cliptown-e2e #24](https://github.com/cliptown/cliptown-e2e/pull/24): pin the exact client and interface source commits and run independent cross-repo Rust checks.

## Migration rule

Keep existing Python validators and their CI jobs enabled until an equivalently strict Rust implementation is validated against **the same fixtures and intentional negative mutations**. Source hash fingerprints, schema and package/version assumptions, expected errors/exit codes, symlink/path escape guards, and multi-platform CI behavior must be preserved. Migrate one workflow/caller at a time with a reviewable PR; never bypass a red gate or delete safety coverage simply to comply with this style policy.

Run the Rust tests against exact PR heads; do not substitute a passing test from a different commit. Maintain [cliptown/.github #22](https://github.com/cliptown/.github/pull/22) and [issue #20](https://github.com/cliptown/.github/issues/20) as the policy rollout record. GitHub's organization community-health fallback does not automatically enforce agent instructions or new workflows in sibling repositories.
