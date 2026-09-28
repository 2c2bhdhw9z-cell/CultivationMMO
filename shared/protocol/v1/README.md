# Gameplay protocol V1

The first protocol uses strict UTF-8 JSON so early messages are easy to inspect.

Limits are defined in `limits.json` and enforced again by `protocol_v1.gd`. The runtime rejects invalid UTF-8 and duplicate object keys before JSON values become dictionaries:

- 16 KiB maximum message
- Eight nested levels
- 256 entries in one collection
- 4 KiB maximum UTF-8 bytes per string
- Required and allowlisted top-level fields
- Top-level protocol/counter/version fields use unsigned integer wire tokens from zero through JavaScript's safe-integer maximum
- Every payload/result number is finite and bounded to the positive or negative safe-integer magnitude
- Reserved identity fields such as `actor` and `actor_id` forbidden anywhere in a command payload
- Duplicate object keys rejected before JSON parsing
- Valid UTF-8 bytes only
- No decoded engine objects
- No actor identity supplied by a player command

`gameplay-command.schema.json`, `gameplay-result.schema.json`, and `world-event.schema.json` are language-neutral documentation contracts. Runtime code must still validate all untrusted input.
