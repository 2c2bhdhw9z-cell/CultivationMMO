# Privileged command registry V1

The registry is intentionally empty and locked in Task 2.

A future command must name its owning service and matching operations feature. The owner-control API may request a bounded command, but the owning service must independently verify it. No command may provide arbitrary SQL, shell input, direct database editing, or client-side owner authority.

With no configured owner identity and no durable audit sink, all privileged routes remain unavailable.
