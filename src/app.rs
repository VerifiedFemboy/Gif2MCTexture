use std::str::Chars;

use crossterm::event::KeyCode;

pub struct App {
    pub exit: bool,
    pub input: String,
    pub app_state: AppState,
}

impl App {
    
    pub fn new() -> Self {
        App { exit: false, input: String::new(), app_state: AppState::INPUT_NAME }
    }
    
    pub fn handle_input(&mut self, code: KeyCode) {
        if self.is_state(AppState::INPUT_NAME) {
            match code {
                KeyCode::Char(c) => {
                    self.input.push(c);
                },
                KeyCode::Backspace => {
                    self.input.pop();
                },
                _=> {}
            }
        }
    }
    
    
    pub fn is_state(&self, _state: AppState) -> bool {
            matches!(&self.app_state, _state)
    }
}

pub enum AppState {
    INPUT_NAME,
    CONVERTING,
    DONE,
}