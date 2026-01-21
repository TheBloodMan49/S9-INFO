
# GoLang-WalkThrough

## Serveur HTTP de challenge interactif

Dans cet exercice, vous allez implémenter la partie serveur d’une application de challenge en Go.
Le code HTML du client est déjà fourni pour la structure et l’interface (partie coffre), vous n’avez donc qu’à vous concentrer sur la logique serveur.

Objectif

* Créer un serveur HTTP en Go qui expose plusieurs routes en fonction de ce que demande le challenge.
* Gérer des requêtes GET et POST avec des réponses précises attendues par le client.
* Pouvoir conserver l'état entre plusieurs requêtes (cas dans le challenge).

---

### Énoncé

Le client mystérieux vous envoie des requêtes et c'est à vous de les traiter et de lui renvoyer ce qu'il attend.
A la fin de votre échange, le client mystérieux vous donnera une récompense.

---

### Commandes d'exécution

```bash
go run 3.go
```

Puis accédez à la page qui donne accès au coffre. Vous y trouverez le client mystérieux et pourrez alors communiquer avec lui.
Votre serveur doit tourner sur le port 8593 de votre localhost.
