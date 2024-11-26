//! # Litelens-tui
//!
//! Simple terminal-based user interface (TUI) app to view existing SQLite database files, built with pure Rust
//!
//! ## Features
//!
//! - [x] Vim like navigation
//! - [x] TUI Interface
//! - [x] View data
//! - [ ] Modify data
//! - [ ] Better optimized
//! - [ ] Error handler

use std::io;

use app::App;
use clap::Parser;

/// Contains the core logic and main loop of the application.
mod app;
/// Widgets and components to construct the app.
mod components;
/// Includes SQL queries and helper functions.
mod database;
/// CLI argument parsers.
mod models;
/// Entry point for TUI.
mod tui;
/// General-purpose helper functions.
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
