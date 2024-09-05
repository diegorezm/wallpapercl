use super::wallpaper::Wallpaper;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::read_dir};

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct Dir {
    pub dir_path: String,
    pub dir_files: Vec<Wallpaper>,
    dir_files_hashed: HashMap<String, Wallpaper>,
}

impl Dir {
    pub fn new() -> Self {
        Self {
            dir_path: String::default(),
            dir_files: Vec::default(),
            dir_files_hashed: HashMap::default(),
        }
    }

    pub fn set_path(&mut self, path: &str) {
        self.dir_path = path.to_string();
        self.dir_files = Self::list_dir(self);
        self.dir_files_hashed = Self::list_dir_hashed(self);
    }

    fn list_dir_hashed(&self) -> HashMap<String, Wallpaper> {
        let mut file_paths: HashMap<String, Wallpaper> = HashMap::new();
        if let Ok(entries) = read_dir(&self.dir_path) {
            entries.for_each(|entry| {
                if let Ok(entry) = entry {
                    let file_path = entry.path();
                    let wallpaper = Wallpaper::new(&file_path);
                    if wallpaper.is_image() {
                        file_paths.insert(wallpaper.file_name.to_string(), wallpaper);
                    }
                }
            });
        };
        file_paths
    }

    pub fn list_dir(&self) -> Vec<Wallpaper> {
        let mut file_paths: Vec<Wallpaper> = Vec::new();
        if let Ok(entries) = read_dir(&self.dir_path) {
            entries.for_each(|entry| {
                if let Ok(entry) = entry {
                    let file_path = entry.path();
                    let wallpaper = Wallpaper::new(&file_path);
                    if wallpaper.is_image() {
                        file_paths.push(wallpaper);
                    }
                }
            });
        };
        file_paths
    }

    pub fn search(&self, query: &str) -> Vec<Wallpaper> {
        if self.dir_files_hashed.contains_key(query) {
            if let Some(el) = self.dir_files_hashed.get(query) {
                return vec![el.clone()];
            }
        }
        let mut file_paths: Vec<Wallpaper> = Vec::new();
        self.dir_files_hashed.iter().for_each(|(key, value)| {
            if key.contains(query) {
                file_paths.push(value.clone());
            }
        });
        file_paths
    }
    pub fn is_initialized(&self) -> bool {
        self.dir_path != String::default()
            || !self.dir_files.is_empty()
            || !self.dir_files_hashed.is_empty()
    }
}

impl Default for Dir {
    fn default() -> Self {
        Self::new()
    }
}
