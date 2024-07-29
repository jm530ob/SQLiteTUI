use std::io::{self, stdout};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    QueueableCommand,
};
use ratatui::{prelude::*, Terminal};

pub type Tui = Terminal<CrosstermBackend<io::Stdout>>;

pub fn init() -> io::Result<Tui> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;
    enable_raw_mode()?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    Ok(terminal)
}

pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen, EnableMouseCapture)?;
    disable_raw_mode()?;
    Ok(())
}
