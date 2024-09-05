use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use wallpaper;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wallpaper {
    pub file_path: PathBuf,
    pub file_name: String,
    pub path: String,
}

impl Wallpaper {
    pub fn new(path: &Path) -> Self {
        let file_path = path.to_path_buf();
        let file_name_raw = file_path.file_name().unwrap_or_default().to_str();
        let file_name = file_name_raw.unwrap().to_string();
        let path = Self::convert_path_to_url(file_path.to_str().unwrap());
        Self {
            file_path,
            file_name,
            path,
        }
    }

    fn convert_path_to_url(path: &str) -> String {
        const PREFIX: &str = "asset://localhost/";
        let path_raw = path.to_string();
        let path_it = path_raw.replace('/', "%2F");
        let final_string = format!("{PREFIX}{path_it}");
        final_string
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

    pub fn change_bg(&self, mode: Option<wallpaper::Mode>) -> bool {
        if let Some(path_string) = self.file_path.to_str() {
            if wallpaper::set_from_path(path_string).is_ok() {
                let mode = mode.unwrap_or(wallpaper::Mode::Fit);
                if wallpaper::set_mode(mode).is_ok() {
                    return true;
                }
            }
        }
        false
    }
}
