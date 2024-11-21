use std::error::Error;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;
use crate::app::{App, Area};
use crate::components::{Component, KeyState};
use crate::models::args::Args;

pub struct SelectTableComponent {
    is_visible: bool,
}

impl Component for SelectTableComponent {
    fn draw(&self, frame: &mut Frame, _area: &mut Rect, app: &App) {
        if !self.is_visible {
            return;
        }

        let width = 100;
        let height = width;
        let area = Rect::new(
            frame.size().x.saturating_sub(width),
            frame.size().y.saturating_sub(height),
            width,
            height);
    }

    fn handle_event(&mut self, key: KeyEvent, active: &Area) -> KeyState {
        if !self.is_visible {
            return KeyState::NotConsumed;
        }
        return KeyState::NotConsumed;
    }

    fn setup(&mut self, args: &Args) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn hide(&mut self) {
        if self.is_visible {
            self.is_visible = false;
        }
    }

    fn show(&mut self) {
        if !self.is_visible {
            self.is_visible = true;
        }
    }

}