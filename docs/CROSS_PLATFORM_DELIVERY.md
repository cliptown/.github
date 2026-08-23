# Cross-platform application delivery contract

Updated **2026-08-22**.

This is the organization-level acceptance contract for ClipTown applications.
It states what must be proven before a platform is called shipped. It does not,
by itself, prove that a current branch, store build, or production service has
passed those gates.

## Products and supported targets

| Product | Required targets | Relationship |
|---|---|---|
| [`cliptown-flutter`](https://github.com/cliptown/cliptown-flutter) | Windows, macOS, Linux, Android, iOS | Desktop and mobile product |
| [`cliptown-desktop.rs`](https://github.com/cliptown/cliptown-desktop.rs) | Windows, macOS, Linux | Independent native Rust/GPUI desktop product |
| [`cliptown-e2e`](https://github.com/cliptown/cliptown-e2e) | Windows, macOS, Linux plus mobile journeys where applicable | Cross-product acceptance evidence |

The desktop products are perpetual peers. They share externally observable
contracts and fixtures but not source, UI architecture, databases, encryption
keys, packaging, or release lifecycles. A green Flutter suite cannot stand in
for Rust evidence, and a green Rust suite cannot stand in for Flutter evidence.

## Pull-request and exact-head CI

GitHub Actions, or an equivalently reviewable hosted runner, must prove at the
exact proposed commit:

1. formatting, static analysis, dependency/security policy, and unit tests;
2. Flutter builds for Windows, macOS, Linux, Android, and iOS;
3. Rust builds for Windows, macOS, and Linux without a WebView;
4. app-specific installed-app journeys and crash/hang diagnostics;
5. paired desktop contract journeys on Windows, macOS, and Linux using immutable
   revisions and the same versioned fixtures;
6. artifacts named with commit identity and retained long enough for review;
7. immutable action/source pins, least-privilege permissions, timeouts, and
   concurrency cancellation; and
8. a precise waiver when hosted automation cannot reproduce a real-device,
   permission, signing, tray, shortcut, share-extension, or store path.

At minimum the installed-app journeys cover first launch, database creation,
clipboard permission behavior, text/rich-text/image/file capture, deduplication,
configurable retention, pin exemption, lexical/vector search, copy/paste,
restart persistence, tray/single-instance behavior, global shortcuts, and clean
shutdown. Crash artifacts include process exit status, application logs, and
platform crash reports when available.

Mobile tests must respect OS constraints. Android and iOS share/import,
extension, foreground, and user-initiated flows are tested as real platform
flows; CI must not claim unrestricted background clipboard monitoring when the
OS forbids it. Emulator/simulator coverage is necessary but does not replace
release-candidate validation on at least one supported physical device per OS.

## Storage and search acceptance

- Text, HTML/rich text, PNG/images, and file lists are indexed locally; private
  absolute paths must not enter telemetry, URLs, or cloud metadata.
- The saved unpinned-item count is configurable and bounded. Pinned items are
  exempt from ordinary retention but remain explicitly deletable.
- Each app uses its own encrypted SQLite database and platform credential store.
  Missing or incorrect keys fail closed; plaintext SQLite headers and known
  clipboard markers are rejected in at-rest tests.
- Text embeddings use a versioned model identifier and fixed dimensions and are
  stored/searched in local SQLite. Lexical search remains available if vector
  generation is unavailable.
- Opted-in embedding backup is device-encrypted. PostgreSQL and CockroachDB store
  ciphertext and bounded dimensions, never searchable plaintext vectors.
- Images and files are device-encrypted into bounded chunks before Cloudflare R2.
  Relational databases store encrypted manifests, randomized storage keys,
  sizes, digests, lifecycle state, and provider receipts—not local paths, object
  plaintext, content keys, or R2 credentials.

The primary application schema is PostgreSQL/Supabase. The portable encrypted
backup schema is owned by
[`ORESoftware/k8s-libs-and-shared-defs`](https://github.com/ORESoftware/k8s-libs-and-shared-defs)
and must converge on real PostgreSQL and CockroachDB instances through an exact
revision of
[`declarative-postgres-migrate`](https://github.com/declarative-migrations/declarative-postgres-migrate.rs).
Applications do not run migration DDL at startup.

R2 is not considered live until a disposable-bucket canary proves encrypted
upload, download, digest verification, retry/idempotency, delete/lifecycle
cleanup, revocation, and redacted logs through the authenticated production
adapter. Schema and fake-adapter tests alone are contract evidence, not a cloud
deployment.

## Competitive parity discipline

ClipTown benchmarks a representative eight-product set: Paste, CopyQ, Ditto,
Maccy, Raycast Clipboard History, PastePal, Pastebot, and Microsoft SwiftKey.
This is a comparison set, not an objective ranking. The detailed capability and
source matrix lives in
[`cliptown-flutter/docs/competitive-parity.md`](https://github.com/cliptown/cliptown-flutter/blob/main/docs/competitive-parity.md).

The matrix is reviewed at least quarterly and whenever a benchmark product has
a material release. A feature is marked complete only with executable ClipTown
evidence on every applicable platform. The roadmap must explicitly track gaps
such as OCR/QR processing, mobile extensions, shared collections/pinboards,
advanced transformations, sequential paste, accessibility, sync conflict
handling, and platform-specific clipboard formats. “Parity” never means copying
another product's branding, proprietary implementation, or unsupported OS claim.

## Release and deployment gates

| Target | Minimum promoted artifact evidence |
|---|---|
| Windows | Signed installer/package, clean install/upgrade/uninstall, Defender/SmartScreen result, installed-app E2E |
| macOS | Signed app, hardened runtime, notarization and stapling, clean install/upgrade, installed-app E2E |
| Linux | Reproducible package(s), dependency manifest/SBOM, install/upgrade on supported distributions, desktop integration E2E |
| Android | Signed AAB/APK from the exact commit, emulator plus physical-device journeys, Play-track or equivalent promotion evidence |
| iOS | Signed archive from the exact commit, simulator plus physical-device journeys, TestFlight/App Store Connect or equivalent promotion evidence |

Backend and storage deployment uses immutable OCI digests and reviewed GitOps
desired state. Database promotion requires reviewed declarative diffs, backup
and roll-forward plans, RLS/security tests, and convergence evidence on every
supported engine. Production cloud credentials belong only in approved secret
delivery and protected environments.

Use these status terms precisely:

- **implemented locally** — code and local tests exist;
- **hosted CI verified** — required checks passed at the exact remote commit;
- **artifact produced** — an immutable output exists, but may be unsigned;
- **release candidate verified** — signed/notarized installed-app and real-device
  gates passed;
- **deployed/shipped** — the reviewed artifact is promoted and externally
  observable through the intended store/channel/environment.

## Change checklist

- [ ] Flutter impact evaluated for Windows, macOS, Linux, Android, and iOS.
- [ ] Rust desktop impact evaluated for Windows, macOS, and Linux.
- [ ] Shared fixture and paired-desktop impact evaluated.
- [ ] Text/image/file, retention, pin, and lexical/vector-search behavior tested.
- [ ] Local encrypted SQLite/key-store behavior tested independently in both apps.
- [ ] PostgreSQL/CockroachDB/R2 contract and migration impact evaluated.
- [ ] Exact-head hosted CI and artifacts inspected; missing evidence is named.
- [ ] Signing, notarization, store, real-device, accessibility, and rollback or
  roll-forward evidence recorded for a release.
- [ ] Competitive gaps have explicit owners and Linear follow-up work.

Planning belongs in the
[`github.com/cliptown` Linear project](https://linear.app/denman/project/githubcomcliptown-adf62fab3f42);
GitHub pull requests, checks, artifacts, releases, and deployment attestations
remain the authoritative execution evidence.
