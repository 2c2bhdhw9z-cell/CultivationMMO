# First Playable Version Requirements

## Referenced living vision

#[[file:../../../docs/GAME_VISION.md]]
#[[file:../../../docs/OWNER_OPERATIONS_SECURITY.md]]

## Introduction

This feature spec defines the first private playable `.ipa` for the Cultivation MMO. It is intentionally small: one online valley that proves third-person exploration, combat, cultivation, persistence, light building, and a few-player shared world on an iPhone.

The complete, changeable direction is recorded in `docs/GAME_VISION.md`.

This first version does **not** attempt the full childhood experience, public MMO scale, deep economy, reincarnation, true Systems, family simulation, dangerous-region PvP, cosmic realms, or monetization. Its foundations must avoid blocking those features later.

## Requirement R1: GitHub-built iPhone package

**User Story:** As the owner using only an iPhone, I want GitHub to produce the game package, so that I can install and test it through ESign without owning a computer.

### Acceptance Criteria

1. WHEN an authorized cloud build is started from the repository
   THE SYSTEM SHALL build the iOS client on a hosted macOS runner.
2. WHEN the iOS build succeeds
   THE SYSTEM SHALL provide a downloadable `.ipa` artifact suitable for signing or installing through the owner's ESign workflow.
3. IF signing material is needed
   THE SYSTEM SHALL obtain it only from protected build secrets and SHALL NOT store certificates, passwords, profiles, or private keys in the repository.
4. WHEN a build fails
   THE SYSTEM SHALL show a useful failure summary in the GitHub build log.
5. THE SYSTEM SHALL keep the first build process usable from an iPhone browser.
6. THE SYSTEM SHALL NOT require TrollStore, SideStore, or a seven-day re-signing workflow for the owner's planned ESign testing.

## Requirement R2: Private online access

**User Story:** As the owner, I want the first world to remain private, so that unfinished builds are not exposed publicly.

### Acceptance Criteria

1. WHEN the game launches without a saved session
   THE SYSTEM SHALL offer a simple private-test sign-in or account-creation flow.
2. WHEN a player lacks permission for the private test
   THE SYSTEM SHALL deny world entry without revealing private server data.
3. WHEN internet access is unavailable
   THE SYSTEM SHALL explain that this build requires an online connection and offer Retry.
4. WHEN a valid returning player launches the game
   THE SYSTEM SHALL restore the correct account without requiring code knowledge.
5. THE SYSTEM SHALL support revoking a test account without deleting unrelated player data.

## Requirement R3: Prototype character creation

**User Story:** As a player, I want to create my cultivator, so that the character feels like mine.

### Acceptance Criteria

1. WHEN a new account enters character creation
   THE SYSTEM SHALL allow selection of a name and a useful initial set of appearance options.
2. WHEN appearance options change
   THE SYSTEM SHALL preview the character in 3D before confirmation.
3. WHEN the player confirms a valid character
   THE SYSTEM SHALL save the character and place them at the starter location.
4. IF a name is invalid, unsafe, or already unavailable under the chosen naming rules
   THE SYSTEM SHALL explain the problem and allow correction.
5. THE SYSTEM SHALL use a young-adult character for this prototype and SHALL keep origin, age, progression path, and class fields extensible for later child and adult starts.
6. THE SYSTEM SHALL NOT force a permanent combat or cultivation class during creation.

## Requirement R4: Third-person mobile control

**User Story:** As an iPhone player, I want clear touch controls and a camera behind my character, so that exploring and fighting feel direct.

### Acceptance Criteria

1. WHEN the player touches the movement control
   THE SYSTEM SHALL move the character relative to the current camera direction.
2. WHEN the player drags the camera area
   THE SYSTEM SHALL rotate the third-person camera without moving unrelated UI.
3. THE SYSTEM SHALL provide touch actions for interaction, jump or traversal, sprint, dodge, basic attack, defensive action, and technique use when available.
4. WHEN the camera approaches solid scenery
   THE SYSTEM SHALL avoid remaining inside the scenery or fully hiding the player.
5. THE SYSTEM SHALL respect iPhone safe areas and SHALL NOT place required controls beneath screen cutouts or system gestures.
6. THE SYSTEM SHALL provide adjustable camera and aiming sensitivity.

## Requirement R5: Open starter valley

**User Story:** As a player, I want a small open region with distinct places, so that I can explore instead of moving through menus.

### Acceptance Criteria

1. WHEN the player enters the starter region
   THE SYSTEM SHALL provide freely traversable routes among a village, forest, mountain area, and cave.
2. THE SYSTEM SHALL include visible landmarks that help navigation without requiring a permanent guiding arrow.
3. WHEN world sections stream in or out
   THE SYSTEM SHALL prevent the player from falling through missing ground or interacting with unloaded objects.
4. THE SYSTEM SHALL include at least one optional route or discovery that is not required by the introductory path.
5. THE SYSTEM SHALL use handcrafted major locations while supporting data-driven placement of repeatable resources and encounters.

## Requirement R6: Grounded combat foundation

**User Story:** As a player, I want directly controlled sword and unarmed combat, so that early mortal fighting feels physical and dangerous.

### Acceptance Criteria

1. WHEN a player equips no weapon
   THE SYSTEM SHALL provide a basic unarmed attack set.
2. WHEN a player equips the prototype sword
   THE SYSTEM SHALL provide a distinct basic sword attack set.
3. THE SYSTEM SHALL support attack, defensive action, dodge, hit reaction, health, defeat, and target selection or lock-on suitable for touch controls.
4. WHEN an attack is validated as a hit
   THE SYSTEM SHALL apply its result once and show clear visual and sound feedback.
5. WHEN the player is defeated by a prototype creature
   THE SYSTEM SHALL return the player to a safe recovery point without deleting permanent progress or equipped gear.
6. THE SYSTEM SHALL keep movement and ability data extensible for future air movement, flight, ranged techniques, and path-specific combat speeds.

## Requirement R7: Adjustable violence presentation

**User Story:** As a player, I want control over visible gore, so that combat presentation matches my comfort level.

### Acceptance Criteria

1. THE SYSTEM SHALL provide Off, Low, Medium, and Full gore presets.
2. WHEN Off is selected
   THE SYSTEM SHALL show no blood or dismemberment and SHALL substitute non-graphic effects where feedback is needed.
3. WHEN Low is selected
   THE SYSTEM SHALL allow mild blood and SHALL disable dismemberment.
4. WHEN Medium is selected
   THE SYSTEM SHALL allow stronger wound presentation and limited gore.
5. WHEN Full is selected
   THE SYSTEM SHALL enable the complete intended combat presentation for eligible adult content.
6. WHEN a gore setting changes
   THE SYSTEM SHALL apply it without changing damage, rewards, collision, or combat advantage.
7. THE SYSTEM SHALL save the selected setting and default conservatively on first launch.

## Requirement R8: First cultivation breakthrough

**User Story:** As a player, I want to cultivate and complete a breakthrough, so that advancement noticeably changes what I can do.

### Acceptance Criteria

1. WHEN the player has access to the starter method and a valid location
   THE SYSTEM SHALL allow a directly chosen meditation session.
2. WHEN cultivation conditions change
   THE SYSTEM SHALL calculate progress from explicit data such as method, location, character state, and available resources.
3. WHEN the player reaches the prototype threshold
   THE SYSTEM SHALL require the player to begin and complete an interactive breakthrough event rather than advancing silently.
4. WHEN the breakthrough succeeds
   THE SYSTEM SHALL save the new stage and unlock at least one noticeable ability or movement improvement.
5. IF the breakthrough is interrupted or fails under prototype rules
   THE SYSTEM SHALL explain the outcome and SHALL NOT corrupt the character save.
6. THE SYSTEM SHALL NOT provide unlimited progress merely for opening the app or pressing a reward button.

## Requirement R9: Gathering and inventory

**User Story:** As an explorer, I want to gather and keep useful discoveries, so that leaving safe paths has a purpose.

### Acceptance Criteria

1. WHEN the player approaches an eligible resource
   THE SYSTEM SHALL show a clear interaction prompt.
2. WHEN gathering succeeds
   THE SYSTEM SHALL place the correct item and amount in the player's authoritative inventory.
3. WHEN inventory space or another requirement prevents gathering
   THE SYSTEM SHALL explain why and SHALL NOT remove the world resource incorrectly.
4. THE SYSTEM SHALL include prototype herbs, creature materials, a cultivation resource, and at least one discovered item.
5. THE SYSTEM SHALL allow viewing useful item descriptions without requiring external guides.

## Requirement R10: Small living-NPC sample

**User Story:** As a player, I want people in the valley to behave like residents, so that the world does not feel lifeless.

### Acceptance Criteria

1. THE SYSTEM SHALL include at least three named NPCs with a location, simple schedule, role, and current goal.
2. WHEN the player performs a supported helpful or harmful action involving a named NPC
   THE SYSTEM SHALL update at least one remembered relationship or reputation value.
3. WHEN a named NPC changes schedule state
   THE SYSTEM SHALL move or transition them to an appropriate activity without creating duplicates.
4. THE SYSTEM SHALL allow at least one NPC interaction to provide information or opportunity without presenting it as a mandatory quest.
5. WHEN the server restarts
   THE SYSTEM SHALL restore persistent NPC state required by the prototype.

## Requirement R11: Claimable cultivation home

**User Story:** As a player, I want a small place of my own, so that territory building begins inside the physical world.

### Acceptance Criteria

1. WHEN an eligible unclaimed prototype home or cave is available
   THE SYSTEM SHALL allow an authorized player to claim it.
2. WHEN a claim succeeds
   THE SYSTEM SHALL persist ownership and prevent a second player from silently taking it.
3. THE SYSTEM SHALL allow the owner to place, move, and remove a small set of decorative or functional objects within valid boundaries.
4. IF placement is invalid
   THE SYSTEM SHALL show the reason and SHALL NOT consume or duplicate the object.
5. WHEN the owner reconnects
   THE SYSTEM SHALL restore the claim and placed objects.

## Requirement R12: Few-player shared world and agreed duels

**User Story:** As a tester, I want to share the valley with a few other players, so that the MMO foundation can be felt early.

### Acceptance Criteria

1. WHEN two to ten authorized testers enter the same valley instance
   THE SYSTEM SHALL show their movement, appearance, basic actions, and relevant state to one another.
2. WHEN network updates are delayed
   THE SYSTEM SHALL smooth ordinary remote movement without granting authority to the remote phone.
3. THE SYSTEM SHALL keep the starter valley safe from unwanted player attacks.
4. WHEN one player requests a duel
   THE SYSTEM SHALL begin it only after the other player clearly accepts.
5. WHEN a duel ends
   THE SYSTEM SHALL restore participants according to duel rules and SHALL NOT remove items or permanent progress.
6. WHEN a player disconnects during a duel
   THE SYSTEM SHALL resolve the duel consistently and protect the save from duplication.

## Requirement R13: Authoritative persistence and recovery

**User Story:** As a player, I want my progress to survive closing the app and connection problems, so that my time is respected.

### Acceptance Criteria

1. THE SYSTEM SHALL store character identity, appearance, location, cultivation stage, inventory, equipment, home claim, and required NPC effects on the server.
2. WHEN an important change succeeds
   THE SYSTEM SHALL save it atomically or make it safely repeatable.
3. WHEN a player reconnects after an interruption
   THE SYSTEM SHALL restore the last confirmed valid state without duplicating rewards.
4. IF saved data cannot be loaded
   THE SYSTEM SHALL block unsafe play, preserve diagnostic information without exposing secrets, and provide a clear retry message.
5. THE SYSTEM SHALL keep recoverable backups appropriate for a private prototype.
6. WHEN a character is active in a valley
   THE SYSTEM SHALL allow only the lease-holding authoritative valley server to write gameplay state and SHALL NOT accept a client-provided gameplay save through the account API.

## Requirement R14: Coherent visual direction

**User Story:** As a player, I want the prototype to show the intended identity, so that I can judge more than gray test shapes.

### Acceptance Criteria

1. THE SYSTEM SHALL use a semi-realistic visual foundation for the character and valley.
2. THE SYSTEM SHALL use brighter fantasy treatment for at least one safe or cultivated location.
3. THE SYSTEM SHALL use an ink-inspired effect for meditation, Qi, or breakthrough presentation.
4. THE SYSTEM SHALL use a darker treatment for the cave or another dangerous location.
5. Any temporary third-party asset SHALL have a recorded license that permits its project use and redistribution.

## Requirement R15: Understandable mobile interface

**User Story:** As a player unfamiliar with code, I want simple language and readable controls, so that I can test without technical help.

### Acceptance Criteria

1. THE SYSTEM SHALL use plain player-facing language and SHALL NOT expose internal code terms in ordinary errors.
2. THE SYSTEM SHALL provide readable text sizes and touch targets on the iPhone 17 Pro Max.
3. THE SYSTEM SHALL allow adjustment of text size, camera sensitivity, master volume, music volume, effects volume, and gore.
4. WHEN a new action becomes available
   THE SYSTEM SHALL teach it briefly and allow the explanation to be viewed again.
5. THE SYSTEM SHALL allow the owner to reach the first cultivation breakthrough without reading an external guide.

## Requirement R16: Prototype performance and reliability

**User Story:** As an iPhone player, I want stable performance, so that the prototype remains playable during a real session.

### Acceptance Criteria

1. THE SYSTEM SHALL target a sustained minimum of 30 frames per second on the owner's iPhone 17 Pro Max using default settings in the starter valley.
2. THE SYSTEM SHALL offer scalable graphics settings with a higher-frame-rate option when the device can sustain it.
3. THE SYSTEM SHALL avoid an unhandled crash during a 30-minute owner play session covering travel, combat, cultivation, and reconnecting.
4. WHEN the app moves briefly into the background
   THE SYSTEM SHALL either reconnect safely or explain that the session ended.
5. THE SYSTEM SHALL record useful performance and error diagnostics without collecting unnecessary personal information.

## Requirement R17: Security and content boundaries

**User Story:** As the owner, I want a safe foundation, so that an unequal world is not confused with cheating, unsafe content, or leaked secrets.

### Acceptance Criteria

1. THE SERVER SHALL validate important movement, combat, inventory, cultivation, duel, and building requests before accepting them.
2. THE CLIENT SHALL NOT contain server administrator credentials, database passwords, signing secrets, or hidden master authority.
3. THE FIRST PLAYABLE VERSION SHALL contain no real-money purchases.
4. THE FIRST PLAYABLE VERSION SHALL contain no sexual scenes or sexual content.
5. THE FIRST PLAYABLE VERSION SHALL use only young-adult player characters and SHALL NOT expose unfinished child-character features.
6. WHEN diagnostic logs are produced
   THE SYSTEM SHALL avoid recording secrets or unnecessary personal information.

## Requirement R18: Complete first playable loop

**User Story:** As the owner, I want one complete short experience, so that I can decide whether the foundation feels worth expanding.

### Acceptance Criteria

1. WHEN starting with a new private-test account
   THE SYSTEM SHALL allow the owner to create a character, enter the valley, learn movement, and reach the village.
2. THE SYSTEM SHALL allow the owner to explore, gather an herb, obtain or equip a sword, survive a creature encounter, meditate, and attempt the first breakthrough.
3. THE SYSTEM SHALL allow the owner to discover and claim the prototype home or cave and place at least one object.
4. BEFORE the first version is accepted
   THE SYSTEM SHALL demonstrate two concurrent clients seeing one another and completing an agreed duel, using either two physical clients or one physical client plus a controlled headless client.
5. WHEN the owner closes and reopens the app
   THE SYSTEM SHALL restore the confirmed character, cultivation, inventory, and home state.
6. THE SYSTEM SHALL make this loop available in an `.ipa` produced by the documented GitHub build.

## Requirement R19: iPhone-operable online services

**User Story:** As the owner without a computer, I want to deploy and check the private online services from an iPhone browser, so that the playable world does not depend on owning a computer.

### Acceptance Criteria

1. WHEN an authorized server deployment is started from GitHub or the chosen host's dashboard
   THE SYSTEM SHALL deploy the account service, valley server, and required configuration without a local computer.
2. WHEN a database schema change is deployed
   THE SYSTEM SHALL run versioned migration steps safely and report success or failure.
3. THE SYSTEM SHALL provide a phone-readable health view for the account service, valley server, and database connection without exposing secrets.
4. WHEN a newly deployed server version fails its health check
   THE SYSTEM SHALL preserve the last recoverable player data and support returning to the previous compatible version.
5. THE SYSTEM SHALL provide backups and restoration instructions suitable for the private prototype.
6. THE SYSTEM SHALL expose a clear spending limit or cost alert through the selected hosting providers.
7. BEFORE distributing a client that requires a new server protocol
   THE SYSTEM SHALL deploy a compatible server or reject the client with a plain-language version message.

## Requirement R20: Complete owner development controls

**User Story:** As the sole owner and developer, I want complete development and operations controls that work from my iPhone, so that I can build, diagnose, test, repair, and operate every implemented feature.

### Acceptance Criteria

1. THE SYSTEM SHALL provide a separate, phone-readable owner console and SHALL NOT include any developer menu or local diagnostic overlay in a distributable `.ipa`.
2. FOR EVERY implemented gameplay or service feature
   THE SYSTEM SHALL provide the owner with documented inspection, diagnostics, configuration, safe repair/reset, and verification controls—or a written reason that a control does not apply—before that feature task is considered complete.
3. THE SYSTEM SHALL keep the per-feature owner operations matrix in `docs/OWNER_OPERATIONS_SECURITY.md` current in the same change that adds or changes a feature.
4. THE FIRST PLAYABLE VERSION SHALL let the owner inspect and diagnose builds, releases, protocols, deployments, service/database/network health, performance, costs, logs, accounts, sessions, character creation, appearance, movement, camera, collision, streaming, combat, health, abilities, settings, gore definitions, cultivation, inventory, equipment, gathering, NPC schedules/memories, creatures, resources, duels, claims, persistence, migrations, write leases, security, and audit health.
5. THE FIRST PLAYABLE VERSION SHALL provide bounded controls for approved builds, deploy, drain, restart, migration, backup, tested restore, rollback, test invites, session revocation, test teleportation, unstuck/recovery, test spawning/removal, state repair, stuck-duel termination, claim repair, feature flags, emergency login/write pause, bad-event stop, and player notice as relevant to implemented features.
6. WHEN the owner views or changes state
   THE CONSOLE SHALL use plain English, show the exact environment, show the affected feature and target, and never require access to raw shell commands or arbitrary database queries.
7. THE SYSTEM SHALL maintain separate test and live controls and SHALL default development actions to test.
8. THE SYSTEM SHALL provide emergency controls to revoke all privileged sessions, pause new logins, pause valuable writes, drain or restart a zone safely, stop a bad event, and communicate a service notice.
9. THE SYSTEM SHALL make the owner console, service health, audit viewer, backups, deployment, rollback, and cost controls usable from an iPhone browser.

## Requirement R21: Sole-owner privileged-access security

**User Story:** As the sole owner, I want privileged tools to fail closed for every other person and client, so that nobody else can view a developer menu or execute a developer command.

### Acceptance Criteria

1. THE SYSTEM SHALL authorize privileged human access only for one configured owner subject using an origin-bound, phishing-resistant WebAuthn passkey with user verification; no game role, player recovery, repository invitation, or in-game action SHALL create or replace that subject.
2. WHEN the first owner is enrolled
   THE SYSTEM SHALL use a short-lived single-use challenge while no owner exists, bind it to the exact environment and console origin, invalidate it after use, and create no permanent master token.
3. WHEN an owner passkey is added, removed, replaced, or recovered
   THE SYSTEM SHALL require the documented high-assurance ceremony, rate limits, owner alerts, complete audit, and revocation of old privileged sessions and credentials.
4. WHEN an unauthorized, tester, moderator, copied app, modified app, expired session, revoked session, forged owner, replayed request, wrong-origin request, or recovery request attempts privileged access
   THE SYSTEM SHALL deny access before protected console assets or data are returned, leave game state unchanged, and record the attempt safely.
5. THE OWNER CONSOLE SHALL use exact-origin WebAuthn challenges, short-lived server-side sessions, Secure/HttpOnly/SameSite cookies, anti-CSRF protection, restrictive CORS and Content Security Policy, frame blocking, no state-changing `GET` requests, no production source maps, and no cached privileged pages.
6. WHEN the owner-control API forwards a command
   THE RECEIVING SERVICE SHALL independently verify a private workload identity and a short-lived replay-resistant envelope containing the owner, environment, audience, command, target, limits, nonce, expiry, request ID, and expected state version.
7. THE SYSTEM SHALL reject direct public calls to downstream privileged service endpoints and SHALL expose no permanent raw database or server-shell command through the console.
8. THE SYSTEM SHALL use a dormant, bounded maintenance executor for outage repair; it SHALL require owner re-authentication, maintenance mode, a fresh snapshot, a short-lived environment-specific identity, an exclusive lease, a versioned repair plan, and complete audit.
9. THE SYSTEM SHALL isolate test and live domains, databases, backup stores, secrets, keys, workload identities, command audiences, and sessions so changing a request parameter cannot retarget a test command to live.
10. WHEN a high-impact live command is requested
    THE SYSTEM SHALL require fresh owner verification, reason, dry-run result, affected-record count, expected state version, strict batch limit, recoverable snapshot, rollback plan, and explicit confirmation.
11. THE SYSTEM SHALL inventory GitHub, hosting, database, backup, DNS, identity, logging, monitoring, deployment, and distribution control planes; human access SHALL be owner-only and machine identities SHALL be short-lived and limited to one purpose and environment.
12. THE SYSTEM SHALL encrypt and integrity-check backups, keep owner identity/revocation/audit state outside gameplay restores, prove restoration in test, and invalidate all sessions, leases, pending commands, and short-lived service credentials after a restore.
13. WHEN any privileged read, denial, authentication, recovery, credential change, command, deployment, backup, restore, infrastructure action, or emergency action occurs
    THE SYSTEM SHALL write the complete event described in `docs/OWNER_OPERATIONS_SECURITY.md` to append-only storage outside the gameplay database and SHALL alert the owner for security-sensitive events.
14. IF the external audit sink cannot accept or durably queue an event
    THE SYSTEM SHALL fail closed for non-emergency privileged mutations; emergency safety commands SHALL use a signed queue that must upload before normal writes resume.
15. THE SYSTEM SHALL use the privileged command registry to run every authorization and state-unchanged case listed in `docs/OWNER_OPERATIONS_SECURITY.md` and SHALL block release if any route fails.
16. BEFORE release
    THE SYSTEM SHALL scan the repository, relevant history, `.ipa`, owner-console bundle, server artifacts, containers, build/deployment logs, backup metadata, and audit exports for secrets, developer UI, unsafe endpoints, source maps, or unintended client authority.
17. THE SYSTEM SHALL grant no other person developer access unless the owner explicitly changes this requirement.

## Requirement R22: Current zero-cost development stage

**User Story:** As the sole current tester, I want development to cost $0 for now, so that I can decide on a budget later without receiving an unexpected bill.

### Acceptance Criteria

1. UNTIL the owner explicitly approves a new budget
   THE SYSTEM SHALL NOT activate a paid plan, billable resource, automatic overage, purchase, or service that cannot enforce a zero-dollar limit.
2. BEFORE activating any free hosted service
   THE PROJECT SHALL verify its current free allowance, hard spending controls, sleep/expiry behavior, data handling, security limits, and deletion path in plain English.
3. IF a required hosted service cannot safely guarantee zero cost
   THE PROJECT SHALL keep its deployment prepared but inactive and SHALL NOT weaken security, privacy, backup, audit, or owner-only access requirements to obtain free hosting.
4. WHILE remote MMO services remain inactive
   THE PROJECT MAY produce an owner-only `.ipa` technical preview using an in-process authoritative simulation and local prototype data, provided it is clearly labeled as offline and cannot be mistaken for completed multiplayer or online persistence.
5. WHEN local and hosted modes share gameplay logic
   THE PROJECT SHALL keep authority boundaries behind the same versioned command interfaces so the zero-cost preview does not make future public clients authoritative.
6. THE PROJECT SHALL monitor included GitHub build usage and any selected free-service limits and SHALL stop or defer work before a charge can occur.
7. BEFORE asking the owner to approve spending
   THE PROJECT SHALL explain the reason, expected monthly and one-time costs, hard cap options, free limitations, and what can continue at $0.
8. THE long-term online MMO, few-player test, and 1,000-concurrent-player goals SHALL remain unchanged unless the owner changes them.
