# Account API skeleton

- Default local address: `127.0.0.1:8080`
- `GET /healthz`: local liveness only
- `GET /readyz`: `503` because identity and database are not configured
- Every account/world route: `404`
- Non-loopback bind: refused

This is not an account system and accepts no gameplay save.
