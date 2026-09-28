# External Control-Plane Inventory

> **Recorded:** September 28, 2026
> **Rule:** Human developer/operations access is owner-only. Inactive means no project, service, secret, or billable resource has been created for that provider role.

This document records systems capable of changing source, builds, signing, hosting, data, identity, logs, or releases. It contains no passwords, tokens, certificates, private keys, recovery codes, or private billing information.

## Current inventory

| Control plane | Status | Human access | Machine access | Audit/evidence | Cost state | Disable, revoke, export, or delete |
|---|---|---|---|---|---|---|
| GitHub personal account `2c2bhdhw9z-cell` | Active; security/billing details not visible to the integration | Owner only is required; passkey/2FA and recovery confirmation still needed from owner | Kiro's authorized GitHub integration can work in selected repositories | Git history, repository events, account security log; owner must inspect account access | Account plan and account-wide Actions allowance could not be read by API | Owner can revoke installed applications/sessions and use GitHub account recovery; never share recovery material |
| Private repository `2c2bhdhw9z-cell/CultivationMMO` | Active | Repository owner; no collaborator is intentionally added | Kiro integration; future read-only workflow token | Commit history; default branch `main` | Repository storage only; no Actions runs or artifacts yet | Remove access in GitHub settings; repository export is a normal Git clone |
| GitHub Actions | Enabled but unused; runs/artifacts/environments are zero, while secret/cache/deployment state is not readable by this integration | Repository owner can manually start/cancel future workflow | Default `GITHUB_TOKEN` is read-only; any other stored identity is unknown until owner check | Zero runs and zero artifacts at recording time | **Blocked:** personal billing budget/allowance unavailable through API; no workflow may run until owner verifies $0 hard stop | Disable Actions in repository settings; cancel runs; delete artifacts; remove secrets/environments |
| GitHub-hosted macOS runner | Inactive | No interactive human access | Ephemeral GitHub runner only during an approved job | Exact image/Xcode/tool versions must be logged | Standard runner only; included allowance unknown for this account | Cancel workflow; runner VM is destroyed after job |
| Godot upstream release downloads | Selected dependency, not an account | Public downloads | Future build downloads exact pinned files | SHA-256 pins in `TECHNICAL_BASELINE.md` | $0 download; GitHub minutes still apply during builds | Change version only through reviewed baseline update; remove cache/artifacts |
| Kiro GitHub integration | Active for repository work | Owner controls installation/authorization | Kiro service credential; never exposed to game or workflow | Git commits and GitHub installation settings | No game-hosting service activated | Revoke through GitHub installed-application settings; reconnect only by owner choice |
| ESign and owner's signing certificate | Owner-managed outside repository | Owner only | No project machine access now | Successful installation will be owner-confirmed; certificate details are not logged | Existing owner arrangement; no project purchase approved | Owner manages/revokes certificate in their signing source; never send certificate/private key in chat or repository |
| Apple Developer / App Store Connect | Not configured for this project | None recorded | None | None | $0 project commitment now | Nothing to revoke; public release decision is later |
| Railway account/workspace | Not created or connected for this project | None | None | Official terms only; no test results | $0 now; Hobby is a future preference, not approved | No project/data exists; future project must have phone-visible stop/delete controls |
| Railway game/API services | Inactive | None | None | None | $0 | No service exists |
| Railway PostgreSQL/volume/backups | Inactive | None | None | None | $0 | No database or stored data exists |
| Owner console and owner-control API | Designed, not deployed | Future single owner subject only | Future narrow service identities | Future append-only external audit | $0; inactive | No service exists; future emergency revoke-all is mandatory |
| Game identity/passkey provider | Not selected | None | None | None | $0; inactive | No identity/data exists |
| External audit/anchor store | Not selected | None | None | None | $0; inactive | No audit data exists yet |
| Logs, monitoring, and alerts | Not selected | None | None | Local/Kiro/GitHub logs only | $0; inactive for game runtime | No service/data exists |
| Backup/object storage | Not selected | None | None | Git is current documentation backup; no game data exists | $0; inactive | No service/data exists |
| DNS/domain provider | Not selected | None | None | None | $0; inactive | No domain exists |
| Public app distribution/store | Not configured | None | None | None | $0; inactive | No listing/build exists |

## Immediate blockers before the first GitHub iOS workflow

The owner must confirm, without sharing secrets:

- GitHub account passkey or equivalent phishing-resistant sign-in is enabled.
- GitHub 2FA/recovery is configured and recovery codes are stored somewhere separate from the iPhone when possible.
- GitHub Actions current usage, billing-cycle date, and `$0` paid-overage budget are visible.
- `Stop usage when budget limit is reached` is enabled for Actions.
- Included-usage alerts are enabled.
- GitHub repository access lists only the owner and recognized Kiro integration.
- Repository Actions secrets, variables, environments, deployments, and installed apps contain nothing unrecognized.

The safest evidence is a screenshot of the GitHub Actions budget/usage page with payment details hidden, plus a simple yes/no confirmation for passkey, 2FA, and safely stored recovery codes. Never send the codes themselves. GitHub explains the phishing resistance of [passkeys](https://docs.github.com/en/authentication/authenticating-with-a-passkey/about-passkeys) and recommends securely storing [2FA recovery methods](https://docs.github.com/en/authentication/securing-your-account-with-two-factor-authentication-2fa/configuring-two-factor-authentication-recovery-methods).

## Planned owner identity defaults

These values are defaults for the future owner console. They remain inactive until a provider and exact HTTPS domains are selected.

| Setting | Test default | Live default |
|---|---:|---:|
| Human owner subjects | Exactly 1 | Exactly 1, same human but separate credential registration |
| Enrollment challenge | 32 random bytes; single use; 10-minute expiry | Same, created only while no live owner exists |
| Enrollment attempts | 5 per 15 minutes, then 24-hour lock and owner alert | 3 per 24 hours, then manual recovery halt |
| Required owner factor | Origin-bound WebAuthn passkey with user verification | Separate origin-bound WebAuthn passkey with user verification |
| Ordinary privileged-session idle limit | 10 minutes | 5 minutes |
| Absolute privileged-session limit | 60 minutes | 30 minutes |
| Fresh authentication age for sensitive read/change | 5 minutes | 3 minutes |
| High-impact confirmation validity | One command only | One command only |
| Recovery package | 256-bit random secret, shown once, encrypted/offline outside phone | Separate 256-bit secret, shown once, encrypted/offline outside phone |
| Lost-phone recovery delay | 1 hour with alerts | 24 hours with alerts and cancellation window |
| Failed recovery attempts | 5 in 24 hours, then 24-hour lock | 3 in 24 hours, then 72-hour lock and incident mode |
| Command envelope expiry | 30 seconds | 15 seconds |
| Maintenance identity expiry | 15 minutes | 10 minutes |
| Default repair batch | At most 100 records | At most 25 records |

No console deployment may begin while its exact test/live domain, WebAuthn RP ID, owner alert destination, and offline recovery storage method remain blank.

## Credential rotation ceremony

1. Owner signs into the exact environment with the existing passkey.
2. Owner performs fresh user verification no more than three minutes before a live change.
3. Console displays the environment and a credential-change summary.
4. Owner proves possession of the separate recovery package for live credential changes.
5. New passkey is registered and tested before the old one is disabled.
6. Test changes activate immediately; live changes wait 24 hours and send alerts.
7. Owner can cancel during the delay with the old passkey or recovery package.
8. Activation revokes every privileged session, pending command, unused challenge, and old credential.
9. External audit anchor must confirm the event before new live privileged work is allowed.
10. The owner confirms access, then the old passkey record is removed.

Removing the final working passkey is forbidden outside lost-device recovery.

## Lost-phone recovery ceremony

1. Open only the exact bookmarked owner-console origin on the replacement device.
2. Select recovery; do not use player-account or support recovery.
3. Supply the offline recovery package through a one-time encrypted challenge.
4. After three failed live attempts in 24 hours, lock recovery for 72 hours.
5. Announce the pending recovery through every configured owner alert channel.
6. Wait 24 hours for live; allow cancellation with any still-valid passkey or recovery package.
7. Revoke all passkeys, owner sessions, pending commands, maintenance identities, and service command envelopes.
8. Register and test a new passkey.
9. Rotate the recovery package and relevant short-lived service trust.
10. Anchor the recovery audit externally before reopening privileged changes.

## Environment isolation map

No runtime environment exists yet. When activated:

- Test and live use different domains, WebAuthn RP IDs, projects, databases, backup stores, encryption keys, service identities, command audiences, sessions, and artifacts.
- A build has one immutable environment identifier.
- Test commands cannot accept a live target; live services reject test identities and audiences.
- Content moves from test to live through a versioned promotion record, never by changing an environment request field.
- Dangerous controls open on test by default. Live requires a separately authenticated live session.

## Private-prototype backup defaults

These defaults begin only when persistent online data exists and a budget/provider is approved:

- Database snapshot every 24 hours; keep seven daily copies.
- Weekly encrypted export; keep four copies in a separate approved store.
- Pre-migration and pre-restore snapshot; keep at least 14 days.
- Audit records and anchors: keep at least one year and never roll them back with gameplay data.
- Target recovery point: no more than 24 hours of private-test data loss.
- Target restore time: four hours for the private test.
- Verify backup integrity after creation; prove one test-environment restore every month and before a live restore.
- After restore, revoke all player/owner sessions, write leases, pending commands, and temporary service credentials.
- Backups stay inactive if their cost or separate-store security cannot fit the approved budget.

These private-test values must be raised before public launch based on measured risk and player expectations.

## Dormant maintenance defaults

1. Owner reauthenticates on the exact environment.
2. Console shows a dry run and enters maintenance/read-only mode.
3. A fresh integrity-checked snapshot must finish before repair.
4. New logins and valuable writes stop; active zones drain for at most five minutes.
5. Executor receives a single-environment identity valid for at most 10 minutes live or 15 minutes test.
6. It acquires one exclusive maintenance lease and invalidates stale zone leases.
7. It runs only a committed, versioned repair plan—never phone-entered shell text or SQL.
8. Default live batch is 25 records; larger work needs a new dry run and confirmation.
9. A repair exceeding its time, version, batch, or state precondition aborts and rolls back.
10. Result, before/after versions, snapshot, parameters, and actor are externally audited.
11. Temporary identity and lease are destroyed.
12. Normal writes resume only after health and audit checks pass.

## Review cadence

Update this inventory:

- Before the first workflow
- Before creating any provider account/project
- Before adding any secret or machine identity
- Before moving from offline to online
- Before any paid activation
- Before public testing or release
- After any account, security, billing, or provider change
- After any suspected incident

External documentation was checked on September 28, 2026. Source material was paraphrased for compliance with licensing restrictions.
