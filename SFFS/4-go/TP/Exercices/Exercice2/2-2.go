package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"sync"
	"time"

	"golang.org/x/crypto/bcrypt"
)

func worker(words []string, target string, wg *sync.WaitGroup, result chan<- string) {
	defer wg.Done()
	for _, word := range words {
		// TODO : Comparer le hash avec le mot de passe candidat
		err := bcrypt.CompareHashAndPassword([]byte(target), []byte(word))
		if err == nil {
			result <- word
			return
		}
	}
}

func main() {
	if len(os.Args) < 3 {
		fmt.Println("Usage: go run main.go '<hash>' <wordlist>")
		return
	}

	// Récupération des paramètres
	target := os.Args[1]
	wordlist := os.Args[2]

	fmt.Println("Hash : ", target, ", longueur : ", len(target))

	// TODO : Ouvrir le fichier
	file, err := os.Open(wordlist)
	if err != nil {
		panic(err)
	}

	// TODO : Lecture du fichier
	scanner := bufio.NewScanner(file)
	words := []string{}
	for scanner.Scan() {
		words = append(words, strings.TrimSpace(scanner.Text()))
	}

	// Démarrer le chronomètre
	start := time.Now()

	// Variables pour channel et synchronisation
	result := make(chan string, 1)
	var wg sync.WaitGroup

	// TODO : Assigner un espace de recherche pour chaque worker
	chunkSize := 100
	for i := 0; i < len(words); i += chunkSize {
		end := i + chunkSize
		if end > len(words) {
			end = len(words)
		}
		wg.Add(1)
		go worker(words[i:end], target, &wg, result)
	}

	// TODO : Go routine pour attendre que tous les workers aient fini
	go func() {
		wg.Wait()
		close(result)
	}()

	// TODO : Regarder la réponse du channel
	// Si le hash est trouvé alors break
	found := ""
	for res := range result {
		found = res
		break
	}

	// TODO : Affichage du résultat
	elapsed := time.Since(start)
	if found != "" {
		fmt.Println("Mot de passe trouvé : ", found)
	} else {
		fmt.Println("Mot de passe non trouvé")
	}
	fmt.Println("Temps écoulé : ", elapsed)
}
