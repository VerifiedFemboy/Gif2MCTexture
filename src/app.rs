use crossterm::event::KeyCode;

pub struct App {
    pub exit: bool,
    pub input: String,
    pub app_state: AppState,
    pub logs: Vec<String>,
}

impl App {
    
    pub fn new() -> Self {
        App { exit: false, input: String::new(), app_state: AppState::INPUTNAME, logs: Vec::new(), }
    }
    
    pub fn handle_input(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Enter => {
                self.app_state = AppState::CONVERTING;
            }
            _ => {}
        }
    }
    
    pub fn add_log(&mut self, str: &str) {
        self.logs.push(str.to_string());
    }
    
    pub fn is_state(&self, state: AppState) -> bool {
            matches!(&self.app_state, state)
    }
}

#[derive(Debug, PartialEq)]
pub enum AppState {
    INPUTNAME,
    CONVERTING,
    DONE,
}