# GoLang-WalkThrough

## Bruteforce concurrent d'un hash

Dans cet exercice, vous allez implémenter un programme en Go pour casser un mot de passe dont le hash bcrypt est fourni.


L’objectif est de comparer deux approches :

* Version séquentielle : tester chaque mot de passe un par un.
* Version concurrente : utiliser des goroutines et des workers pour paralléliser le cassage.

Vous mesurerez la différence de temps d’exécution entre les deux techniques.

Votre objectif est de trouver le mot de passe hashé dans le rockyou.txt (avec la méthode séquentielle) et celui présent dans le 10k-most-common.txt (avec la méthode parallélisée).


---

### Rappel

Qu’est-ce qu’une fonction de hashage ?


Une fonction de hashage est un algorithme qui transforme une donnée (ex. un mot de passe) en une empreinte unique appelée hash.
Caractéristiques :

* Irréversible : on ne peut pas retrouver le mot de passe original à partir du hash.
* Déterministe : le même mot de passe donne toujours le même hash.
* Rapide à calculer, mais difficile à inverser (d’où le bruteforce).


Qu’est-ce que bcrypt ?


bcrypt est une fonction de hashage conçue pour les mots de passe.

* Elle est lente volontairement pour rendre les attaques bruteforce plus coûteuses.
* Elle inclut un salt (valeur aléatoire) pour éviter les attaques par tables arc-en-ciel.
* Utilisée couramment pour sécuriser les mots de passe dans les bases de données.

---


### Objectifs :


2-1.go -> Version initiale (séquentielle) :

```bash
Lire le hash cible et la wordlist.
Tester chaque mot séquentiellement.
Comparer son hash bcrypt avec le hash cible.
Afficher le mot trouvé et le temps d’exécution.
```

2-2.go -> Version concurrente :

```bash
Créer un pool de workers avec des goroutines.
Distribuer les mots de passe via un channel.
Synchroniser avec sync.WaitGroup.
Afficher le mot trouvé et le temps d’exécution.
```


Bonus :

* Limiter le nombre de goroutines avec un pool.
* Ajouter un timer pour mesurer la vitesse entre les 2 versions.
* Que se passe-t-il si vous augmentez le nombre de workers ?
* Quels sont les risques d'un trop grand nombre de goroutines ?

---

Entrée :

* Hash cible
* Fichier wordlist.txt.

Sortie :

* Mot de passe trouvé (ou message si absent).
* Temps total d’exécution.

---

### Exécution de commande:

Version séquentielle :

```bash
go run 2-1.go '$2a$10$fMseHU97hUrALqerox/xluLxU2L7aI9zY5HW94jmJxnvTdMz.uWGu' .\rockyou.txt
```

Version concurrente :

```bash
go run 2-2.go '$2a$10$rne/oP9aH7CgPbkhI..ZvOBhzePQkhW4s8gbpaTeeJG9nZnb0EtwG' .\10k-most-common.txt
```
