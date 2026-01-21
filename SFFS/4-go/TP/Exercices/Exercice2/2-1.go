package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"time"

	"golang.org/x/crypto/bcrypt"
)

func main() {
	if len(os.Args) < 3 {
		fmt.Println("Usage: go run main.go '<hash>' <wordlist>")
		return
	}

	// Récupération des paramètres
	target := os.Args[1]
	wordlist := os.Args[2]

	fmt.Println("Hash : ", target, ", longueur : ", len(target))

	// TODO: Ouvrir le fichier
	file, err := os.Open(wordlist)
	if err != nil {
		panic(err)
	}

	// TODO : Lire le fichier
	scanner := bufio.NewScanner(file)
	words := []string{}
	for scanner.Scan() {
		words = append(words, strings.TrimSpace(scanner.Text()))
	}

	// Démarrer le chronomètre
	start := time.Now()

	found := ""
	fmt.Println("Cassage du hash ", target, "...")

	// TODO : Casser le hash
	for _, word := range words {
		err := bcrypt.CompareHashAndPassword([]byte(target), []byte(word))
		if err == nil {
			found = word
			break
		}
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
