use serde::{Deserialize, Serialize};
use std::env::home_dir;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug,Serialize, Deserialize, Clone)]
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
        const SUFFIX: &str = "asset://localhost/";
        let path_raw = path.to_string();
        let path_it = path_raw.split('/');
        let mut final_string = SUFFIX.to_string();
        for p in path_it {
            let s = format!("%2F{p}");
            final_string.push_str(&s.to_string());
        }
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

    pub fn change_bg(&self) -> bool {
        if let Some(mut path) = home_dir() {
            path.push(".local/bin/scripts");
            let command_path = path.join("changer");
            match self.is_image() {
                true => {
                    let command = Command::new(command_path).arg(&self.file_path).output();

                    if let Ok(_output) = command {
                        return true;
                    } else {
                        return false;
                    }
                }
                false => return false,
            }
        } else {
            println!("Impossible to get your home dir!");
            return false;
        }
    }
}
