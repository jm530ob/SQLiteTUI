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
    text: String,
    is_visible: bool,
}

impl ModifyTableComponent {
    fn new() -> Self {
        return Self {
            section: Section::Row,
            text: String::new(),
            is_visible: false,
        };
    }
    fn toggle_section(&self) -> Section {
        if let Section::Row = self.section {
            return Section::Column;
        } else {
            return Section::Row;
        };
    }
}

impl super::Component for ModifyTableComponent {
    fn draw(&self, frame: &mut Frame, app: &App) -> Result<(), Box<dyn std::error::Error>> {
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
            // } else if let Some(Section::Column) = self.section {
            //     frame.render_widget(Clear, frame.size());
            // }
        }
        if self.is_visible {
            // match self.section {
            //     Section::Row => {}
            // }
        }
        Ok(())
    }

    fn event(&mut self, key: KeyEvent) {
        // if key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            KeyCode::Char('n') => {
                self.is_visible = true;
            }
            KeyCode::Char(ch) => {
                if matches!(self.section, Section::Column) {
                    self.text.push(ch);
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
