# Technical Baseline

> **Recorded:** September 28, 2026
> **Status:** Task 1 provisional baseline. No paid service is active. Decisions remain changeable, but implementation should use this baseline until the documents are updated.

## Plain-English result

- Use **Godot 4.7.2-stable** and its simple GDScript language.
- Build the first game for the owner's iPhone with GitHub's standard macOS runner.
- Build a separate Linux headless version from the same Godot version for the future valley server.
- Keep the current build offline and $0 until GitHub's billing hard stop is confirmed from the owner's account page.
- For the first small online test, start with secure WebSockets (`wss://`). They are the simplest option that works through Railway's normal HTTPS entry point.
- Keep networking behind one command interface so the offline preview and future online server use the same game rules.
- Do not select external art, audio, fonts, models, or addons yet.

## 1. Pinned game-engine baseline

### Decision

| Part | Current choice |
|---|---|
| Engine | Godot `4.7.2-stable`, standard build |
| Engine source commit | `ed1daf0bf` |
| Script language | GDScript; no C# or Mono for the first build |
| Client renderer | Mobile renderer over Metal |
| First device architecture | iOS `arm64` |
| First deployment target | iOS 16.0 or newer |
| Prototype performance target | Sustained 30 FPS default on the owner's iPhone 17 Pro Max |
| Headless server | Same Godot 4.7.2 project, Linux x86_64 dedicated-server export with visuals stripped where safe |

Godot's [4.7.2 maintenance release](https://godotengine.org/article/maintenance-release-godot-4-7-2/) is the current stable release found during this check. The standard build is selected because the project uses GDScript and Godot describes C# iOS support as experimental in its [iOS export guide](https://docs.godotengine.org/en/stable/tutorials/export/exporting_for_ios.html).

Godot's current [system requirements](https://docs.godotengine.org/en/stable/about/system_requirements.html) list iOS 16 as the baseline for the Mobile/Metal path. This is a technical minimum for the early build, not the final public device-support promise.

### Pinned official downloads

| File | Purpose | SHA-256 |
|---|---|---|
| `Godot_v4.7.2-stable_macos.universal.zip` | macOS editor/export command | `c58a24e31d720be9d62f60cb5627c4e695fb72f21b0cfe1bc9ccaa9a3b3ba63e` |
| `Godot_v4.7.2-stable_export_templates.tpz` | iOS and Linux export templates | `f298490b8d44d934be425a5a65a51bf15f422428b229a06a6e11d9ffea248011` |
| `Godot_v4.7.2-stable_linux.x86_64.zip` | Linux editor/headless verification | `cadd3204e728a35d3f13adb7fd0d7902636b79f6b95c40c265eb73b6c35329e4` |

The files and digests come from the official [Godot 4.7.2 build release](https://github.com/godotengine/godot-builds/releases/tag/4.7.2-stable). A build must download only these exact files and fail before use if a digest differs.

### Headless server choice

Godot documents both `--headless` and a dedicated-server export mode. Dedicated-server exports can strip visual resources and add a server feature tag. The first server will use that mode rather than a graphical editor binary. See [Godot's dedicated-server export guide](https://docs.godotengine.org/en/stable/tutorials/export/exporting_for_dedicated_servers.html).

## 2. Pinned cloud-build baseline

### Runner and Xcode

| Part | Current choice |
|---|---|
| Runner class | Standard GitHub-hosted runner only; never a billable larger runner |
| Runner label | `macos-15` |
| Requested Xcode | `26.2` at `/Applications/Xcode_26.2.app` |
| Xcode build observed in image record | `17C52` |
| Export command | Godot command-line release export from a committed preset |
| Build target | Physical iPhone device, not Simulator |
| Artifact retention | Three days for early `.ipa` artifacts |
| Trigger | Owner-only manual run; no automatic push build |
| Concurrency | One iOS build at a time; a newer manual run cancels an older unfinished run |
| Job timeout | 30 minutes until measurements justify a different limit |

The selected image and Xcode were present in GitHub's official [`macos-15` runner-image record](https://github.com/actions/runner-images/blob/main/images/macos/macos-15-Readme.md) on the recorded date. GitHub updates hosted images, so every run must print the runner-image version, `xcodebuild -version`, SDK list, Godot version, and dependency digests. It must stop if Xcode 26.2 or the pinned Godot files are unavailable.

Godot requires macOS, Xcode, and matching export templates for iOS. It also requires a Team ID-shaped value and unique bundle identifier in the export settings. Godot's command-line exporter creates an iOS/Xcode ZIP; Xcode performs the device build. See [Godot iOS export](https://docs.godotengine.org/en/stable/tutorials/export/exporting_for_ios.html) and [command-line export](https://docs.godotengine.org/en/stable/tutorials/export/exporting_projects.html).

### ESign packaging status

The owner will sign/install with ESign. The intended Task 3 experiment is:

1. Export the Xcode project using the pinned Godot editor and templates.
2. Build an `iphoneos` Release app with Xcode while keeping signing material out of the repository.
3. Package the device `.app` under `Payload/` as an unsigned `.ipa`.
4. Let the owner sign and install that `.ipa` with ESign.

This exact unsigned path is a **hypothesis, not yet verified**. Godot's official guide covers export to Xcode, while GitHub's [Apple signing guide](https://docs.github.com/en/actions/guides/installing-an-apple-certificate-on-macos-runners-for-xcode-development) covers protected CI signing. Task 3 must prove which package ESign accepts before deeper gameplay is built. No Apple certificate, ESign certificate, provisioning profile, password, Team ID, or private key is requested or stored now.

## 3. GitHub Actions $0 evidence and gate

### Evidence collected automatically

As of September 28, 2026:

- `CultivationMMO` is private.
- Actions is enabled, but the repository has **zero workflow runs, zero artifacts, and zero configured environments**.
- Default workflow-token permission is read-only.
- Other private repositories cannot consume this repository as an Action.
- External actions are currently broadly allowed and full-SHA pinning is not required yet.
- Repository Actions secret names, deployment records, cache state, and any external deployment identity are not readable by this integration and remain unknown until the owner check.
- The connected GitHub integration cannot read the owner's personal plan, payment method, current account-wide allowance, or budget page; the billing API returned HTTP 403.

### Official limits, not account confirmation

GitHub's [current Actions billing guide](https://docs.github.com/en/billing/concepts/product-billing/github-actions) lists 2,000 included monthly minutes and 500 MB of shared artifact/package storage for GitHub Free, but the owner's plan could not be confirmed through the integration. The same guide lists standard macOS usage beyond an allowance at $0.062 per minute and explains that private-repository use is charged to the repository owner.

GitHub supports metered-product budgets that stop further Actions usage when the hard-stop option is enabled. GitHub warns that a newly created budget does not cover usage that occurred before the budget was created. See [GitHub budgets](https://docs.github.com/en/billing/how-tos/set-up-budgets).

### Hard gate before the first workflow

No macOS workflow may be committed or run until the owner confirms from `https://github.com/settings/billing` that:

1. Account plan name, total included Actions allowance, current usage, remaining allowance, and billing-cycle reset date are visible.
2. The owner has stated only whether a payment method is present, without sharing its details.
3. An Actions budget of `$0` for paid overage is active.
4. **Stop usage when budget limit is reached** is enabled.
5. Included-usage alerts at 90% and 100% are enabled.
6. Repository collaborators, installed apps, Actions secrets/variables, environments, and deployment records/connections contain nothing unrecognized.
7. The owner understands that reaching the included allowance will stop builds rather than charge money.

The owner should follow `docs/OWNER_ACTIONS_CHECK.md` and send a screenshot that shows the Actions usage/budget state but hides payment details. The integration must never ask for card numbers, recovery codes, passwords, certificates, or tokens.

Before Task 3, repository Actions policy should also be narrowed to required GitHub-owned actions, each action should use a full commit SHA, workflow permissions should remain read-only unless one explicit job needs more, and artifact retention should be kept short. GitHub documents these controls in [repository Actions settings](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository).

## 4. One authority boundary for offline and online play

The user interface never changes valuable game state directly. It sends a command to an authority interface.

```text
Touch/UI input
    -> GameplayCommandV1
        -> InProcessAuthorityAdapter (offline technical preview)
        OR
        -> WssAuthorityAdapter (future hosted valley)
            -> the same AuthorityCore validators and handlers
                -> GameplayResultV1 + WorldEventV1
```

### Command envelope V1

```json
{
  "protocol": 1,
  "type": "movement.intent",
  "command_id": "random-128-bit-id",
  "client_sequence": 42,
  "expected_state_version": 18,
  "payload": {}
}
```

The authenticated remote session supplies the actor identity. A client cannot choose another actor in the payload.

### Result/event envelope V1

```json
{
  "protocol": 1,
  "type": "movement.confirmed",
  "event_id": "server-generated-id",
  "command_id": "matching-command-id",
  "server_sequence": 87,
  "state_version": 19,
  "status": "accepted",
  "error_code": null,
  "payload": {}
}
```

### V1 rules

- Use strict UTF-8 JSON for the small prototype because it is easy to inspect and works with Godot's built-in [JSON support](https://docs.godotengine.org/en/stable/classes/class_json.html).
- Cap one message at 16 KiB and reject unknown top-level fields, unknown command types, invalid ranges, excessive nesting, non-finite numbers, and malformed text.
- Never decode network-provided engine objects. `SceneMultiplayer.allow_object_decoding` remains false because Godot warns that decoded objects can execute code. See [SceneMultiplayer](https://docs.godotengine.org/en/stable/classes/class_scenemultiplayer.html).
- Use string identifiers and explicitly validate integer fields because JSON does not preserve a separate integer number type in Godot.
- The authority owns time, randomness, physics validation, inventory, damage, cultivation outcomes, claims, and state versions.
- Cache completed command IDs for safe retry. A repeated ID returns the original result and cannot grant a second reward.
- Reject stale state versions when an action could overwrite newer state.
- The in-process adapter must serialize and parse through the same V1 codec and call the same validators as the remote adapter; it cannot receive a private shortcut to mutate state.
- Keep codec and transport interfaces replaceable. A compact binary codec can replace JSON later without changing command meaning.

This gameplay command boundary is separate from the more strongly protected owner/developer command boundary.

## 5. Real-time transport comparison

| Question | Secure WebSocket (`wss://`) | Godot ENet/UDP |
|---|---|---|
| Railway fit | Works through Railway HTTPS/WebSocket public networking | Railway currently documents HTTP/HTTPS and raw TCP ingress, not UDP ingress |
| Cellular/NAT fit | Usually passes through port 443 and normal proxies | Can work well, but some networks and hosts restrict UDP |
| Encryption | Standard TLS with hostname checking | Needs a separately designed secure/authenticated layer |
| Delivery | Reliable and ordered; stale traffic can delay newer traffic | Supports low-latency unreliable, ordered, reliable, and separate channels |
| First implementation | Smallest operational setup | More hosting and security work |
| Action-game ceiling | Adequate for a tiny prototype; must be measured | Better long-term latency control when an appropriate host exists |

Godot supports WebSocket and ENet peers through the same multiplayer abstraction. Its documentation explains the TCP/UDP tradeoffs and server-authoritative security model in [high-level multiplayer](https://docs.godotengine.org/en/stable/tutorials/networking/high_level_multiplayer.html). Godot's [WebSocket peer](https://docs.godotengine.org/en/stable/classes/class_websocketmultiplayerpeer.html) verifies host certificates for `wss://`; its [ENet peer](https://docs.godotengine.org/en/stable/classes/class_enetmultiplayerpeer.html) uses UDP.

Railway identifies WebSockets as suitable for multiplayer games, supports their HTTP upgrade, and exempts them from request timeouts, while still warning that deploys and network changes can disconnect them. See [Railway's WebSocket comparison](https://docs.railway.com/guides/sse-vs-websockets). Railway also supports [private service networking](https://docs.railway.com/private-networking), but that does not make the player's public connection private.

### Provisional first-online choice

- HTTPS for sign-in/bootstrap.
- One authenticated `wss://` connection for valley commands and events.
- TLS hostname verification must remain enabled.
- Pass the short-lived access token in a protected handshake header, never the URL or logs.
- Authentication timeout: 5 seconds.
- Heartbeat while foregrounded: 15 seconds.
- Reconnect delays: approximately 1, 2, 4, 8, then at most 15 seconds, with random jitter.
- Treat iOS backgrounding as a likely disconnect; on return, reauthenticate and restore only server-confirmed state.
- Authority simulation: provisional 30 ticks/second.
- Client movement intents: at most 20/second.
- Server state updates: provisional 15/second, with sequences so stale movement is ignored.
- Important inventory/cultivation/build events are idempotent and acknowledged.

These values are starting limits, not permanent performance promises.

### Required iPhone transport spike

Task 6 must measure the same tiny authenticated movement scenario on the owner's iPhone using Wi-Fi and cellular. Record date, chosen Railway region, five-minute sample count, median and 95th-percentile round-trip time, jitter, disconnects, reconnect time, foreground/background behavior, received/sent bytes, queue growth, server CPU/RAM, and battery/heat observations.

WSS remains only provisional until that test. If it produces harmful delayed-input queues or poor combat response, choose a UDP-capable host and test ENet rather than hiding the problem.

## 6. Hosting disposition

- No online game host, database, identity service, audit store, monitoring service, DNS domain, or owner console is active.
- Railway Hobby remains the owner's preferred future paid-test candidate, not a permanent decision.
- Railway's [cost-control guide](https://docs.railway.com/pricing/cost-control) documents compute and agent hard limits, but its public page checked on September 28, 2026 did not state the lowest selectable compute hard-limit value. Therefore a $5 ceiling must not be promised. The exact dashboard floor must be checked immediately before activation; if it exceeds the owner's approved maximum, Railway remains inactive.
- Railway activation remains blocked until the user chooses a budget later and approves the exact maximum immediately before activation.
- The offline technical preview can proceed without Railway.

Current Railway details and the required activation checklist are in `docs/COST_DECISIONS.md`.

## 7. Temporary content baseline

No third-party art, models, animation, audio, fonts, shaders, code addons, or sample projects are selected. The first graybox uses only project-authored primitive meshes, flat materials, simple generated tones if needed, and Godot's built-in capabilities.

Godot is MIT licensed and allows commercial use, but its copyright and license notice must accompany a distributed game. See [Godot's license](https://godotengine.org/license/). The project ledger is `docs/TEMPORARY_ASSET_LICENSES.md`.

## 8. Decisions deliberately left open

- Whether the ESign path accepts the first unsigned package
- ESign certificate Team ID and final bundle identifier
- Final GitHub account plan and remaining included Actions allowance
- Final online transport after real iPhone measurements
- Permanent host and regions
- Public minimum iPhone/iOS support
- Final binary network codec and multi-region protocol
- Public identity, App Store signing, and release accounts

Leaving these open is safer than inventing answers. Each has a clear later verification task.

## Source note

External documentation was checked on September 28, 2026. Source material was paraphrased for compliance with licensing restrictions. Prices, runner images, plans, and provider behavior must be rechecked before use.
