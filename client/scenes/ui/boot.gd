extends Control

@onready var status_label: Label = %StatusLabel
@onready var detail_label: Label = %DetailLabel


func _ready() -> void:
    var result: Dictionary = AppState.authority_port.ping()
    var online_result: Dictionary = WssAuthorityAdapter.new().submit({
        "command_id": "online-self-check",
        "expected_state_version": 0,
    })
    var ready: bool = (
        result.get("status") == "accepted"
        and online_result.get("status") == "rejected"
        and online_result.get("error_code") == "online_authority_not_configured"
    )

    status_label.text = "Local foundation ready" if ready else "Foundation check failed"
    detail_label.text = (
        "Offline skeleton %s • No online world • No account • No developer menu"
        % AppState.BUILD_VERSION
        if ready
        else "Nothing changed. Error: %s" % str(result.get("error_code", "unknown"))
    )

    if "--self-check" in OS.get_cmdline_user_args():
        if ready:
            print(
                "CLIENT_SELF_CHECK_OK protocol=1 mode=offline_skeleton "
                + "online=locked version=%s" % AppState.BUILD_VERSION
            )
            get_tree().quit(0)
        else:
            print("CLIENT_SELF_CHECK_FAILED code=%s" % result.get("error_code", "unknown"))
            get_tree().quit(1)
