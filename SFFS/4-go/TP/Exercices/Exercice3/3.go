package main

import (
    "fmt"
    "io"
    "net/http"
	"strconv"
	"sync"
)

// TODO : Créer les différents endpoints pour interagir avec le serveur

var (
	clientANum chan int
	clientBNum chan int
	once sync.Once
)

func init() {
	clientANum = make(chan int, 1)
	clientBNum = make(chan int, 1)
}

func HandlePing(w http.ResponseWriter, r *http.Request) {
    fmt.Fprint(w, "pong")
}

func HandleFlag(w http.ResponseWriter, r *http.Request) {
    if r.Method != http.MethodPost {
        http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
        return
    }

    body, err := io.ReadAll(r.Body)
    if err != nil {
        http.Error(w, "Internal server error", http.StatusInternalServerError)
        return
    }
    flag := string(body)
    fmt.Println("Flag received: ", flag)
    fmt.Fprint(w, "ok")
}

func HandleEcho(w http.ResponseWriter, r *http.Request) {
    
    token := r.URL.Query().Get("token")
    if token == "" {
        http.Error(w, "Bad request", http.StatusBadRequest)
        return
    }
    fmt.Println("Echo token received: ", token)
    fmt.Fprint(w, token)
}

func HandleClientA(w http.ResponseWriter, r *http.Request) {
	numStr := r.URL.Query().Get("num")
	num, err := strconv.Atoi(numStr)
	if err != nil {
		http.Error(w, "Invalid num parameter", http.StatusBadRequest)
		return
	}

    fmt.Println("Client A received num: ", num)
	
	select {
	case clientANum <- num:
	default:
		<-clientANum
		clientANum <- num
	}
	
	fmt.Fprintf(w, "%d", num)
}

func HandleClientB(w http.ResponseWriter, r *http.Request) {
	numStr := r.URL.Query().Get("num")
	num, err := strconv.Atoi(numStr)
	if err != nil {
		http.Error(w, "Invalid num parameter", http.StatusBadRequest)
		return
	}

    fmt.Println("Client B received num: ", num)
	
	select {
	case clientBNum <- num:
	default:
		<-clientBNum
		clientBNum <- num
	}
	
	fmt.Fprintf(w, "%d", num)
}

func HandleResult(w http.ResponseWriter, r *http.Request) {
	numA := <-clientANum
	numB := <-clientBNum
	
	sum := numA + numB
	fmt.Println("Result: ", sum)
	fmt.Fprintf(w, "%d", sum)
}

// endpoint
func Handler(w http.ResponseWriter, r *http.Request) {
    fmt.Fprint(w, "")
}

func main() {
    http.HandleFunc("/", Handler)
    http.HandleFunc("/ping", HandlePing)
    http.HandleFunc("/flag1", HandleFlag)
    http.HandleFunc("/echo", HandleEcho)
    http.HandleFunc("/flag2", HandleFlag)
	http.HandleFunc("/clientA", HandleClientA)
	http.HandleFunc("/clientB", HandleClientB)
	http.HandleFunc("/result", HandleResult)
    http.HandleFunc("/flag3", HandleFlag)
    

    fmt.Println("Serveur démarré sur le port 8593...")
    http.ListenAndServe(":8593", nil)
}
