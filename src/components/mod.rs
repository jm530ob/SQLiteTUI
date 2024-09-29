use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};
use std::error::Error;

use crate::{app::App, models};

pub mod modify_table;
pub mod tree;

pub enum KeyState {
    Consumed,
    NotConsumed,
}

pub trait Component {
    fn draw(&self, frame: &mut Frame, area: Rect, app: &App);
    fn event(&mut self, key: Option<KeyEvent>) -> KeyState;
    fn setup(&mut self, args: &models::args::Args) -> Result<(), Box<dyn std::error::Error>>;
    fn hide(&mut self);
    fn show(&mut self);
}
