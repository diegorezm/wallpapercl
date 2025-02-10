use ratatui::style::{Color, Modifier, Style};

pub const FOREGROUND_COLOR: Color = Color::Rgb(186, 194, 222);
pub const BACKGROUND_COLOR: Color = Color::Rgb(30, 30, 46);

// pub const CARD_BACKGROUND: Color = Color::Rgb(17, 17, 21);

pub const PRIMARY_COLOR: Color = Color::Rgb(203, 166, 247);
pub const SUBTEXT_COLOR: Color = Color::Rgb(147, 153, 178);
pub const RED_COLOR: Color = Color::Rgb(243, 139, 168);

pub const SELECTED_STYLE: Style = Style::new()
    .bg(PRIMARY_COLOR)
    .fg(BACKGROUND_COLOR)
    .add_modifier(Modifier::BOLD);
