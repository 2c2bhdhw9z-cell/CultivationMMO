class_name GameplayCommandPort
extends RefCounted

enum Mode {
    IN_PROCESS,
    WSS,
}

var _adapter: AuthorityPort
var _client_sequence: int = 0
var _state_version: int = 0


func _init(mode: Mode = Mode.IN_PROCESS) -> void:
    match mode:
        Mode.IN_PROCESS:
            _adapter = InProcessAuthorityAdapter.new()
        Mode.WSS:
            _adapter = WssAuthorityAdapter.new()
        _:
            _adapter = AuthorityPort.new()


func ping() -> Dictionary:
    _client_sequence += 1
    var random_id: String = Crypto.new().generate_random_bytes(16).hex_encode()
    var command: Dictionary = {
        "protocol": ProtocolV1.VERSION,
        "type": "skeleton.ping",
        "command_id": random_id,
        "client_sequence": _client_sequence,
        "expected_state_version": _state_version,
        "payload": {},
    }
    var result: Dictionary = _adapter.submit(command)
    if result.get("status") == "accepted":
        _state_version = int(result.get("state_version", _state_version))
    return result
