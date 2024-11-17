package tea

import (
	"fmt"

	"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/diegorezm/wallpapercl/internal/models"
)

var docStyle = lipgloss.NewStyle().Margin(1, 2)

type wallpaperItem struct {
	models.Wallpaper
}

func (w wallpaperItem) Title() string       { return w.Name }
func (w wallpaperItem) Description() string { return w.Path }
func (w wallpaperItem) FilterValue() string { return w.Name }

type wallpaperModel struct {
	list         list.Model
	selectedMode models.WallpaperMode
	modes        []models.WallpaperMode
	setWallpaper func(models.Wallpaper) error
	setMode      func(models.WallpaperMode) error
	config       *models.Config
	modeIndex    int
}

func (m wallpaperModel) Init() tea.Cmd {
	return nil
}

func (m *wallpaperModel) cycleMode() {
	// Cycle through available modes
	m.modeIndex = (m.modeIndex + 1) % len(m.modes)
	m.selectedMode = m.modes[m.modeIndex]
	m.config.SetMode(m.selectedMode)
}

func (m wallpaperModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c", "q":
			return m, tea.Quit
		case "enter":
			// Apply the wallpaper with the selected mode
			selected := m.list.SelectedItem().(wallpaperItem)
			selected.Apply(m.selectedMode)
			m.config.SetCurrentWallpaper(selected.Wallpaper)
			return m, nil
		case "tab":
			m.cycleMode()
		}
	case tea.WindowSizeMsg:
		h, v := docStyle.GetFrameSize()
		m.list.SetSize(msg.Width-h, msg.Height-v)
	}

	var cmd tea.Cmd
	m.list, cmd = m.list.Update(msg)
	return m, cmd
}

func (m wallpaperModel) View() string {
	modeDisplay := fmt.Sprintf("Mode: %s (Press TAB to change)", m.selectedMode)
	return docStyle.Render(m.list.View() + "\n" + modeDisplay)
}

func NewWallpaperSelector(config *models.Config) wallpaperModel {
	dir, err := models.NewDir(*config.CurrentDirectory)
	if err != nil {
		panic(err)
	}

	// Convert the wallpapers into list items
	items := make([]list.Item, len(dir.Wallpapers))
	for i, wp := range dir.Wallpapers {
		items[i] = wallpaperItem{Wallpaper: wp}
	}

	// Available wallpaper modes
	modes := []models.WallpaperMode{
		models.Center,
		models.Max,
		models.Stretch,
		models.Zoom,
	}

	// Create the list model
	wallpaperList := list.New(items, list.NewDefaultDelegate(), 0, 0)
	wallpaperList.Title = "Select Your Wallpaper"
	wallpaperList.SetShowHelp(true)

	return wallpaperModel{
		list:         wallpaperList,
		config:       config,
		modes:        modes,
		selectedMode: *config.Mode,
		modeIndex:    0,
	}
}
