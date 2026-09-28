# Recommended Owner Account Safety Check

This is no longer a blocker for the first build once the repository is public. GitHub's [Actions billing guide](https://docs.github.com/en/billing/concepts/product-billing/github-actions) says standard GitHub-hosted runner use is free for public repositories.

The owner-provided screenshot on September 28, 2026 confirmed:

- Account plan: GitHub Free
- The account page showed no payment currently due
- Included usage shown was greater than metered usage shown

The screenshot did not show the Actions-specific tab or private account-security settings. Those details are unnecessary for free public standard-runner compute, but the safety checks below remain strongly recommended before any signing secret, paid service, online deployment, or owner console is added.

## Recommended GitHub account check

Open **GitHub Settings → Password and authentication** and confirm privately:

- A passkey is enabled.
- Two-factor authentication is enabled.
- Recovery codes are stored securely somewhere other than only this iPhone, when possible.

Open the repository's **Settings → Collaborators**, **Secrets and variables → Actions**, **Environments**, deployment history/connections, and installed-app/access pages. Confirm nothing is unfamiliar.

Do not send Kiro a password, recovery code, card detail, passkey, token, signing certificate, provisioning profile, or private key. A screenshot is not needed unless a later paid/private build decision specifically requires one.

## Public-build rule

The first workflow may run only after GitHub confirms the repository is public and the workflow itself passes the safeguards in `docs/PUBLIC_REPOSITORY.md`:

- Owner-authorized manual trigger only
- Standard runner only
- No larger runner
- No cache
- No signing secret
- No secret-bearing logs
- Public test-build warning
- No paid external service

If the repository becomes private again, builds stop until private-repository usage and a $0 hard-stop budget are rechecked.

## Current status after the visibility switch

- Repository visibility: public and API-verified
- Repository collaborators with write access: owner only
- Repository Actions runs: 0
- Repository Actions artifacts: 0
- Repository workflows: none
- Paid project service activated: none
- Railway activated: no
- Public standard-runner build: permitted after Task 3 adds and verifies the required workflow safeguards
- Private vulnerability reporting: not yet enabled; required before Task 3 publishes a workflow/build because the integration received HTTP 403

External documentation was checked on September 28, 2026. Source material was paraphrased for compliance with licensing restrictions.
