# Owner-Only Developer and Operations Plan

> **Status:** Provisional. This document defines the strict current direction: the sole owner must have the tools needed to inspect, diagnose, configure, test, repair, and operate every implemented feature from an iPhone, while no other person can view or execute privileged tools.

## 1. Honest security boundary

No internet-connected system can truthfully promise a zero-percent chance of compromise. The enforceable goal is:

- Only one configured human owner is authorized for developer and operations tools.
- No player, tester, moderator, copied app, modified app, expired session, stolen ordinary account, or guessed URL receives privileged access.
- Ordinary `.ipa` files contain no developer menu or local diagnostic overlay to unlock.
- Server-side checks—not hidden buttons—protect every privileged read and command.
- Machine service identities receive only the narrow access needed to build or run services; they cannot open the human owner console.
- Every privileged attempt, including denied reads, is independently recorded.

If another human ever needs access, the owner must explicitly change the requirements first. Access must never spread automatically from a game role, support role, repository invitation, or player-account recovery.

## 2. Where the tools live

Developer and operations tools live in a separate, phone-friendly web console behind an authentication gateway. They do not live inside the game app.

The game can send normal, privacy-limited diagnostics to the server. The owner views those diagnostics in the console. This allows inspection of frame rate, network delay, collision problems, streaming cells, combat state, and other runtime information without shipping a debug menu to players.

Unauthorized requests receive a generic denial or not-found response before console HTML, JavaScript, source maps, operational names, player data, or menu definitions are sent. Console responses use `Cache-Control: no-store`.

## 3. Owner identity lifecycle

### Initial enrollment

1. A new environment begins with no active owner and all privileged endpoints locked.
2. The deployment creates a high-entropy, single-use enrollment challenge inside the protected secret manager.
3. Enrollment is allowed only while no owner exists, expires quickly, is rate-limited, and is bound to the exact console origin and environment.
4. The owner completes the challenge from the iPhone and registers a phishing-resistant WebAuthn passkey with required user verification.
5. The system creates one immutable owner subject, invalidates the enrollment challenge, records the event externally, and issues no permanent master token.
6. The owner stores an offline recovery package separately from the phone. The package is shown once and is never recoverable from the game database.

### Adding, replacing, or removing credentials

- Adding or removing a passkey requires a current passkey, fresh user verification, the offline recovery factor for high-impact changes, and a delay/notification before activation where practical.
- Removing the last passkey is forbidden unless the full recovery ceremony is completed.
- Every credential change revokes all privileged sessions and invalidates old challenges.
- Ordinary player-account recovery, email support, moderator tools, and app reinstall flows cannot alter the owner identity.
- Owner-subject replacement is a special disaster-recovery ceremony, not a normal settings action.

### Lost or stolen phone

- Recovery requires the offline recovery package through the exact protected console origin.
- Recovery is heavily rate-limited, delayed when practical, and announces the pending change through every available owner alert channel.
- Completion invalidates all old passkeys, privileged sessions, service command tokens, and pending commands.
- The recovery event is anchored in the external audit store before new privileged access is enabled.

## 4. Browser and session defenses

The console uses:

- WebAuthn bound to the exact HTTPS relying-party ID and approved origin
- One-time server challenges with required user verification
- Short-lived, rotating, server-side privileged sessions
- `Secure`, `HttpOnly`, `SameSite=Strict` cookies; no owner token in browser local storage
- Anti-CSRF tokens on every state-changing request
- No state changes through `GET`
- A strict allowlist for origins, methods, headers, and redirect targets
- A restrictive Content Security Policy with no unapproved inline or third-party script
- Frame blocking through CSP `frame-ancestors 'none'` and equivalent headers
- No production source maps, directory listings, verbose errors, or cached privileged pages
- Re-authentication before sensitive reads, exports, credential changes, live commands, restore, deployment, rollback, or destructive work
- Short idle expiry, absolute session expiry, session listing, individual revocation, and revoke-all

A privileged page fails closed if authentication, authorization, environment binding, or audit recording cannot be confirmed.

## 5. External control planes

The owner-console rules also apply to every service that could bypass it. Maintain an inventory of:

- GitHub repository, Actions, environments, secrets, and release artifacts
- Game-server host and deployment service
- Database host and migration service
- Backup and object storage
- Domain/DNS provider
- Identity/passkey provider
- Logging, monitoring, alerts, and error reporting
- App distribution and future store accounts

Current rules:

- Human access is owner-only and protected with the strongest available passkey/MFA controls.
- No other collaborator, support account, shared password, or standing human administrator is added.
- Build and deployment jobs use narrow, short-lived workload identities such as OIDC rather than copied long-lived keys where supported.
- Each service identity is limited to one environment and one purpose.
- The live database has no public network endpoint and no standing browser-based raw query access.
- Backups are encrypted, integrity-checked, retention-limited, and owner-only.
- Provider audit logs are exported to the protected audit store where supported.
- Access inventory and provider memberships are reviewed before every public release and after any security event.
- During the current owner-only stage, every provider must enforce a hard $0 limit or remain inactive. A free label or usage alert alone is not permission to risk an overage.
- No weaker authentication, public database, missing backup, reduced audit, or broader human access is accepted merely because it is available on a free plan.

Cloud providers necessarily operate their own infrastructure. Sensitive state should be encrypted and permissions minimized so provider access does not become ordinary game-developer access.

## 6. Service-to-service privileged commands

The owner console never edits game tables directly and never sends arbitrary shell or database commands.

1. The console sends a bounded command to the owner-control API.
2. The API verifies the owner session, fresh-auth time, exact environment, command schema, limits, reason, and state-version preconditions.
3. The API creates a short-lived, signed command envelope containing a unique request ID, one-time nonce, audience, environment, expiry, owner subject, command type, target, sanitized parameters, and expected state version.
4. The account or valley service is reachable only through a private service boundary. It independently verifies the workload identity and command envelope.
5. The owning service performs a dry run when supported, enforces command-specific limits, writes the change transactionally, and returns before/after state versions.
6. The control API writes the final result to independent audit storage and shows a plain-English result.

Replayed, expired, wrong-audience, wrong-environment, wrong-target, unsigned, stale-version, or directly submitted commands are rejected without changing state.

## 7. Safe maintenance when a normal service is broken

A dormant maintenance executor handles failures that the normal owning service cannot repair.

- It is not internet-accessible and holds no standing raw database credential.
- It activates only after owner re-authentication, a reason, an automatic backup, and entry into maintenance/read-only mode.
- It receives a short-lived environment-specific identity.
- It can run only versioned, reviewed repair plans with bounded parameters.
- It acquires an exclusive maintenance lease, invalidates zone write leases, and checks schema and state versions.
- It cannot run arbitrary shell text or arbitrary SQL from the phone console.
- It records plans, parameters, affected records, before/after versions, snapshot, and result externally.
- It destroys its temporary identity and exits after completion or timeout.

Emergency stop, pause new logins, pause valuable writes, drain/restart a zone, stop a bad event, and send a player notice remain available even when normal gameplay is unhealthy. If the external audit sink is temporarily unavailable, emergency safety commands write to a signed local queue that must upload before normal writes resume.

## 8. Separate test and live environments

Test and live use separate:

- Domains and service endpoints
- Cloud projects or accounts where practical
- Databases and backup locations
- Encryption keys and secrets
- Workload identities
- Owner sessions and command audiences
- Build labels and artifacts

Changing a request parameter cannot turn a test command into a live command.

Dangerous commands default to test. Live changes require a live-specific session, fresh passkey verification, reason, dry-run summary, affected-record count, expected state version, strict batch limit, and explicit confirmation. Deletion, restore, migration, mass grant/removal, and world-wide changes require a fresh backup and a command-specific rollback plan.

## 9. Backup and restore security

- Gameplay backups are encrypted with keys separate from the backup files.
- Every backup has an integrity digest, schema version, environment, creation time, and retention/expiry record.
- Restore is first proven in the test environment.
- Owner identity, passkey status, privileged-session revocations, audit history, and external-control membership are stored outside gameplay backups and are never rolled backward with world data.
- A live restore pauses writes, creates a pre-restore snapshot, verifies integrity and compatibility, and records a dry-run summary.
- After restore, all player and privileged sessions, zone leases, pending commands, and short-lived service credentials are invalidated or rotated before entry resumes.
- Audit history remains continuous and records the backup source, reason, affected versions, and result.
- A restore is not considered available merely because a backup file exists; restoration must be demonstrated.

## 10. Audit and alerts

Every privileged read, export, authentication event, enrollment/recovery event, passkey change, session issue/revoke, command request, denial, dry run, change, deployment, rollback, backup, restore, infrastructure access, and emergency action records:

- Unique request/correlation ID
- Owner or machine-service identity
- Credential and fresh-auth event reference
- Environment and service
- Action and sanitized parameters
- Target and affected-record count
- Reason
- Before/after state versions
- Snapshot and rollback references
- Request, start, and completion times
- Result and safe error category
- Previous audit hash

Audit records go to append-only storage outside the gameplay database. Hash-chain anchors are written on a fixed schedule to a second protected location. Retention outlives gameplay backups. The owner console provides a readable viewer and export plus alerts for denied privileged attempts, enrollment/recovery, credential changes, live commands, mass changes, restore, deployment, and audit failure.

Non-emergency privileged mutations fail closed if the audit sink cannot accept or durably queue the event.

## 11. First-playable feature operations matrix

A task cannot be marked complete until its row is implemented and verified. “Not applicable” needs a written reason.

| Feature | Owner can inspect/diagnose | Owner can configure/repair safely | Required evidence |
|---|---|---|---|
| Builds and releases | Commit, version, artifact type, dependency and failure logs | Start approved build, stop release, select test/live-compatible version, rollback | Tester artifact contains no dev UI; artifact and secret scan passes |
| Services and database | Health, latency, errors, protocol, migration, connection pool, costs | Deploy, drain, restart, migrate, backup, restore, rollback, enter maintenance | Phone operation, test restore, outage-repair exercise |
| Accounts and sessions | Invite, account status, active sessions, denials | Create/revoke invite, suspend test account, revoke sessions | Non-owner and player-recovery denial evidence |
| Character creation | Name, appearance IDs, creation errors, identity/state link | Repair invalid prototype name/appearance/state link; never expose private credentials | Broken-record repair in test |
| Movement and camera | Position, velocity, authoritative corrections, camera diagnostics, collision and stream cell | Safe teleport, unstuck, return to safe point, reset movement state | Modified-position denial and unstuck exercise |
| World and streaming | Loaded cells, spawn points, resource nodes, world version, collision failures | Enable/disable test spawn, reload cell, repair node, stop bad event | Cell reload and wrong-version denial |
| Combat, health, and abilities | Health, combat state, hit decisions, cooldowns, equipped style, ability data | Reset stuck combat, recover test character, spawn/remove test target, toggle test ability flag | Forged-hit denial and combat repair |
| Violence and settings | Content/settings definition versions and client diagnostics, not private unrelated preferences | Publish corrected defaults or content definitions; personal player choice remains theirs | All presets and no-gameplay-effect evidence |
| Cultivation and breakthrough | Method, condition inputs, progress transaction, stage, failure reason | Repair corrupted state, reset a stuck attempt, run bounded test-world scenario | Forged-progress denial and safe repair |
| Gathering, inventory, and equipment | Resource state, item instances, transaction ledger, slots, duplication warnings | Spawn/remove test item in test, repair stuck stack/equipment, quarantine duplicate | Replay/duplication denial and repair |
| NPC schedules and memories | Location, schedule, goal, memory, relationship, duplicate ID | Move/reset stuck NPC, repair schedule or duplicate, start test scenario | Restart persistence and duplicate repair |
| Homes, claims, and placement | Owner, boundaries, objects, invalid placements, version | Release/restore test claim, move/remove broken object, repair ownership conflict | Unauthorized claim denial and restore |
| Multiplayer and duels | Presence, latency, interest set, duel state, disconnect reason | End stuck duel, disconnect test session, move player to safe state | Two-client flow and disconnect repair |
| Persistence and write leases | State version, writer lease, command IDs, failed transactions, backups | Expire stuck lease, run reviewed repair plan, restore and rotate sessions | Stale-writer denial and test restore |
| Security and audit | Active owner sessions, provider inventory, denied attempts, audit health and anchors | Revoke all, rotate credentials, pause privileged mutations, export audit | Full authorization matrix and audit-integrity check |

As features are added, this matrix grows in the same change. The owner console is not a final task added after gameplay; each feature delivers its own owner-facing controls and evidence.

## 12. Required security verification

The privileged command registry drives repeatable authorization checks for every read and command. Each route is checked as:

- Unauthenticated
- Ordinary player
- Invited tester
- Moderator or future limited role
- Copied or modified app
- Expired and revoked owner session
- Replayed or forged owner request
- Wrong origin, CSRF attempt, and framed/clickjacked page
- Wrong environment, audience, service, target, or state version
- Direct downstream-service request
- Recovery flow and lost-device flow
- Audit sink unavailable
- Concurrent/racing request
- Valid freshly authenticated owner

Every denied case must show unchanged game state and the correct safe audit record. No route is released merely because one example denial worked.

Release security gates also scan:

- Repository and relevant history
- Tester `.ipa` contents
- Owner-console production bundle and source-map settings
- Server and migration artifacts
- Container images
- Build and deployment logs
- Backup metadata and audit exports

The scans look for secrets, private keys, passwords, tokens, owner identifiers that should be private, debug menus, source maps, unsafe endpoints, and accidental client authority. Manual owner checks supplement these repeatable gates; they do not replace them.

Before public release, an independent security review is strongly recommended even though no other person receives ongoing developer-tool access.

## 13. Long-term developer tool families

The console expands with the game. The current long-term tool families are:

- **Content authoring:** inspect, draft, validate, preview in test, version, publish, disable, and roll back items, techniques, abilities, realms, bloodlines, Systems, NPCs, dialogue, quests, encounters, loot, crafting, buildings, cosmetics, localization, and balance data.
- **World operation:** inspect maps, cells, portals, realms, time, weather, resources, creatures, NPC populations, factions, events, territory, wars, economies, and server capacity; safely start, stop, repair, migrate, or roll back supported state.
- **Character and QA:** owner-only test profiles, safe teleport, controlled invulnerability, movement diagnostics, temporary test resources, realm/ability scenario setup, combat inspection, camera/collision/streaming diagnostics, and repeatable scenario reset. Test powers are clearly marked, audited, and barred from ordinary competitive outcomes.
- **Player support:** account/session status, reports, appeals, mute/block/ban actions, item/state repair from verified evidence, notices, privacy requests, and complete support audit. Support access remains owner-only unless the user later changes that rule.
- **Economy and monetization:** currency and item flow, duplication alerts, store catalog drafts, purchase/refund records, cosmetic entitlements, fraud signals, and rollback. Developer tools cannot silently create unrecorded competitive value.
- **Build and content release:** source revision, dependencies, licenses, artifact provenance, environment compatibility, migrations, feature flags, staged rollout, pause, rollback, and crash/performance comparison.
- **Security and privacy:** owner identity, sessions, external control inventory, access denials, audit anchors, secret scans, backup/restore, retention, exports, deletion requests, alerts, incident mode, and credential rotation.
- **Scale and reliability:** region load, player counts, queues, latency, bandwidth, database pressure, stuck jobs, service health, capacity changes, draining, failover, and disaster recovery.

Every future spec must add its new feature to this catalog and the operations matrix. A tool family may grow, but it may not bypass the sole-owner authorization, environment isolation, command limits, audit, backup, and release gates in this document.

## 14. Task 1 concrete control record

The dated inventory and numeric private-prototype defaults are maintained in `docs/CONTROL_PLANE_INVENTORY.md`. It is binding alongside this document.

Current facts:

- GitHub and its Kiro integration are the only active project control planes; the source repository is API-verified public while only the owner appears in the collaborator list.
- No game runtime, Railway project, database, owner console, game identity provider, audit store, backup store, monitoring provider, domain, Apple project, or public distribution entry is active.
- No owner-console identity exists yet, so nobody—including the owner—currently has a game developer menu or game-server privilege to leak.
- GitHub Free and no-current-payment-due status were confirmed by the owner's screenshot; passkey, 2FA, recovery, repository-secret, and deployment-record details remain private and should be reviewed before adding secrets or online services, but do not block a public standard-runner unsigned build.
- The future test/live enrollment values, session times, recovery delays, isolation rules, backup retention, recovery targets, and maintenance limits are recorded in the inventory.
- Exact domains, RP IDs, alert destinations, and provider project IDs remain intentionally blank; activation is blocked until they are selected and written down.

A blank value is a deployment blocker, not permission to choose a value silently. Never request or record the owner's passwords, recovery codes, private passkeys, payment details, signing certificates, provisioning profiles, or private keys.