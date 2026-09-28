# Project Structure

This repository now contains a **local foundation**, not a finished game.

```text
CultivationMMO/
├── project.godot                 One shared Godot project
├── client/                       Future iPhone game
│   ├── autoload/                 App-wide state
│   ├── authority/                Offline and future-online command adapters
│   ├── content/                  Future game definitions
│   ├── scenes/                   Boot, character, combat, and world scenes
│   └── shaders/                  Future visual effects
├── server/
│   ├── api/                      Locked local account API shell
│   ├── zone/                     Headless Godot valley shell
│   ├── owner-control/            Locked owner-command gateway
│   ├── maintenance/              Dormant repair-plan executor
│   ├── database/                 Unconfigured migration checker
│   └── internal/                 Shared Go-only safety code
├── owner-console/
│   ├── public/                   Generic Not Found response
│   └── protected/                Phone-shaped source shell, never served yet
├── shared/
│   ├── protocol/v1/              Gameplay command/result/event rules
│   ├── authority/                Shared authoritative Godot logic
│   ├── owner-control/v1/         Empty locked privileged registry
│   ├── audit/v1/                 Independent audit contract
│   ├── maintenance/v1/           Empty reviewed repair-plan registry
│   └── operations/v1/            Machine-readable feature-control coverage
└── docs/                         Vision, security, costs, setup, and decisions
```

## Why one Godot project sits at the root

The iPhone client and headless zone need to use the same command parser and authority rules. A root `project.godot` lets both load code under `shared/` without copying it.

The client starts at `client/scenes/ui/boot.tscn`. The headless zone starts from `server/zone/main.tscn`. Future export presets will select only the correct scenes/resources for each package.

Go and owner-console directories contain `.gdignore` markers so Godot does not import them. Task 3 must still inspect the exported `.ipa` and prove it contains no server, owner-console, developer UI, or private file.

## Current behavior

- The client shows a plain status screen and performs one harmless local ping.
- The local ping passes through the strict V1 command codec and shared authority core.
- The future WSS adapter returns `online_authority_not_configured`.
- The zone has no listener and changes no state.
- The account API and owner-control shells bind only to loopback.
- Account/world/owner routes return generic `404` responses.
- Owner-control has no owner identity, command, or durable audit sink.
- Maintenance has no listener, credential, SQL, shell input, or executable plan.
- Database tooling has no driver, connection, version, schema, or migration.
- The protected console source is never served.

## What is not complete

There is no playable character, open valley, combat, cultivation, inventory, account, online server, PostgreSQL database, Railway project, owner login, developer command, GitHub workflow, `.ipa`, or ESign result.

Those remain later tasks. This skeleton exists to prevent them from being built with the wrong trust boundaries.
