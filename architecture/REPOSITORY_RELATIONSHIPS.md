# `cliptown` repository relationships

Generated from reviewed policy and the current **public** repository inventory.

- Public repositories declared: **11**
- Private repository names withheld: **1**
- Relationship edges: **33**

## Repository roles

| Repository | Role | Lifecycle |
|---|---|---|
| [`.github`](https://github.com/cliptown/.github) | `organization_governance` | `active` |
| [`cliptown-interfaces`](https://github.com/cliptown/cliptown-interfaces) | `interfaces` | `active` |
| [`cliptown-clients`](https://github.com/cliptown/cliptown-clients) | `client_sdk` | `active` |
| [`cliptown-rust-backend.rs`](https://github.com/cliptown/cliptown-rust-backend.rs) | `api_service` | `active` |
| [`cliptown-flutter`](https://github.com/cliptown/cliptown-flutter) | `application` | `active` |
| [`cliptown-extension`](https://github.com/cliptown/cliptown-extension) | `browser_extension` | `active` |
| [`cliptown-cli`](https://github.com/cliptown/cliptown-cli) | `cli` | `active` |
| [`cliptown.github.io`](https://github.com/cliptown/cliptown.github.io) | `site` | `active` |
| [`cliptown-infra`](https://github.com/cliptown/cliptown-infra) | `infrastructure` | `active` |
| [`cliptown-monorepo`](https://github.com/cliptown/cliptown-monorepo) | `composition_workspace` | `active` |
| [`homebrew-cliptown`](https://github.com/cliptown/homebrew-cliptown) | `uncategorized` | `active` |

## Declared edges

| From | Relationship | To | Status/basis |
|---|---|---|---|
| `cliptown/.github` | `governs` | `cliptown/cliptown-cli` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `cliptown/.github` | `governs` | `cliptown/cliptown-clients` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `cliptown/.github` | `governs` | `cliptown/cliptown-extension` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `cliptown/.github` | `governs` | `cliptown/cliptown-flutter` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `cliptown/.github` | `governs` | `cliptown/cliptown-infra` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `cliptown/.github` | `governs` | `cliptown/cliptown-interfaces` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `cliptown/.github` | `governs` | `cliptown/cliptown-monorepo` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `cliptown/.github` | `governs` | `cliptown/cliptown-rust-backend.rs` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `cliptown/.github` | `governs` | `cliptown/cliptown.github.io` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `cliptown/.github` | `governs` | `cliptown/homebrew-cliptown` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `cliptown/cliptown-cli` | `calls` | `cliptown/cliptown-rust-backend.rs` | `inferred` / `role-convention`: client uses the product service boundary |
| `cliptown/cliptown-clients` | `generated_from` | `cliptown/cliptown-interfaces` | `inferred` / `role-convention`: SDK bindings derive from canonical contracts |
| `cliptown/cliptown-extension` | `calls` | `cliptown/cliptown-rust-backend.rs` | `inferred` / `role-convention`: client uses the product service boundary |
| `cliptown/cliptown-flutter` | `calls` | `cliptown/cliptown-rust-backend.rs` | `inferred` / `role-convention`: client uses the product service boundary |
| `cliptown/cliptown-infra` | `deploys` | `cliptown/cliptown-cli` | `inferred` / `role-convention`: product infrastructure declares runtime resources |
| `cliptown/cliptown-infra` | `deploys` | `cliptown/cliptown-extension` | `inferred` / `role-convention`: product infrastructure declares runtime resources |
| `cliptown/cliptown-infra` | `deploys` | `cliptown/cliptown-flutter` | `inferred` / `role-convention`: product infrastructure declares runtime resources |
| `cliptown/cliptown-infra` | `deploys` | `cliptown/cliptown-rust-backend.rs` | `inferred` / `role-convention`: product infrastructure declares runtime resources |
| `cliptown/cliptown-monorepo` | `composes` | `cliptown/cliptown-cli` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `cliptown/cliptown-monorepo` | `composes` | `cliptown/cliptown-clients` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `cliptown/cliptown-monorepo` | `composes` | `cliptown/cliptown-extension` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `cliptown/cliptown-monorepo` | `composes` | `cliptown/cliptown-flutter` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `cliptown/cliptown-monorepo` | `composes` | `cliptown/cliptown-infra` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `cliptown/cliptown-monorepo` | `composes` | `cliptown/cliptown-interfaces` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `cliptown/cliptown-monorepo` | `composes` | `cliptown/cliptown-rust-backend.rs` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `cliptown/cliptown-monorepo` | `composes` | `cliptown/cliptown.github.io` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `cliptown/cliptown-monorepo` | `composes` | `cliptown/homebrew-cliptown` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `cliptown/cliptown-rust-backend.rs` | `implements_contracts_from` | `cliptown/cliptown-interfaces` | `inferred` / `role-convention`: service boundary implements canonical contracts |
| `organization://cliptown` | `coordinates_via` | `capability://fiducia-cloud/distributed-coordination` | `platform-default` / `explicit-platform-decision`: locks, leases, idempotency, elections, schedules, budgets, and task claims |
| `organization://cliptown` | `authenticates_via` | `capability://shared-auth/human-identity` | `platform-default` / `explicit-platform-decision`: platform human identity and session authority |
| `organization://cliptown` | `uses_capability` | `organization://3FA-app` | `declared` / `explicit-product-decision`: trusted-device and step-up authentication |
| `organization://cliptown` | `deployed_via` | `platform://ORESoftware/k8s-cluster` | `platform-default` / `platform-policy`: immutable artifacts are promoted by digest through GitOps |
| `organization://cliptown` | `packaged_via` | `platform://zed-pkg` | `platform-default` / `platform-policy`: Zed resolves artifacts while submodules compose editable source |

## Composition, service, and observability contract

Git submodules compose editable source; Zed packages resolve packages/artifacts; dual-managed commits must match. Production deploys immutable image digests, not runtime source builds. Cross-service access uses APIs/SDKs/events rather than another service database. MCP uses the product API/SDK. Services emit OpenTelemetry traces, bounded metrics, and correlated structured logs.

## Privacy boundary

This public registry deliberately omits private repository names and edges; the count above makes the boundary explicit.
