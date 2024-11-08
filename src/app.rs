use crate::converter::Converter;

#[derive(Debug, Clone)]
pub struct App {
    pub exit: bool,
    pub input: String,
    pub app_state: AppState,
    pub converter: Converter,
}

impl App {
    pub fn new() -> Self {
        App {
            exit: false,
            input: String::new(),
            app_state: AppState::INPUTNAME,
            converter: Converter::new(),
        }
    }
    pub fn is_state(&self, state: AppState) -> bool {
        matches!(&self.app_state, state)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AppState {
    INPUTNAME,
    CONVERTING,
    DONE,
}
