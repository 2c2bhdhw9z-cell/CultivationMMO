class_name WssAuthorityAdapter
extends AuthorityPort

# Task 6 will add authenticated WSS transport after hosting is approved.
# Until then this adapter must fail closed and never invent an online session.


func submit(command: Dictionary) -> Dictionary:
    return {
        "protocol": ProtocolV1.VERSION,
        "type": "command.rejected",
        "event_id": "online-not-configured",
        "command_id": str(command.get("command_id", "")),
        "server_sequence": 0,
        "state_version": int(command.get("expected_state_version", 0)),
        "status": "rejected",
        "error_code": "online_authority_not_configured",
        "payload": {},
    }
