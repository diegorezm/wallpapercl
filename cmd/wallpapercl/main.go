package main

import (
	"flag"
	"fmt"

	bubbletea "github.com/charmbracelet/bubbletea"
	"github.com/diegorezm/wallpapercl/internal/models"
	"github.com/diegorezm/wallpapercl/internal/server"
	"github.com/diegorezm/wallpapercl/internal/tea"
)

func main() {

	var current bool
	var changeDirectory bool
	var serve bool

	flag.BoolVar(&serve, "serve", false, "Start the server")
	flag.BoolVar(&current, "current", false, "Refresh the wallpapers")
	flag.BoolVar(&changeDirectory, "change-directory", false, "Change the wallpaper directory")

	flag.Parse()

	config := models.NewConfig()

	if config.CurrentDirectory == nil || config.Mode == nil || config.CurrentWallpaper == nil {
		dp := tea.NewDirectoryPicker()
		pp := bubbletea.NewProgram(&dp, bubbletea.WithAltScreen())
		if _, err := pp.Run(); err != nil {
			fmt.Println(err)
		}

		dir, err := models.NewDir(dp.GetSelectedDir())
		if err != nil {
			panic(err)
		}

		config.SetCurrentDirectory(dir.Path)
		config.SetMode(models.Zoom)
		config.SaveConfig()
	}

	if serve {
		dir, err := models.NewDir(*config.CurrentDirectory)
		if err != nil {
			panic(err)
		}
		s := server.NewServer(&server.ServerOpts{
			Config: &config,
			Dir:    dir,
		})
		s.Start()
	}

	if current {
		config.CurrentWallpaper.Apply(*config.Mode)
	} else if changeDirectory {
		dp := tea.NewDirectoryPicker()
		pp := bubbletea.NewProgram(&dp, bubbletea.WithAltScreen())
		if _, err := pp.Run(); err != nil {
			fmt.Println(err)
		}

		dir, err := models.NewDir(dp.GetSelectedDir())
		if err != nil {
			panic(err)
		}

		config.SetCurrentDirectory(dir.Path)
		config.SaveConfig()
	} else {
		m := tea.NewWallpaperSelector(&config)
		p := bubbletea.NewProgram(m, bubbletea.WithAltScreen())
		if _, err := p.Run(); err != nil {
			fmt.Println(err)
		}
	}

}
