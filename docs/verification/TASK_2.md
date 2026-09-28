# Task 2 Verification Record

> **Date:** September 28, 2026
> **Scope:** Local project skeleton only

## Result

**Passed after independent review and correction.** The local client, zone, service, owner-control, maintenance, database, protocol, audit, and console foundations behave as documented.

This does not verify an `.ipa`, ESign, online accounts, hosted multiplayer, PostgreSQL, Railway, WebAuthn, external audit storage, a phone-hosted owner console, or real owner commands.

## Toolchain

- Godot: `4.7.2.stable.official.ed1daf0bf`
- Godot Linux package SHA-256: matched `cadd3204e728a35d3f13adb7fd0d7902636b79f6b95c40c265eb73b6c35329e4`
- Go: `go1.25.1 linux/amd64`
- External Go modules: none

## Passing Godot checks

- Editor/project parse completed with no script error.
- Client: `CLIENT_SELF_CHECK_OK protocol=1 mode=offline_skeleton online=locked version=0.0.1-skeleton`
- Zone: `ZONE_SELF_CHECK_OK protocol=1 duplicate=idempotent actor-cache=bounded utf8=strict duplicate-keys=denied numbers=bounded limits=bounded listener=disabled`

The final zone self-check directly samples and confirms:

- Valid harmless ping and same-actor immediate retry
- Delimiter-containing actor/command pairs do not collide
- One actor cannot evict another actor's cache
- 64-actor capacity, overflow denial, explicit release, and reuse
- 128-byte actor ID limit
- Missing actor denial before cache access
- Nested reserved actor identity denial
- Benign non-empty ping payload denial by the ping handler
- Unknown command denial
- Malformed JSON, trailing comma, invalid UTF-8, literal duplicate key, and escaped duplicate key denial
- Missing field, wrong type, exponent counter, and rounded fractional counter denial
- Unsafe native payload number, lossy near-bound decimal, safe ordinary fraction, depth, collection, UTF-8 string, and total-message limits
- Invalid result status denial
- Locked WSS adapter and displayed build version

Code review additionally confirms the UTF-8 validator covers overlong sequences, bad continuations, surrogate code points, and values above U+10FFFF. Runtime validation—not JSON Schema alone—remains authoritative for byte and nesting limits.

The in-memory idempotency cache is safe only for the current non-mutating ping. Before a valuable command is registered, a durable idempotency ledger must cover same-actor cache eviction, restart, and zone transfer. Before Task 6 enables a listener, the session layer must call `release_actor` on disconnect and expiry.

## Passing Go checks

- `gofmt -d`: no difference
- `go build ./...`: passed
- `go vet ./...`: passed
- Account API: `API_SELF_CHECK_OK ready=false listener=loopback-only`
- Owner control: `OWNER_CONTROL_SELF_CHECK_OK commands=0 features=10 ready=false audit=unavailable strict-json=true`
- Maintenance: `MAINTENANCE_SELF_CHECK_OK plans=0 listener=none credentials=none strict-json=true`
- Database: `DATABASE_SELF_CHECK_OK configured=false migrations=0 connection=none strict-json=true`

The final Go self/negative checks confirm:

- Empty and public bind addresses are rejected before listening.
- Health is sanitized JSON `200`; readiness is JSON `503` with the local reason.
- Unfinished/protected routes are generic `404`.
- No-store, CSP, referrer, content-type protection, and frame-denial headers are checked.
- Registry contains zero commands and every command would need operations coverage.
- Ten unique feature rows contain only local controls or written not-applicable reasons.
- Audit `Health` and `Append` both fail with the stable unavailable result.
- Invalid UTF-8, exact/escaped duplicate keys, case-folded field aliases, case-only semantic duplicates, unknown fields, missing required fields, null required scalar/array values, and omitted required booleans are rejected.
- Local controls require a non-empty method and no non-empty reason; not-applicable controls require an empty method and a written reason.
- Normal maintenance and database invocations refuse work.

## Contract, separation, and repository checks

- Fourteen JSON schema/manifest files parse.
- Privileged command count is zero.
- Executable maintenance plan count is zero.
- Database migration count is zero.
- Two console HTML files parse; no console HTML exists under `client/`.
- Client source contains no owner-control, passkey, recovery, or privileged command material.
- No external Go module or `go.sum` exists.
- No `.env`, workflow, Railway configuration, database driver, external game asset, signing material, or source map exists.
- Generated `.godot/` files are ignored; Godot `.uid` sidecars are retained for stable resource identities.
- Repository visibility remains public.
- GitHub Actions runs and artifacts remain zero.
- No paid or hosted service was activated.
- Source scans found no credential-like value.

## Issues found and fixed before completion

1. Added explicit GDScript types after the strict parser rejected inferred variants.
2. Replaced delimiter-joined/global command caching with bounded per-actor caches and release support.
3. Added recursive reserved identity checks and command-specific empty ping payload validation.
4. Added pre-conversion UTF-8 validation, escaped duplicate-key detection, lexical integer validation, and safe numeric bounds.
5. Aligned language-neutral schemas with recursive payload, numeric, identity, and operations-control rules; documented runtime-only byte/depth enforcement.
6. Replaced permissive Go manifest decoding with exact case-sensitive shape validation, required-field enforcement, duplicate-key/UTF-8 rejection, and semantic validation.
7. Expanded HTTP checks to verify response bodies, content type, and every documented security header; empty bind is denied.
8. Changed startup messages from `READY` to `STARTING` before socket binding.
9. Replaced deferred operations controls with implemented local controls or written current not-applicable reasons.
10. Added actor-cache capacity and required Task 6 disconnect cleanup.

## Test-policy note

No general automated test suite was added. These are local compile, parser, runtime self-check, denial, manifest, and source-boundary checks required to verify the skeleton itself.
