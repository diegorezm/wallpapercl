package server

import (
	"log"
	"net/http"
	"path"
	"strings"

	"github.com/diegorezm/wallpapercl"
	"github.com/diegorezm/wallpapercl/internal/models"
	"github.com/diegorezm/wallpapercl/internal/views/pages"
)

type ServerOpts struct {
	Config *models.Config
	Dir    *models.Dir
}

type server struct {
	ServerOpts
}

func NewServer(opts *ServerOpts) *server {
	return &server{
		ServerOpts: *opts,
	}
}

func servePublic(w http.ResponseWriter, r *http.Request) {
	// Remove the "/public" prefix from the URL to find the correct file
	filePath := strings.TrimPrefix(r.URL.Path, "/public/")

	// Check if the file exists in the embedded content
	data, err := wallpapercl.StaticFiles.ReadFile(path.Join("public", filePath))
	if err != nil {
		http.Error(w, "File not found", http.StatusNotFound)
		return
	}

	// Set appropriate headers based on file extension
	ext := path.Ext(filePath)
	switch ext {
	case ".css":
		w.Header().Set("Content-Type", "text/css")
	case ".js":
		w.Header().Set("Content-Type", "application/javascript")
	case ".jpg", ".jpeg":
		w.Header().Set("Content-Type", "image/jpeg")
	case ".png":
		w.Header().Set("Content-Type", "image/png")
	case ".gif":
		w.Header().Set("Content-Type", "image/gif")
	default:
		w.Header().Set("Content-Type", "application/octet-stream")
	}

	// Serve the file content
	w.Write(data)
}

func (s *server) Start() {
	http.HandleFunc("/public/", servePublic)

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		if r.Method == http.MethodGet {
			ctx := r.Context()
			index := pages.Index()
			if err := index.Render(ctx, w); err != nil {
				log.Fatal(err)
			}
		}
	})

	http.HandleFunc("/api/wallpapers", func(w http.ResponseWriter, r *http.Request) {
		if r.Method == http.MethodGet {
			query := r.URL.Query().Get("q")

			var results []models.Wallpaper
			if query == "" {
				results = s.Dir.Wallpapers
			} else {
				for _, wallpaper := range s.Dir.Wallpapers {
					if strings.Contains(strings.ToLower(wallpaper.Name), strings.ToLower(query)) {
						results = append(results, wallpaper)
					}
				}
			}

			var htmlResults string
			for _, wallpaper := range results {

				htmlResults += "<li>" +
					"<form  method='post' class='wallpaper-form' hx-post='/api/wallpapers' hx-target='#wallpapers' hx-swap='none'>" +
					"<button type='submit' class='btn btn-ghost'>" +
					"<input type='hidden' name='name' value='" + wallpaper.Name + "' />" +
					"<div>" +
					"<img src='" + wallpaper.DataURL + "' alt='" + wallpaper.Name + "' class='wallpaper-img'>" +
					"<h3>" + wallpaper.Name + "</h3>" +
					"</div>" +
					"</button>" +
					"</form>" +
					"</li>"
			}

			if htmlResults == "" {
				htmlResults = "<li>No wallpapers found</li>"
			}

			w.Header().Set("Content-Type", "text/html")
			w.Write([]byte(htmlResults))
		} else if r.Method == http.MethodPost {
			var wallpaper models.Wallpaper

			name := r.FormValue("name")

			if name == "" {
				http.Error(w, "name is required", http.StatusBadRequest)
				return
			}

			for _, w := range s.Dir.Wallpapers {
				if w.Name == name {
					wallpaper = w
				}
			}
			wallpaper.Apply(*s.Config.Mode)
			w.WriteHeader(http.StatusOK)
			w.WriteHeader(http.StatusNoContent)
		}
	})
	log.Println("Server started on http://localhost:8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
