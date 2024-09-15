use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};
use std::error::Error;

use crate::app::App;

mod modify_table;

pub trait Component {
    fn draw(&self, frame: &mut Frame, area: Rect, app: &App);
    fn event(&mut self, key: KeyEvent);
    fn hide(&mut self);
    fn show(&mut self);
}
