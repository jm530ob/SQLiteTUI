use crossterm::event::KeyEvent;
use ratatui::Frame;
use std::error::Error;

use crate::app::App;

mod modify_table;

pub trait Component {
    fn draw(&self, frame: &mut Frame, app: &App) -> Result<(), Box<dyn Error>>;
    fn event(&mut self, key: KeyEvent);
    fn hide(&mut self);
    fn show(&mut self);
}
