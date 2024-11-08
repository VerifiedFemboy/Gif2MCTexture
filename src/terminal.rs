
use std::fmt::format;

use crossterm::event::{read, Event};
use ratatui::{layout::Layout, widgets::{Block, Borders, Paragraph}, DefaultTerminal};

use crate::app::App;

pub fn run(terminal: &mut DefaultTerminal, app: &mut App) {
    while !app.exit {
        draw(terminal, app);
        handle_input(app);
    }
}

pub fn draw(terminal: &mut DefaultTerminal, app: &mut App) {
    
    let _ = terminal.draw(|frame| {
        let area = frame.area();
        
        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    ratatui::layout::Constraint::Length(3),
                    ratatui::layout::Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(area);
        
        let paragraph = Paragraph::new(format!("{}", app.input)).block(Block::default().title(format!("{:?}", app.app_state)).borders(Borders::ALL));
        
        frame.render_widget(paragraph, chunks[0]);

    });
}

pub fn handle_input(app: &mut App) {
    if let Event::Key(event) = read().unwrap() {
        match app.app_state {
            crate::app::AppState::INPUTNAME => {
                app.handle_input(event.code);
            },
            crate::app::AppState::DONE => {
                // do nothing
            },
            _ => {}
        }
    }
}