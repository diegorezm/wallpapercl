#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::{PathBuf, Path};
use std::process::Command;
use serde::{Serialize, Deserialize};
use reqwest;

#[derive(Serialize,Deserialize,Debug)]
struct APIResponse {
    id: String,
    path: String,
}
#[derive(Serialize,Deserialize,Debug)]
struct Data {
    data: Vec<APIResponse>
}
#[derive(Serialize,Deserialize,Debug)]
struct Error {
    error: String,
    message: String
}


#[tauri::command]
async fn get_wallpaper_api() -> Result<Data, Error> {
    let response = reqwest::get("https://wallhaven.cc/api/v1/search?sorting=toplist").await;
    match response {
        Ok(response) if response.status().is_success() => {
            let body = response.text().await;
            match body {
                Ok(body) => {
                    match serde_json::from_str::<Data>(&body) {
                        Ok(parsed) => {
                            Ok(parsed)
                        }
                        Err(err) => {
                            let error = Error {
                                error: err.to_string(),
                                message: "There was a problem while trying to parse server response!".to_string()
                            };
                            Err(error)
                        }
                    }
                }
                Err(err) => {
                    let error = Error {
                        error: err.to_string(),
                        message: "There was a problem while trying to get response body!".to_string()
                    };
                    Err(error)
                }
            }
        }
        Err(err) => {
            let error = Error {
                error: err.to_string(),
                message: "There was a problem while trying to get server response!".to_string()

            };
            Err(error)
        }
        _ => {
            panic!("Idk lol");
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Wallpapers {
    file_path: PathBuf,
    file_name: String,
}

fn is_image(file_path: &Path) -> bool {
    if let Some(extension) = file_path.extension() {
        if let Some(ext_str) = extension.to_str() {
            match ext_str.to_lowercase().as_str() {
                "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "ico" | "tiff" | "tga" | "svg" => {
                    return true;
                }
                _ => return false,
            }
        }
    }
    false
}

#[tauri::command]
fn change_bg_image(img_path: &str) -> bool {
    let path = Path::new(&img_path);
    match is_image(path) {
        true => { 
            let command = Command::new("changer").arg(img_path).output();
            if let Ok(_output) = command {
                return true;
            } else {
                return false;
            }
        },
        false => {
            return false
        }
    }
}

#[tauri::command]
fn list_dir(dir_path: &str) -> Result<Vec<Wallpapers>, ()> {
    let mut file_paths: Vec<Wallpapers> = Vec::new();
    let entries = match fs::read_dir(dir_path) {
        Ok(entries) => entries,
        Err(_) => return Ok(Vec::new()),
    };
    for entry in entries {
        if let Ok(entry) = entry {
            let file_path = entry.path();
            if is_image(&file_path){
                match file_path.file_name(){
                    Some(file_name_os) => {
                        if let Some(file_name) = file_name_os.to_str() {
                            let new_image = Wallpapers {
                                file_path: file_path.clone(),
                                file_name: file_name.to_string(),
                            };
                            file_paths.push(new_image);
                        }
                    }
                    _ => (),
                }
            }
        }
    }
    Ok(file_paths)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_dir, change_bg_image, get_wallpaper_api])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
