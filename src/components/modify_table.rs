use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

type KeyState = super::KeyState;

use crate::{
    app::{App, Area},
    models,
};

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
    pub fn new() -> Self {
        return Self {
            section: Section::Row,
            input: vec![],
            cursor_index: 0,
            is_visible: false,
        };
    }
}

impl super::Component for ModifyTableComponent {
    fn draw(&self, frame: &mut Frame, _area: &mut Rect, app: &App) {
        if !self.is_visible {
            return;
        }

        let normal_style = Style::new().bg(Color::Rgb(25, 23, 36));
        let selected_style = Style::new().bg(Color::Rgb(30, 28, 40));
        let w = frame.size().width;
        let h = frame.size().height;
        let centered = Rect::new(w.saturating_sub(100) / 2, h.saturating_sub(30) / 2, 100, 30);
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(30), Constraint::Min(1)])
            .split(centered);

        let create_option = |title: &'static str, is_selected: bool| -> Paragraph<'static> {
            let p = Paragraph::new(title)
                .block(Block::bordered())
                .style(if is_selected {
                    selected_style
                } else {
                    normal_style
                });
            p
        };

        let row = create_option("Row", matches!(self.section, Section::Row));
        let column = create_option("Column", matches!(self.section, Section::Column));

        frame.render_widget(row, chunks[0]);
        frame.render_widget(column, chunks[1]);
    }

    fn handle_event(&mut self, key: KeyEvent, active: &Area) -> KeyState {
        if !self.is_visible {
            return KeyState::NotConsumed;
        }
        if !matches!(active, Area::ModifyTableComponent) {
            return KeyState::NotConsumed;
        }

        match key.code {
            KeyCode::Char(ch) => {
                if matches!(self.section, Section::Column) {
                    self.input.insert(self.cursor_index as usize, ch);
                    self.cursor_index += 1;
                }
                return KeyState::Consumed;
            }

            KeyCode::Delete | KeyCode::Backspace => {
                if matches!(self.section, Section::Column) {
                    if self.cursor_index > 0 && !self.input.is_empty() {
                        self.input.remove(self.cursor_index as usize);
                    }
                }
                return KeyState::Consumed;
            }
            // todo: posun kurzora a sekcii
            KeyCode::Esc => {
                self.hide();
                return KeyState::Consumed;
            }

            KeyCode::Enter => {
                return KeyState::Consumed;
            }

            _ => return KeyState::NotConsumed,
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

    fn setup(&mut self, _args: &models::args::Args) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
