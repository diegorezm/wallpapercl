package server

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
	"path/filepath"
	"strings"

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
	t := NewTemplateManager()
	mux := http.NewServeMux()

	imageHandler := http.FileServer(http.Dir(s.Dir.Path))
	mux.Handle("GET /images/", http.StripPrefix("/images/", imageHandler))

	mux.HandleFunc("GET /static/", serveStatic)

	mux.HandleFunc("GET /", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Contet-Type", "text/html")
		ctx := context.Background()
		t.Render(w, "index.html", nil, ctx)
	})

	mux.HandleFunc("GET /api/wallpapers", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(s.Dir.Wallpapers)
	})

	mux.HandleFunc("POST /api/wallpapers", func(w http.ResponseWriter, r *http.Request) {
		var wallpaper models.Wallpaper
		err := json.NewDecoder(r.Body).Decode(&wallpaper)
		if err != nil {
			json.NewEncoder(w).Encode(apiError{Error: err.Error()})
		}
		wallpaper.Apply(*s.Config.Mode)
		w.WriteHeader(http.StatusNoContent)
	})

	mux.HandleFunc("GET /api/config", func(w http.ResponseWriter, r *http.Request) {
		json.NewEncoder(w).Encode(s.Config)
	})

	mux.HandleFunc("POST /api/config", func(w http.ResponseWriter, r *http.Request) {
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
	})

	var port string

	if s.Port == "" {
		port = ":7272"
	} else {
		port = s.Port
	}

	log.Printf("Listening on http://localhost%s", port)

	log.Fatal(http.ListenAndServe(port, mux))
}

func getContentType(filePath string) string {
	ext := strings.ToLower(filepath.Ext(filePath))
	switch ext {
	case ".js":
		return "application/javascript"
	case ".css":
		return "text/css"
	case ".html":
		return "text/html"
	case ".json":
		return "application/json"
	case ".jpg", ".jpeg":
		return "image/jpeg"
	case ".png":
		return "image/png"
	case ".gif":
		return "image/gif"
	case ".svg":
		return "image/svg+xml"
	default:
		return "application/octet-stream"
	}
}

func serveStatic(w http.ResponseWriter, r *http.Request) {
	assets, err := wallpapercl.Assets()

	if err != nil {
		panic(err)
	}

	requestPath := r.URL.Path

	filePath := strings.Replace(requestPath, "/static/", "", 1)

	contentType := getContentType(filePath)

	file, err := assets.Open(filePath)

	if err != nil {
		http.Error(w, "File does not exist.", 404)
		return
	}

	defer file.Close()

	fileInfo, err := file.Stat()

	if err != nil {
		http.Error(w, "Unable to get file information.", 500)
		return
	}

	bytes := make([]byte, fileInfo.Size())

	_, err = file.Read(bytes)

	if err != nil {
		log.Printf("Error: %v", err)
		http.Error(w, "Something went wrong.", 500)
		return
	}
	w.Header().Set("Content-Type", contentType)
	w.Write(bytes)
}
