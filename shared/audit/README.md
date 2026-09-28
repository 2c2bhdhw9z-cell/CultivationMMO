# Independent audit interface

The Go `audit.Sink` contract supports append and health operations. Task 2 supplies only `UnavailableSink`, which always returns `ErrUnavailable`.

That failure is intentional:

- No production mutation can pretend an in-memory log is durable audit.
- No audit event is written to the gameplay database.
- No privileged command exists yet.
- Task 5/19 must add protected external append-only storage and an emergency durable queue before real privileged mutations.

The V1 event schema documents the minimum skeleton fields. The complete future event is defined in `docs/OWNER_OPERATIONS_SECURITY.md`.
