package main

import (
	"encoding/base64"
	"fmt"
	"net/http"
	"os"
	"sync"
)

func worker(id int, jobs <-chan string, results chan<- string, wg *sync.WaitGroup) {
	defer wg.Done()
	for encoded := range jobs {
		// TODO : Décoder base64
		decoded, err := base64.StdEncoding.DecodeString(encoded)
		// fmt.Printf("worker %d: decoded %s to %s\n", id, encoded, decoded)
		if err != nil {
			fmt.Printf("worker %d: error decoding %s: %v", id, encoded, err)
			continue
		}

		// TODO : Requête HTTP
		url := "http://localhost:8080" + string(decoded)
		resp, err := http.Get(url)
		if err != nil {
			fmt.Printf("worker %d: error fetching %s: %v", id, decoded, err)
		}

		// fmt.Printf("worker %d: fetched %s with status %d\n", id, decoded, resp.StatusCode)

		// TODO : Vérification du code retour pour filtrer les bonnes réponses
		if (resp.StatusCode == 200) || (resp.StatusCode == 401) || (resp.StatusCode == 403) {
			fmt.Println("Valid path: ", url)
			results <- string(decoded)
		}
	}
}

func main() {
	// TODO : Ouvrir le fichier
	file, err := os.Open("paths.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	// TODO : Initialisation channels et synchronisation variable
	jobs := make(chan string, 25000)
	results := make(chan string, 25000)
	var wg sync.WaitGroup

	// TODO : Lecture du fichier
	var paths []string
	for {
		var path string
		_, err := fmt.Fscanf(file, "%s\n", &path)
		if err != nil {
			break
		}
		paths = append(paths, path)
	}

	fmt.Println("Read done")

	for _, path := range paths {
		jobs <- path
	}

	fmt.Println("Jobs sent")

	// TODO : Collecte des résultats
	for w := 1; w <= 20; w++ {
		wg.Add(1)
		go worker(w, jobs, results, &wg)
	}

	fmt.Println("Workers created")

	close(jobs)

	wg.Wait()
	close(results)

	// TODO : Affichage des résultats

	// for res := range results {
	// 	fmt.Println("Valid path found: ", res)
	// }
}
