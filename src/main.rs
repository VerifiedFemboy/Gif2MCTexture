mod converter;
mod terminal;
mod app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut app = app::App::new();
    let mut terminal = ratatui::init();
    
    terminal::run(&mut terminal, &mut app);
    
    let _ = ratatui::restore();
    Ok(())
}