package models

import (
	"encoding/base64"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

type Dir struct {
	Name       string      `json:"name"`
	Path       string      `json:"path"`
	Wallpapers []Wallpaper `json:"wallpapers"`
}

func setupDir(path string) (*Dir, error) {
	validFileExt := map[string]bool{
		".jpg":  true,
		".png":  true,
		".jpeg": true,
	}

	if _, err := os.Stat(path); os.IsNotExist(err) {
		return nil, fmt.Errorf("directory does not exist: %s", path)
	}

	files, err := os.ReadDir(path)

	if err != nil {
		return nil, err
	}

	var wallpapers []Wallpaper
	for _, file := range files {
		if file.IsDir() {
			continue
		}

		extension := strings.ToLower(filepath.Ext(file.Name()))
		if !validFileExt[extension] {
			continue
		}

		filePath := path + "/" + file.Name()
		fileData, err := os.ReadFile(filePath)
		if err != nil {
			return nil, err
		}

		base64Data := base64.StdEncoding.EncodeToString(fileData)

		wallpapers = append(wallpapers, Wallpaper{
			Name:    file.Name(),
			Path:    path + "/" + file.Name(),
			DataURL: "data:image/" + extension[1:] + ";base64," + base64Data,
		})
	}

	dirName := filepath.Base(path)

	dir := &Dir{
		Name:       dirName,
		Path:       path,
		Wallpapers: wallpapers,
	}

	return dir, nil
}

func NewDir(path string) (*Dir, error) {
	dir, err := setupDir(path)
	if err != nil {
		e := fmt.Errorf("error setting up dir: %v", err)
		return nil, e
	}
	return dir, nil
}
