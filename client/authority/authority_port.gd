class_name AuthorityPort
extends RefCounted


func submit(_command: Dictionary) -> Dictionary:
    return {
        "ok": false,
        "status": "rejected",
        "error_code": "authority_not_implemented",
        "payload": {},
    }
