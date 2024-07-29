use std::io;

use app::App;
use crossterm::event::{self, Event};
use ratatui::Frame;

mod app;
mod tui;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app = App::new();
    let res = run_app(&mut terminal, &app);

    match res {
        Ok(val) => {
            if val {
                app.print_json()?;
            }
        }
        Err(err) => eprint!("{err:?}"),
    }
    tui::restore()?;

    Ok(())
}

fn run_app(terminal: &mut tui::Tui, app: &App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        handle_events(event::read()?);
    }
}

fn handle_events(event: Event) {
    match event {
        Event::Key(key_event) => println!("{:?}", key_event.code),
        _ => {}
    }
}

fn ui(f: &mut Frame, app: &App) {}
