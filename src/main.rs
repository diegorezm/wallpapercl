mod config;
mod tui;
mod wallpapercl;

use std::{env, process::Command};

use color_eyre::Result;
use config::Config;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Vec<String> = env::args().collect();

    let cfg = Config::new();

    if args.len() == 2 {
        let cmd = &args[1];
        if cmd == "restore" {
            let home = env::var("HOME").expect("$HOME not set.");
            let command = format!("{}/.fehbg", home);

            Command::new(command)
                .spawn()
                .expect("Could not restore the wallpaper.");
            return Ok(());
        }
    }

    let terminal = ratatui::init();

    let app_result = tui::App::new(cfg).run(terminal);
    ratatui::restore();
    app_result
}
