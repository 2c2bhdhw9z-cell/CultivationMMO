# Owner-control skeleton

- Default local address: `127.0.0.1:8081`
- Loads and exactly validates the empty locked command registry and operations manifest
- Rejects invalid UTF-8, duplicate/case-folded/unknown fields, missing fields, and null required arrays
- Confirms durable audit is unavailable
- Serves no protected HTML, menu, player data, or command route
- Returns `404` for all unfinished routes
- Refuses non-loopback binding

Source visibility does not grant runtime access. Task 5 and Task 19 will add real identity, audit, service envelopes, and phone access.
