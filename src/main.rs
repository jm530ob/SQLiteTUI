use std::io;

use app::App;

mod app;
mod tui;
mod ui;

fn main() -> io::Result<()> {
    let mut app = App::default();
    let mut terminal = tui::init()?;
    app.run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
