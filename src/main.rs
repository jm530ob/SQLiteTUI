use std::io;

use app::App;
use clap::Parser;

mod app;
mod components;
mod database;
mod models;
mod tui;
mod utils;

fn main() -> io::Result<()> {
    let args = models::args::Args::parse();
    let mut terminal = tui::init()?;
    let mut app = App::new();
    app.setup(args);
    app.run(&mut terminal)?;
    tui::clear()?;
    Ok(())
}
