import cv2
from ultralytics import YOLO

model = YOLO("dataset_part_3/model.pt")

RULES = {
    "Rock": "Scissors",
    "Paper": "Rock",
    "Scissors": "Paper"
}

cap = cv2.VideoCapture(0)
cap.set(3, 1280) # Largeur de la fenêtre
cap.set(4, 720)  # Hauteur

while True:
    ret, frame = cap.read()
    if not ret: break
    
    # On récupère les dimensions pour diviser l'écran
    height, width, _ = frame.shape
    center_x = width // 2

    # Dessin de la ligne de séparation (L'Arène)
    cv2.line(frame, (center_x, 0), (center_x, height), (255, 255, 255), 2)
    cv2.putText(frame, "JOUEUR 1", (50, 50), cv2.FONT_HERSHEY_SIMPLEX, 1, (255, 0, 0), 2)
    cv2.putText(frame, "JOUEUR 2", (width - 250, 50), cv2.FONT_HERSHEY_SIMPLEX, 1, (0, 0, 255), 2)

    results = model(frame, verbose=False)
    
    p1_move = None
    p2_move = None

    # On parcourt TOUTES les boîtes détectées
    for box in results[0].boxes:
        # TODO: récupérer le nom du coup joué (stocker dans move_name)
        # les coordonnées (x1;y1) et (x2;y2), respectivement le coin supérieur gauche et inférieur droit de l'objet détecté.
        # et l'indice de confiance du résultat; on ne souhaite garder que les résultats dont la confiance est sup. ou égale à 0.7
        x1, y1, x2, y2 = map(int, box.xyxy[0])
        conf = box.conf[0].item()
        class_id = int(box.cls[0].item())
        move_name = model.names[class_id]
        if conf < 0.7:
            continue

        # Calcul du centre de la main pour savoir à qui elle appartient
        box_center_x = (x1 + x2) // 2
        
        if box_center_x < center_x:
            p1_move = move_name
        else:
            p2_move = move_name
        
        # Dessin de la boîte (Bleu pour J1, Rouge pour J2)
        color = (255, 0, 0) if box_center_x < center_x else (0, 0, 255)
        cv2.rectangle(frame, (x1, y1), (x2, y2), color, 2)
        cv2.putText(frame, move_name, (x1, y1 - 10), cv2.FONT_HERSHEY_SIMPLEX, 0.8, color, 2)

    # Le duel ne se lance que si les DEUX joueurs sont détectés
    if p1_move is not None and p2_move is not None:
        winner_text = ""
        
        if p1_move == p2_move:
            winner_text = "EGALITE !"
        elif RULES[p1_move] == p2_move:
            winner_text = "J1 GAGNE !"
        else:
            winner_text = "J2 GAGNE !"
        
        # Affichage du résultat au centre
        cv2.putText(frame, winner_text, (center_x - 150, height - 50), 
                    cv2.FONT_HERSHEY_SIMPLEX, 1.5, (0, 255, 0), 3)

    elif p1_move is None and p2_move is None:
        cv2.putText(frame, "EN ATTENTE DES JOUEURS...", (center_x - 200, height//2), 
                    cv2.FONT_HERSHEY_SIMPLEX, 0.8, (200, 200, 200), 2)
        
    cv2.imshow("PFC BATTLE ROYALE", frame)
    if cv2.waitKey(1) & 0xFF == ord('q'): break

cap.release()
cv2.destroyAllWindows()