use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct Wallpaper {
    file_path: PathBuf,
    file_name: String,
}

impl Wallpaper {
    pub fn new(path: &PathBuf) -> Self {
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

    pub fn change_bg(&self) -> bool {
        match self.is_image() {
            true => {
                let command = Command::new("changer").arg(&self.file_path).output();
                if let Ok(_output) = command {
                    return true;
                } else {
                    return false;
                }
            }
            false => return false,
        }
    }
}
