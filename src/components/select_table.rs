use crate::app::{App, Area};
use crate::components::{Component, KeyState};
use crate::database::Database;
use crate::models::args::Args;
use crate::utils::scroll_state::ScrollState;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Paragraph};
use ratatui::{symbols, Frame};
use std::error::Error;

pub struct SelectTableComponent {
    is_visible: bool,
    is_selected: bool,
    scroll_state: ScrollState,
    count: u16,
    tables_total: u16,
}

impl SelectTableComponent {
    pub fn new() -> Self {
        return Self {
            is_visible: false,
            is_selected: false,
            scroll_state: ScrollState::new(),
            count: 0,
            tables_total: 0,
        };
    }
}

impl SelectTableComponent {}

impl Component for SelectTableComponent {
    fn update(&mut self, db: &Database) {
        if !self.is_visible {
            return;
        }

        self.tables_total = (db.list_tables(&db.conn).unwrap().len()) as u16;
    }

    fn draw(&self, frame: &mut Frame, _area: &mut Rect, app: &App) {
        if !self.is_visible {
            return;
        }
        if !matches!(app.active, Area::SelectTableComponent) {
            return;
        }

        let width = 60;
        let height = 30;
        let areal = Rect::new(
            (frame.area().width - width) / 2,
            (frame.area().height - height) / 2,
            width,
            height,
        );

        let db = app.db.as_ref().unwrap();
        let tables = db.list_tables(&db.conn);

        let mut count: i16 = -1;

        let lines = tables
            .unwrap()
            .iter()
            .map(|table| {
                count += 1;
                if count == self.count as i16 {
                    Line::from(table.to_owned()).style(
                        Style::new()
                            .bg(Color::Rgb(75, 74, 84))
                            //.fg(Color::Rgb(145, 145, 145))
                            .bold(),
                    )
                } else {
                    Line::from(table.to_owned()) //.style(Style::new().fg(Color::Rgb(145, 145, 145)))
                }
            })
            .collect::<Vec<Line>>();

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(90), Constraint::Min(2)])
            .split(areal);

        let inner_bottom_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[1]);

        let paragraph = Paragraph::new(lines).block(
            Block::bordered()
                .style(Style::new().bg(Color::Rgb(42, 39, 42)))
                //.border_set(symbols::border::PLAIN)
                .border_style(Style::new().fg(Color::Rgb(68, 68, 68)))
                .title(Line::from("Tables").bold().style(Color::Rgb(146, 150, 240))),
        );
        let button_block = Block::bordered()
            .border_set(symbols::border::EMPTY)
            .style(Style::new().fg(Color::Rgb(186, 187, 192)));

        let button_1 = Paragraph::new(Line::from(vec![
            Span::styled("<Esc>", Style::default().fg(Color::Rgb(255, 255, 0))),
            Span::raw(" to close tab"),
        ]))
        .block(button_block.clone())
        .left_aligned();

        let button_2 = Paragraph::new(Line::from(vec![
            Span::styled("<Enter>", Style::default().fg(Color::Rgb(255, 255, 0))),
            Span::raw(" to view table"),
        ]))
        .block(button_block.clone())
        .right_aligned();

        frame.render_widget(paragraph, layout[0]);
        frame.render_widget(button_1, inner_bottom_layout[0]);
        frame.render_widget(button_2, inner_bottom_layout[1]);
    }

    fn handle_event(
        &mut self,
        key: KeyEvent,
        active: &mut Area,
        db: &mut Option<Database>,
    ) -> KeyState {
        if !self.is_visible {
            return KeyState::NotConsumed;
        }
        if !matches!(active, Area::SelectTableComponent) {
            return KeyState::NotConsumed;
        }

        match key.code {
            KeyCode::Char('j') => {
                if self.tables_total == 0 {
                    return KeyState::Consumed;
                }
                if self.count + 1 < self.tables_total {
                    self.count += 1;
                } else {
                    self.count = 0;
                }
                return KeyState::Consumed;
            }
            KeyCode::Char('k') => {
                if self.tables_total == 0 {
                    return KeyState::Consumed;
                }

                if self.count != 0 {
                    self.count = self.count.saturating_sub(1);
                } else {
                    self.count = self.tables_total - 1;
                }
                return KeyState::Consumed;
            }
            KeyCode::Esc => {
                self.hide();
                *db = None;
                *active = Area::TreeComponent;
                return KeyState::Consumed;
            }

            KeyCode::Enter => {
                self.hide();
                *active = Area::ViewTableComponent;
                let db_ref = db.as_mut().unwrap();
                let conn = &db_ref.conn;

                let table_list = db_ref.list_tables(conn).unwrap();
                let table = table_list.get(self.count as usize);
                db_ref.table = Some(table.unwrap().to_owned());
                //  let query = db_ref.get_query(conn, table.unwrap()).unwrap();
                return KeyState::Consumed;
            }

            _ => {}
        }
        return KeyState::NotConsumed;
    }

    fn setup(&mut self, _args: &Args) -> Result<(), Box<dyn Error>> {
        Ok(())
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
