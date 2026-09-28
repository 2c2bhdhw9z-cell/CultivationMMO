class_name ProtocolV1
extends RefCounted

const VERSION: int = 1
const MAX_MESSAGE_BYTES: int = 16 * 1024
const MAX_DEPTH: int = 8
const MAX_COLLECTION_ITEMS: int = 256
const MAX_STRING_BYTES: int = 4096
const MAX_SAFE_INTEGER: int = 9_007_199_254_740_991
const MAX_SAFE_INTEGER_TEXT: String = "9007199254740991"

const COMMAND_FIELDS: Dictionary = {
    "protocol": true,
    "type": true,
    "command_id": true,
    "client_sequence": true,
    "expected_state_version": true,
    "payload": true,
}
const RESULT_FIELDS: Dictionary = {
    "protocol": true,
    "type": true,
    "event_id": true,
    "command_id": true,
    "server_sequence": true,
    "state_version": true,
    "status": true,
    "error_code": true,
    "payload": true,
}
const TOP_LEVEL_INTEGER_FIELDS: Dictionary = {
    "protocol": true,
    "client_sequence": true,
    "expected_state_version": true,
    "server_sequence": true,
    "state_version": true,
}
const FORBIDDEN_PAYLOAD_IDENTITY_FIELDS: Dictionary = {
    "actor": true,
    "actor_id": true,
    "authenticated_actor": true,
    "owner_subject": true,
    "session_actor_id": true,
}


static func encode_command(command: Dictionary) -> Dictionary:
    var error_code: String = _validate_command(command)
    if not error_code.is_empty():
        return _failure(error_code)
    return _encode(command)


static func decode_command(bytes: PackedByteArray) -> Dictionary:
    var decoded: Dictionary = _decode(bytes)
    if not decoded.ok:
        return decoded
    var error_code: String = _validate_command(decoded.value)
    if not error_code.is_empty():
        return _failure(error_code)
    return decoded


static func encode_result(result: Dictionary) -> Dictionary:
    var error_code: String = _validate_result(result)
    if not error_code.is_empty():
        return _failure(error_code)
    return _encode(result)


static func decode_result(bytes: PackedByteArray) -> Dictionary:
    var decoded: Dictionary = _decode(bytes)
    if not decoded.ok:
        return decoded
    var error_code: String = _validate_result(decoded.value)
    if not error_code.is_empty():
        return _failure(error_code)
    return decoded


static func make_result(
    result_type: String,
    event_id: String,
    command_id: String,
    server_sequence: int,
    state_version: int,
    status: String,
    error_code: Variant,
    payload: Dictionary
) -> Dictionary:
    return {
        "protocol": VERSION,
        "type": result_type,
        "event_id": event_id,
        "command_id": command_id,
        "server_sequence": server_sequence,
        "state_version": state_version,
        "status": status,
        "error_code": error_code,
        "payload": payload,
    }


static func _encode(message: Dictionary) -> Dictionary:
    var text: String = JSON.stringify(message)
    var bytes: PackedByteArray = text.to_utf8_buffer()
    if bytes.size() > MAX_MESSAGE_BYTES:
        return _failure("message_too_large")
    return {"ok": true, "value": bytes, "error_code": ""}


static func _decode(bytes: PackedByteArray) -> Dictionary:
    if bytes.is_empty():
        return _failure("message_empty")
    if bytes.size() > MAX_MESSAGE_BYTES:
        return _failure("message_too_large")

    if not _is_valid_utf8(bytes):
        return _failure("utf8_invalid")
    var text: String = bytes.get_string_from_utf8()
    if text.to_utf8_buffer() != bytes:
        return _failure("utf8_invalid")

    var strict_error: String = _strict_text_error(text)
    if not strict_error.is_empty():
        return _failure(strict_error)

    var parser: JSON = JSON.new()
    if parser.parse(text) != OK:
        return _failure("json_invalid")
    if typeof(parser.data) != TYPE_DICTIONARY:
        return _failure("message_not_object")

    var value_error: String = _validate_json_value(parser.data, 0)
    if not value_error.is_empty():
        return _failure(value_error)
    return {"ok": true, "value": parser.data, "error_code": ""}


static func _validate_command(command: Dictionary) -> String:
    var field_error: String = _validate_fields(
        command,
        COMMAND_FIELDS,
        ["protocol", "type", "command_id", "client_sequence", "expected_state_version", "payload"]
    )
    if not field_error.is_empty():
        return field_error
    if not _is_integer(command.protocol) or int(command.protocol) != VERSION:
        return "protocol_unsupported"
    if not _is_identifier(command.type, 96):
        return "command_type_invalid"
    if not _is_identifier(command.command_id, 128):
        return "command_id_invalid"
    if not _is_nonnegative_integer(command.client_sequence):
        return "client_sequence_invalid"
    if not _is_nonnegative_integer(command.expected_state_version):
        return "state_version_invalid"
    if typeof(command.payload) != TYPE_DICTIONARY:
        return "payload_invalid"

    var payload_error: String = _validate_json_value(command.payload, 1)
    if not payload_error.is_empty():
        return payload_error
    return _validate_payload_identity(command.payload)


static func _validate_result(result: Dictionary) -> String:
    var field_error: String = _validate_fields(
        result,
        RESULT_FIELDS,
        [
            "protocol",
            "type",
            "event_id",
            "command_id",
            "server_sequence",
            "state_version",
            "status",
            "error_code",
            "payload",
        ]
    )
    if not field_error.is_empty():
        return field_error
    if not _is_integer(result.protocol) or int(result.protocol) != VERSION:
        return "protocol_unsupported"
    if not _is_identifier(result.type, 96):
        return "result_type_invalid"
    if not _is_identifier(result.event_id, 128):
        return "event_id_invalid"
    if typeof(result.command_id) != TYPE_STRING or result.command_id.to_utf8_buffer().size() > 128:
        return "command_id_invalid"
    if not result.command_id.is_empty() and not _is_identifier(result.command_id, 128):
        return "command_id_invalid"
    if not _is_nonnegative_integer(result.server_sequence):
        return "server_sequence_invalid"
    if not _is_nonnegative_integer(result.state_version):
        return "state_version_invalid"
    if result.status != "accepted" and result.status != "rejected":
        return "result_status_invalid"
    if result.error_code != null and not _is_identifier(result.error_code, 96):
        return "error_code_invalid"
    if typeof(result.payload) != TYPE_DICTIONARY:
        return "payload_invalid"
    return _validate_json_value(result.payload, 1)


static func _validate_fields(message: Dictionary, allowed: Dictionary, required: Array) -> String:
    for key: Variant in message.keys():
        if typeof(key) != TYPE_STRING or not allowed.has(key):
            return "unknown_top_level_field"
    for key: String in required:
        if not message.has(key):
            return "required_field_missing"
    return ""


static func _validate_json_value(value: Variant, depth: int) -> String:
    if depth > MAX_DEPTH:
        return "payload_too_deep"

    match typeof(value):
        TYPE_NIL, TYPE_BOOL:
            return ""
        TYPE_INT:
            return "" if value >= -MAX_SAFE_INTEGER and value <= MAX_SAFE_INTEGER else "number_out_of_range"
        TYPE_FLOAT:
            return (
                ""
                if is_finite(value) and value >= -MAX_SAFE_INTEGER and value <= MAX_SAFE_INTEGER
                else "number_out_of_range"
            )
        TYPE_STRING:
            return "" if value.to_utf8_buffer().size() <= MAX_STRING_BYTES else "string_too_large"
        TYPE_ARRAY:
            if value.size() > MAX_COLLECTION_ITEMS:
                return "collection_too_large"
            for item: Variant in value:
                var item_error: String = _validate_json_value(item, depth + 1)
                if not item_error.is_empty():
                    return item_error
            return ""
        TYPE_DICTIONARY:
            if value.size() > MAX_COLLECTION_ITEMS:
                return "collection_too_large"
            for key: Variant in value.keys():
                if typeof(key) != TYPE_STRING or key.to_utf8_buffer().size() > 128:
                    return "object_key_invalid"
                var child_error: String = _validate_json_value(value[key], depth + 1)
                if not child_error.is_empty():
                    return child_error
            return ""
        _:
            return "value_type_invalid"


static func _validate_payload_identity(value: Variant) -> String:
    if typeof(value) == TYPE_DICTIONARY:
        for key: Variant in value.keys():
            if FORBIDDEN_PAYLOAD_IDENTITY_FIELDS.has(str(key).to_lower()):
                return "payload_identity_field_forbidden"
            var child_error: String = _validate_payload_identity(value[key])
            if not child_error.is_empty():
                return child_error
    elif typeof(value) == TYPE_ARRAY:
        for item: Variant in value:
            var item_error: String = _validate_payload_identity(item)
            if not item_error.is_empty():
                return item_error
    return ""


static func _strict_text_error(text: String) -> String:
    var in_string: bool = false
    var escaped: bool = false

    for index: int in range(text.length()):
        var character: String = text[index]
        if in_string:
            if escaped:
                escaped = false
                continue
            if character == "\\":
                escaped = true
                continue
            if character == "\"":
                in_string = false
                continue
            if text.unicode_at(index) < 0x20:
                return "json_control_character"
            continue

        if character == "\"":
            in_string = true
            continue
        if character == ",":
            var next: int = index + 1
            while next < text.length() and text[next] in [" ", "\t", "\r", "\n"]:
                next += 1
            if next < text.length() and text[next] in ["}", "]"]:
                return "json_trailing_comma"

    if in_string or escaped:
        return "json_invalid"
    var duplicate_error: String = _duplicate_key_error(text)
    if not duplicate_error.is_empty():
        return duplicate_error
    return _number_lexeme_error(text)


static func _duplicate_key_error(text: String) -> String:
    var contexts: Array = []
    var in_string: bool = false
    var escaped: bool = false
    var string_start: int = -1

    for index: int in range(text.length()):
        var character: String = text[index]
        if in_string:
            if escaped:
                escaped = false
                continue
            if character == "\\":
                escaped = true
                continue
            if character != "\"":
                continue

            in_string = false
            var next: int = index + 1
            while next < text.length() and text[next] in [" ", "\t", "\r", "\n"]:
                next += 1
            if next < text.length() and text[next] == ":":
                if contexts.is_empty() or typeof(contexts.back()) != TYPE_DICTIONARY:
                    return "json_invalid"
                var raw_key: String = text.substr(string_start, index - string_start + 1)
                var decoded_key: Variant = JSON.parse_string(raw_key)
                if typeof(decoded_key) != TYPE_STRING:
                    return "json_invalid"
                var keys: Dictionary = contexts[contexts.size() - 1]
                if keys.has(decoded_key):
                    return "json_duplicate_key"
                keys[decoded_key] = true
                contexts[contexts.size() - 1] = keys
                if contexts.size() == 1 and TOP_LEVEL_INTEGER_FIELDS.has(decoded_key):
                    var value_start: int = next + 1
                    while value_start < text.length() and text[value_start] in [" ", "\t", "\r", "\n"]:
                        value_start += 1
                    var value_end: int = value_start
                    while (
                        value_end < text.length()
                        and text[value_end] not in [" ", "\t", "\r", "\n", ",", "}"]
                    ):
                        value_end += 1
                    var integer_lexeme: String = text.substr(value_start, value_end - value_start)
                    if not _is_unsigned_integer_lexeme(integer_lexeme):
                        return "integer_lexeme_invalid"
            continue

        match character:
            "\"":
                in_string = true
                string_start = index
            "{":
                contexts.append({})
            "[":
                contexts.append([])
            "}":
                if contexts.is_empty() or typeof(contexts.back()) != TYPE_DICTIONARY:
                    return "json_invalid"
                contexts.pop_back()
            "]":
                if contexts.is_empty() or typeof(contexts.back()) != TYPE_ARRAY:
                    return "json_invalid"
                contexts.pop_back()

    return "" if contexts.is_empty() else "json_invalid"


static func _number_lexeme_error(text: String) -> String:
    var index: int = 0
    var in_string: bool = false
    var escaped: bool = false

    while index < text.length():
        var character: String = text[index]
        if in_string:
            if escaped:
                escaped = false
            elif character == "\\":
                escaped = true
            elif character == "\"":
                in_string = false
            index += 1
            continue

        if character == "\"":
            in_string = true
            index += 1
            continue
        if character == "-" or _is_digit_character(character):
            var start: int = index
            index += 1
            while (
                index < text.length()
                and (
                    _is_digit_character(text[index])
                    or text[index] in [".", "e", "E", "+", "-"]
                )
            ):
                index += 1
            var lexeme: String = text.substr(start, index - start)
            var lexeme_error: String = _validate_number_lexeme(lexeme)
            if not lexeme_error.is_empty():
                return lexeme_error
            continue
        index += 1
    return ""


static func _validate_number_lexeme(lexeme: String) -> String:
    var index: int = 0
    if lexeme.is_empty():
        return "number_lexeme_invalid"
    if lexeme[index] == "-":
        index += 1
    if index >= lexeme.length():
        return "number_lexeme_invalid"

    var integer_start: int = index
    if lexeme[index] == "0":
        index += 1
        if index < lexeme.length() and _is_digit_character(lexeme[index]):
            return "number_lexeme_invalid"
    elif lexeme[index] >= "1" and lexeme[index] <= "9":
        while index < lexeme.length() and _is_digit_character(lexeme[index]):
            index += 1
    else:
        return "number_lexeme_invalid"
    var integer_digits: String = lexeme.substr(integer_start, index - integer_start)

    var fraction_digits: String = ""
    if index < lexeme.length() and lexeme[index] == ".":
        index += 1
        var fraction_start: int = index
        while index < lexeme.length() and _is_digit_character(lexeme[index]):
            index += 1
        if index == fraction_start:
            return "number_lexeme_invalid"
        fraction_digits = lexeme.substr(fraction_start, index - fraction_start)

    var exponent: int = 0
    if index < lexeme.length() and lexeme[index] in ["e", "E"]:
        index += 1
        var exponent_negative: bool = false
        if index < lexeme.length() and lexeme[index] in ["+", "-"]:
            exponent_negative = lexeme[index] == "-"
            index += 1
        var exponent_start: int = index
        while index < lexeme.length() and _is_digit_character(lexeme[index]):
            if exponent < 100_000:
                exponent = min(exponent * 10 + int(lexeme.unicode_at(index) - 48), 100_000)
            index += 1
        if index == exponent_start:
            return "number_lexeme_invalid"
        if exponent_negative:
            exponent = -exponent

    if index != lexeme.length():
        return "number_lexeme_invalid"
    if not _number_magnitude_is_safe(integer_digits, fraction_digits, exponent):
        return "number_out_of_range"
    return ""


static func _number_magnitude_is_safe(
    integer_digits: String,
    fraction_digits: String,
    exponent: int
) -> bool:
    var digits: String = integer_digits + fraction_digits
    var first_nonzero: int = 0
    while first_nonzero < digits.length() and digits[first_nonzero] == "0":
        first_nonzero += 1
    if first_nonzero == digits.length():
        return true

    var decimal_position: int = integer_digits.length() + exponent
    var digits_before_decimal: int = decimal_position - first_nonzero
    if digits_before_decimal <= 0:
        return true
    if digits_before_decimal > MAX_SAFE_INTEGER_TEXT.length():
        return false
    if digits_before_decimal < MAX_SAFE_INTEGER_TEXT.length():
        return true

    var comparable: String = ""
    for offset: int in range(MAX_SAFE_INTEGER_TEXT.length()):
        var digit_index: int = first_nonzero + offset
        comparable += digits[digit_index] if digit_index < digits.length() else "0"
    if comparable < MAX_SAFE_INTEGER_TEXT:
        return true
    if comparable > MAX_SAFE_INTEGER_TEXT:
        return false

    for trailing_index: int in range(
        first_nonzero + MAX_SAFE_INTEGER_TEXT.length(),
        digits.length()
    ):
        if digits[trailing_index] != "0":
            return false
    return true


static func _is_digit_character(character: String) -> bool:
    return character >= "0" and character <= "9"


static func _is_unsigned_integer_lexeme(value: String) -> bool:
    if value.is_empty() or (value.length() > 1 and value[0] == "0"):
        return false
    for index: int in range(value.length()):
        if value[index] < "0" or value[index] > "9":
            return false
    return true


static func _is_valid_utf8(bytes: PackedByteArray) -> bool:
    var index: int = 0
    while index < bytes.size():
        var first: int = bytes[index]
        if first <= 0x7F:
            index += 1
            continue
        if first >= 0xC2 and first <= 0xDF:
            if not _has_continuations(bytes, index, 1):
                return false
            index += 2
            continue
        if first == 0xE0:
            if index + 2 >= bytes.size() or bytes[index + 1] < 0xA0 or bytes[index + 1] > 0xBF:
                return false
            if not _is_continuation(bytes[index + 2]):
                return false
            index += 3
            continue
        if (first >= 0xE1 and first <= 0xEC) or (first >= 0xEE and first <= 0xEF):
            if not _has_continuations(bytes, index, 2):
                return false
            index += 3
            continue
        if first == 0xED:
            if index + 2 >= bytes.size() or bytes[index + 1] < 0x80 or bytes[index + 1] > 0x9F:
                return false
            if not _is_continuation(bytes[index + 2]):
                return false
            index += 3
            continue
        if first == 0xF0:
            if index + 3 >= bytes.size() or bytes[index + 1] < 0x90 or bytes[index + 1] > 0xBF:
                return false
            if not _is_continuation(bytes[index + 2]) or not _is_continuation(bytes[index + 3]):
                return false
            index += 4
            continue
        if first >= 0xF1 and first <= 0xF3:
            if not _has_continuations(bytes, index, 3):
                return false
            index += 4
            continue
        if first == 0xF4:
            if index + 3 >= bytes.size() or bytes[index + 1] < 0x80 or bytes[index + 1] > 0x8F:
                return false
            if not _is_continuation(bytes[index + 2]) or not _is_continuation(bytes[index + 3]):
                return false
            index += 4
            continue
        return false
    return true


static func _has_continuations(bytes: PackedByteArray, index: int, count: int) -> bool:
    if index + count >= bytes.size():
        return false
    for offset: int in range(1, count + 1):
        if not _is_continuation(bytes[index + offset]):
            return false
    return true


static func _is_continuation(byte: int) -> bool:
    return byte >= 0x80 and byte <= 0xBF


static func _is_identifier(value: Variant, max_bytes: int) -> bool:
    if typeof(value) != TYPE_STRING or value.is_empty():
        return false
    if value.to_utf8_buffer().size() > max_bytes:
        return false
    for index: int in range(value.length()):
        var character: String = value[index]
        var allowed: bool = (
            (character >= "a" and character <= "z")
            or (character >= "A" and character <= "Z")
            or (character >= "0" and character <= "9")
            or character in ["-", "_", ".", ":"]
        )
        if not allowed:
            return false
    return true


static func _is_integer(value: Variant) -> bool:
    if typeof(value) == TYPE_INT:
        return true
    return typeof(value) == TYPE_FLOAT and is_finite(value) and value == floor(value)


static func _is_nonnegative_integer(value: Variant) -> bool:
    return _is_integer(value) and value >= 0 and value <= MAX_SAFE_INTEGER


static func _failure(code: String) -> Dictionary:
    return {"ok": false, "value": {}, "error_code": code}
