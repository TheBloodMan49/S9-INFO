# Introduction : Séquentielle vs Parallèle en Go 

## Objectifs

* Comprendre la différence entre exécution séquentielle et parallèle.
* Découvrir les goroutines, les channels et le modèle des workers.
* Tester du code simple pour voir concrètement les bénéfices de la concurrence en Go.

---

### 1. Exécution séquentielle

On commence par un exemple simple : exécuter plusieurs tâches l'une après l'autre.

```go
package main

import (
    "fmt"
    "time"
)

func task(name string) {
    fmt.Println("Début de", name)
    time.Sleep(1 * time.Second) // Simule un travail
    fmt.Println("Fin de", name)
}

func main() {
    task("Tâche 1")
    task("Tâche 2")
    task("Tâche 3")
}
```
Ici, chaque tâche attend la fin de la précédente.
Durée totale environ 3 secondes.

---

### 2. Exécution parallèle avec goroutines

En Go, on peut lancer une fonction dans une goroutine avec le mot-clé **go**.

```go
func main() {
    go task("Tâche 1")
    go task("Tâche 2")
    go task("Tâche 3")

    // Attendre que les goroutines finissent
    time.Sleep(2 * time.Second)
}
```

Les trois tâches démarrent en même temps.
Durée totale environ 1 seconde (exécution en parallèle).
Ici, on utilise **time.Sleep** pour attendre, mais ce n'est pas une bonne pratique. On verra mieux avec les channels.

---

### 3. Synchronisation avec channels

Les channels permettent de communiquer entre goroutines et de synchroniser leur fin.

```go
func task(name string, done chan bool) {
    fmt.Println("Début de", name)
    time.Sleep(1 * time.Second)
    fmt.Println("Fin de", name)
    done <- true // signaler que la tâche est terminée
}

func main() {
    done := make(chan bool, 3)

    go task("Tâche 1", done)
    go task("Tâche 2", done)
    go task("Tâche 3", done)

    // Attendre les 3 signaux
    for i := 0; i < 3; i++ {
        <-done
    }
}
```

Ici, on attend explicitement que les trois goroutines aient fini.
Durée totale environ 1 seconde, mais avec une synchronisation propre.

---

### 4. Modèle des workers

Les workers sont des goroutines qui consomment des tâches depuis un channel. C'est un modèle classique pour paralléliser du travail.

```go
func worker(id int, jobs <-chan int, results chan<- int) {
    for j := range jobs {
        fmt.Printf("Worker %d traite le job %d\n", id, j)
        time.Sleep(time.Second)
        results <- j * 2
    }
}

func main() {
    jobs := make(chan int, 5)
    results := make(chan int, 5)

    // Lancer 3 workers
    for w := 1; w <= 3; w++ {
        go worker(w, jobs, results)
    }

    // Envoyer 5 jobs
    for j := 1; j <= 5; j++ {
        jobs <- j
    }
    close(jobs)

    // Récupérer les résultats
    for a := 1; a <= 5; a++ {
        fmt.Println("Résultat :", <-results)
    }
}
```

Ici, 3 workers traitent les jobs en parallèle.
Durée totale environ 2 secondes au lieu de 5 si tout était séquentiel.

---

## Conclusion de l'introduction

* **Séquentiel** : simple mais lent.
* **Goroutines** : permettent de lancer des tâches concurrentes.
* **Channels** : synchronisation et communication.
* **Workers** : modèle efficace pour distribuer du travail.

