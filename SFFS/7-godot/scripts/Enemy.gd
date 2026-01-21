extends Area2D

@export var speed = 200.0
@export var direction = 1
@export var min_x = 50.0
@export var max_x = 1100.0
@export var drop_distance = 50.0

func _physics_process(delta: float) -> void:
	var newpos = position.x + direction*speed*delta
	if newpos < min_x:
		newpos = min_x
		position.y += drop_distance
		direction = -direction
	if newpos > max_x:
		newpos = max_x
		position.y += drop_distance
		direction = -direction
	position.x = newpos

func on_hit() -> void:
	Events.enemy_died.emit()
	queue_free()
