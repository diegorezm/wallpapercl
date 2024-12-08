package server

import (
	"encoding/json"
	"log"
	"net/http"

	"github.com/diegorezm/wallpapercl"
	"github.com/diegorezm/wallpapercl/internal/models"
)

type ServerOpts struct {
	Config *models.Config
	Dir    *models.Dir
	Port   string
}

type server struct {
	ServerOpts
}

type apiError struct {
	Error string `json:"error"`
}

func NewServer(opts *ServerOpts) *server {
	return &server{
		ServerOpts: *opts,
	}
}

func (s *server) Start() {
	assets, err := wallpapercl.Assets()
	if err != nil {
		panic(err)
	}

	fs := http.FileServer(http.FS(assets))
	http.Handle("/", http.StripPrefix("/", fs))

	imageHandler := http.FileServer(http.Dir(s.Dir.Path))
	http.Handle("/images/", http.StripPrefix("/images/", imageHandler))

	http.HandleFunc("/api/wallpapers", func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case "GET":
			w.Header().Set("Content-Type", "application/json")
			json.NewEncoder(w).Encode(s.Dir.Wallpapers)
		case "POST":
			var wallpaper models.Wallpaper
			err := json.NewDecoder(r.Body).Decode(&wallpaper)
			if err != nil {
				json.NewEncoder(w).Encode(apiError{Error: err.Error()})
			}
			wallpaper.Apply(*s.Config.Mode)
			w.WriteHeader(http.StatusNoContent)
		}
	})

	http.HandleFunc("/api/config", func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case "GET":
			json.NewEncoder(w).Encode(s.Config)
		case "POST":
			type request struct {
				Mode string `json:"mode"`
			}
			var req request
			err := json.NewDecoder(r.Body).Decode(&req)
			if err != nil {
				json.NewEncoder(w).Encode(apiError{Error: err.Error()})
			}
			m := models.WallpaperMode(req.Mode)
			s.Config.SetMode(m)
			json.NewEncoder(w).Encode(s.Config)
		}
	})
	var port string

	if s.Port == "" {
		port = ":7272"
	} else {
		port = s.Port
	}

	log.Printf("Listening on http://localhost%s", port)

	log.Fatal(http.ListenAndServe(port, nil))
}
