class_name InProcessAuthorityAdapter
extends AuthorityPort

const LOCAL_ACTOR_ID := "offline-preview-owner"

var _authority: AuthorityCore = AuthorityCore.new()


func submit(command: Dictionary) -> Dictionary:
    var encoded: Dictionary = ProtocolV1.encode_command(command)
    if not encoded.ok:
        return _local_error(encoded.error_code)

    var response_bytes: PackedByteArray = _authority.handle(encoded.value, LOCAL_ACTOR_ID)
    var decoded: Dictionary = ProtocolV1.decode_result(response_bytes)
    if not decoded.ok:
        return _local_error(decoded.error_code)
    return decoded.value


func _local_error(code: String) -> Dictionary:
    return {
        "protocol": ProtocolV1.VERSION,
        "type": "command.rejected",
        "event_id": "local-codec-error",
        "command_id": "",
        "server_sequence": 0,
        "state_version": 0,
        "status": "rejected",
        "error_code": code,
        "payload": {},
    }
