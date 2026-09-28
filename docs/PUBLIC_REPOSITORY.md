# Public Repository Policy

> **Decision:** The owner asked to make this repository public so standard GitHub-hosted Actions runners can build the iPhone game at $0.

## What public means

Anyone can read and download:

- The game vision and roadmap
- Requirements, technical design, and tasks
- Source code and future non-secret assets
- Security architecture (but never credentials)
- Public workflow files, run logs, and published test builds
- Every commit in repository history

People may also fork the public repository. A fork is their copy; it does not give them write, owner, server, signing, or developer-tool access to this repository or game services.

Public does **not** mean open source. The repository uses the root `LICENSE`, which currently reserves the project owner's rights. Separate third-party terms remain in `THIRD_PARTY_NOTICES.md`.

## Why it is public

GitHub's [Actions billing guide](https://docs.github.com/en/billing/concepts/product-billing/github-actions) says standard GitHub-hosted runner use is free for public repositories. Larger runners remain billable even for public repositories and are forbidden for this project unless the owner later changes the budget.

## Build safeguards

Until the owner changes them, every iPhone workflow must:

1. Use only the explicit standard GitHub-hosted runner label `macos-latest`, which GitHub defines as its newest stable macOS image. Do not use `macOSrunner-latest`, `xcode-27`, a beta/preview image, or any `-large`/`-xlarge` label.
2. Use only owner-authorized manual `workflow_dispatch`; the owner may start it directly or instruct the recognized Kiro integration to dispatch it. No `push`, `pull_request`, schedule, issue, comment, fork, or external dispatch trigger.
3. Use a concurrency group allowing only one iOS build; cancel an older unfinished build.
4. Use a 30-minute job timeout.
5. Keep default workflow permissions read-only. The current single build job may have job-level `contents: write` only to create/update the prerelease, and the token is passed as an environment value only to the final release command—not to build commands.
6. Use only GitHub-owned actions pinned to full commit SHAs, or plain reviewed shell commands. The workflow must self-enforce this even if repository-level policy cannot be changed through the integration.
7. Use no larger runner, custom runner image, paid marketplace action, Git LFS, Codespaces, Actions cache, signing secret, or paid service for the zero-cost preview. Any later signed workflow or cache requires an explicit requirements and budget change.
8. Print exact runner-image, Xcode, Godot, and dependency versions and verify pinned SHA-256 downloads.
9. Never print environment dumps, credentials, tokens, certificates, profiles, private keys, or payment data.
10. Build an unsigned `.ipa` for owner-side ESign; no signing secret is allowed in the zero-cost preview workflow.
11. Put the test `.ipa` in a clearly labeled public GitHub prerelease. Never use Actions artifact/cache storage for this preview.
12. Show a plain warning that anyone can download a public test build.
13. Scan the repository, history, workflow, logs, and `.ipa` for secrets and developer UI before publishing.
14. Refuse to run if repository visibility is not public or the selected runner label is not exactly `macos-latest`.
15. Enable and verify GitHub private vulnerability reporting after the repository becomes public.

GitHub's [release documentation](https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases) currently permits release assets under 2 GiB and states no total-release-size or bandwidth limit. This must be rechecked before the workflow relies on it.

## Access boundary

- GitHub currently reports one repository collaborator: the owner.
- Public readers and fork owners receive no write permission.
- Only the repository owner—or the recognized Kiro integration acting on the owner's explicit instruction—can intentionally dispatch this repository's manual build.
- `GITHUB_TOKEN` defaults to read-only.
- No developer menu or credential is included in the `.ipa`.
- Future owner/server commands still require the separately authenticated owner-control system.

## Secrets and private data

Before public visibility, the complete tracked history was scanned. No actual token, password, key, certificate, profile, recovery code, payment detail, private contact detail, private endpoint, or secret-bearing historical file was found.

From now on:

- `.gitignore` blocks common secret/signing files, but ignore rules are only backup protection.
- Every staged change and build receives a secret scan.
- Never commit `.env` values, certificates, provisioning profiles, signing keys, owner recovery data, service credentials, or player data.
- Never place a secret in a workflow argument, log, artifact, release, issue, or commit message.
- Public workflows must treat pull-request and fork content as untrusted and must never expose secrets to it.

## Reversing the decision

The owner may make the repository private later. Public forks and copies may continue to exist, so changing visibility cannot make previously public history secret again. A return to private visibility also restores private-repository Actions billing rules and requires a new cost check before builds.

External documentation was checked on September 28, 2026. Source material was paraphrased for compliance with licensing restrictions.
