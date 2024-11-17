package models

import (
	"encoding/json"
	"os"
	"path/filepath"
)

type Config struct {
	Mode             *WallpaperMode `json:"mode,omitempty"`
	CurrentDirectory *string        `json:"currentDirectory,omitempty"`
	CurrentWallpaper *Wallpaper     `json:"currentWallpaper,omitempty"`
}

const (
	CONFIG_FILE = "config.json"
	CONFIG_DIR  = ".config/wallpapercl"
)

var defaultConfig = Config{
	Mode:             nil,
	CurrentDirectory: nil,
	CurrentWallpaper: nil,
}

func NewConfig() Config {
	return setupConfigFile()
}

func (c *Config) SetCurrentDirectory(dir string) error {
	c.CurrentDirectory = &dir
	return c.SaveConfig()
}

func (c *Config) SetMode(mode WallpaperMode) error {
	c.Mode = &mode
	return c.SaveConfig()
}

func (c *Config) SetCurrentWallpaper(wp Wallpaper) error {
	c.CurrentWallpaper = &wp
	return c.SaveConfig()
}

func (c *Config) SaveConfig() error {
	home := os.Getenv("HOME")
	if home == "" {
		return os.ErrNotExist
	}

	configDir := filepath.Join(home, CONFIG_DIR)
	configFile := filepath.Join(configDir, CONFIG_FILE)

	file, err := os.Create(configFile)
	if err != nil {
		return err
	}
	defer file.Close()

	encoder, err := json.MarshalIndent(c, "", "\t")

	if err != nil {
		return err
	}

	_, err = file.Write(encoder)
	if err != nil {
		return err
	}
	return nil
}

func setupConfigFile() Config {
	home := os.Getenv("HOME")
	if home == "" {
		panic("HOME environment variable not set")
	}

	configDir := filepath.Join(home, CONFIG_DIR)
	configFile := filepath.Join(configDir, CONFIG_FILE)

	// Ensure the config directory exists
	if _, err := os.Stat(configDir); os.IsNotExist(err) {
		err := os.MkdirAll(configDir, 0755)
		if err != nil {
			panic("failed to create config directory: " + err.Error())
		}
	}

	// If the config file doesn't exist, create it with default values
	if _, err := os.Stat(configFile); os.IsNotExist(err) {
		saveConfigToFile(configFile, defaultConfig)
		return defaultConfig
	}

	// Load existing config file
	config, err := loadConfigFromFile(configFile)
	if err != nil {
		saveConfigToFile(configFile, defaultConfig)
		return defaultConfig
	}

	return config
}

// saveConfigToFile saves the config to a file
func saveConfigToFile(filePath string, config Config) {
	file, err := os.Create(filePath)
	if err != nil {
		panic("failed to create config file: " + err.Error())
	}
	defer file.Close()

	encoder, err := json.MarshalIndent(config, "", "\t")
	if err != nil {
		panic("failed to write to config file: " + err.Error())
	}
	_, err = file.Write(encoder)
	if err != nil {
		panic("failed to write to config file: " + err.Error())
	}
}

// loadConfigFromFile loads the config from a file
func loadConfigFromFile(filePath string) (Config, error) {
	file, err := os.Open(filePath)
	if err != nil {
		return Config{}, err
	}
	defer file.Close()

	var config Config
	decoder := json.NewDecoder(file)
	err = decoder.Decode(&config)
	if err != nil {
		return Config{}, err
	}

	return config, nil
}
