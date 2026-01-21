extends Node2D

@export var player_scene: PackedScene
@export var enemy_scene: PackedScene
@export var score_inc = 1
@export var gameover_scene: PackedScene

var hud: CanvasLayer

func _spawn_player() -> void:
	var player = player_scene.instantiate()
	add_child(player)
	var viewport_size = get_viewport_rect().size
	player.position = Vector2(viewport_size.x * 0.5, viewport_size.y - 40)
	

func _spawn_enemies() -> void:
	var rows := 3
	var cols := 5
	var start_x := 80.0
	var start_y := 60.0
	var spacing_x := 150.0
	var spacing_y := 100.0
	
	for row in range(rows):
		for col in range(cols):
			var enemy = enemy_scene.instantiate()
			add_child(enemy)
			enemy.position = Vector2(start_x + spacing_x*row, start_y + spacing_y*col)

func _ready() -> void:
	_spawn_player()
	_spawn_enemies()
	hud = $Hud
	Events.enemy_died.connect(add_score)
	
func add_score() -> void:
	hud.add_score(score_inc)
	if get_tree().get_nodes_in_group("enemy").size() <= 1:
		var gameover = gameover_scene.instantiate()
		gameover
		add_child(gameover)
	
