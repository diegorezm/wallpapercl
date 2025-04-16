use crate::xresources::get_xresource_colors;
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
        let xr = get_xresource_colors();

        let foreground_str = xr.get("wallpapercl.fg:");
        let background_str = xr.get("wallpapercl.bg:");
        let primary_str = xr.get("wallpapercl.pr:");
        let subtext_str = xr.get("wallpapercl.sb:");
        let red_str = xr.get("*color1");

        // This feels wrong, but surely there will be no problems
        let foreground = *foreground_str.unwrap_or(&Color::Rgb(186, 194, 222));
        let background = *background_str.unwrap_or(&Color::Rgb(30, 30, 46));
        let primary = *primary_str.unwrap_or(&Color::Rgb(203, 166, 247));
        let subtext = *subtext_str.unwrap_or(&Color::Rgb(147, 153, 178));
        let red = *red_str.unwrap_or(&Color::Rgb(243, 139, 168));

        Theme {
            foreground,
            background,
            primary,
            subtext,
            red,
            selected_style: Style::new()
                .bg(foreground)
                .fg(background)
                .add_modifier(Modifier::BOLD),
        }
    }
}
