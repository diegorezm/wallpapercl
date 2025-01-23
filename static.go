package wallpapercl

import (
	"embed"
	"io/fs"
)

//go:embed public/*
var assets embed.FS

func Assets() (fs.FS, error) {
	return fs.Sub(assets, "public")
}
