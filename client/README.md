# Client skeleton

This directory contains the future iPhone game client.

Current behavior:

- Opens a plain Godot boot/status screen.
- Uses the local in-process authority adapter only.
- Sends one harmless `skeleton.ping` through the same command codec intended for the future server.
- Contains no account, online world, developer menu, secret, signing material, or private endpoint.

The WSS adapter intentionally returns `online_authority_not_configured` until the authenticated server work is implemented and approved.
