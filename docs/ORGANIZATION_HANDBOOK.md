# cliptown organization handbook

> Shared operating defaults for repositories maintained under **cliptown**. Repository-local policy may strengthen these rules but should not silently weaken them.

## Mission

cliptown maintains clipboard, content-capture, pinning, storage, and synchronization software across devices. This `.github` repository is the canonical home for organization-wide community health files, reusable templates, engineering policy, and planning links.

## Repository contract

Each active repository must document purpose, ownership, maturity, supported platforms, development and test commands, authoritative data and sync formats, release and rollback procedures, compatibility policy, and GitHub Project/Linear links. Capture and sync components should also document encryption boundaries, retention, conflict resolution, offline behavior, ordering, deduplication, attachment limits, provider constraints, and deletion semantics.

## Change and review workflow

1. Anchor work in an issue, Linear item, or documented maintenance objective.
2. Keep branches and pull requests focused.
3. Explain motivation, scope, data and privacy impact, validation, compatibility, migration, and rollback.
4. Test offline, reconnect, conflict, duplicate, deletion, large-item, permission, and partial-failure paths as relevant.
5. Resolve conflicts semantically by reconstructing both sides' intent.
6. Prefer squash merges for focused work unless commit structure materially improves auditability.

## Evidence and quality

Pull requests should include reproducible commands, synthetic or sanitized fixtures, expected and observed results, negative-path coverage, documentation updates, and CI or local-equivalent evidence. Data-format and sync changes require multi-version and multi-device compatibility analysis.

## Security and data

Never commit credentials, encryption keys, personal clipboard contents, production attachments, or sensitive logs. Use synthetic or irreversibly sanitized fixtures. Follow `SECURITY.md` for private vulnerability reporting and pin dependencies and actions where supply-chain integrity matters.

## Documentation and decisions

Keep examples executable and sanitized, links current, assumptions explicit, and storage and trust boundaries clear. Record encryption, sync, retention, conflict, compatibility, privacy, and operational decisions that future maintainers would otherwise have to rediscover.

## Planning ownership

GitHub owns code, reviews, checks, releases, and delivery evidence. Linear owns priority, dependencies, sequencing, and cross-project planning. The organization GitHub Project is the cross-repository execution view; see `PROJECTS.md` for routing details.

## Organization health

- [ ] Profiles, descriptions, topics, and READMEs are current.
- [ ] Contribution, security, support, governance, issue, and PR guidance is present.
- [ ] Encryption, sync, retention, deletion, and conflict behavior is documented.
- [ ] Required checks reflect privacy, durability, compatibility, and supply-chain risk.
- [ ] Stale repositories are archived or clearly marked.
- [ ] Project links resolve and completed work is reflected in GitHub and Linear.
