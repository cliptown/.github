# DEN-2259 — MemeBank transfer production activation

Linear `DEN-2259` is the authoritative cross-organization delivery issue. This document maps the ClipTown-owned GitHub work to that issue and to the dependent shared-auth and MemeBank workstreams.

## ClipTown responsibility

ClipTown owns the versioned subject-owned transfer resource, official API contract and SDK behavior, authorization at the resource boundary, PostgreSQL persistence and RLS, idempotency, transfer state, deployment configuration, and operational readiness.

The canonical integration is HTTPS API plus official SDKs. It must work from web, desktop, CLI, worker, and server callers when neither phone app is installed. Native clipboard Copy remains an independent foreground feature and is never an authentication mechanism, availability probe, or fallback integration transport.

## Active GitHub delivery chain

| Order | Repository item | Purpose | Merge gate |
|---:|---|---|---|
| 1 | `shared-auth/shared-auth-server.rs#38` | Expose current delegated `jti` alongside `parent_jti` through protected exact-audience introspection. | Server format, Clippy, all-target tests, locked release build; independent service credential remains mandatory. |
| 2 | `shared-auth/shared-auth-clients#34` | Add exact-audience protected introspection and complete delegated claim decoding to the official Rust SDK. | SDK format, Clippy, tests, request tuple and credential-isolation evidence. |
| 3 | `cliptown/cliptown-rust-backend.rs#8` | Mount create/list/get/acknowledge/cancel; apply shared-auth policy; persist with SeaORM/PostgreSQL; add readiness and headless E2E. | Rust format, Clippy, all-target tests, PostgreSQL 17 headless flow, release build, Postgres-only graph. |
| 4 | `cliptown-test/memebank-image-interop#7` | Qualify the exact backend head on a dedicated public test runner. | Immutable source SHAs, no production secrets, complete green evidence. |
| 5 | `memebank/mbk-rest-api#7` | Compose the official shared-auth and ClipTown SDKs in the active MemeBank Go service and pin reviewed releases. | Go race/vet/tests, no direct 3FA dependency, no raw ClipTown transport, no-phone-app E2E. |

The existing tracker for the ClipTown backend is `cliptown/cliptown-rust-backend.rs#7`.

## Authorization invariants

Every request must fail closed unless protected shared-auth verification provides:

- the configured issuer;
- sole audience `cliptown-api`;
- authorized party `memebank-api`;
- an active revocation-aware session;
- a non-empty current `jti` and a distinct `parent_jti`;
- exactly one operation-appropriate `cliptown:memebank:*` scope;
- valid not-before, expiry, and delegated lifetime;
- recent LOA2 for write and delete operations.

ClipTown does not call 3FA, identify the factor application, accept a 3FA proof/header/bearer, or inspect app installation. Passkey, TOTP, OTP, and compatible 3FA-imported ceremonies reach ClipTown only as normalized shared-auth assurance.

## Data and concurrency invariants

- Transfer rows are subject-owned and ciphertext-only.
- Transaction-local RLS context and explicit `subject_id` predicates are both required.
- Cross-subject identifiers return not found.
- Create and acknowledgement idempotency bind subject, normalized route, operation, canonical digest, and expiry.
- Concurrent reuse of one key is serialized with a PostgreSQL transaction advisory lock.
- Identical replay returns the prior result; digest mismatch returns conflict.
- Terminal states cannot be reopened.
- Schema application is deployment-controlled; the API never auto-runs the declarative schema at startup.

## GitHub Project mapping

Add every active item above to the ClipTown organization project using these fields:

- **Linear**: `DEN-2259`
- **Workstream**: `MemeBank interoperability`
- **Organization**: owning GitHub organization
- **Repository**: exact repository name
- **Status**: Backlog / In progress / In review / Qualified / Merged / Released / Deployed
- **Environment**: none / test / staging / production
- **Evidence**: link to the exact green workflow run or release
- **Blocked by**: upstream PR or issue URL

Do not mark the backend item Qualified from a queued job, a job with zero steps, or a formatting-only run. Do not mark it Deployed until the reviewed schema, exact delegation policy, independent introspection credential, readiness, and non-production canary are confirmed.

## Completion and rollback

Completion requires merged/released shared-auth contracts, a merged ClipTown backend, official MemeBank SDK composition, a headless no-phone-app flow, and staging canary evidence. Rollback is an application deployment rollback while retaining rows and idempotency records; it must not weaken RLS/scopes, accept direct factor artifacts, drop tables, or introduce a local-app/clipboard bridge.
