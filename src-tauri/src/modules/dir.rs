use super::wallpaper::Wallpaper;
use serde::{Deserialize, Serialize};
use std::fs::read_dir;

#[derive(Serialize, Deserialize)]
pub struct Dir {
    pub dir_path: String,
    pub dir_files: Vec<Wallpaper>,
}

impl Dir {
    pub fn new(path: &str) -> Self {
        let dir_path = path.to_string();
        let dir_files = Self::list_dir(path);
        Self {
            dir_path,
            dir_files,
        }
    }

    pub fn list_dir(dir_path: &str) -> Vec<Wallpaper> {
        let mut file_paths: Vec<Wallpaper> = Vec::new();
        match read_dir(dir_path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let file_path = entry.path();
                        let wallpaper = Wallpaper::new(&file_path);
                        if wallpaper.is_image(){
                            file_paths.push(wallpaper);
                        }
                    }
                }
                return file_paths;
            }
            Err(_) => return file_paths,
        };
    }
}
