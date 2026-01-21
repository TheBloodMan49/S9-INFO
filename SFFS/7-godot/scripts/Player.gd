extends CharacterBody2D

@export var min_x = 50.0
@export var max_x = 1100.0
@export var speed = 1000.0
@export var bullet_scene: PackedScene

@onready var muzzle: Marker2D = $Marker2D

func _physics_process(delta: float) -> void:
	var input_dir := 0.0
	if Input.is_action_pressed("ui_left"):
		input_dir -= 25.0
	if Input.is_action_pressed("ui_right"):
		input_dir += 25.0
	
	velocity.x = input_dir * speed * delta
	velocity.y = 0.0
	move_and_slide()
	global_position.x = clamp(global_position.x, min_x, max_x)

func _shoot() -> void:
	var bullet = bullet_scene.instantiate()
	bullet.global_position = muzzle.global_position
	get_tree().current_scene.add_child(bullet)
func _unhandled_input(event: InputEvent) -> void:
	if event.is_action_pressed("ui_accept"):
		_shoot()
