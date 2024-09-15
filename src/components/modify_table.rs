use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

use crate::app::App;

enum Section {
    Row,
    Column,
}

pub struct ModifyTableComponent {
    section: Section,
    input: Vec<char>,
    cursor_index: u16,
    is_visible: bool,
}

impl ModifyTableComponent {
    fn new() -> Self {
        return Self {
            section: Section::Row,
            input: vec![],
            cursor_index: 0,
            is_visible: false,
        };
    }
}

impl super::Component for ModifyTableComponent {
    fn draw(&self, frame: &mut Frame, _area: Rect, app: &App) {
        if self.is_visible {
            if self.is_visible {
                let modify_style = Style::new().bg(Color::DarkGray);
                let w = frame.size().width;
                let h = frame.size().height;
                let centered =
                    Rect::new(w.saturating_sub(100) / 2, h.saturating_sub(30) / 2, 100, 30);
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
            }
        }
        if self.is_visible {
            // match self.section {
            //     Section::Row => {}
            // }
        }
    }

    fn event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('n') => {
                self.is_visible = true;
            }
            KeyCode::Char(ch) => {
                if matches!(self.section, Section::Column) {
                    self.input.insert(self.cursor_index as usize, ch);
                    self.cursor_index += 1;
                }
            }

            KeyCode::Delete | KeyCode::Backspace => {
                if matches!(self.section, Section::Column) {
                    if self.cursor_index > 0 && !self.input.is_empty() {
                        self.input.remove(self.cursor_index as usize);
                    }
                }
            }

            KeyCode::Esc => {}
            KeyCode::Enter => {}
            _ => {}
        }
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
