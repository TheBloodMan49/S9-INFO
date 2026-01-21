# GoLang-WalkThrough

## Exploration de routes HTTP

Vous vous appêtez à aider le petit Gopher à retrouver sa clé égarée. Les indices vous orientent sur la piste de chemins à explorer. C'est donc par là que vous allez commencer votre investigation... Pour ce faire, vous trouvez un fichier paths.txt contenant des chemins encodés en base64. Décodez-les un par un, testez et récupérez les chemins utiles.

---

### Objectif

* Explorer les différents chemins pour trouver ceux qui permettent de continuer le challenge.
* Utiliser dans un premier temps une approche séquentielle puis dans un second temps une approche parallélisée pour augmenter la vitesse de recherche.

---

### Rappel pédagogique

**Goroutine** : une fonction exécutée de manière concurrente avec d’autres fonctions.


**Channel** : un mécanisme de communication sécurisé entre goroutines.


**Pourquoi ?** → Ce modèle permet de paralléliser des tâches simples et de comprendre la base de la concurrence en Go.


