# Local Foundation Setup

This guide is for Kiro or a future contributor using a development machine. The owner does not need to understand or run these commands on the iPhone.

## Safety first

- This setup uses no Railway project, cloud database, domain, signing key, or paid service.
- Do not add a password, token, certificate, provisioning profile, recovery code, or player data.
- Do not start a GitHub workflow; Task 3 owns the first cloud build.
- All local HTTP shells bind only to `127.0.0.1`.

## Required tools

- Godot `4.7.2-stable`, standard build
- Go `1.25.1`
- A recent Python 3 only for simple JSON/HTML inspection if needed

Official Godot file names and SHA-256 values are in `docs/TECHNICAL_BASELINE.md`. A downloaded file must match before it is used.

## Check the Godot client shell

From the repository root:

```sh
godot --headless --path . --editor --quit
godot --headless --path . -- --self-check
```

Expected final line:

```text
CLIENT_SELF_CHECK_OK protocol=1 mode=offline_skeleton online=locked version=0.0.1-skeleton
```

The normal project opens a status screen. It is not a game world.

## Check the headless zone shell

From the repository root:

```sh
godot --headless --path . server/zone/main.tscn -- --self-check
```

Expected final line:

```text
ZONE_SELF_CHECK_OK protocol=1 duplicate=idempotent actor-cache=bounded utf8=strict duplicate-keys=denied numbers=bounded limits=bounded listener=disabled
```

## Check the Go shells

From the `server` directory:

```sh
go build ./...
go vet ./...
go run ./api --self-check
go run ./owner-control --self-check
go run ./maintenance --self-check
go run ./database --self-check
```

Expected success lines begin with:

- `API_SELF_CHECK_OK`
- `OWNER_CONTROL_SELF_CHECK_OK`
- `MAINTENANCE_SELF_CHECK_OK`
- `DATABASE_SELF_CHECK_OK`

The normal maintenance and database commands intentionally refuse work with exit code 2.

## Optional local HTTP check

Start one process at a time from `server/`:

```sh
go run ./api
go run ./owner-control
```

Use another local process to request `/healthz`, `/readyz`, `/`, and `/owner`. Expected behavior:

- `/healthz`: `200`, local skeleton status
- `/readyz`: `503`, clear not-configured reason
- `/` and `/owner`: generic `404`
- Every response: `Cache-Control: no-store`
- A non-loopback `--listen` value: refused

Stop the process normally. It should shut down without corrupting state because no state exists.

## Generated files

Godot creates `.godot/`. Go may create temporary build files. These are ignored and must not be committed.

To reset the local Godot import cache, remove only `.godot/` and rerun the editor check. Never delete source or documentation as a “reset.”

## Common messages

- `online_authority_not_configured`: expected; no online world exists.
- `identity_and_database_not_configured`: expected; the API is locked.
- `owner_identity_and_durable_audit_not_configured`: expected; owner tools are locked.
- `plans=0`: expected; maintenance cannot run a repair.
- `migrations=0`: expected; no database has been selected.

If a command reports something is ready when its required identity, audit, or database is absent, treat that as a failure.
