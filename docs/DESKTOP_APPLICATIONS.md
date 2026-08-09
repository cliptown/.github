# Desktop application allocation

Verified **2026-08-06**.

Cliptown uses the paired desktop application standard:

- Rust: [`cliptown/cliptown-desktop.rs`](https://github.com/cliptown/cliptown-desktop.rs) — **planned**, not yet verified as a published repository.
- Flutter: [`cliptown/cliptown-flutter`](https://github.com/cliptown/cliptown-flutter) — **live**.

The Rust URL is an allocation target, not proof that the remote exists. Do not mark it live until the repository, native targets, tests, packaging, and platform matrix are verified.

## Why both Rust and Flutter remain active

The two applications are first-class, side-by-side product implementations. They exist to compare native performance, memory use, clipboard/tray integration, accessibility, cross-platform consistency, developer velocity, Flutter mobile reuse, release engineering, and long-term maintenance with real feature work.

Every desktop-facing feature must inspect both repositories, use shared acceptance criteria and fixtures, and normally update both. A one-sided change requires a no-change rationale and recorded parity gap. Neither app may be neglected while the comparison program is active.

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

Both implementations should support semantic parity for clipboard monitoring, tray behavior, global shortcuts, pinned and historical items, search, local storage, offline sync, authentication, cross-device state, notifications, import/export, deep links, and recovery.

Shared schemas, clients, route fixtures, clipboard-item formats, sync contracts, and conformance tests must be versioned deliberately.

## Repository-local documentation

The live Flutter repository records the companion contract in [`COMPANION_DESKTOP.md`](https://github.com/cliptown/cliptown-flutter/blob/main/COMPANION_DESKTOP.md), introduced through [PR #6](https://github.com/cliptown/cliptown-flutter/pull/6).

Central toolkit assignments: [`approved-private-registry`](private-registry://canonical/registry/rust-desktop-strategies.md).

## Project routing

- GitHub Project: [`cliptown-project` — Project 1](https://github.com/orgs/cliptown/projects/1)
- Linear project: `github.com/cliptown`
- Central registry: [`approved-private-registry`](private-registry://canonical/registry/desktop-applications.json)
- Portfolio rollout: [`DEN-2469`](https://linear.app/denman/issue/DEN-2469/roll-out-paired-rust-flutter-desktop-repositories-across-the-portfolio)

Repository creation, renames, toolkit changes, deep-link changes, transfers, archival, or platform-status changes must update this document, Linear, the central registry/strategy, and both companion repositories together.
