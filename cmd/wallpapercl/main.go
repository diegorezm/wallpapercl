package main

import (
	"flag"
	"fmt"

	bubbletea "github.com/charmbracelet/bubbletea"
	"github.com/diegorezm/wallpapercl/internal/models"
	"github.com/diegorezm/wallpapercl/internal/tea"
)

func main() {

	var serve bool
	var current bool
	var changeDirectory bool

	flag.BoolVar(&serve, "serve", false, "Start the server")

	flag.BoolVar(&current, "current", false, "Refresh the wallpapers")
	flag.BoolVar(&changeDirectory, "change-directory", false, "Change the wallpaper directory")

	flag.Parse()

	config := models.NewConfig()

	if config.Dir == nil || config.Mode == nil || config.CurrentWallpaper == nil {
		dp := tea.NewDirectoryPicker()
		pp := bubbletea.NewProgram(&dp, bubbletea.WithAltScreen())
		if _, err := pp.Run(); err != nil {
			fmt.Println(err)
		}

		dir, err := models.NewDir(dp.GetSelectedDir())
		if err != nil {
			panic(err)
		}

		config.SetDir(*dir)
		config.SetMode(models.Zoom)
		config.SaveConfig()
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
		config.SetDir(*dir)
	} else {
		m := tea.NewWallpaperSelector(&config)
		p := bubbletea.NewProgram(m, bubbletea.WithAltScreen())
		if _, err := p.Run(); err != nil {
			fmt.Println(err)
		}
	}

}
