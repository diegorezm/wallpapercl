#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod modules;
use std::sync::Mutex;

use crate::modules::cache::Cache;

use self::modules::dir::Dir;
use self::modules::wallpaper::Wallpaper;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref DIR: Mutex<Dir> = Mutex::new(Dir::new());
}

fn set_cached_info(dir: &Dir) {
    let cache: Cache = Cache::new();
    let _ = cache.save_to_cache(dir);
}

#[tauri::command]
fn get_cached_dir() -> Option<Dir> {
    let cache: Cache = Cache::new();
    if let Some(dir) = cache.get_image_from_cache() {
        return Some(dir);
    };
    None
}

#[tauri::command]
fn list_dir(path: &str) -> Dir {
    let mut dir = DIR.lock().expect("Failed to acquire lock on dir");
    dir.set_path(path);
    let dir_clone = dir.clone();
    set_cached_info(&dir_clone);
    dir_clone
}

#[tauri::command]
fn search(query: &str) -> Vec<Wallpaper> {
    let dir = DIR.lock().expect("Failed to acquire lock on dir");
    match dir.is_initialized() {
        true => dir.search(query),
        false => vec![],
    }
}

#[derive(Serialize, Deserialize)]
enum WallpaperMode {
    Center,
    Crop,
    Fit,
    Span,
    Stretch,
    Tile,
}

#[tauri::command]
fn change_wallpaper(wallpaper: Wallpaper, mode: WallpaperMode) -> Result<bool, String> {
    let m = match mode {
        WallpaperMode::Center => wallpaper::Mode::Center,
        WallpaperMode::Crop => wallpaper::Mode::Crop,
        WallpaperMode::Fit => wallpaper::Mode::Fit,
        WallpaperMode::Span => wallpaper::Mode::Span,
        WallpaperMode::Stretch => wallpaper::Mode::Stretch,
        WallpaperMode::Tile => wallpaper::Mode::Tile,
    };

    Ok(wallpaper.change_bg(m))
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            let mut dir = DIR.lock().expect("Failed to acquire lock on dir");
            if let Some(rdir) = get_cached_dir() {
                *dir = rdir;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            change_wallpaper,
            list_dir,
            search,
            get_cached_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
