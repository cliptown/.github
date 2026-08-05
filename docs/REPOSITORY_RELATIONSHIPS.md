<!-- ore-org-baseline:begin -->
# Repository relationships for `cliptown`

This file is rendered from `repository-relationships.json`. The JSON registry is authoritative.

- Audience: `public`
- Repositories represented: **11**
- Relationships represented: **13**
- Inventory digest: `sha256:e5a58d5c99db91c0cd3ba6c1424612a89833a81dae492deb2a66a7de8eba7f69`

## Immutable routing identity

| Field | Value |
|---|---|
| Mapping ID | `context:cliptown` |
| GitHub owner ID | `309492430` |
| Linear project ID | `83b03121-db08-4e34-a69f-99fd1c873ced` |
| Linear team ID | `eb8ab169-5afe-4b6f-9cab-3f2aa3e887dc` |

## Repositories

| Repository | Visibility | Roles | Archived |
|---|---|---|---|
| `cliptown/.github` | `public` | `community-health`, `governance`, `relationship-registry` | no |
| `cliptown/cliptown-cli` | `public` | `repository` | no |
| `cliptown/cliptown-clients` | `public` | `clients` | no |
| `cliptown/cliptown-extension` | `public` | `repository` | no |
| `cliptown/cliptown-flutter` | `public` | `repository` | no |
| `cliptown/cliptown-infra` | `public` | `infrastructure` | no |
| `cliptown/cliptown-interfaces` | `public` | `interfaces` | no |
| `cliptown/cliptown-monorepo` | `public` | `monorepo` | no |
| `cliptown/cliptown-rust-backend.rs` | `public` | `backend` | no |
| `cliptown/cliptown.github.io` | `public` | `documentation-site` | no |
| `cliptown/homebrew-cliptown` | `public` | `repository` | no |

## Relationships

| From | Type | To | Status | Required |
|---|---|---|---|---|
| `cliptown/.github` | `governs` | `cliptown/cliptown-cli` | `declared` | yes |
| `cliptown/.github` | `governs` | `cliptown/cliptown-clients` | `declared` | yes |
| `cliptown/.github` | `governs` | `cliptown/cliptown-extension` | `declared` | yes |
| `cliptown/.github` | `governs` | `cliptown/cliptown-flutter` | `declared` | yes |
| `cliptown/.github` | `governs` | `cliptown/cliptown-infra` | `declared` | yes |
| `cliptown/.github` | `governs` | `cliptown/cliptown-interfaces` | `declared` | yes |
| `cliptown/.github` | `governs` | `cliptown/cliptown-monorepo` | `declared` | yes |
| `cliptown/.github` | `governs` | `cliptown/cliptown-rust-backend.rs` | `declared` | yes |
| `cliptown/.github` | `governs` | `cliptown/cliptown.github.io` | `declared` | yes |
| `cliptown/.github` | `governs` | `cliptown/homebrew-cliptown` | `declared` | yes |
| `cliptown/cliptown-clients` | `depends_on` | `cliptown/cliptown-interfaces` | `inferred` | no |
| `cliptown/cliptown-infra` | `deploys` | `cliptown/cliptown-monorepo` | `inferred` | no |
| `cliptown/cliptown.github.io` | `documents` | `cliptown/.github` | `inferred` | no |

## Editing relationships

Put reviewed public declarations in `repository-relationships.manual.json`; do not edit the generated registry directly.
Private repository names and private-only relationships belong in the private `ORESoftware/project-registry` mirror.
Inferred edges are advisory and must remain visibly labeled until reviewed.
<!-- ore-org-baseline:end -->
