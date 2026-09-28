# Valley zone skeleton

The zone uses the same repository-root Godot project and shared authority code as the client.

It currently:

- Starts without a network listener.
- Performs no valuable write.
- Handles only a harmless local `skeleton.ping` during self-check.
- Confirms a repeated command ID returns the original result inside an actor-isolated cache.
- Rejects malformed UTF-8/JSON, duplicate keys, unsafe numbers, oversized data, spoofed actor fields, and unimplemented commands.
- Bounds actor caches and requires Task 6 to release them on disconnect/expiry.
- Requires durable idempotency before any future valuable command is added.

Task 6 will add authenticated WSS networking and real iPhone Wi-Fi/cellular measurements. Railway remains inactive.
