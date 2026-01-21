extends Area2D

@export var speed = 1000.0

func _physics_process(delta: float) -> void:
	position.y -= delta*speed
	if position.y < 0:
		queue_free()

func _on_Bullet_body_entered(body: Node2D) -> void:
	if body.get_groups().has("enemy"):
		body.on_hit()
		queue_free()
