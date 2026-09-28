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
5. Confirm compute and agent hard limits can enforce that maximum, then set them before starting workloads.
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
