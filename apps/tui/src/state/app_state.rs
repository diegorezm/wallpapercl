use ratatui::widgets::ListState;
use wallpaper_control::Dir;

pub struct WallpaperState {
    pub dir: Dir,
    pub list_state: ListState,
    pub filtered_indices: Vec<usize>,
    pub selected_wallpaper_idx: usize,
    pub wallpaper_mode: wallpaper::Mode,
}

#[derive(Default)]
pub struct ErrorState {
    pub message: Option<String>,
}

impl WallpaperState {
    pub fn new(dir: Dir, selected_index: Option<usize>) -> Self {
        let idx = selected_index.unwrap_or(0);
        Self {
            dir,
            list_state: ListState::default().with_selected(Some(idx)),
            filtered_indices: Vec::new(),
            selected_wallpaper_idx: idx,
            wallpaper_mode: wallpaper::Mode::Fit,
        }
    }
}
