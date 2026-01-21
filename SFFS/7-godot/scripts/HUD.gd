extends CanvasLayer

@onready var label = $Label

var score = 0

func add_score(ajout: int) -> void:
	score += ajout
	label.text = "Score: "+str(score)
