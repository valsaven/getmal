package main

import (
	"encoding/json"
	"github.com/bmizerany/pat"
	"github.com/gocolly/colly/v2"
	"log"
	"net/http"
	"os"
	"strconv"
)

func parse(w http.ResponseWriter, r *http.Request) {
	params := r.URL.Query()

	// anime/manga
	list := params.Get(":list")
	username := params.Get(":username")
	status := params.Get(":status")

	// Status validation
	statuses := [6]int{1, 2, 3, 4, 6, 7}
	s, _ := strconv.Atoi(status)

	for i := range statuses {
		if statuses[i] == s {
			url := "https://myanimelist.net/" + list + "list/" + username + "?status=" + status

			c := colly.NewCollector()

			c.OnHTML(".list-table ", func(e *colly.HTMLElement) {
				data := e.Attr("data-items")
				formattedData, _ := json.MarshalIndent(data, "", "\t")
				w.Write(formattedData)
			})

			c.Visit(url)

			return
		}
	}

	w.WriteHeader(http.StatusBadRequest)
	w.Write([]byte("400 - Wrong status code!"))
}

func main() {
	port := os.Getenv("PORT")

	if port == "" {
		log.Fatal("$PORT must be set")
	}

	mux := pat.New()
	mux.Get("/:list/:username/:status", http.HandlerFunc(parse))

	http.Handle("/", mux)

	log.Println("Listening...")
	http.ListenAndServe(":" + port, nil)
}