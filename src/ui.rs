// use std::io::Error;
use std::vec;

use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, Paragraph,
    },
};

use crate::app::{App, ViewState};

mod popup;
use popup::Popup;

const CUSTOM_BLUE: Color = Color::Rgb(96, 164, 229);
const CUSTOM_BG: Color = Color::Rgb(16, 31, 65);

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
                Span::from("<q> ".fg(CUSTOM_BLUE).bold()),
                Span::from("to quit"),
            ]))
            .block(Block::bordered().title("Q"))
            .centered();

            let open = Paragraph::new(Line::from(vec![
                Span::from("<Space> ".fg(CUSTOM_BLUE).bold()),
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
        _ => {}
    }
    if app.display_popup {
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
        .style(Style::new().bg(CUSTOM_BG))
        .w(35)
        .h((lines.len() + 2) as u16);
    frame.render_widget(input_popup, frame.size());
}

fn draw_err_popup(frame: &mut Frame, err: &std::io::Error) {
    let message = format!("{}", err);

    let error_dialog = Popup::new()
        .block(
            Block::bordered().title(Title::from("<Error>".red())).title(
                Title::from(format!("{} {}", "<Esc>".fg(CUSTOM_BLUE).bold(), "to close"))
                    .alignment(Alignment::Right)
                    .position(Position::Bottom),
            ),
        )
        .content(Text::from(message.clone()))
        .style(Style::new().bg(CUSTOM_BG))
        .w((message.chars().count() + 2) as u16)
        .h(4);

    frame.render_widget(error_dialog, frame.size());
}

fn draw_goto_popup(frame: &mut Frame) {
    let main_dialog = Popup::new()
        .block(
            Block::bordered()
                .title("<Space>".fg(CUSTOM_BLUE).bold())
                .title(
                    Title::from("<Space>")
                        .alignment(Alignment::Right)
                        .position(Position::Bottom),
                ),
        )
        .content(Text::from(vec![
            Line::from("c - create new database"),
            Line::from("r - read/select database"),
            Line::from("u - update database"),
            Line::from("d - delete current database"),
            Line::from("q - exit"),
        ]))
        .style(Style::new().bg(CUSTOM_BG))
        .w(30)
        .h(7);

    frame.render_widget(main_dialog, frame.size());
}

fn draw_exit_popup(frame: &mut Frame) {}

fn draw_background(frame: &mut Frame) {
    let bg = Block::default().style(Style::new().bg(CUSTOM_BG));
    frame.render_widget(bg, frame.size());
}
