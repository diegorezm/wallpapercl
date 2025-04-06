use std::process::Command;

use ratatui::style::Color;

pub fn get_xresource_color(color_name: &str) -> Option<Color> {
    let output = Command::new("xrdb").arg("-query").output().ok()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.starts_with(color_name) {
                if let Some(color_value) = line.split_whitespace().last() {
                    if let Ok(rgb) = hex_to_rgb(color_value) {
                        return Some(Color::Rgb(rgb.0, rgb.1, rgb.2));
                    }
                }
            }
        }
    }
    None
}

fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), String> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 {
        return Err("Invalid hex code length".to_string());
    }

    if let Ok(r) = u8::from_str_radix(&hex[0..2], 16) {
        if let Ok(g) = u8::from_str_radix(&hex[2..4], 16) {
            if let Ok(b) = u8::from_str_radix(&hex[4..6], 16) {
                return Ok((r, g, b));
            }
        }
    }

    Err("Invalid hex code format".to_string())
}
