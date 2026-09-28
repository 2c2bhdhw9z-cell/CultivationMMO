# Shared authority foundation

Both the offline preview and future hosted valley use the same `AuthorityCore`.

Rules:

- UI code sends commands; it never edits valuable state directly.
- The in-process adapter serializes and parses the same V1 messages as the future WSS adapter.
- The authenticated session supplies actor identity.
- Unknown, malformed, unauthenticated, and unimplemented commands are rejected.
- A repeated command ID returns the same result within that authenticated actor's isolated cache. The only current command is non-mutating.
- Before any valuable command exists, durable idempotency storage must cover cache eviction, restart, and zone transfer so a retry cannot grant a second reward.
- Per-actor cache pressure cannot evict another actor's completed command result.
- At most 64 actor caches exist in one skeleton authority core, actor IDs are capped at 128 UTF-8 bytes, and the session layer must call `release_actor` on disconnect or expiry before Task 6 enables a listener.
- Actor identity comes only from the authenticated session; reserved actor fields inside command payloads are rejected.
- The skeleton implements only `skeleton.ping`, which changes no game state.
- Time, randomness, combat, inventory, cultivation, claims, and persistence remain authority-owned when implemented later.

This is separate from the owner/developer privileged-command system.
