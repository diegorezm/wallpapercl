mod colors;
mod state;

use std::env;
use std::path::Path;
use std::process::Command;

use colors::{
    BACKGROUND_COLOR, CARD_BACKGROUND, FOREGROUND_COLOR, PRIMARY_COLOR, RED_COLOR, SELECTED_STYLE,
    SUBTEXT_COLOR,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::prelude::{Stylize, Widget};
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{
    Block, Borders, HighlightSpacing, List, ListItem, Paragraph, StatefulWidget,
};
use ratatui::{DefaultTerminal, Frame};
use state::app_state::{ErrorState, WallpaperState};
use state::input_state::{ChangeDirectoryState, SearchState, TextInputState};
use wallpaper_control::Dir;

use color_eyre::Result;

struct App {
    should_exit: bool,
    wallpaper_state: WallpaperState,
    error_state: ErrorState,
    change_directory_state: ChangeDirectoryState,
    search_state: SearchState,
    cfg: config::Config,
}

impl App {
    fn new() -> Self {
        let cfg = config::Config::new();
        let wallpaper_dir = if !cfg.stored_wallpaper_path.is_empty() {
            Dir::new(cfg.stored_wallpaper_path.clone())
        } else {
            Dir::new(String::default())
        };

        let wallpaper_state = WallpaperState::new(wallpaper_dir, Some(cfg.stored_wallpaper_index));
        let error_state = ErrorState::default();
        let change_directory_state =
            ChangeDirectoryState::new(Some(cfg.stored_wallpaper_path.clone()));
        let search_state = SearchState::new();

        Self {
            wallpaper_state,
            error_state,
            change_directory_state,
            search_state,
            cfg,
            should_exit: false,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| {
                let vertical_layout = Layout::vertical([
                    Constraint::Percentage(8),
                    Constraint::Percentage(88),
                    Constraint::Percentage(4),
                ])
                .horizontal_margin(2);
                let [input_area, list_area, help_area] = vertical_layout.areas(frame.area());

                if self.wallpaper_state.dir.dir_path == String::default()
                    || self.change_directory_state.is_input_focused
                {
                    self.render_change_directory_popup(frame);
                }

                self.render_search_input(input_area, frame);
                self.render_wallpaper_list(list_area, frame.buffer_mut());
                self.render_help_menu(help_area, frame.buffer_mut());
                self.render_error_popup(frame);
            })?;

            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        if self.error_state.message.is_some() {
            if key.code == KeyCode::Esc {
                self.error_state.message = None;
            }
            return;
        }

        if self.change_directory_state.is_input_focused {
            match key.code {
                KeyCode::Esc => self.change_directory_state.is_input_focused = false,
                KeyCode::Enter => {
                    if Path::new(&self.change_directory_state.input).is_dir() {
                        self.cfg.set_config(
                            self.change_directory_state.input.clone(),
                            self.wallpaper_state.selected_wallpaper_idx,
                        );
                        self.wallpaper_state.dir =
                            Dir::new(self.change_directory_state.input.clone());
                        self.change_directory_state.is_input_focused = false;
                    } else {
                        self.error_state.message =
                            Some("Invalid directory. Please enter a valid path.".to_string());
                    }
                }
                KeyCode::Char(c) => self.change_directory_state.enter_char(c),
                KeyCode::Backspace => self.change_directory_state.delete_char(),
                KeyCode::Left => self.change_directory_state.move_cursor_left(),
                KeyCode::Right => self.change_directory_state.move_cursor_right(),
                _ => {}
            }
        } else if self.search_state.is_input_focused {
            match key.code {
                KeyCode::Esc | KeyCode::Enter => self.search_state.is_input_focused = false,
                KeyCode::Char(c) => self.search_state.enter_char(c),
                KeyCode::Backspace => self.search_state.delete_char(),
                KeyCode::Left => self.search_state.move_cursor_left(),
                KeyCode::Right => self.search_state.move_cursor_right(),
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
                KeyCode::Char('h') | KeyCode::Left => self.wallpaper_state.list_state.select(None),
                KeyCode::Char('j') | KeyCode::Down => self.wallpaper_state.list_state.select_next(),
                KeyCode::Char('k') | KeyCode::Up => {
                    self.wallpaper_state.list_state.select_previous()
                }
                KeyCode::Char('g') | KeyCode::Home => {
                    self.wallpaper_state.list_state.select(Some(0))
                }
                KeyCode::Char('G') | KeyCode::End => self.wallpaper_state.list_state.select(Some(
                    self.wallpaper_state.dir.dir_files.len().saturating_sub(1),
                )),
                KeyCode::Char('/') => self.search_state.is_input_focused = true,
                KeyCode::Char(' ') => self.change_bg(),
                KeyCode::Char('c') => self.search_state.clean_input(),
                KeyCode::Char('d') => self.change_directory_state.is_input_focused = true,
                KeyCode::Char('o') => self.open_file(),
                _ => {}
            }
        }
    }

    fn open_file(&mut self) {
        if let Some(filtered_index) = self.wallpaper_state.list_state.selected() {
            if let Some(real_index) = self.wallpaper_state.filtered_indices.get(filtered_index) {
                let wallpaper = &self.wallpaper_state.dir.dir_files[*real_index];
                let wallpaper_path = wallpaper.file_path.to_str();
                if let Err(e) = Command::new("sxiv").args(wallpaper_path).spawn() {
                    self.error_state.message = Some(e.to_string());
                }
            }
        }
    }

    fn change_bg(&mut self) {
        if let Some(filtered_index) = self.wallpaper_state.list_state.selected() {
            if let Some(real_index) = self.wallpaper_state.filtered_indices.get(filtered_index) {
                let w = self.wallpaper_state.dir.dir_files[*real_index].clone();
                w.change_bg(self.wallpaper_state.wallpaper_mode.clone());
                self.wallpaper_state.selected_wallpaper_idx = *real_index;
                self.cfg
                    .set_config(self.change_directory_state.input.clone(), *real_index);
            }
        }
    }
}

impl App {
    fn render_search_input(&mut self, area: Rect, frame: &mut Frame) {
        let block = Block::new()
            .title("Search")
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .bg(BACKGROUND_COLOR);

        let input = Paragraph::new(self.search_state.input.clone())
            .style(Style::default().fg(FOREGROUND_COLOR).bg(BACKGROUND_COLOR))
            .block(block);

        input.render(area, frame.buffer_mut());

        if self.search_state.is_input_focused {
            frame.set_cursor_position(Position::new(
                area.x + self.search_state.character_index as u16 + 1,
                area.y + 1,
            ));
        }
    }

    fn render_wallpaper_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Wallpapers").centered())
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .bg(BACKGROUND_COLOR);

        let mut filtered_indices: Vec<usize> = Vec::new();

        let predicate: Vec<_> = self
            .wallpaper_state
            .dir
            .dir_files
            .iter()
            .enumerate()
            .filter_map(|(original_idx, wallpaper)| {
                if self.search_state.input.is_empty()
                    || wallpaper.file_name.contains(&self.search_state.input)
                {
                    filtered_indices.push(original_idx);
                    Some(wallpaper)
                } else {
                    None
                }
            })
            .collect();

        self.wallpaper_state.filtered_indices = filtered_indices;

        let items: Vec<ListItem> = predicate
            .iter()
            .enumerate()
            .map(|(i, wallpaper)| {
                let fg_color = if Some(i) == self.wallpaper_state.list_state.selected() {
                    PRIMARY_COLOR
                } else {
                    FOREGROUND_COLOR
                };

                let paragraph = format!("{}: {}", i, wallpaper.file_name);

                ListItem::new(Line::raw(paragraph))
                    .bg(BACKGROUND_COLOR)
                    .fg(fg_color)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.wallpaper_state.list_state);
    }

    fn render_help_menu(&self, area: Rect, buf: &mut Buffer) {
        let help_text = if self.search_state.is_input_focused {
            vec!["ESC: Get out of search"]
        } else {
            vec![
                "ESC/Q: Quit",
                "SPACE: Change wallpaper",
                "/: search",
                "c: clean search",
                "o: open file",
                "d: change directory",
                "m: change wallpaper mode",
            ]
        };

        let block = Block::new().bg(BACKGROUND_COLOR);

        let paragraph = Paragraph::new(help_text.join(" | "))
            .style(Style::default().fg(SUBTEXT_COLOR).bg(BACKGROUND_COLOR))
            .block(block);

        paragraph.render(area, buf);
    }

    fn render_change_directory_popup(&self, frame: &mut Frame) {
        let popup_area = Rect {
            x: frame.area().width / 4,
            y: frame.area().height / 3,
            width: frame.area().width / 2,
            height: 3,
        };

        let block = Block::new()
            .title(" Set Wallpaper Directory ")
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .bg(BACKGROUND_COLOR);

        let text = format!("Path: {}", self.change_directory_state.input);
        let paragraph = Paragraph::new(text)
            .block(block)
            .style(Style::default().fg(FOREGROUND_COLOR));

        paragraph.render(popup_area, frame.buffer_mut());
        frame.set_cursor_position(Position::new(
            popup_area.x + self.change_directory_state.character_index as u16 + 7,
            popup_area.y + 1,
        ));
    }

    fn render_error_popup(&self, frame: &mut Frame) {
        if let Some(ref error_message) = self.error_state.message {
            let area = self.centered_rect(60, 20, frame.area());

            let block = Block::new()
                .title("Error")
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .style(Style::default().fg(RED_COLOR).bg(CARD_BACKGROUND));

            let paragraph = Paragraph::new(vec![
                Line::from(error_message.clone()).fg(RED_COLOR),
                Line::from(" "),
                Line::from("Press ESC to close").fg(SUBTEXT_COLOR).italic(),
            ])
            .alignment(ratatui::layout::Alignment::Center)
            .block(block);

            frame.render_widget(paragraph, area);
        }
    }

    fn centered_rect(&self, percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Vec<String> = env::args().collect();

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

    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}
