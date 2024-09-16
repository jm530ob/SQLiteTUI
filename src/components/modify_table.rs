use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

type KeyState = super::KeyState;

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
                let normal_style = Style::new().bg(Color::DarkGray);
                let selected_style = Style::new().bg(Color::Blue);
                let w = frame.size().width;
                let h = frame.size().height;
                let centered =
                    Rect::new(w.saturating_sub(100) / 2, h.saturating_sub(30) / 2, 100, 30);
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Percentage(30), Constraint::Min(1)])
                    .split(centered);

                let create_option = |title: &'static str,
                                     is_selected: bool|
                 -> Paragraph<'static> {
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
        }
    }

    fn event(&mut self, key: KeyEvent) -> KeyState {
        match key.code {
            KeyCode::Char('n') => {
                self.show();
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
            // todo: posun kurzora a sekcii
            KeyCode::Esc => {
                self.hide();
            }

            KeyCode::Enter => {}
            _ => return KeyState::NotConsumed,
        }
        KeyState::Consumed
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
