# Technical Baseline

> **Recorded:** September 28, 2026
> **Status:** Task 1 provisional baseline. No paid service is active. Decisions remain changeable, but implementation should use this baseline until the documents are updated.

## Plain-English result

- Use **Godot 4.7.2-stable** and its simple GDScript language.
- Build the first game for the owner's iPhone with GitHub's standard macOS runner.
- Build a separate Linux headless version from the same Godot version for the future valley server.
- Keep the current build offline and $0 by using only an owner-authorized standard runner after public visibility is confirmed; the owner may ask the recognized Kiro integration to dispatch it. No larger runner, cache, signing secret, or paid service.
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

The files and digests come from the official [Godot 4.7.2 build release](https://github.com/godotengine/godot-builds/releases/tag/4.7.2-stable). A build must download only these exact files and fail before use if a digest differs. Because the repository and logs are public, checksums and package labels are public while no signing or account secret may enter the workflow.

### Headless server choice

Godot documents both `--headless` and a dedicated-server export mode. Dedicated-server exports can strip visual resources and add a server feature tag. The first server will use that mode rather than a graphical editor binary. See [Godot's dedicated-server export guide](https://docs.godotengine.org/en/stable/tutorials/export/exporting_for_dedicated_servers.html).

### Task 2 implementation baseline

| Part | Current choice |
|---|---|
| Godot project root | Repository root `project.godot` so client and zone share `shared/` authority code |
| Client main scene | `client/scenes/ui/boot.tscn` |
| Headless zone scene | `server/zone/main.tscn` |
| Local service language | Go `1.25.1`, pinned in `server/go.mod` |
| Go dependencies | Standard library only; no external module |
| Local API addresses | Account API `127.0.0.1:8080`; owner-control `127.0.0.1:8081` |
| Owner-console source | Vanilla HTML/CSS; no JavaScript dependency, package manager, CDN, or source map |
| Database | PostgreSQL remains intended, but exact version, driver, host, and migration tool are unselected and inactive |
| Privileged registry | V1 machine-readable registry, locked with zero commands |
| Audit | Interface plus deliberately unavailable sink; real mutation remains impossible |

The root Godot layout prevents copied client/server authority logic. Future export presets must explicitly select the client or dedicated-server entry point and prove that Go services, owner-console files, documentation, and protected source are absent from the `.ipa`.

Go 1.25.1 is available in the current development environment and is used only for dependency-free local service shells. Go binary distribution must retain the Go license notice recorded in `THIRD_PARTY_NOTICES.md`. PostgreSQL implementation waits for Task 5 rather than inventing a version or schema in Task 2.

## 2. Pinned cloud-build baseline

### Runner and Xcode

| Part | Current choice |
|---|---|
| Runner class | Standard GitHub-hosted runner only in the public repository; never a billable larger runner |
| Runner label | `macos-latest` |
| Requested Xcode | Default newest stable Xcode on `macos-latest`; currently `26.6` at `/Applications/Xcode.app` |
| Xcode build observed in image record | `17F113` |
| Export command | Godot command-line release export from a committed preset |
| Build target | Physical iPhone device, not Simulator |
| Build delivery | Clearly labeled public prerelease asset; no Actions artifact/cache storage |
| Trigger | Owner-authorized manual run; owner may explicitly ask recognized Kiro integration to dispatch; no automatic trigger |
| Concurrency | One iOS build at a time; a newer manual run cancels an older unfinished run |
| Job timeout | 30 minutes until measurements justify a different limit |

GitHub's [hosted-runner reference](https://docs.github.com/en/actions/reference/runners/github-hosted-runners) defines `macos-latest` as the newest stable standard macOS image, although it may not equal Apple's newest release. At the recorded time it maps to the standard Apple-silicon macOS 26 image. The official [`macos-26` Apple-silicon image record](https://github.com/actions/runner-images/blob/main/images/macos/macos-26-arm64-Readme.md) lists macOS 26.6.2 and Xcode 26.6 (`17F113`) as default. Xcode 27 is marked public preview and is not selected.

The rolling alias is intentional because the owner wants the newest stable runner. Every run must print the resolved OS, architecture, runner-image version, `xcodebuild -version`, SDK list, Godot version, and dependency digests. If `macos-latest` advances and the pinned Godot build no longer works, the workflow fails and the baseline is reviewed; it must not silently select a beta, preview, larger, or older runner.

Godot requires macOS, Xcode, and matching export templates for iOS. It also requires a Team ID-shaped value and unique bundle identifier in the export settings. Godot's command-line exporter creates an iOS/Xcode ZIP; Xcode performs the device build. See [Godot iOS export](https://docs.godotengine.org/en/stable/tutorials/export/exporting_for_ios.html) and [command-line export](https://docs.godotengine.org/en/stable/tutorials/export/exporting_projects.html).

### ESign packaging status

The owner will sign/install with ESign. The intended Task 3 experiment is:

1. Export the Xcode project using the pinned Godot editor and templates.
2. Build an `iphoneos` Release app with Xcode while keeping signing material out of the repository.
3. Package the device `.app` under `Payload/` as an unsigned `.ipa`.
4. Let the owner sign and install that `.ipa` with ESign.

This exact unsigned path is a **hypothesis, not yet verified**. Godot's official guide covers export to Xcode, while GitHub's [Apple signing guide](https://docs.github.com/en/actions/guides/installing-an-apple-certificate-on-macos-runners-for-xcode-development) covers protected CI signing. Task 3 must prove which package ESign accepts before deeper gameplay is built. No Apple certificate, ESign certificate, provisioning profile, password, Team ID, or private key is requested or stored now.

## 3. GitHub Actions $0 evidence and gate

### Public visibility evidence

GitHub's API confirmed `visibility: public` and `private: false` on September 28, 2026. The collaborator API listed only `2c2bhdhw9z-cell` with write/administrative access.

As of September 28, 2026:

- The owner-provided billing screenshot shows GitHub Free, included usage covering the displayed metered usage, and no payment currently due.
- The owner approved making the full repository and history public for free standard-runner builds.
- Before the visibility switch, every historical Git blob was scanned with no actual credential, signing material, payment detail, private contact detail, private endpoint, binary, or secret-bearing revision found.
- GitHub reports one collaborator: the repository owner.
- Actions is enabled, but the repository has zero workflow runs, zero artifacts, zero configured environments, and no committed workflow.
- Default workflow-token permission is read-only.
- Repository-level selected-action/full-SHA enforcement could not be changed by the integration (HTTP 403); the future workflow must contain only exact reviewed GitHub-owned action SHAs and not rely on that setting.

The public repository intentionally exposes the game vision, roadmap, security architecture, device/ESign workflow, provider preferences, source, workflow logs, and published technical previews. Public readers and fork owners receive no write or game-developer access.

### Official public-runner rule

GitHub's [Actions billing guide](https://docs.github.com/en/billing/concepts/product-billing/github-actions) says standard GitHub-hosted runners are free for public repositories. Larger runners remain charged even for public repositories and are forbidden for this project. Actions artifact and cache storage must not be treated as unlimited merely because runner compute is free.

GitHub's [release documentation](https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases) currently permits each release asset under 2 GiB and states no total-release-size or bandwidth limit. The first unsigned `.ipa` will therefore use a clearly labeled public prerelease asset instead of Actions artifact storage. Anyone will be able to download it.

### Gate before the first workflow

The first workflow may run only after:

1. GitHub confirms repository visibility is public.
2. The committed workflow has only owner-authorized `workflow_dispatch`; the owner starts it directly or explicitly asks the recognized Kiro integration—no push, pull-request, schedule, issue, comment, fork, or external trigger.
3. It uses only rolling standard `macos-latest`, the newest stable default Xcode on that image, and rejects any beta/preview Xcode or unapproved runner label.
4. It uses one-build concurrency, cancellation of an older unfinished run, and a 30-minute timeout.
5. It uses no Actions cache, paid service, larger runner, custom runner image, or signing secret.
6. GitHub-owned actions are pinned to full commit SHAs and the workflow token stays read-only except for a narrow release-upload step.
7. The workflow and public log pass credential, private-data, environment-dump, and developer-menu review.
8. The output and release page clearly say `Offline Technical Preview`, unsigned, public, and not completed online play.

If the repository becomes private again, the workflow must refuse to build until private-repository usage and a $0 hard-stop budget are rechecked.

The account passkey, 2FA, recovery, and unrecognized-access review in `docs/OWNER_ACTIONS_CHECK.md` remains strongly recommended before any secret, online deployment, or owner console is added. It is no longer a public standard-runner compute blocker.

## 4. One authority boundary for offline and online play

The user interface never changes valuable game state directly. It sends a command to an authority interface. Public source visibility does not make the player's app or a fork authoritative.

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
- The current per-actor cache makes immediate retries of the non-mutating skeleton ping return the original result. It limits each actor to 256 entries and one core to 64 actor caches; Task 6 must call `release_actor` on disconnect/expiry. Before any valuable handler is registered, a durable idempotency ledger must cover cache eviction, process restart, and zone transfer so a repeated ID cannot grant a second reward.
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
- Final online transport after real iPhone measurements
- Permanent host and regions
- Public minimum iPhone/iOS support
- Final binary network codec and multi-region protocol
- Public identity, App Store signing, and release accounts

Leaving these open is safer than inventing answers. Each has a clear later verification task.

## Source note

External documentation was checked on September 28, 2026. Source material was paraphrased for compliance with licensing restrictions. Prices, runner images, plans, and provider behavior must be rechecked before use.
