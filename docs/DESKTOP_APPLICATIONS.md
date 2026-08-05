# Desktop application allocation

Verified **2026-08-05**.

Cliptown uses the paired native desktop application standard:

- Rust: [`cliptown/cliptown-desktop.rs`](https://github.com/cliptown/cliptown-desktop.rs) — **planned**, not yet verified as a published repository.
- Flutter: [`cliptown/cliptown-flutter`](https://github.com/cliptown/cliptown-flutter) — **live**.

The Rust URL is an allocation target, not proof that the remote exists. Do not mark it live until the repository, native targets, tests, packaging, and supported-platform matrix are verified.

The live Flutter repository records the companion contract in [`COMPANION_DESKTOP.md`](https://github.com/cliptown/cliptown-flutter/blob/main/COMPANION_DESKTOP.md), merged through [PR #6](https://github.com/cliptown/cliptown-flutter/pull/6).

## Product boundary

Both implementations should support semantic parity for clipboard monitoring, tray behavior, global shortcuts, pinned and historical items, search, local storage, offline sync, authentication, cross-device state, notifications, import/export, and recovery.

The Rust and Flutter implementations remain independently buildable, testable, releasable applications. Shared schemas, clients, fixtures, clipboard-item formats, sync contracts, and conformance tests should be versioned deliberately.

## Feature-delivery rule

Every desktop-facing change must inspect both implementations, define shared acceptance criteria, update both or record an explicit no-change rationale, and report Rust and Flutter status separately.

## Project routing

- GitHub Project: [`cliptown-project` — Project 1](https://github.com/orgs/cliptown/projects/1)
- Linear project: `github.com/cliptown`
- Central registry: [`approved-private-registry`](private-registry://canonical/registry/desktop-applications.json)
- Portfolio rollout: [`DEN-2469`](https://linear.app/denman/issue/DEN-2469/roll-out-paired-rust-flutter-desktop-repositories-across-the-portfolio)

Repository creation, renames, transfers, archival, or platform-status changes must update this document, Linear, the central registry, and both companion repositories together.
