# Desktop application allocation

Updated **2026-08-22**. Repository and branch existence were verified on that
date; merge, signed-release, store, and live-service status must still be proven
at the exact commit being promoted.

Cliptown uses the paired desktop application standard:

- Rust: [`cliptown/cliptown-desktop.rs`](https://github.com/cliptown/cliptown-desktop.rs) — published, native GPUI/no-WebView desktop product.
- Flutter: [`cliptown/cliptown-flutter`](https://github.com/cliptown/cliptown-flutter) — published Flutter desktop and mobile product.

Both are active products and will be developed perpetually side by side. Neither
is a prototype, temporary fallback, rewrite destination, or replacement for the
other. A repository existing is not proof of feature parity, packaging, or a
production deployment.

## Why both Rust and Flutter remain active

The two applications are first-class, side-by-side product implementations. They exist to compare native performance, memory use, clipboard/tray integration, accessibility, cross-platform consistency, developer velocity, Flutter mobile reuse, release engineering, and long-term maintenance with real feature work.

Every desktop-facing feature must inspect both repositories, use shared
acceptance criteria and fixtures, and normally update both. A one-sided change
requires a no-change rationale and recorded parity gap. App-specific tests stay
independent; a separate paired suite must run both exact revisions against the
same fixtures on Windows, macOS, and Linux.

## Rust desktop kit: GPUI

**Selected strategy:** GPUI from the Zed project.

**WebView policy:** prohibited.

Cliptown must be fully native. GPUI is selected for a keyboard-first productivity UI, large virtualized clipboard histories, fast custom rendering, low-latency interaction, native windows, and tight tray/global-shortcut behavior without embedding a browser engine.

The Rust repository must maintain `docs/DESKTOP_TOOLKIT.md` covering the GPUI version policy, platform adapters, clipboard/tray/global-shortcut boundaries, deep links, tests, packaging, and the Flutter companion. If GPUI lacks a required target capability, a toolkit change requires an ADR; silently adding Tauri, Dioxus, or any other WebView is prohibited.

## HTTPS-first deep linking

Canonical form:

```text
https://<verified-cliptown-owned-host>/open/<route>?<bounded-query>
```

Fallback scheme:

```text
cliptown://<route>?<bounded-query>
```

The same route type and fixtures must be implemented in `cliptown-interfaces`, the Rust app, the Flutter app, and the browser fallback.

Required behavior:

- support cold start and already-running/single-instance delivery;
- validate the exact host, route, item/workspace identifiers, action, and bounded query parameters;
- treat links as untrusted input;
- never put clipboard contents, authentication tokens, encryption keys, private text, files, or personal data in a URL;
- use one-time, short-lived codes for share/import handoffs;
- require explicit confirmation before importing or opening externally supplied clipboard data; and
- test macOS, Windows, Linux, Android, and iOS app/universal links plus browser fallback.

GPUI receives OS URL events through narrow platform modules and forwards only validated routes into application state.

## Product boundary

Both implementations must support semantic parity for text, rich text, images,
and file lists; automatic clipboard monitoring; configurable unpinned history
retention; pinning; lexical and vector search; encrypted local SQLite storage;
offline behavior; tray and global-shortcut access; deduplication; secure sync;
authentication; cross-device state; notifications; import/export; deep links;
and recovery. Platform limitations must be explicit rather than simulated.

Each app owns a separate SQLite database and local index. Text embeddings are
fixed-width vectors stored and searched locally in SQLite. Opted-in cloud backup
encrypts the model identifier and vector on the device before PostgreSQL or
CockroachDB sees them. Image and file bytes are encrypted on the device before
Cloudflare R2; object keys are randomized and never contain a local path or a
plaintext hash.

Shared schemas, clients, route fixtures, clipboard-item formats, sync contracts, and conformance tests must be versioned deliberately.

## Required platform evidence

- Flutter: Windows, macOS, Linux, Android, and iOS builds and automated tests.
- Rust: Windows, macOS, and Linux native builds and automated tests.
- Paired desktop: the same versioned local-history fixture and semantic journeys
  on all three desktop operating systems, with independent database/key stores.
- Installed-app E2E: startup, capture text/image/file data, search, retention,
  pin exemption, restart persistence, single-instance/tray/shortcut behavior,
  and crash diagnostics. Headless storage tests do not replace installed-app E2E.
- Release: immutable exact-commit artifacts plus Windows signing, macOS signing
  and notarization, Linux package verification, Android app signing, and iOS
  signing/TestFlight or store evidence. A build artifact is not a deployment.

The complete gates and honest current-status vocabulary live in
[`CROSS_PLATFORM_DELIVERY.md`](CROSS_PLATFORM_DELIVERY.md).

## Repository-local documentation

The Flutter repository records the companion contract in
[`COMPANION_DESKTOP.md`](https://github.com/cliptown/cliptown-flutter/blob/main/COMPANION_DESKTOP.md).
The cross-implementation suite is owned by
[`cliptown-e2e`](https://github.com/cliptown/cliptown-e2e).

Central toolkit assignments: [`approved-private-registry`](private-registry://canonical/registry/rust-desktop-strategies.md).

## Project routing

- GitHub Project: [`cliptown-project` — Project 1](https://github.com/orgs/cliptown/projects/1)
- Linear project: `github.com/cliptown`
- Central registry: [`approved-private-registry`](private-registry://canonical/registry/desktop-applications.json)
- Portfolio rollout: [`DEN-2469`](https://linear.app/denman/issue/DEN-2469/roll-out-paired-rust-flutter-desktop-repositories-across-the-portfolio)

Repository creation, renames, toolkit changes, deep-link changes, transfers, archival, or platform-status changes must update this document, Linear, the central registry/strategy, and both companion repositories together.
