use std::{borrow::BorrowMut, cell::RefCell, usize};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Cell, Paragraph, Row, Table, TableState},
    Frame,
};

use crate::{
    app::{App, Area},
    utils::scroll_state::ScrollState,
};

use super::{Component, KeyState};

struct TableColors {
    header: Color,
    variant_1: Color,
    variant_2: Color,
}

pub struct ViewTableComponent {
    is_visible: bool,
    table_colors: TableColors,
    scroll_state: ScrollState,
    table_state: RefCell<TableState>,
}

impl ViewTableComponent {
    pub fn new() -> Self {
        return Self {
            is_visible: false,
            table_colors: TableColors {
                header: Color::Rgb(145, 94, 53),
                variant_1: Color::Rgb(48, 26, 27),
                variant_2: Color::Rgb(101, 59, 46),
            },
            scroll_state: ScrollState::new(),
            table_state: RefCell::new(TableState::default()),
        };
    }
}

impl Component for ViewTableComponent {
    fn update(&mut self, _app: &crate::database::Database) {}

    fn draw(&self, frame: &mut Frame, area: &mut Rect, app: &App) {
        if !self.is_visible {
            return;
        }

        if !matches!(app.active, Area::ViewTableComponent) {
            return;
        }

        let db = app.db.as_ref().unwrap();
        if db.table.is_some() {
            let query = db.get_query(&db.conn, db.table.as_ref().unwrap());
            let (column_count, column) = db
                .column_names(&db.conn, db.table.as_ref().unwrap())
                .unwrap();

            let header = column
                .iter()
                .map(|col| {
                    Text::from(col.to_string()).style(Style::new().bg(self.table_colors.header))
                })
                .collect::<Vec<Text>>();

            let rows = query
                .as_ref()
                .unwrap()
                .into_iter()
                .enumerate()
                .map(|(i, row)| {
                    let color = if i % 2 == 0 {
                        self.table_colors.variant_1
                    } else {
                        self.table_colors.variant_2
                    };

                    row.into_iter()
                        .map(|item| {
                            Cell::from(Text::from(item.to_string()).style(Style::new().bg(color)))
                        })
                        .collect::<Vec<Cell>>()
                })
                .map(|cells| Row::new(cells).height(3))
                .collect::<Vec<Row>>();

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(90), Constraint::Min(1)])
                .split(*area);

            let raw_rows = query.unwrap();

            let constraints = (0..column_count)
                .map(|i| Constraint::Length(db.max_len_str(i, &column, &raw_rows) as u16))
                .collect::<Vec<Constraint>>();

            let mut table_state = self.table_state.borrow_mut();
            table_state.select(Some(self.scroll_state.vertical_scroll));
            // table_state.scroll_up_by(self.scroll_state.vertical_scroll as u16);

            let table = Table::new(rows, &constraints)
                .highlight_symbol(">>")
                .column_spacing(1)
                .header(Row::new(header).height(1));

            frame.render_stateful_widget(table, layout[0], &mut table_state);

            let footer = Paragraph::new(Line::from(vec![
                Span::styled("<Esc>", Style::default().fg(Color::Rgb(255, 255, 0))),
                Span::raw(" to exit "),
                Span::styled("<↓ ↑>", Style::default().fg(Color::Rgb(255, 255, 0))),
                Span::raw(" movement keys"),
            ]))
            .centered();

            frame.render_widget(footer, layout[1]);
        }
    }

    fn handle_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
        active: &mut crate::app::Area,
        db: &mut Option<crate::database::Database>,
    ) -> super::KeyState {
        if !self.is_visible {
            return KeyState::NotConsumed;
        }

        if !matches!(active, Area::ViewTableComponent) {
            return KeyState::NotConsumed;
        }

        if matches!(self.scroll_state.scroll(key_event), KeyState::Consumed) {
            return KeyState::Consumed;
        };

        return KeyState::NotConsumed;
    }

    fn setup(
        &mut self,
        args: &crate::models::args::Args,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn hide(&mut self) {
        self.scroll_state.vertical_scroll = 0;
        self.is_visible = false;
    }

    fn show(&mut self) {
        self.is_visible = true;
    }
}
