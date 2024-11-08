use app::App;
use crossterm::event;
use ratatui::{layout::{self, Layout}, widgets::{Block, Borders, Paragraph}};

mod converter;
mod app;

fn main() {
    let mut app = App::new();
    
    let mut terminal = ratatui::init();

    
    while !app.exit {
        let _ = terminal.draw(|frame| {
            let chunk = Layout::default()
                .direction(layout::Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        layout::Constraint::Percentage(10),
                        layout::Constraint::Percentage(80),
                        layout::Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(frame.area());
            
            let top = chunk[0];
            let middle = chunk[1];
            let bottom = chunk[2];
            
            frame.render_widget(
                Paragraph::new(app.input.clone())
                    .block(Block::default().borders(Borders::ALL).title("Input")),
                top,
            );
            
            frame.render_widget(
                Paragraph::new("Middle")
                    .block(Block::default().borders(Borders::ALL).title("Middle")),
                middle,
            );
            
            frame.render_widget(
                Paragraph::new("Bottom")
                    .block(Block::default().borders(Borders::ALL).title("Bottom")),
                bottom,
            );
        });
        
        if let Ok(event) = event::read() {
            if let event::Event::Key(key_event) = event {
                
                let code = key_event.code;
                
                match code {
                    event::KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    event::KeyCode::Enter => {
                        app.input.clear();
                    }
                    event::KeyCode::Backspace => {
                        app.input.pop();
                    }
                    event::KeyCode::Esc => {
                        app.exit = true;
                    }
                    _ => {}
                }
            }
        }
    }
}