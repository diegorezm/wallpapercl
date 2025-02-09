use std::fs::read_dir;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Wallpaper {
    pub file_path: PathBuf,
    pub file_name: String,
}

impl Wallpaper {
    pub fn new(path: &Path) -> Self {
        let file_path = path.to_path_buf();
        let file_name_raw = file_path.file_name().unwrap_or_default().to_str();
        let file_name = file_name_raw.unwrap().to_string();
        Self {
            file_path,
            file_name,
        }
    }

    pub fn is_image(&self) -> bool {
        if let Some(extension) = self.file_path.extension() {
            if let Some(ext_str) = extension.to_str() {
                match ext_str.to_lowercase().as_str() {
                    "jpg" | "jpeg" | "png" => {
                        return true;
                    }
                    _ => return false,
                }
            }
        }
        false
    }

    pub fn change_bg(&self, mode: wallpaper::Mode) -> bool {
        if let Some(path_string) = self.file_path.to_str() {
            if wallpaper::set_from_path(path_string).is_ok() {
                if wallpaper::set_mode(mode).is_ok() {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug, Clone, Default)]
pub struct Dir {
    pub dir_path: String,
    pub dir_files: Vec<Wallpaper>,
}

impl Dir {
    pub fn new(path: String) -> Dir {
        let mut dir = Dir {
            dir_path: path,
            dir_files: Vec::new(),
        };
        dir.list_dir();
        dir
    }

    fn list_dir(&mut self) {
        let mut file_paths: Vec<Wallpaper> = Vec::new();
        if let Ok(entries) = read_dir(&self.dir_path) {
            for entry in entries.flatten() {
                let file_path = entry.path();
                let wallpaper = Wallpaper::new(&file_path);
                if wallpaper.is_image() {
                    file_paths.push(wallpaper);
                }
            }
        }
        self.dir_files = file_paths;
    }
}
