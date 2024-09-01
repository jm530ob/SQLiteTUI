use ratatui::Frame;
use std::error::Error;

use crate::app::App;

mod alter_table;

pub trait Component {
    fn draw(&self, frame: &mut Frame, app: &App) -> Box<dyn Error>;
    fn is_focused(&self) -> bool;
    fn hide(&self);
    fn show(&self);
}
