use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::app::App;

enum Mode {
    Insert,
    Normal,
}

pub struct ModifyTableComponent {
    pub input: String,
    mode: Mode,
    is_visible: bool,
}

impl super::Component for ModifyTableComponent {
    fn draw(&self, frame: &mut Frame, app: &App) -> Result<(), Box<dyn std::error::Error>> {
        let modify_style = Style::new().bg(Color::DarkGray);
        let w = frame.size().width;
        let h = frame.size().height;
        let centered = Rect::new(w.saturating_sub(100) / 2, h.saturating_sub(30) / 2, 100, 30);
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(centered);

        let option_row = Paragraph::new("Row")
            .block(Block::bordered())
            .style(modify_style);
        let option_column = Paragraph::new("Column")
            .block(Block::bordered())
            .style(modify_style);
        frame.render_widget(option_row, chunks[0]);
        frame.render_widget(option_column, chunks[1]);
        Ok(())
    }

    fn event(&mut self, key: KeyEvent) {
        // if key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            KeyCode::Esc if matches!(self.mode, Mode::Insert) => {
                self.mode = Mode::Normal;
            }
            KeyCode::Char('i') if matches!(self.mode, Mode::Normal) => {
                self.mode = Mode::Insert;
            }
            KeyCode::Char(ch) => {
                self.input.push(ch);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Enter => {
                self.input.clear();
            }
            _ => {}
        }
    }

    fn is_focused(&self) -> bool {
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
