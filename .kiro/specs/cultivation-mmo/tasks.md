# First Playable Version Implementation Plan

## Referenced specification

#[[file:requirements.md]]
#[[file:design.md]]
#[[file:../../../docs/OWNER_OPERATIONS_SECURITY.md]]
#[[file:../../../docs/COST_DECISIONS.md]]
#[[file:../../../docs/TECHNICAL_BASELINE.md]]
#[[file:../../../docs/CONTROL_PLANE_INVENTORY.md]]
#[[file:../../../docs/TEMPORARY_ASSET_LICENSES.md]]
#[[file:../../../docs/PUBLIC_REPOSITORY.md]]
#[[file:../../../docs/OWNER_ACTIONS_CHECK.md]]

> All tasks are provisional. If the game direction changes, update the requirements and design before continuing implementation.

## Feature completion rule

A gameplay or service task cannot be checked off until its matching row in `docs/OWNER_OPERATIONS_SECURITY.md` has working owner inspection, diagnostics, configuration, safe repair/reset, phone UI, authorization, audit, and verification—or a written reason that one of those controls does not apply. Owner tools are built with each feature, not added after the game is finished.

- [x] 1. Record the technical and security baseline
  - **Status:** Complete. GitHub is API-verified public, only the owner appears in the collaborator list, all public-runner safeguards are documented, and private vulnerability reporting is explicitly gated in Task 3 because the integration cannot enable it.
  - [x] Pin Godot 4.7.2 and its official download digests; select the rolling standard `macos-latest` runner with its newest stable Xcode (currently macOS 26/Xcode 26.6), iOS/Metal settings, and Linux headless path in `docs/TECHNICAL_BASELINE.md`.
  - [x] Compare secure WebSocket and ENet/UDP; choose WSS provisionally for the first two-to-ten-player Railway test and define the later Wi-Fi/cellular measurement.
  - [x] Keep every runtime host inactive at $0; retain Railway Hobby as the preferred later paid-test candidate without treating it as activation or a permanent choice.
  - [x] Record current official GitHub/Railway terms, the $0 work path, provisional cost estimate, documented caps and unknown dashboard limit floor, exit plan, and recheck rules in `docs/COST_DECISIONS.md`.
  - [x] Inventory every active, inactive, and planned external control plane plus owner-only access, machine identity, audit, revocation, cost, and deletion status in `docs/CONTROL_PLANE_INVENTORY.md`.
  - [x] Define numeric owner enrollment, passkey rotation, lost-phone recovery, test/live isolation, backup retention, audit, and maintenance defaults.
  - [x] Record that no external game-content asset is selected and establish the approval ledger in `docs/TEMPORARY_ASSET_LICENSES.md`.
  - [x] Owner's screenshot confirms GitHub Free, included usage covers the displayed metered usage, and no current payment is due; owner approved exposing the full clean history for free public standard-runner builds.
  - [x] Verify the repository is public, only the owner has write access, and public-runner safeguards are current. Private vulnerability reporting is a required pre-workflow step in Task 3 because the integration's enable request returned HTTP 403.
  - Requirements: R1, R2, R12, R14, R16, R17, R19, R20, R21, R22, R23
  - Dependencies: None

- [x] 2. Create the multi-part project skeleton
  - **Status:** Complete and locally verified; evidence is in `docs/verification/TASK_2.md`.
  - [x] Create the repository-root Godot project plus `client`, `server/api`, `server/zone`, `server/owner-control`, `server/maintenance`, `server/database`, `owner-console`, and `shared` foundations described by the design.
  - [x] Add a bootable Godot client shell, self-checking headless zone, startable loopback API/owner-control shells, dormant maintenance/database checkers, and a separate phone-readable protected console source shell.
  - [x] Add the strict GameplayCommandV1 codec, shared authority core, empty privileged command registry, machine-readable per-feature operations matrix, unavailable audit interface, and empty maintenance-plan registry.
  - [x] Add plain-English structure, setup, component, licensing, verification, and not-yet-built documentation.
  - [x] Keep all hosted services inactive and keep secrets, owner credentials, source maps, external assets, workflows, and developer UI out of player source/artifacts.
  - Requirements: R1, R2, R17, R20, R21, R23
  - Dependencies: 1

- [ ] 3. Prove the GitHub-to-iPhone build path
  - Before committing or publishing the workflow, the owner enables GitHub private vulnerability reporting in repository Security settings; verify the private reporting URL works.
  - Add a reviewed cloud workflow that exports the Godot client on macOS and packages an `.ipa` artifact.
  - Build one locked player client for owner and tester use with no developer menu or local diagnostic overlay.
  - Use only owner-authorized `workflow_dispatch` (directly or through the recognized Kiro integration after explicit instruction), the rolling standard `macos-latest` runner and its newest stable default Xcode, one-build concurrency, a 30-minute timeout, no Actions cache, no beta/preview Xcode, and no larger/custom runner.
  - Publish the unsigned result as a clearly labeled public prerelease asset instead of an Actions artifact; anyone may download it.
  - Allow a clearly labeled zero-cost solo technical-preview mode that runs the same versioned authoritative gameplay commands in process; do not represent it as completed online play.
  - Build the protected owner-console shell separately; do not place its assets or routes inside the `.ipa` and do not activate hosted resources outside the approved cost boundary.
  - For checkpoint 3.1, use the public GitHub Actions run page as the temporary build control and audit view; the owner authorizes each manual run directly or by explicitly instructing the recognized Kiro integration, all logs are public, and full runtime owner controls remain an online-version requirement.
  - Give artifacts clear environment, type, version, and commit names.
  - Make failure logs understandable; the zero-cost preview SHALL use no signing or deployment credential, while any future separately approved signed/online workflow must use short-lived or protected secrets.
  - Add owner inspection for builds, versions, failures, artifacts, and release compatibility, plus approved start/stop/rollback controls and audit.
  - Add the root `THIRD_PARTY_NOTICES.md` and bundle the Godot copyright, MIT terms, and applicable official engine third-party notices in every distributed package, including the first public test build.
  - Perform the first ESign installation and non-owner artifact-inspection check before deeper gameplay.
  - Requirements: R1, R19, R20, R21, R22, R23
  - Dependencies: 2
  - [ ] 3.1 Produce the reachable zero-cost solo technical-preview checkpoint
    - Add a minimal app shell and settings needed by the preview.
    - Add an in-process authoritative command adapter behind the same versioned interface intended for the future hosted valley.
    - Add a reusable graybox area, third-person touch movement and camera, one interaction, one combat-input target, and one meditation input so the owner can judge basic feel.
    - Keep data local, label the entire build `Offline Technical Preview`, include no developer menu, and make no multiplayer, remote-save, or online-console claim.
    - Build through a free public standard runner, publish the unsigned result as a public prerelease, install through ESign, and do not activate any paid service.
    - Treat this as an experimental checkpoint, not completion of the full gameplay, online, persistence, or owner-control requirements.
    - Requirements: R1, R4, R5, R15, R16, R21, R22, R23
    - Dependencies: 2; runs as the final checkpoint within Task 3

- [ ] 4. Build the mobile app shell and settings
  - Add boot, loading, private sign-in, retry, version-mismatch, and settings screens.
  - Implement safe-area-aware landscape layout and readable touch targets.
  - Add camera, text, sound, graphics, and gore settings with persistence.
  - Send bounded build, configuration, crash, and performance diagnostics without exposing private preference data.
  - Add owner-console inspection for app versions, configuration definitions, errors, and diagnostics; player preferences remain player-controlled.
  - Verify no gesture, local flag, copied file, or modified setting reveals a developer menu or privilege.
  - Requirements: R2, R7, R15, R16, R20, R21, R23
  - Dependencies: 2, 3

- [ ] 5. Implement accounts, sole-owner identity, deployment, and persistence
  - Add invite-based private player accounts, protected sessions, recovery, and revocation.
  - Implement single-use owner enrollment, exact-origin WebAuthn passkey authentication, rotation, revocation, lost-phone recovery, and one immutable owner subject separate from player recovery.
  - Implement short-lived privileged sessions, fresh-auth checks, emergency revoke-all, owner alerts, and no player-facing route to privilege.
  - Create versioned schemas for account identity, character identity, gameplay state, cultivation, inventory, homes, NPC state, duels, writer leases, and request IDs.
  - Keep owner identity/revocation outside gameplay restore data and send complete privileged events to external append-only audit storage.
  - Give the account service sole write ownership of account/session/identity data and the active valley server sole write ownership of gameplay state.
  - Add atomic or safely repeatable writes, encrypted integrity-checked backups, and versioned migrations.
  - Prepare separate test/live deployment definitions, databases, secrets, keys, backup stores, identities, command audiences, health, costs, and rollback controls without a local computer; Railway Hobby is preferred for the first paid test but remains inactive until the owner gives explicit approval at that time.
  - Before requesting any nonzero approval, update and present `docs/COST_DECISIONS.md`, recheck Railway's current terms, confirm enforceable compute and agent hard limits, ask for a clear monthly maximum, and stop until the owner explicitly approves it.
  - Add owner inspection and bounded controls for accounts, sessions, migrations, writer leases, backup, tested restore, deployment, rollback, costs, free-usage limits, and audit.
  - Requirements: R2, R13, R17, R19, R20, R21, R22
  - Dependencies: 1, 2

- [ ] 6. Connect the client to an authoritative hosted valley
  - Establish authenticated live sessions between the Godot client and hosted valley server, reusing the versioned command interface proven by checkpoint 3.1.
  - Do not activate hosting unless it enforces $0 or the owner has explicitly approved a later hard budget cap.
  - Add protocol checks, reconnect, server-controlled spawning, interest management skeleton, request IDs, a single-writer gameplay lease, bounded actor IDs, and guaranteed `AuthorityCore.release_actor` cleanup on disconnect/session expiry.
  - Implement private service identity and replay-resistant owner command envelopes; downstream services independently reject direct or invalid privileged calls.
  - Add owner inspection for sessions, latency, protocol, server state, leases, connection errors, and health; add bounded drain, restart, safe disconnect, and lease-repair controls.
  - Confirm private player connectivity over Wi-Fi and cellular and owner operation through the iPhone console.
  - Requirements: R2, R12, R13, R16, R17, R19, R20, R21, R22
  - Dependencies: 3, 3.1, 4, 5

- [ ] 7. Implement character creation and durable character loading
  - Add the young-adult 3D preview, name rules, and initial appearance choices.
  - Save stable appearance identifiers and reserved future origin fields.
  - Load the same identity and appearance after closing and reopening.
  - Add owner inspection for creation errors, names, appearance IDs, identity/state links, and load state; add bounded test repair/reset for invalid records.
  - Verify owner repair is audited and every non-owner creation or repair bypass is denied.
  - Requirements: R3, R13, R20, R21
  - Dependencies: 4, 5, 6

- [ ] 8. Build the third-person touch controller
  - Add camera-relative movement, sprint, jump/traversal hook, dodge, defense, interaction, attack, and technique controls.
  - Add camera collision, target-lock foundation, sensitivity settings, and safe-area placement.
  - Add responsive local presentation with gentle correction to server-approved movement.
  - Add owner inspection for position, velocity, authoritative corrections, movement state, camera diagnostics, collision, and streaming cell; add bounded safe-teleport, unstuck, and movement-reset controls.
  - Verify modified-position requests fail and are audited.
  - Requirements: R4, R6, R15, R16, R17, R20, R21
  - Dependencies: 6, 7

- [ ] 9. Create and stream the starter valley
  - First create simple blockout versions of the arrival path, village, forest, mountains, and cave.
  - Add landmarks, collision safety, world-section streaming, optional routes, and data-driven spawn points.
  - Replace enough blockout art to demonstrate semi-realistic, bright, ink, and dark directions.
  - Add owner inspection for world/content versions, loaded cells, collision failures, spawns, and resource nodes; add bounded test cell reload, spawn enable/disable, broken-node repair, and event-stop controls.
  - Verify wrong-version and wrong-environment world commands fail safely.
  - Requirements: R5, R14, R16, R20, R21
  - Dependencies: 8

- [ ] 10. Add authoritative interaction, gathering, and inventory
  - Build one shared interaction system for resources, loot, NPCs, and claims.
  - Add prototype herbs, creature materials, a cultivation resource, a sword, and one discovered item.
  - Handle inventory limits, unique item IDs, transaction records, respawning resources, and replay/duplicate protection.
  - Add owner inspection for nodes, item instances, inventory/equipment, transaction history, and duplication warnings; add bounded test spawn/remove, quarantine, and stack/equipment repair controls.
  - Verify replayed grants, forged gathering, and non-owner item commands leave state unchanged and are audited.
  - Requirements: R9, R13, R17, R18, R20, R21
  - Dependencies: 5, 6, 9

- [ ] 11. Implement grounded sword and unarmed combat
  - Build the common combat state model and distinct sword/unarmed data.
  - Add attack, defense, dodge, hit reaction, health, creature defeat, recovery, and clear feedback.
  - Keep hit validation, damage, and rewards authoritative; reserve data for future movement tiers, flight, and path abilities.
  - Add owner inspection for health, combat state, hit decisions, cooldowns, equipment style, and abilities; add bounded test-target spawn/remove, recovery, and stuck-combat reset controls.
  - Verify forged hits, damage, cooldown resets, and non-owner combat commands fail and are audited.
  - Requirements: R6, R12, R16, R17, R18, R20, R21
  - Dependencies: 8, 9, 10

- [ ] 12. Implement the four violence presets
  - Connect Off, Low, Medium, and Full to blood, wounds, dismemberment eligibility, and body lifetime.
  - Substitute readable ink or light feedback when gore is disabled.
  - Keep personal preference under player control and ensure settings never alter collision, damage, loot, or advantage.
  - Add owner inspection for content-definition versions and anonymous diagnostics plus bounded publication/rollback of corrected defaults; do not expose or override unrelated private player preferences.
  - Verify every player preset and confirm the `.ipa` contains no developer UI.
  - Requirements: R7, R15, R16, R20, R21
  - Dependencies: 4, 11

- [ ] 13. Implement meditation and the first breakthrough
  - Add the starter method, meditation locations, explicit start/stop, progress inputs, and one threshold.
  - Add an interactive breakthrough and a noticeable movement or Qi reward.
  - Persist confirmed results and handle interruption or failure without corrupting saves.
  - Add owner inspection for methods, condition inputs, progress transactions, stages, and failure reasons; add bounded test simulation, corrupted-state repair, and stuck-attempt reset controls.
  - Verify forged/replayed progress, stage, and reward requests fail and are audited.
  - Requirements: R8, R13, R17, R18, R20, R21
  - Dependencies: 5, 9, 10, 11

- [ ] 14. Add the first living NPC sample
  - Create at least three named residents with roles, schedules, current goals, and limited memories.
  - Add helpful/harmful relationship changes and one optional information-led discovery.
  - Persist required NPC state and avoid duplication after server restarts.
  - Add owner inspection for location, schedule, goal, memory, relationship, and identity; add bounded test move, scenario start, schedule repair, duplicate quarantine, and reset controls.
  - Verify restart persistence, duplicate repair, and non-owner denial.
  - Requirements: R10, R13, R18, R20, R21
  - Dependencies: 5, 9, 10

- [ ] 15. Add the claimable home or cultivation cave
  - Implement ownership, boundaries, server-validated placement, moving, and removal.
  - Add a small catalog of decorative or functional objects.
  - Restore ownership and placement after reconnecting.
  - Add owner inspection for owner, boundaries, objects, invalid placements, and state version; add bounded test release/restore, broken-object removal, and ownership-conflict repair.
  - Verify unauthorized claims, stale placement, and non-owner repair commands fail and are audited.
  - Requirements: R11, R13, R17, R18, R20, R21
  - Dependencies: 5, 9, 10

- [ ] 16. Add few-player presence and agreed duels
  - Synchronize nearby appearance, movement, actions, and relevant state for two to ten testers.
  - Add interest management and remote-movement smoothing.
  - Keep the valley safe by default; add duel request, acceptance, resolution, disconnect, and no-loss rules.
  - Add preset emotes if needed; defer free-text chat.
  - Add owner inspection for presence, latency, interest sets, duel state, and disconnect reason; add bounded test session disconnect, stuck-duel termination, and safe-state recovery.
  - Verify two-client behavior, disconnect repair, forced-duel denial, and non-owner privileged denial.
  - Requirements: R12, R13, R17, R18, R20, R21
  - Dependencies: 6, 7, 8, 11

- [ ] 17. Complete the visual, audio, and usability pass
  - Improve mobile readability, onboarding, repeatable help, combat feedback, ambience, and cultivation feedback.
  - Confirm the semi-realistic, bright, ink, and dark blend without copying existing works.
  - Audit temporary asset licenses and attribution.
  - Add owner inspection for content versions, missing assets, loading failures, and performance diagnostics plus bounded content feature flags and rollback.
  - Confirm normal players cannot access operational asset lists, unpublished content, or owner controls.
  - Requirements: R7, R14, R15, R18, R20, R21
  - Dependencies: 9, 12, 13, 14, 15, 16

- [ ] 18. Harden performance, recovery, maintenance, and security
  - Profile streaming, shadows, effects, creatures, NPC updates, networking, and diagnostic collection on the target iPhone.
  - Tune for a sustained 30 FPS default and add a higher-frame-rate option.
  - Exercise retry, backgrounding, reconnect, server restart, duplicate requests, save conflicts, invalid inputs, and writer-lease failures.
  - Implement the dormant maintenance executor with owner re-authentication, maintenance mode, snapshot, exclusive lease, versioned repair plans, short-lived identity, audit, and no arbitrary SQL/shell.
  - Prove an encrypted backup restore in test; preserve identity/revocation/audit state and invalidate all sessions, leases, pending commands, and temporary service credentials afterward.
  - Add owner inspection and bounded controls for performance, errors, maintenance, emergency login/write pause, drain/restart, event stop, notices, backup/restore, and audit health.
  - Verify tester artifacts contain no developer UI, source maps, secrets, unnecessary personal data, or client authority.
  - Requirements: R2, R13, R15, R16, R17, R19, R20, R21
  - Dependencies: 10, 11, 12, 13, 14, 15, 16, 17

- [ ] 19. Integrate the owner console and privileged-access release gates
  - Assemble the owner controls delivered by Tasks 3–18 into the separate iPhone-friendly console and complete every first-playable operations-matrix row.
  - Implement exact-origin WebAuthn, protected post-auth assets, short-lived secure sessions, CSRF/CORS/CSP/frame/cache defenses, fresh authentication, alerts, and emergency revoke-all.
  - Implement private service identities, signed replay-resistant command envelopes, independent downstream authorization, isolated test/live controls, and high-impact live dry-run/snapshot/rollback safeguards.
  - Complete credential enrollment, rotation, removal, lost-phone recovery, external-control inventory, audit viewer/export/anchors, and owner-only provider access evidence.
  - Drive repeatable authorization checks from the privileged command registry for every identity, session, origin, replay, environment, target, downstream, recovery, race, and audit-failure case in `docs/OWNER_OPERATIONS_SECURITY.md`.
  - Assert unchanged state and correct audit records for every denial; block release on any failure.
  - Scan repository/history, `.ipa`, console bundle, services, containers, logs, backup metadata, and audit exports for secrets, dev UI, source maps, unsafe endpoints, and unintended authority.
  - Requirements: R17, R19, R20, R21, R23
  - Dependencies: 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18

- [ ] 20. Produce and manually verify the complete first `.ipa`
  - Build the release-candidate player artifact through a free standard public runner and publish the unsigned result as a public prerelease for ESign installation.
  - Confirm the public workflow used only owner-authorized manual dispatch, rolling standard `macos-latest`, newest stable default Xcode, no beta/preview Xcode, no cache/larger runner/paid service, narrow permissions, clean public logs, and no Actions artifact storage.
  - Confirm the `.ipa` contains no developer menu, diagnostic overlay, secret, or owner authority and that protected online-console assets remain unavailable before owner authentication.
  - Use the separate owner console from the iPhone to complete every operations-matrix action relevant to the first online slice.
  - Confirm service/database health, audit integrity, external-control inventory, backup/test restore, compliance with the currently owner-approved hard budget cap, cost alert, revoke-all, maintenance repair, deployment, and rollback.
  - Verify no service or GitHub usage created a charge before explicit approval or exceeded any later approved cap; an offline technical preview is released at checkpoint 3.1, not accepted through this full-online task.
  - Complete the new-character path: create, enter, travel, gather, equip, fight, cultivate, break through, claim, place, close, and restore.
  - Prove shared-world and duel flow with two physical clients or one physical client plus a controlled headless client.
  - Confirm every privileged-access and secret-leakage release gate passes.
  - Run a 30-minute stability check and save plain-English results, known problems, screenshots, and exact build ID.
  - Requirements: R1–R23
  - Dependencies: 19

- [ ] 21. Review the first playable version with the owner
  - Ask only concrete questions based on what the owner can see and feel on the iPhone.
  - Update the living vision, requirements, design, security plan, operations matrix, and follow-up specs from feedback.
  - Update owner controls and security evidence whenever a feature changes.
  - Do not treat any earlier decision as unchangeable.
  - Requirements: R18, R20, R21, R22, R23
  - Dependencies: 20
