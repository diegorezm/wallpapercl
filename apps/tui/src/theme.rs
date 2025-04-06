use crate::xresources::get_xresource_color;
use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub foreground: Color,
    pub background: Color,
    pub primary: Color,
    pub subtext: Color,
    pub red: Color,
    pub selected_style: Style,
}

impl Theme {
    pub fn new() -> Self {
        let foreground =
            get_xresource_color("walcl_foreground").unwrap_or(Color::Rgb(186, 194, 222));
        let background = get_xresource_color("walcl_background").unwrap_or(Color::Rgb(30, 30, 46));
        Theme {
            foreground,
            background,
            primary: get_xresource_color("walcl_primary").unwrap_or(Color::Rgb(203, 166, 247)),
            subtext: get_xresource_color("walcl_subtext").unwrap_or(Color::Rgb(147, 153, 178)),
            red: get_xresource_color("*color1").unwrap_or(Color::Rgb(243, 139, 168)),
            selected_style: Style::new()
                .bg(foreground)
                .fg(background)
                .add_modifier(Modifier::BOLD),
        }
    }
}
