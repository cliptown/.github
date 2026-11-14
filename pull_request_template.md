## Change summary

Describe the user-visible behavior, repositories/components affected, compatibility impact, and rollback path. Mark non-applicable checks as `N/A` with a reason.

## Review gates

### Change control and dependencies

- [ ] This work is on a topic branch; no direct default-branch commit is required.
- [ ] Cross-repository dependencies are pinned by immutable commit, lockfile, or released Zed package.
- [ ] Public contracts are generated from the canonical interface/schema source and consumer compatibility was checked.
- [ ] Breaking changes include migration, rollback, and staged rollout notes.

### SQL, persistence, and state

- [ ] No SQL changes, or every declaration has a stable organization/domain namespace and an explicit owning repository.
- [ ] Domain SQL may remain with its owning org, but identity, ordering, checksums, drift detection, and promotion are registered through `declarative-migrations`.
- [ ] Application startup validates schema compatibility and does not apply production DDL.
- [ ] Destructive changes, tenant isolation, RLS/authorization, idempotency, and state-machine invariants have evidence.

### Infrastructure and security

- [ ] Application manifests remain app-owned; cluster composition is delegated to `oresoftware/k8s-cluster` and shared components to `oresoftware/k8s-libs-and-shared-defs`.
- [ ] Workloads use least privilege, restricted pod security, explicit network policy, non-root execution, immutable images, and bounded resources where applicable.
- [ ] Secrets, credentials, personal data, and user content are excluded from source, logs, fixtures, and build artifacts.
- [ ] Authentication/authorization failures are fail-closed and sensitive operations are auditable.

### Verification and observability

- [ ] Zed lifecycle hooks run deterministic format, lint, build, contract, and publish checks.
- [ ] Unit, integration, adversarial, migration, and end-to-end tests cover the changed behavior in the appropriate test organization.
- [ ] ORES OTEL trace/correlation propagation is present where applicable, with secret and user-content capture disabled by default.
- [ ] Test evidence, residual risks, follow-up work, and any intentionally deferred repositories are listed below.

## Validation evidence and residual risk

Provide commands, checks, fixtures, test-org run links, migration/drift results, and known limitations.

## Salvage check

If this PR supersedes or replaces an older one, say which, and name at least one
concrete thing carried forward from it (a test, a fixture, an error message, a
pin, a doc paragraph). See [`docs/pr-salvage-policy.md`](../docs/pr-salvage-policy.md).

- [ ] Supersedes nothing, **or** the salvaged item is named above.

## Repository boundaries and detailed review evidence

- [ ] All commits are on a non-default branch and this pull request is the only proposed path into the default branch.
- [ ] No generated tool, bot, migration runner, or deployment process writes directly to `main`, `master`, or another protected default branch.
- [ ] The change is small enough to review, or its staged rollout and follow-up pull requests are identified.
- [ ] The change is focused and does not silently cross repository ownership boundaries.
- [ ] No `*-infra` repository is introduced as a Git submodule under `*-monorepo/apps`.
- [ ] Public contracts, compatibility, rollback, telemetry, and failure behavior are documented.
- [ ] Shared functionality is imported from its owning repository rather than copied into a new local implementation.
- [ ] If SQL changes, declarations use the registered logical namespace `<organization>.<domain>` and stable `<domain>_` object prefixes where a shared PostgreSQL schema such as `public` is required.
- [ ] Domain SQL may remain in the owning organization, but identity, ordering, checksums, drift detection, and promotion are registered through `declarative-migrations`.
- [ ] JSON Schema, generated language interfaces, ORM models, fixtures, and migration declarations were updated and checked deterministically together.
- [ ] Destructive changes include compatibility, backfill, rollback, tenant isolation, and row-level-security evidence.
- [ ] Kubernetes application manifests compose through `oresoftware/k8s-cluster` and reuse `oresoftware/k8s-libs-and-shared-defs`; application repositories do not become a second cluster control plane.
- [ ] Workload identity, restricted Pod Security, default-deny networking, explicit egress, probes, resources, secret handling, and immutable image/dependency references were considered.
- [ ] Destructive and cross-runtime tests run in the corresponding `*-test` organization or an isolated e2e environment, with teardown evidence.
- [ ] Zed lifecycle hooks cover the relevant pre-build, pre-test, and pre-publish checks without bypassing local language-native validation.
- [ ] No credentials, customer data, private-repository inventory, or sensitive telemetry is included.
- [ ] Conflicts were resolved semantically using both sides and relevant history.
- [ ] Destructive Git recovery, force pushes to protected branches, and history rewrites were not used.
- [ ] Logs and traces exclude secrets and user content by default and preserve tenant boundaries.
