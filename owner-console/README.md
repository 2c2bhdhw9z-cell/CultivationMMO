# Owner console source shell

This directory is separate from the Godot player client.

- `public/404.html` is the only safe unauthenticated response template.
- `protected/` contains a responsive source shell that a future server may send only after exact-origin passkey authentication and authorization.
- Task 2 does not deploy or serve the protected files.
- The owner-control service returns `404` for every protected path.
- There is no owner identity, menu, command, player data, secret, token, source map, or hosting account.

Never host this directory as an ordinary static website. Public source code is expected; protected runtime delivery and authority still require the server.
