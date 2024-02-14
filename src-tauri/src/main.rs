#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod modules;
use self::modules::dir::Dir;
use self::modules::wallpaper::Wallpaper;

#[tauri::command]
fn list_dir(path: &str) -> Dir {
    let dir = Dir::new(path);
    dir
}

#[tauri::command]
fn change_wallpaper(wallpaper: Wallpaper) -> Result<bool, String> {
    Ok(wallpaper.change_bg())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![change_wallpaper, list_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

