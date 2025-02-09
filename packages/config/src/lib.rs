// This module holds the code for the configuration of this app
use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

pub struct Config {
    config_file_path: PathBuf,
    pub stored_wallpaper_path: String,
    pub stored_wallpaper_index: usize,
}

impl Config {
    pub fn new() -> Config {
        let home = env::var("HOME").expect("$HOME not set.");
        let config_file_path =
            PathBuf::from(format!("{}/.local/share/wallpapercl/config.txt", home));

        let mut config = Config {
            config_file_path,
            stored_wallpaper_path: String::default(),
            stored_wallpaper_index: 0, // Default value
        };

        config.setup();
        config
    }

    fn setup(&mut self) {
        if let Some(parent) = self.config_file_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).expect("Failed to create config directory.");
            }
        }

        if !self.config_file_path.exists() {
            self.reset();
            return;
        }

        let mut file = File::open(&self.config_file_path).expect("Failed to open config file.");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read config file.");

        let mut lines = content.lines();

        // Read the first line as the directory path
        if let Some(dir_path) = lines.next() {
            if !dir_path.trim().is_empty() && Path::new(dir_path.trim()).is_dir() {
                self.stored_wallpaper_path = dir_path.trim().to_string();
            } else {
                self.reset();
                return;
            }
        }

        // Read the second line as the stored number
        if let Some(number) = lines.next() {
            if let Ok(parsed_number) = number.trim().parse::<usize>() {
                self.stored_wallpaper_index = parsed_number;
            } else {
                self.reset();
            }
        }
    }

    pub fn set_config(&mut self, dir_path: String, number: usize) {
        let path = Path::new(&dir_path);
        if path.is_dir() {
            let mut file =
                File::create(&self.config_file_path).expect("Failed to open config file.");
            let content = format!("{}\n{}", dir_path, number);
            file.write_all(content.as_bytes())
                .expect("Failed to write to config file.");

            self.stored_wallpaper_path = dir_path;
            self.stored_wallpaper_index = number;
        } else {
            panic!("Provided path is not a directory.");
        }
    }

    pub fn reset(&mut self) {
        if self.config_file_path.exists() {
            fs::remove_file(&self.config_file_path).expect("Failed to delete config file.");
        }
        let mut file =
            File::create(&self.config_file_path).expect("Failed to create new config file.");
        file.write_all(b"\n0")
            .expect("Failed to write default values to config file.");

        self.stored_wallpaper_path.clear();
        self.stored_wallpaper_index = 0;
    }
}
