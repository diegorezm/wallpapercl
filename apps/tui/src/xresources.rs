use std::{collections::HashMap, process::Command};

use ratatui::style::Color;

pub fn get_xresource_colors() -> HashMap<String, Color> {
    let output_result = Command::new("xrdb").arg("-query").output();
    let mut map: HashMap<String, Color> = HashMap::new();

    match output_result {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let whitespaced_line: Vec<&str> = line.split_whitespace().collect();
                if let Some(&key) = whitespaced_line.first() {
                    if let Some(&color) = whitespaced_line.last() {
                        if let Ok(rgb) = hex_to_rgb(color) {
                            map.insert(key.to_string(), Color::Rgb(rgb.0, rgb.1, rgb.2));
                        }
                    }
                }
            }
            map
        }
        Ok(_) => HashMap::new(),
        Err(e) => {
            eprintln!("Error executing xrdb: {}", e);
            return HashMap::new();
        }
    }
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
