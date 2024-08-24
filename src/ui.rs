// use std::io::Error;
use std::{cmp, vec};

use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, Cell, Paragraph, Row, Table,
    },
};

use crate::app::{App, AppState, ViewState};
use crate::database::Db;

mod popup;
use popup::Popup;

const BLUE: Color = Color::Rgb(129, 161, 193);
const WIDGET_COLOR: Color = Color::Rgb(59, 66, 82);
const BG_COLOR: Color = Color::Rgb(46, 52, 64);

const VAR0: Color = Color::Rgb(46, 52, 64);
const VAR1: Color = Color::Rgb(59, 66, 82);
const VAR2: Color = Color::Rgb(76, 86, 106);

fn info_log(frame: &mut Frame, app: &App) {
    let selected_db = Paragraph::new(format!(
        "Selected db: {}",
        app.db.db_name.clone().unwrap_or("None".to_owned())
    ));

    frame.render_widget(selected_db, frame.size());
}
pub fn draw_ui(app: &App, frame: &mut Frame) {
    draw_background(frame);
    match app.current_view {
        Some(ViewState::Main) => {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Length(3),
                    Constraint::Min(5),
                    Constraint::Length(3),
                ])
                .split(frame.size());

            let inner_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(layout[2]);

            let banner = Paragraph::new(Text::from(vec![
                Line::from(" Welcome to Sqlite TUI Manager ").style(Style::default().fg(Color::Black).bg(Color::Rgb(229, 233, 240))),
                Line::from("This project is meant to be a lightweight and fast alternative"),
                Line::from(
                    "to other Sqlite DB manager tools, usually implemented with graphical user interfaces.",
                ),
                Line::from("It aims to provide a simple yet flexible user experience"),
                Line::from("created by: Jakub Martenek").underlined(),
            ]))
            .centered();

            let quit = Paragraph::new(Line::from(vec![
                Span::from("<q> ".fg(BLUE).bold()),
                Span::from("to quit"),
            ]))
            .centered()
            .block(Block::bordered().title("Q"));

            let open = Paragraph::new(Line::from(vec![
                Span::from("<Ctrl + o> ".fg(BLUE).bold()),
                Span::from("to open/close dialog"),
            ]))
            .centered()
            .block(Block::bordered().title("Space"));

            frame.render_widget(banner, layout[1]);
            frame.render_widget(quit, inner_layout[0]);
            frame.render_widget(open, inner_layout[1]);
        }
        Some(ViewState::Create) => {
            draw_input_popup(frame, app, "Enter table name");
        }

        Some(ViewState::Read) => {
            draw_input_popup(frame, app, "Enter table to read from");
        }

        Some(ViewState::Update) => {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Length(3), Constraint::Min(5)])
                .split(frame.size());

            // let centered = Rect::new(50, 0, 20, 20);

            // println!("{:?} - {:?}", layout[1].width, frame.size().width);

            let header_style = Style::default().bg(WIDGET_COLOR);
            let header = app
                .db
                .col_names
                .iter()
                .map(|col| {
                    Cell::from(
                        Text::from(col.as_str())
                            .style(header_style)
                            .alignment(Alignment::Center),
                    )
                })
                .collect::<Row>()
                .height(1); // apply to all

            let color = |row, col| {
                if row == app.db.cursor.row && col == app.db.cursor.col {
                    VAR0
                } else if row % 2 == 1 {
                    VAR1
                } else {
                    VAR2
                }
            };

            let rows = app
                .db
                .cursor
                .records
                .iter()
                .enumerate()
                .map(|(i, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(j, item)| {
                            Cell::from(
                                Text::from(item.as_str()).style(Style::default().bg(color(i, j))),
                            )
                        })
                        .collect::<Row>() // convert each item into a row
                })
                .collect::<Vec<Row>>(); // and collect them into the vector

            let get_max = |col_index: usize, col: &String| {
                app.db
                    .cursor
                    .records
                    .iter()
                    .map(|row| cmp::max(col.len(), row[col_index].len()))
                    .max()
            };

            // let mut width: usize = 0;

            let constraints = app
                .db
                .col_names
                .iter()
                .enumerate()
                .map(|(i, col)| {
                    let longest_col = (get_max(i, col).unwrap_or(col.len()) + 2) as u16;
                    Constraint::Length(longest_col)
                    // +2 for padding
                })
                .collect::<Vec<Constraint>>();

            // println!("{} {}", frame.size().width, app.db.table_width);
            // let centered = Rect::new(
            //     (frame.size().width - app.db.table_width / 2),
            //     0,
            //     app.db.table_width,
            //     20,
            // );
            let block = Block::bordered().title(app.db.table_name.clone());

            let table = Table::new(rows, constraints)
                .block(block)
                .column_spacing(1)
                .header(header);
            frame.render_widget(table, layout[1]);
        }

        _ => {}
    }

    match app.app_state {
        Some(AppState::Receiving(ViewState::Create)) => {
            draw_app_input(frame, app);
        }

        Some(AppState::Receiving(ViewState::Read)) => {}
        Some(AppState::Editing) => {}
        _ => {}
    }
    if app.display_dialog {
        draw_goto_popup(frame);
    }

    if let Some(err) = &app.error_message {
        draw_err_popup(frame, err);
    }
}

fn draw_input_popup(frame: &mut Frame, app: &App, text: &str) {
    let lines = app.input.split("\n").collect::<Vec<&str>>();
    let input_popup = Popup::new()
        .block(Block::bordered().title(Title::from(Line::from(text.bold()))))
        .content(Text::from(&*app.input.trim()))
        .style(Style::new().bg(WIDGET_COLOR))
        .w(35)
        .h((lines.len() + 2) as u16);
    frame.render_widget(input_popup, frame.size());
}

pub fn draw_app_input(frame: &mut Frame, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(frame.size());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(15),
            Constraint::Percentage(45),
            Constraint::Percentage(20),
        ])
        .split(layout[1]);

    let table_input = Paragraph::new(app.db.table_name.clone())
        .block(Block::bordered().title("Table"))
        .style(Style::new().bg(WIDGET_COLOR));
    let attributes_input = Paragraph::new(app.db.column.clone())
        .block(Block::bordered().title("Attribute/s (separate by comma)"))
        .style(Style::new().bg(WIDGET_COLOR));

    frame.render_widget(table_input, inner_layout[1]);
    frame.render_widget(attributes_input, inner_layout[2]);
}

pub fn draw_items(frame: &mut Frame, db: &Db) {
    let rows = [Row::new(vec!["Cell1", "Cell2", "Cell3"])];
    let widths = [
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(10),
    ];
    let table = Table::new(rows, widths);
    frame.render_widget(table, frame.size());
}

fn draw_err_popup(frame: &mut Frame, err: &std::io::Error) {
    let message = format!("{}", err);

    let error_dialog = Popup::new()
        .block(
            Block::bordered().title(Title::from("<Error>".red())).title(
                Title::from(format!("{} {}", "<Esc>".fg(BLUE).bold(), "to close"))
                    .alignment(Alignment::Right)
                    .position(Position::Bottom),
            ),
        )
        .content(Text::from(message.clone()))
        .style(Style::new().bg(WIDGET_COLOR))
        .w((message.chars().count() + 2) as u16)
        .h(4);

    frame.render_widget(error_dialog, frame.size());
}

fn draw_goto_popup(frame: &mut Frame) {
    let main_dialog = Popup::new()
        .block(Block::bordered().title("<Action>".fg(BLUE).bold()))
        .content(Text::from(vec![
            Line::from("c - create new database"),
            Line::from("r - read/select database"),
            Line::from("u - update database"),
            Line::from("d - delete current database"),
            Line::from("q - exit"),
        ]))
        .style(Style::new().bg(WIDGET_COLOR))
        .w(30)
        .h(7);

    frame.render_widget(main_dialog, frame.size());
}

fn draw_exit_popup(frame: &mut Frame) {}

fn draw_background(frame: &mut Frame) {
    let bg = Block::default().style(Style::new().bg(BG_COLOR));
    frame.render_widget(bg, frame.size());
}
