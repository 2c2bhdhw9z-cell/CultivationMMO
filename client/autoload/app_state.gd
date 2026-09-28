extends Node

const BUILD_MODE := "offline_skeleton"
const BUILD_VERSION := "0.0.1-skeleton"

var authority_port: GameplayCommandPort


func _ready() -> void:
    authority_port = GameplayCommandPort.new(GameplayCommandPort.Mode.IN_PROCESS)
