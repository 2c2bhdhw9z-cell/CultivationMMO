# Cultivation MMO

A provisional game vision and Kiro specification for a third-person, open-world cultivation MMO.

The project begins with a small playable iPhone build and is designed to grow over time. Every design decision is changeable.

> **Public repository:** Anyone can now read and fork this project. That does not grant permission to reuse the original game material or access owner/developer systems. See [`LICENSE`](LICENSE), [`SECURITY.md`](SECURITY.md), and [`docs/PUBLIC_REPOSITORY.md`](docs/PUBLIC_REPOSITORY.md).

## Current implementation

Task 1 (technical baseline) is complete. Task 2 now provides a local foundation:

- Godot client boot shell and headless valley shell
- Shared strict command protocol and authority core
- Local locked API and owner-control service shells
- Dormant maintenance and unconfigured database checkers
- Separate protected owner-console source
- Machine-readable owner-tool and audit contracts

See [`docs/PROJECT_STRUCTURE.md`](docs/PROJECT_STRUCTURE.md) and [`docs/LOCAL_SETUP.md`](docs/LOCAL_SETUP.md).

**Not built yet:** playable character, open world, online accounts/server, Railway, owner login/tools, GitHub workflow, `.ipa`, and ESign installation.

## Planning documents

- [`docs/PROJECT_STRUCTURE.md`](docs/PROJECT_STRUCTURE.md) — current code and component map
- [`docs/LOCAL_SETUP.md`](docs/LOCAL_SETUP.md) — plain-English local verification
- [`docs/GAME_VISION.md`](docs/GAME_VISION.md) — long-term game vision
- [`.kiro/steering/game-principles.md`](.kiro/steering/game-principles.md) — persistent project guardrails
- [`docs/OWNER_OPERATIONS_SECURITY.md`](docs/OWNER_OPERATIONS_SECURITY.md) — owner-only development tools and security
- [`docs/COST_DECISIONS.md`](docs/COST_DECISIONS.md) — current budget and future hosting decisions
- [`docs/TECHNICAL_BASELINE.md`](docs/TECHNICAL_BASELINE.md) — pinned engine, iPhone build, networking, and hosting baseline
- [`docs/CONTROL_PLANE_INVENTORY.md`](docs/CONTROL_PLANE_INVENTORY.md) — active/inactive provider access and security runbooks
- [`docs/TEMPORARY_ASSET_LICENSES.md`](docs/TEMPORARY_ASSET_LICENSES.md) — approved temporary assets and license evidence
- [`docs/OWNER_ACTIONS_CHECK.md`](docs/OWNER_ACTIONS_CHECK.md) — recommended private account-safety check before adding secrets or online deployment
- [`docs/PUBLIC_REPOSITORY.md`](docs/PUBLIC_REPOSITORY.md) — public source and free standard-runner safeguards
- [`THIRD_PARTY_NOTICES.md`](THIRD_PARTY_NOTICES.md) — licenses and notices required in distributed builds
- [`.kiro/specs/cultivation-mmo/requirements.md`](.kiro/specs/cultivation-mmo/requirements.md) — first playable-version requirements
- [`.kiro/specs/cultivation-mmo/design.md`](.kiro/specs/cultivation-mmo/design.md) — first playable-version design
- [`.kiro/specs/cultivation-mmo/tasks.md`](.kiro/specs/cultivation-mmo/tasks.md) — implementation plan
