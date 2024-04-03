use crate::modules::dir::Dir;
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, self},
    io::{copy, stdout, BufWriter},
    io::{Result, Write},
};
use tauri::api::path::cache_dir;

#[derive(Serialize, Deserialize)]
pub struct Cache {
    img_file_name: String,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            img_file_name: "wallpaper_cl_image_cached.json".to_string(),
        }
    }

    fn cache_dir_to_str(path: Option<&str>) -> Option<String> {
        if let Some(str_path) = path {
            return Some(str_path.to_string());
        }
        None
    }

    pub fn save_to_cache(&self, dir: &Dir) -> Result<()> {
        match cache_dir() {
            Some(path) => match Self::cache_dir_to_str(path.to_str()) {
                Some(path_str) => {
                    let file_path = format!("{path_str}/{0}", self.img_file_name);
                    let file = File::create(file_path)?;
                    let mut writer = BufWriter::new(file);
                    let _ = serde_json::to_writer(&mut writer, dir);
                    let _ = writer.flush();
                    Ok(())
                }
                None => Ok(()),
            },
            None => Ok(()),
        }
    }

    pub fn get_image_from_cache(&self) -> Option<Dir> {
        match cache_dir() {
            Some(path) => match Self::cache_dir_to_str(path.to_str()) {
                Some(path_str) => {
                    let file_path = format!("{path_str}/{0}", self.img_file_name);
                    if let Ok(file) = fs::read_to_string(file_path) {
                        let dir: Dir = serde_json::from_str(&file).unwrap();
                        return Some(dir);
                    }
                    None
                }
                None => None,
            },
            None => None,
        }
    }
}
