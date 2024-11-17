package tea

import (
	"os"
	"strings"
	"time"

	"github.com/charmbracelet/bubbles/filepicker"
	"github.com/charmbracelet/bubbles/help"
	"github.com/charmbracelet/bubbles/key"
	tea "github.com/charmbracelet/bubbletea"
)

type keyMap struct {
	Up    key.Binding
	Down  key.Binding
	Enter key.Binding
	Space key.Binding
	Left  key.Binding
	Right key.Binding
	Help  key.Binding
	Quit  key.Binding
}

func (k keyMap) ShortHelp() []key.Binding {
	return []key.Binding{k.Help, k.Quit, k.Space, k.Enter}
}

func (k keyMap) FullHelp() [][]key.Binding {
	return [][]key.Binding{
		{k.Up, k.Down, k.Enter, k.Space},
		{k.Left, k.Right, k.Quit, k.Help},
	}
}

var keys = keyMap{
	Up: key.NewBinding(
		key.WithKeys("up", "k"),
		key.WithHelp("↑/k", "move up"),
	),
	Down: key.NewBinding(
		key.WithKeys("down", "j"),
		key.WithHelp("↓/j", "move down"),
	),
	Left: key.NewBinding(
		key.WithKeys("left", "h"),
		key.WithHelp("←/h", "move left"),
	),
	Right: key.NewBinding(
		key.WithKeys("right", "l"),
		key.WithHelp("→/l", "move right"),
	),
	Help: key.NewBinding(
		key.WithKeys("?"),
		key.WithHelp("?", "toggle help"),
	),
	Quit: key.NewBinding(
		key.WithKeys("q", "esc", "ctrl+c"),
		key.WithHelp("q", "quit"),
	),
	Enter: key.NewBinding(
		key.WithKeys("enter"),
		key.WithHelp("enter", "Selects the current directory and quits"),
	),
	Space: key.NewBinding(
		key.WithKeys(" "),
		key.WithHelp("space", "Selects the current directory"),
	),
}

type directoryPickerModel struct {
	filepicker   filepicker.Model
	selectedFile string
	selectedDir  string
	quitting     bool
	keys         keyMap
	help         help.Model
	err          error
}

type clearErrorMsg struct{}

func clearErrorAfter(t time.Duration) tea.Cmd {
	return tea.Tick(t, func(_ time.Time) tea.Msg {
		return clearErrorMsg{}
	})
}

func (m directoryPickerModel) Init() tea.Cmd {
	return m.filepicker.Init()
}

func (m *directoryPickerModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch {
		case key.Matches(msg, keys.Quit):
			m.quitting = true
			return m, tea.Quit
		case key.Matches(msg, keys.Space):
			m.selectedFile = ""
			m.selectedDir = m.filepicker.CurrentDirectory
		case key.Matches(msg, keys.Enter):
			m.selectedDir = m.filepicker.CurrentDirectory
			return m, tea.Quit
		case key.Matches(msg, keys.Help):
			m.help.ShowAll = !m.help.ShowAll
		}
	case clearErrorMsg:
		m.err = nil
	}
	var cmd tea.Cmd
	m.filepicker, cmd = m.filepicker.Update(msg)
	return m, cmd
}

func (m *directoryPickerModel) View() string {
	if m.quitting {
		return ""
	}

	var s strings.Builder

	// Show error or directory selection status
	if m.err != nil {
		s.WriteString(m.filepicker.Styles.DisabledFile.Render(m.err.Error()))
	} else if m.selectedDir == "" {
		s.WriteString("Pick a directory:")
	} else {
		s.WriteString("Selected directory: " + m.filepicker.Styles.Selected.Render(m.selectedDir))
	}

	// Add filepicker view
	s.WriteString("\n" + m.filepicker.View() + "\n")

	// Add help view
	s.WriteString(m.help.View(keys))

	return docStyle.Render(s.String())
}

func (m *directoryPickerModel) GetSelectedDir() string {
	return m.selectedDir
}

func NewDirectoryPicker() directoryPickerModel {
	fp := filepicker.New()
	fp.AllowedTypes = nil
	fp.CurrentDirectory, _ = os.UserHomeDir()
	fp.DirAllowed = true

	m := directoryPickerModel{
		filepicker: fp,
		help:       help.New(),
	}

	return m
}
