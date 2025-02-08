use std::{path::Path, usize};

use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget,
        Widget,
    },
    DefaultTerminal, Frame,
};
use wallpaper::Mode;

use crate::{config::Config, wallpapercl::Dir};

const FOREGROUND_COLOR: Color = Color::Rgb(186, 194, 222);
const BACKGROUND_COLOR: Color = Color::Rgb(30, 30, 46);

const CARD_BACKGROUND: Color = Color::Rgb(17, 17, 21);

const PRIMARY_COLOR: Color = Color::Rgb(203, 166, 247);
const SUBTEXT_COLOR: Color = Color::Rgb(147, 153, 178);
const RED_COLOR: Color = Color::Rgb(243, 139, 168);

const SELECTED_STYLE: Style = Style::new()
    .bg(PRIMARY_COLOR)
    .fg(BACKGROUND_COLOR)
    .add_modifier(Modifier::BOLD);

// TODO: implement change of mode

pub struct App {
    should_exit: bool,
    wallpaper_dir: Dir,
    wallpaper_state: ListState,
    selected_wallpaper_idx: usize,
    search_query: String,
    input_focused: bool,
    character_index: usize,
    cfg: Config,
    dir_input_focused: bool,
    dir_input: String,
    global_error: Option<String>,
    filtered_indices: Vec<usize>,
    wallpaper_mode: wallpaper::Mode,
}

impl App {
    pub fn new(cfg: Config) -> Self {
        let wallpaper_dir = if !cfg.stored_wallpaper_path.is_empty() {
            Dir::new(cfg.stored_wallpaper_path.clone())
        } else {
            Dir::new("".to_string())
        };
        let path = cfg.stored_wallpaper_path.clone();
        let idx = cfg.stored_wallpaper_index.clone();

        Self {
            should_exit: false,
            wallpaper_dir,
            search_query: String::default(),
            input_focused: false,
            dir_input_focused: cfg.stored_wallpaper_path.is_empty(),
            character_index: 0,
            cfg,
            dir_input: path,
            selected_wallpaper_idx: idx,
            wallpaper_state: ListState::default().with_selected(Some(idx)),
            global_error: None,
            filtered_indices: Vec::new(),
            wallpaper_mode: Mode::Fit,
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

                if self.dir_input_focused {
                    self.render_directory_popup(frame);
                } else {
                    self.render_search_input(input_area, frame);
                    self.render_wallpaper_list(list_area, frame.buffer_mut());
                    self.render_help_menu(help_area, frame.buffer_mut());
                }
                self.render_global_error_popup(frame);
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

        if self.global_error.is_some() {
            if key.code == KeyCode::Esc {
                self.global_error = None;
            }
            return;
        }

        if self.dir_input_focused {
            match key.code {
                KeyCode::Esc => self.dir_input_focused = false,
                KeyCode::Enter => {
                    if Path::new(&self.dir_input).is_dir() {
                        self.cfg
                            .set_config(self.dir_input.clone(), self.selected_wallpaper_idx);
                        self.wallpaper_dir = Dir::new(self.dir_input.clone());
                        self.dir_input_focused = false;
                    } else {
                        self.global_error =
                            Some("Invalid directory. Please enter a valid path.".to_string());
                    }
                }
                KeyCode::Char(c) => self.dir_input.push(c),
                KeyCode::Backspace => {
                    self.dir_input.pop();
                }
                _ => {}
            }
        } else if self.input_focused {
            match key.code {
                KeyCode::Esc | KeyCode::Enter => self.input_focused = false,
                KeyCode::Char(to_insert) => self.enter_char(to_insert),
                KeyCode::Backspace => self.delete_char(),
                KeyCode::Left => self.move_cursor_left(),
                KeyCode::Right => self.move_cursor_right(),
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
                KeyCode::Char('h') | KeyCode::Left => self.wallpaper_state.select(None),
                KeyCode::Char('j') | KeyCode::Down => self.select_next(),
                KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
                KeyCode::Char('g') | KeyCode::Home => self.wallpaper_state.select(Some(0)),
                KeyCode::Char('G') | KeyCode::End => self
                    .wallpaper_state
                    .select(Some(self.wallpaper_dir.dir_files.len().saturating_sub(1))),
                KeyCode::Char('/') => self.input_focused = true,
                KeyCode::Char(' ') => self.change_bg(),
                KeyCode::Char('c') => self.clean_search(),
                KeyCode::Char('d') => self.dir_input_focused = true,
                _ => {}
            }
        }
    }

    fn change_bg(&mut self) {
        if let Some(filtered_index) = self.wallpaper_state.selected() {
            if let Some(real_index) = self.filtered_indices.get(filtered_index) {
                self.wallpaper_dir.dir_files[*real_index].change_bg(self.wallpaper_mode.clone());
                self.selected_wallpaper_idx = *real_index;
                self.cfg.set_config(self.dir_input.clone(), *real_index);
            }
        }
    }

    fn clean_search(&mut self) {
        self.input_focused = false;
        self.search_query = String::default();
        self.character_index = 0;
    }

    fn select_next(&mut self) {
        let i = self.wallpaper_state.selected().unwrap_or(0);
        let new_index = (i + 1).min(self.wallpaper_dir.dir_files.len().saturating_sub(1));
        self.wallpaper_state.select(Some(new_index));
    }

    fn select_previous(&mut self) {
        let i = self.wallpaper_state.selected().unwrap_or(0);
        let new_index = i.saturating_sub(1);
        self.wallpaper_state.select(Some(new_index));
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.search_query.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.search_query.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.search_query = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.search_query.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.search_query
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.search_query.len())
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.search_query.chars().count())
    }
}

impl App {
    fn render_global_error_popup(&self, frame: &mut Frame) {
        if let Some(ref error_message) = self.global_error {
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

    fn render_wallpaper_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Wallpapers").centered())
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .bg(BACKGROUND_COLOR);

        let mut filtered_indices: Vec<usize> = Vec::new();

        let predicate: Vec<_> = self
            .wallpaper_dir
            .dir_files
            .iter()
            .enumerate()
            .filter_map(|(original_idx, wallpaper)| {
                if self.search_query.is_empty() || wallpaper.file_name.contains(&self.search_query)
                {
                    filtered_indices.push(original_idx);
                    Some(wallpaper)
                } else {
                    None
                }
            })
            .collect();

        self.filtered_indices = filtered_indices;

        let items: Vec<ListItem> = predicate
            .iter()
            .enumerate()
            .map(|(i, wallpaper)| {
                let fg_color = if Some(i) == self.wallpaper_state.selected() {
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

        StatefulWidget::render(list, area, buf, &mut self.wallpaper_state);
    }

    fn render_search_input(&mut self, area: Rect, frame: &mut Frame) {
        let block = Block::new()
            .title("Search")
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .bg(BACKGROUND_COLOR);

        let input = Paragraph::new(self.search_query.clone())
            .style(Style::default().fg(FOREGROUND_COLOR).bg(BACKGROUND_COLOR))
            .block(block);

        input.render(area, frame.buffer_mut());

        if self.input_focused {
            frame.set_cursor_position(Position::new(
                area.x + self.character_index as u16 + 1,
                area.y + 1,
            ));
        }
    }

    fn render_help_menu(&self, area: Rect, buf: &mut Buffer) {
        let help_text = if self.input_focused {
            vec!["ESC: Get out of search"]
        } else {
            vec![
                "ESC/Q: Quit",
                "SPACE: Change wallpaper",
                "/: search",
                "c: clean search",
                "d: change directory",
                "m: change wallpaper mode",
                "Vim keys: Navigate list",
            ]
        };

        let block = Block::new().bg(BACKGROUND_COLOR);

        let paragraph = Paragraph::new(help_text.join(" | "))
            .style(Style::default().fg(SUBTEXT_COLOR).bg(BACKGROUND_COLOR))
            .block(block);

        paragraph.render(area, buf);
    }

    fn render_directory_popup(&self, frame: &mut Frame) {
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

        let text = format!("Path: {}", self.dir_input);
        let paragraph = Paragraph::new(text)
            .block(block)
            .style(Style::default().fg(FOREGROUND_COLOR));

        paragraph.render(popup_area, frame.buffer_mut());
        frame.set_cursor_position(Position::new(
            popup_area.x + self.dir_input.len() as u16 + 7,
            popup_area.y + 1,
        ));
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
