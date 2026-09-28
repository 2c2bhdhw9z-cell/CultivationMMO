---
inclusion: always
---

# Cultivation MMO Project Principles

- Treat every product decision as provisional. The user may change anything at any time. Update all affected documents and implementation instead of defending an old decision.
- Use short, plain English when explaining work to the user. Do not expect the user to understand code.
- There is no fixed deadline. Prefer correctness and complete work over rushing.
- The owner tests with ESign, not TrollStore or SideStore, and is not planning around a seven-day signing limit.
- Consult `docs/GAME_VISION.md` before changing gameplay, architecture, content, or scope.
- Consult `docs/OWNER_OPERATIONS_SECURITY.md` before adding or changing any gameplay feature, service, developer tool, deployment, credential, backup, or privileged route.
- Consult `docs/COST_DECISIONS.md` before choosing, activating, scaling, or paying for any hosted service.
- Follow `docs/PUBLIC_REPOSITORY.md` for every source, workflow, log, build, artifact, release, fork, and visibility decision.
- Use `docs/TECHNICAL_BASELINE.md` for pinned engine, build, command-boundary, and provisional transport choices until a verified update replaces them.
- Update `docs/CONTROL_PLANE_INVENTORY.md` before adding or changing any provider, identity, secret, environment, backup, deployment, or release access.
- Do not add an external game asset or addon until `docs/TEMPORARY_ASSET_LICENSES.md` records and approves its exact license and redistribution rights.
- Build a third-person 3D open-world cultivation MMO, beginning with a small iPhone `.ipa` and a private online valley. The source repository and unsigned test build may be public; the online world is not.
- Preserve the long-term direction: deep freedom, meaningful consequences, living NPCs, territory building, reincarnation, immortality, multiple power systems, and eventual Omniverse-scale growth.
- Do not artificially equalize talent, birth, luck, power, opportunities, or outcomes. Apply fairness to consistent rules, security, moderation, payments, and essential protections.
- Keep progression classless but constrained by access, compatibility, resources, time, understanding, and danger.
- True in-world Systems are rare, unequal, limited powers. Ordinary game UI is not automatically a System.
- Use safe regions, agreed duels, and dangerous PvP regions. Dangerous-region defeat currently drops gathered materials, not equipped gear or permanent progress.
- Adult romance, marriage, and families may exist, but never include sexual scenes. Child characters cannot access romance, open PvP, or graphic gore.
- Support Off, Low, Medium, and Full gore settings. Gore changes presentation only.
- The current money rule is cosmetics and decorations only—never direct or indirect power. The first playable build has no purchases.
- The current development budget is $0 while the owner is the only tester. Never activate a paid plan, automatic overage, billable resource, larger GitHub runner, or purchase without the owner's explicit later approval.
- The source repository is public for free standard GitHub-hosted builds. Public readers/forks receive no write, game-owner, server, signing, or developer-tool access. Never commit secrets or private player data, and never add an automatic public workflow trigger.
- Railway Hobby is the owner's preferred future online-test candidate and may become permanent after testing. It is not active, purchased, permanently chosen, or approved for spending yet; follow `docs/COST_DECISIONS.md` and obtain a fresh explicit budget cap immediately before activation.
- Free services may be used only after confirming a hard zero-cost boundary and all security/privacy requirements; if that is impossible, prepare but do not activate the hosted service.
- Keep secrets, signing material, and private player data out of the repository.
- Do not pretend the whole MMO can be built at once. Deliver small playable foundations that can grow without discarding the north star.
- The online services, database, health checks, backups, cost alerts, and rollback controls must be operable from an iPhone browser; GitHub builds the app but does not host the MMO world.
- Every implemented feature must include the owner-only inspection, diagnostics, configuration, and safe repair/reset controls needed to operate it from an iPhone.
- Put privileged tools in a separate, strongly authenticated owner console. Compile developer menus out of tester/public builds, verify every privileged command on the server, audit every attempt, and grant no other person developer access unless the owner explicitly changes this rule.