# One Owner Check Before the First iPhone Build

Kiro's GitHub connection cannot see private billing or account-security settings. The owner must do this short check on the iPhone before any GitHub iOS workflow runs.

## GitHub cost check

1. Open [GitHub billing](https://github.com/settings/billing).
2. Note the account plan name.
3. Note the total included Actions allowance, current usage, remaining allowance, and reset date.
4. Note only whether a payment method is present—**do not show or send its details**.
5. Open **Budgets and alerts**.
6. Find or create the **Actions** product budget.
7. Set paid overage to **$0**.
8. Turn on **Stop usage when budget limit is reached**.
9. Turn on included-usage alerts at 90% and 100%.
10. Open the repository's **Settings → Collaborators**, **Secrets and variables → Actions**, **Environments**, deployment history/connections, and installed-app/access pages.
11. Confirm only your account and the recognized Kiro connection have access, and that no unknown secret, variable, environment, deployment record/connection, or app exists.
12. Send Kiro a screenshot showing the plan, Actions allowance/usage, and budget status, with every payment detail hidden. Tell Kiro only **yes** or **no** for whether a payment method exists.

GitHub says metered products such as Actions can use a hard-stop budget, and warns that a new budget does not apply to usage that happened before it was created. See [GitHub budget instructions](https://docs.github.com/en/billing/how-tos/set-up-budgets).

## GitHub account-safety check

Open **GitHub Settings → Password and authentication** and confirm only these yes/no facts:

- A passkey is enabled.
- Two-factor authentication is enabled.
- Recovery codes are saved securely somewhere other than only this iPhone, when possible.
- No person, app, secret, variable, deployment connection, or environment you do not recognize has account or repository access.

Do **not** send Kiro a password, recovery code, card detail, passkey, token, signing certificate, provisioning profile, or private key.

## Current status

- Repository Actions runs: 0
- Repository Actions artifacts: 0
- Paid project service activated: none
- Railway activated: no
- First workflow allowed: **not yet**

External documentation was checked on September 28, 2026. Source material was paraphrased for compliance with licensing restrictions.
