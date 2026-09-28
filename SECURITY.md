# Security Policy

## Current stage

CultivationMMO is an early design and prototype project. No public game server, player database, owner console, signing key, or production service exists yet.

## Reporting a security problem

Do not publish a possible vulnerability, credential, private player data, or exploit in an issue, discussion, pull request, or public chat.

Use GitHub's **private vulnerability reporting** flow after it is enabled and verified for this public repository:

[Privately report a security vulnerability](https://github.com/2c2bhdhw9z-cell/CultivationMMO/security/advisories/new)

If the private-reporting page is not available, do not post sensitive details publicly. The repository must not publish a build until the owner enables that reporting route.

## Important boundaries

- Public source access is not developer or owner access.
- No player action may unlock developer privileges.
- Never commit or send passwords, tokens, certificates, provisioning profiles, private keys, recovery codes, or payment information.
- Test only systems and accounts you own or are explicitly authorized to test.

The detailed owner-control design is public in `docs/OWNER_OPERATIONS_SECURITY.md`, but no credential, private endpoint, or recovery material belongs in this repository.
