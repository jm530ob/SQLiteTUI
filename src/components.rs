use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

use crate::{
    app::{App, Area},
    database::Database,
    models,
};

pub mod select_table;
pub mod tree;
pub mod view_table;

pub enum KeyState {
    Consumed,
    NotConsumed,
}

pub trait Component {
    fn draw(&self, frame: &mut Frame, area: &mut Rect, app: &App);
    fn update(&mut self, app: &Database);
    fn handle_event(
        &mut self,
        key: KeyEvent,
        active: &mut Area,
        db: &mut Option<Database>,
    ) -> KeyState;
    fn setup(&mut self, args: &models::args::Args) -> Result<(), Box<dyn std::error::Error>>;
    fn hide(&mut self);
    fn show(&mut self);
}
