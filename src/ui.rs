// use std::io::Error;
use std::vec;

use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};
use serde::de::value::Error;

use crate::app::{App, ViewState};

mod popup;
use popup::Popup;

pub fn draw_ui(app: &App, frame: &mut Frame) {
    draw_background(frame);
    let selected_db = Paragraph::new(format!(
        "Selected db: {}",
        app.database.clone().unwrap_or("None".to_owned()),
    ));

    frame.render_widget(selected_db, frame.size());
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
                Line::from(" Welcome to Sqlite TUI Manager ").black().on_white(),
                Line::from("This project is meant to be a lightweight and fast alternative"),
                Line::from(
                    "to other Sqlite DB manager tools, usually implemented with graphical user interfaces.",
                ),
                Line::from("It aims to provide a simple yet flexible user experience"),
                Line::from("created by: Jakub Martenek").underlined(),
            ]))
            .centered();

            let quit = Paragraph::new(Line::from(vec![
                Span::from("<q> ".light_blue().bold()),
                Span::from("to quit"),
            ]))
            .block(Block::bordered().title("Q"))
            .centered();

            let open = Paragraph::new(Line::from(vec![
                Span::from("<Space> ".light_blue().bold()),
                Span::from("to open/close dialog"),
            ]))
            .centered()
            .block(Block::bordered().title("Space"));

            frame.render_widget(banner, layout[1]);
            frame.render_widget(quit, inner_layout[0]);
            frame.render_widget(open, inner_layout[1]);
        }
        Some(ViewState::Create) => {
            let lines = &app.input.split("\n").collect::<Vec<&str>>();
            let popup = Popup::new("Enter table name", Text::from(&*app.input.trim()))
                .width(35)
                .height((lines.len() + 2) as u16);
            frame.render_widget(popup, frame.size());
        }
        _ => {}
    }
    if app.display_popup {
        draw_goto_popup(frame);
    }

    if let Some(err) = &app.error_message {
        draw_err_popup(frame, err);
    }
}

pub fn draw_err_popup(frame: &mut Frame, err: &std::io::Error) {
    let message = format!("{}", err);
    frame.render_widget(
        Popup::new("<Error>".red().bold(), &*message)
            .width((message.chars().count() + 2) as u16)
            .height(3),
        frame.size(),
    );
}

fn draw_goto_popup(frame: &mut Frame) {
    let goto_popup = Popup::new(
        "<Space>".light_blue().bold(),
        Text::from(vec![
            Line::from("c - create new database"),
            Line::from("r - retrieve database data"),
            Line::from("u - update database"),
            Line::from("d - delete database"),
            Line::from("q - exit"),
        ]),
    )
    .width(30)
    .height(7);

    frame.render_widget(goto_popup, frame.size());
}

fn draw_exit_popup(frame: &mut Frame) {}

fn draw_background(frame: &mut Frame) {
    let bg = Block::default().style(Style::default().bg(Color::Rgb(16, 31, 65)));
    frame.render_widget(bg, frame.size());
}
