package models

import (
	"errors"
	"os/exec"
)

type Wallpaper struct {
	Name string `json:"name"`
	Path string `json:"path"`
}

type WallpaperMode string

// Options for xwallpaper
const (
	Center  WallpaperMode = "center"
	Max     WallpaperMode = "maximize"
	Stretch WallpaperMode = "stretch"
	Zoom    WallpaperMode = "zoom"
)

func CheckXWallpaperInstalled() error {
	_, err := exec.LookPath("xwallpaper")
	if err != nil {
		return errors.New("xwallpaper is not installed or not in the PATH")
	}
	return nil
}

func (w *Wallpaper) Apply(mode WallpaperMode) error {
	if err := CheckXWallpaperInstalled(); err != nil {
		return err
	}

	var args []string

	switch mode {
	case Center:
		args = []string{"--center"}
	case Max:
		args = []string{"--maximize"}
	case Stretch:
		args = []string{"--stretch"}
	case Zoom:
		args = []string{"--zoom"}
	}

	args = append(args, w.Path)
	err := exec.Command("xwallpaper", args...).Run()

	return err
}
