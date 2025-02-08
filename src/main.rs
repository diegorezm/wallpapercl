mod config;
mod tui;
mod wallpapercl;

use color_eyre::Result;
use config::Config;

fn main() -> Result<()> {
    color_eyre::install()?;

    let cfg = Config::new();

    let terminal = ratatui::init();

    let app_result = tui::App::new(cfg).run(terminal);
    ratatui::restore();
    app_result
}
