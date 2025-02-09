pub struct SearchState {
    pub input: String,
    pub is_input_focused: bool,
    pub character_index: usize,
}

pub struct ChangeDirectoryState {
    pub is_input_focused: bool,
    pub input: String,
    pub character_index: usize,
}

impl TextInputState for SearchState {
    fn input(&mut self) -> &mut String {
        &mut self.input
    }

    fn character_index(&mut self) -> &mut usize {
        &mut self.character_index
    }
}

impl TextInputState for ChangeDirectoryState {
    fn input(&mut self) -> &mut String {
        &mut self.input
    }

    fn character_index(&mut self) -> &mut usize {
        &mut self.character_index
    }
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            is_input_focused: false,
            character_index: 0,
        }
    }
}

impl ChangeDirectoryState {
    pub fn new(initial_value: Option<String>) -> Self {
        let input = initial_value.unwrap_or(String::default());
        let character_index = input.len();
        Self {
            input,
            character_index,
            is_input_focused: false,
        }
    }
}

pub trait TextInputState {
    fn input(&mut self) -> &mut String;
    fn character_index(&mut self) -> &mut usize;

    fn clean_input(&mut self) {
        *self.input() = String::default();
        *self.character_index() = 0;
    }

    fn delete_char(&mut self) {
        if *self.character_index() > 0 {
            let current_index = *self.character_index();
            let from_left_to_current_index = current_index - 1;

            let input_clone = self.input().clone();

            let before_char_to_delete = input_clone.chars().take(from_left_to_current_index);
            let after_char_to_delete = input_clone.chars().skip(current_index);

            *self.input() = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn move_cursor_left(&mut self) {
        let current_index = *self.character_index();
        let new_index = self.clamp_cursor(current_index.saturating_sub(1));
        *self.character_index() = new_index;
    }

    fn move_cursor_right(&mut self) {
        let current_index = *self.character_index();
        let new_index = self.clamp_cursor(current_index.saturating_add(1));
        *self.character_index() = new_index;
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input().insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&mut self) -> usize {
        let current_index = *self.character_index();
        self.input()
            .char_indices()
            .map(|(i, _)| i)
            .nth(current_index)
            .unwrap_or(self.input().len())
    }

    fn clamp_cursor(&mut self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input().chars().count())
    }
}
