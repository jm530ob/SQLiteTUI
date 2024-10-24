use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::ScrollbarState;

use crate::components::KeyState;

#[derive(Default)]
pub struct ScrollState {
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
}

impl ScrollState {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn scroll(&mut self, key_event: KeyEvent) -> KeyState {
        match key_event.code {
            KeyCode::Up => {
                self.vertical_scroll = self.vertical_scroll.saturating_add(1);
                self.vertical_scroll_state =
                    self.vertical_scroll_state.position(self.vertical_scroll);
                KeyState::Consumed
            }
            KeyCode::Down => {
                self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
                self.vertical_scroll_state =
                    self.vertical_scroll_state.position(self.vertical_scroll);
                KeyState::Consumed
            }
            KeyCode::Left => {
                self.horizontal_scroll = self.horizontal_scroll.saturating_sub(1);
                self.horizontal_scroll_state = self
                    .horizontal_scroll_state
                    .position(self.horizontal_scroll);
                KeyState::Consumed
            }
            KeyCode::Right => {
                self.horizontal_scroll = self.horizontal_scroll.saturating_add(1);
                self.horizontal_scroll_state = self
                    .horizontal_scroll_state
                    .position(self.horizontal_scroll);
                KeyState::Consumed
            }
            _ => KeyState::NotConsumed,
        }
    }
}
