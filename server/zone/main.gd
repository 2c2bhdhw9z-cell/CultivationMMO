extends Node

var _authority: AuthorityCore = AuthorityCore.new()


func _ready() -> void:
    if "--self-check" in OS.get_cmdline_user_args():
        _run_self_check()
        return

    print("ZONE_SKELETON_READY mode=local-only listener=disabled protocol=1")


func _run_self_check() -> void:
    var first_command: Dictionary = _command("skeleton.ping", "zone-self-check-0001")
    var first: Dictionary = _submit(first_command, "zone-self-check")
    var repeated: Dictionary = _submit(first_command, "zone-self-check")
    var unauthenticated: Dictionary = _submit(first_command, "")
    var unknown_result: Dictionary = _submit(
        _command("world.not-implemented", "zone-self-check-0002"),
        "zone-self-check"
    )
    var nonempty_ping_command: Dictionary = _command("skeleton.ping", "nonempty-ping")
    nonempty_ping_command.payload = {"note": "benign"}
    var nonempty_ping: Dictionary = _submit(nonempty_ping_command, "zone-self-check")
    var long_actor: Dictionary = _submit(
        _command("skeleton.ping", "long-actor"),
        "a".repeat(AuthorityCore.MAX_ACTOR_ID_BYTES + 1)
    )

    # These two actor/command pairs collided in the old delimiter-based cache key.
    var collision_one: Dictionary = _submit(_command("skeleton.ping", "c"), "a:b")
    var collision_two: Dictionary = _submit(_command("skeleton.ping", "b:c"), "a")

    # One actor filling its cache must not evict another actor's result.
    for number: int in range(AuthorityCore.CACHE_LIMIT_PER_ACTOR + 1):
        var noisy_command: Dictionary = _command("skeleton.ping", "noisy-%d" % number)
        var noisy_result: Dictionary = _submit(noisy_command, "noisy-actor")
        if not noisy_result.ok or noisy_result.value.get("status") != "accepted":
            _fail(
                "noisy_actor_setup-%d-%s"
                % [number, str(noisy_result.get("error_code", "unknown"))]
            )
            return
    var after_other_actor_eviction: Dictionary = _submit(first_command, "zone-self-check")

    var capacity_authority: AuthorityCore = AuthorityCore.new()
    var capacity_setup_ok: bool = true
    for number: int in range(AuthorityCore.MAX_ACTIVE_ACTOR_CACHES):
        var capacity_result: Dictionary = _submit_with_authority(
            capacity_authority,
            _command("skeleton.ping", "capacity-command"),
            "capacity-actor-%d" % number
        )
        if not capacity_result.ok or capacity_result.value.get("status") != "accepted":
            capacity_setup_ok = false
            break
    var capacity_rejected: Dictionary = _submit_with_authority(
        capacity_authority,
        _command("skeleton.ping", "capacity-overflow"),
        "capacity-actor-overflow"
    )
    capacity_authority.release_actor("capacity-actor-0")
    var capacity_after_release: Dictionary = _submit_with_authority(
        capacity_authority,
        _command("skeleton.ping", "capacity-after-release"),
        "capacity-actor-after-release"
    )

    var malformed: Dictionary = ProtocolV1.decode_command("{not-json".to_utf8_buffer())
    var invalid_utf8: Dictionary = ProtocolV1.decode_command(PackedByteArray([0xFF]))
    var trailing_comma: Dictionary = ProtocolV1.decode_command(
        (
            '{"protocol":1,"type":"skeleton.ping","command_id":"trailing",'
            + '"client_sequence":1,"expected_state_version":0,"payload":{},}'
        ).to_utf8_buffer()
    )
    var duplicate_key: Dictionary = ProtocolV1.decode_command(
        (
            '{"protocol":1,"protocol":1,"type":"skeleton.ping",'
            + '"command_id":"duplicate","client_sequence":1,'
            + '"expected_state_version":0,"payload":{}}'
        ).to_utf8_buffer()
    )
    var escaped_duplicate_key: Dictionary = ProtocolV1.decode_command(
        (
            '{"protocol":1,"\\u0070rotocol":1,"type":"skeleton.ping",'
            + '"command_id":"escaped-duplicate","client_sequence":1,'
            + '"expected_state_version":0,"payload":{}}'
        ).to_utf8_buffer()
    )
    var missing_field: Dictionary = ProtocolV1.decode_command(
        '{"protocol":1,"type":"skeleton.ping"}'.to_utf8_buffer()
    )
    var wrong_type: Dictionary = ProtocolV1.decode_command(
        (
            '{"protocol":1,"type":"skeleton.ping","command_id":"wrong-type",'
            + '"client_sequence":"1","expected_state_version":0,"payload":{}}'
        ).to_utf8_buffer()
    )
    var unsafe_integer: Dictionary = ProtocolV1.decode_command(
        (
            '{"protocol":1,"type":"skeleton.ping","command_id":"huge",'
            + '"client_sequence":1e100,"expected_state_version":0,"payload":{}}'
        ).to_utf8_buffer()
    )
    var rounded_fraction: Dictionary = ProtocolV1.decode_command(
        (
            '{"protocol":1,"type":"skeleton.ping","command_id":"fraction",'
            + '"client_sequence":1.0000000000000001,"expected_state_version":0,"payload":{}}'
        ).to_utf8_buffer()
    )
    var top_level_actor: Dictionary = ProtocolV1.decode_command(
        (
            '{"protocol":1,"type":"skeleton.ping","command_id":"actor-field",'
            + '"client_sequence":1,"expected_state_version":0,"payload":{},'
            + '"actor":"not-allowed"}'
        ).to_utf8_buffer()
    )
    var payload_actor_command: Dictionary = _command("skeleton.ping", "payload-actor")
    payload_actor_command.payload = {"nested": {"actor_id": "not-allowed"}}
    var payload_actor: Dictionary = ProtocolV1.encode_command(payload_actor_command)
    var unsafe_payload_number_command: Dictionary = _command("skeleton.ping", "unsafe-payload-number")
    unsafe_payload_number_command.payload = {"number": ProtocolV1.MAX_SAFE_INTEGER + 1}
    var unsafe_payload_number: Dictionary = ProtocolV1.encode_command(unsafe_payload_number_command)
    var lossy_payload_number: Dictionary = ProtocolV1.decode_command(
        (
            '{"protocol":1,"type":"skeleton.ping","command_id":"lossy-number",'
            + '"client_sequence":1,"expected_state_version":0,'
            + '"payload":{"number":9007199254740991.25}}'
        ).to_utf8_buffer()
    )
    var safe_fractional_payload: Dictionary = ProtocolV1.decode_command(
        (
            '{"protocol":1,"type":"skeleton.ping","command_id":"safe-fraction",'
            + '"client_sequence":1,"expected_state_version":0,'
            + '"payload":{"number":1.25}}'
        ).to_utf8_buffer()
    )

    var deep_payload: Dictionary = _nested_payload(ProtocolV1.MAX_DEPTH + 1)
    var deep_command: Dictionary = _command("skeleton.ping", "too-deep")
    deep_command.payload = deep_payload
    var too_deep: Dictionary = ProtocolV1.encode_command(deep_command)

    var large_collection_command: Dictionary = _command("skeleton.ping", "large-collection")
    var large_collection: Array = []
    large_collection.resize(ProtocolV1.MAX_COLLECTION_ITEMS + 1)
    large_collection_command.payload = {"items": large_collection}
    var too_many_items: Dictionary = ProtocolV1.encode_command(large_collection_command)

    var large_string_command: Dictionary = _command("skeleton.ping", "large-string")
    large_string_command.payload = {"text": "x".repeat(ProtocolV1.MAX_STRING_BYTES + 1)}
    var string_too_large: Dictionary = ProtocolV1.encode_command(large_string_command)

    var oversized_bytes: PackedByteArray = PackedByteArray()
    oversized_bytes.resize(ProtocolV1.MAX_MESSAGE_BYTES + 1)
    var oversized: Dictionary = ProtocolV1.decode_command(oversized_bytes)

    var invalid_result: Dictionary = ProtocolV1.decode_result(
        (
            '{"protocol":1,"type":"skeleton.pong","event_id":"bad-result",'
            + '"command_id":"bad","server_sequence":0,"state_version":0,'
            + '"status":"maybe","error_code":null,"payload":{}}'
        ).to_utf8_buffer()
    )

    var passed: bool = (
        first.ok
        and repeated.ok
        and unauthenticated.ok
        and unknown_result.ok
        and nonempty_ping.ok
        and long_actor.ok
        and collision_one.ok
        and collision_two.ok
        and after_other_actor_eviction.ok
        and first.value.get("status") == "accepted"
        and first.value == repeated.value
        and first.value == after_other_actor_eviction.value
        and collision_one.value.get("event_id") != collision_two.value.get("event_id")
        and unauthenticated.value.get("error_code") == "session_actor_missing"
        and unknown_result.value.get("error_code") == "command_not_implemented"
        and nonempty_ping.value.get("error_code") == "skeleton_payload_must_be_empty"
        and long_actor.value.get("error_code") == "session_actor_invalid"
        and capacity_setup_ok
        and capacity_rejected.ok
        and capacity_after_release.ok
        and capacity_authority.active_actor_cache_count() == AuthorityCore.MAX_ACTIVE_ACTOR_CACHES
        and capacity_rejected.value.get("error_code") == "actor_cache_capacity_reached"
        and capacity_after_release.value.get("status") == "accepted"
        and _failed_with(malformed, "json_invalid")
        and _failed_with(invalid_utf8, "utf8_invalid")
        and _failed_with(trailing_comma, "json_trailing_comma")
        and _failed_with(duplicate_key, "json_duplicate_key")
        and _failed_with(escaped_duplicate_key, "json_duplicate_key")
        and _failed_with(missing_field, "required_field_missing")
        and _failed_with(wrong_type, "integer_lexeme_invalid")
        and _failed_with(unsafe_integer, "integer_lexeme_invalid")
        and _failed_with(rounded_fraction, "integer_lexeme_invalid")
        and _failed_with(top_level_actor, "unknown_top_level_field")
        and _failed_with(payload_actor, "payload_identity_field_forbidden")
        and _failed_with(unsafe_payload_number, "number_out_of_range")
        and _failed_with(lossy_payload_number, "number_out_of_range")
        and safe_fractional_payload.ok
        and _failed_with(too_deep, "payload_too_deep")
        and _failed_with(too_many_items, "collection_too_large")
        and _failed_with(string_too_large, "string_too_large")
        and _failed_with(oversized, "message_too_large")
        and _failed_with(invalid_result, "result_status_invalid")
    )

    if passed:
        print(
            "ZONE_SELF_CHECK_OK protocol=1 duplicate=idempotent actor-cache=bounded "
            + "utf8=strict duplicate-keys=denied numbers=bounded limits=bounded "
            + "listener=disabled"
        )
        get_tree().quit(0)
    else:
        _fail("authority_boundary")


func _submit(command: Dictionary, actor_id: String) -> Dictionary:
    return _submit_with_authority(_authority, command, actor_id)


func _submit_with_authority(
    authority: AuthorityCore,
    command: Dictionary,
    actor_id: String
) -> Dictionary:
    var encoded: Dictionary = ProtocolV1.encode_command(command)
    if not encoded.ok:
        return encoded
    return ProtocolV1.decode_result(authority.handle(encoded.value, actor_id))


func _command(command_type: String, command_id: String) -> Dictionary:
    return {
        "protocol": ProtocolV1.VERSION,
        "type": command_type,
        "command_id": command_id,
        "client_sequence": 1,
        "expected_state_version": 0,
        "payload": {},
    }


func _nested_payload(depth: int) -> Dictionary:
    if depth <= 0:
        return {}
    return {"next": _nested_payload(depth - 1)}


func _failed_with(result: Dictionary, error_code: String) -> bool:
    return not result.ok and result.error_code == error_code


func _fail(code: String) -> void:
    print("ZONE_SELF_CHECK_FAILED code=%s" % code)
    get_tree().quit(1)
