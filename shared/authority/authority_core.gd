class_name AuthorityCore
extends RefCounted

const CACHE_LIMIT_PER_ACTOR := 256
const MAX_ACTIVE_ACTOR_CACHES := 64
const MAX_ACTOR_ID_BYTES := 128

var _cached_results_by_actor: Dictionary = {}
var _cache_order_by_actor: Dictionary = {}
var _server_sequence: int = 0
var _state_version: int = 0


func handle(command_bytes: PackedByteArray, session_actor_id: String) -> PackedByteArray:
    var decoded: Dictionary = ProtocolV1.decode_command(command_bytes)
    if not decoded.ok:
        return _encode_result(
            "command.rejected",
            "",
            "",
            "rejected",
            decoded.error_code,
            {}
        )

    var command: Dictionary = decoded.value
    var command_id: String = command.command_id
    if session_actor_id.is_empty():
        return _encode_result(
            "command.rejected",
            command_id,
            "",
            "rejected",
            "session_actor_missing",
            {}
        )
    if session_actor_id.to_utf8_buffer().size() > MAX_ACTOR_ID_BYTES:
        return _encode_result(
            "command.rejected",
            command_id,
            "",
            "rejected",
            "session_actor_invalid",
            {}
        )
    if (
        not _cached_results_by_actor.has(session_actor_id)
        and _cached_results_by_actor.size() >= MAX_ACTIVE_ACTOR_CACHES
    ):
        return _encode_result(
            "command.rejected",
            command_id,
            "",
            "rejected",
            "actor_cache_capacity_reached",
            {}
        )

    var actor_cache: Dictionary = _cached_results_by_actor.get(session_actor_id, {})
    if actor_cache.has(command_id):
        var cached_result: PackedByteArray = actor_cache[command_id]
        return cached_result

    var result_bytes: PackedByteArray
    if command.type == "skeleton.ping":
        if not command.payload.is_empty():
            result_bytes = _encode_result(
                "command.rejected",
                command_id,
                _next_event_id(),
                "rejected",
                "skeleton_payload_must_be_empty",
                {}
            )
        else:
            result_bytes = _encode_result(
                "skeleton.pong",
                command_id,
                _next_event_id(),
                "accepted",
                null,
                {"component": "authority-core", "mode": "skeleton"}
            )
    else:
        result_bytes = _encode_result(
            "command.rejected",
            command_id,
            _next_event_id(),
            "rejected",
            "command_not_implemented",
            {}
        )

    _remember(session_actor_id, command_id, result_bytes)
    return result_bytes


func active_actor_cache_count() -> int:
    return _cached_results_by_actor.size()


func release_actor(session_actor_id: String) -> void:
    _cached_results_by_actor.erase(session_actor_id)
    _cache_order_by_actor.erase(session_actor_id)


func _encode_result(
    result_type: String,
    command_id: String,
    event_id: String,
    status: String,
    error_code: Variant,
    payload: Dictionary
) -> PackedByteArray:
    if event_id.is_empty():
        event_id = _next_event_id()
    var result: Dictionary = ProtocolV1.make_result(
        result_type,
        event_id,
        command_id,
        _server_sequence,
        _state_version,
        status,
        error_code,
        payload
    )
    var encoded: Dictionary = ProtocolV1.encode_result(result)
    if encoded.ok:
        return encoded.value

    # This literal is intentionally tiny and contains no mutable or private state.
    return (
        '{"command_id":"","error_code":"authority_result_invalid",'
        + '"event_id":"fallback","payload":{},"protocol":1,'
        + '"server_sequence":0,"state_version":0,"status":"rejected",'
        + '"type":"command.rejected"}'
    ).to_utf8_buffer()


func _next_event_id() -> String:
    _server_sequence += 1
    return "skeleton-event-%d" % _server_sequence


func _remember(
    session_actor_id: String,
    command_id: String,
    result: PackedByteArray
) -> void:
    if not _cached_results_by_actor.has(session_actor_id):
        _cached_results_by_actor[session_actor_id] = {}
        _cache_order_by_actor[session_actor_id] = []

    var actor_cache: Dictionary = _cached_results_by_actor[session_actor_id]
    var actor_order: Array = _cache_order_by_actor[session_actor_id]
    actor_cache[command_id] = result
    actor_order.append(command_id)

    if actor_order.size() > CACHE_LIMIT_PER_ACTOR:
        var oldest: String = actor_order.pop_front()
        actor_cache.erase(oldest)

    _cached_results_by_actor[session_actor_id] = actor_cache
    _cache_order_by_actor[session_actor_id] = actor_order
