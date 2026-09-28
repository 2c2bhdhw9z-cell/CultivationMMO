# Cost Decisions

> **Status:** Living record. Prices and provider rules can change, so every amount must be rechecked before purchase or activation.

## Current approved budget

- **Approved now:** $0
- **Current testers:** Owner only
- **Paid services activated:** None
- **Automatic overage approved:** No
- **Permanent hosting provider chosen:** No

Development continues through local tools and included GitHub usage only after checking the remaining allowance. Nothing paid is activated until the owner explicitly approves it at the time it is needed.

## Preferred future online-test candidate: Railway Hobby

The owner currently prefers **Railway Hobby** when online testing becomes necessary and may keep Railway permanently if testing goes well. This is a preferred candidate—not a purchase, activation, permanent commitment, or current budget change.

An owner-provided Railway screenshot on September 28, 2026 showed:

- $5 minimum usage
- $5 of monthly usage credits
- Extra resource usage charged after the included credit is consumed
- One developer workspace
- Up to 5 GB storage
- Community support, a 99.9% availability target, seven-day log history, and global regions

Railway's official billing guide explains that Hobby is currently a $5 monthly minimum: the subscription covers the first $5 of resource use, while a month using more than that can cost more than $5. Railway also documents compute and agent hard limits; reaching a compute hard limit takes workloads offline to stop further compute usage. See [Railway billing](https://docs.railway.com/pricing/understanding-your-bill) and [Railway cost controls](https://docs.railway.com/pricing/cost-control).

Content from Railway documentation was rephrased for compliance with licensing restrictions.

## What Railway may host later

Subject to technical and cost checks, the first Railway test may include:

- Private account/API service
- Small authoritative valley server
- PostgreSQL database
- Separate owner-control service and console
- Private service-to-service networking

The exact service split remains provisional. A feature is not activated merely because Railway can host it.

## Required checks before subscribing

Before the owner is asked to activate Railway:

1. Recheck the official plan, prices, taxes, limits, and included usage.
2. Explain why online hosting is needed at that point and what still works at $0.
3. Estimate the likely one-time and monthly usage for the proposed test.
4. Ask the owner to approve a clear monthly maximum—not only the $5 minimum.
5. Confirm the dashboard's lowest selectable compute limit and the agent limit; verify they can enforce the owner-approved maximum, then set them before starting workloads.
6. Set lower usage alerts visible from the owner's iPhone.
7. Use low CPU/RAM/replica limits and private networking.
8. Use serverless sleep only where it will not corrupt or break the persistent MMO test.
9. Disable unnecessary preview deployments and duplicate environments.
10. Confirm backup, restore, deletion, security, log-retention, and owner-only access needs.
11. Confirm the nearest suitable region, iPhone latency, long-lived connection support, and expected network charges.
12. Deploy to the isolated test environment first.
13. Confirm how to stop every service and prevent further usage from the owner's iPhone.
14. Obtain the owner's explicit final approval immediately before subscription or activation.

If a reliable hard maximum cannot be enforced, Railway remains inactive until the owner knowingly approves that different risk. No security, privacy, audit, or backup requirement is weakened to reduce cost.

## Test before permanent use

Railway is not considered the permanent provider until the test measures:

- Stability and reconnect behavior
- Wi-Fi and cellular latency
- Server CPU and memory use
- Database and storage growth
- Network usage
- Backup and restore behavior
- Phone-only deployment and emergency control
- Real monthly cost
- How cleanly the system could move elsewhere

Long-term hosting for 1,000 or more simultaneous players will be reconsidered using real measurements. Railway may remain suitable, form one part of the system, or be replaced.

## Work that continues at $0

Without activating Railway, work can continue on:

- Game and server code
- Data and network-command design
- Local/headless server checks in the development workspace
- The owner-only offline technical-preview `.ipa`
- Touch movement and camera
- Graybox world, interaction, combat input, and meditation input
- Documentation, security boundaries, deployment templates, and cost estimates

The offline preview never counts as proof of online multiplayer, remote saving, Railway deployment, or the hosted owner console.

## Task 1 verification — September 28, 2026

### GitHub Actions

Repository evidence:

- The repository is private.
- GitHub Actions is enabled but has zero runs and zero artifacts.
- Default workflow-token access is read-only.
- No workflow, run, artifact, or GitHub environment exists yet.
- The integration received HTTP 403 when checking repository Actions secrets and deployment records, so those states are **unknown**, not assumed empty; the owner check must confirm there is nothing unrecognized.
- The Kiro GitHub integration cannot read the owner's personal billing plan, payment state, current shared allowance, or budget; the billing API returned HTTP 403.

GitHub's [Actions billing guide](https://docs.github.com/en/billing/concepts/product-billing/github-actions) currently lists 2,000 monthly minutes and 500 MB shared artifact/package storage for GitHub Free, but the owner's plan is not assumed. It lists standard macOS runner overage at $0.062 per minute. GitHub's [budget guide](https://docs.github.com/en/billing/how-tos/set-up-budgets) says an Actions budget can stop metered usage, but also warns that a new budget does not cover usage from before the budget was created.

**Decision:** No GitHub workflow may run until the owner completes `docs/OWNER_ACTIONS_CHECK.md`, including account plan, total/current/remaining allowance, reset date, payment-method presence without details, hard-stop budget, and unrecognized deployment/access checks. Early workflows will be manual only, use standard runners, run one at a time, time out after 30 minutes, keep `.ipa` artifacts for three days, and stop rather than spend beyond included usage.

### Railway current details

Railway's [current plan guide](https://docs.railway.com/reference/pricing/plans) lists a $0 Free plan with $1 monthly credit and a $5 Hobby subscription with $5 included resource usage. Hobby remains the preferred paid test choice, but neither plan is active for this project. Trial/free limits, verification, and network behavior are not assumed suitable merely because they cost $0.

Railway currently lists these usage rates:

- RAM: $10 per GB-month
- CPU: $20 per vCPU-month
- Network sent out: $0.05 per GB
- Volume storage: $0.15 per GB-month

A tiny private test is provisionally estimated at **$5–$15 per month plus possible tax**, depending mainly on actual memory, CPU, always-on time, database use, backups, and network traffic. This is not a quote or approved budget. One-time infrastructure setup is expected to be $0 unless a domain, Apple membership, or other purchase is later approved.

Railway's [cost-control guide](https://docs.railway.com/pricing/cost-control) documents separate compute and agent hard limits and lower alerts, but the official page checked did not state the lowest selectable compute limit. That exact floor must be read from the owner's Railway dashboard immediately before activation. If it is higher than the owner's approved maximum, Railway remains inactive. Railway Agent use should remain off or at the lowest enforceable limit, and server/service resource limits must be set. Hitting the hard compute limit takes workloads offline, which is acceptable for a private test but not a final public reliability plan.

Railway's [WebSocket guide](https://docs.railway.com/guides/sse-vs-websockets) supports the provisional first-online transport. Its [private-networking guide](https://docs.railway.com/private-networking) supports keeping database/service traffic off public endpoints.

Railway [volume backups](https://docs.railway.com/volumes/backups) are usage-billed, restore only inside the same project/environment, and deleting a volume also deletes its backups. Therefore Railway-only backups cannot satisfy the final separate-backup rule by themselves.

**Decision:** Railway remains inactive. A permanent decision waits for measured latency, memory, CPU, database, backup, phone-operation, and real monthly cost.

### Railway exit plan

Before Railway can become permanent:

1. Keep source, content definitions, schemas, and deployment definitions in GitHub rather than provider-only editors.
2. Use standard PostgreSQL and versioned migrations.
3. Prove a database export and restore into a separate test database.
4. Export required audit records to an independently protected store.
5. Keep player-uploaded or large world assets out of provider-only storage formats.
6. Document DNS changes and service endpoints without hard-coding Railway domains into saved characters.
7. Before leaving, stop new writes, take an integrity-checked export, verify it, move services, then remove Railway deployments and data from the owner's phone dashboard.
8. Confirm billing has stopped and retain only the minimum required security/audit evidence.

### Source note

External source content in this section was paraphrased for compliance with licensing restrictions. Prices and limits must be rechecked immediately before any activation.