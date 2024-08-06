use crossterm::style::style;
use layout::Flex;
use ratatui::{
    prelude::*,
    widgets::{block, Block, Borders, Paragraph, Wrap},
};
use symbols::border;
use tui_popup::Popup;

use crate::app::{App, ViewState};

mod popup;
use popup::Popupini;

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
                Line::from(" Welcome to Sqlite TUI Manager ").black().on_white(),
                Line::from("This project is meant to be a lightweight and fast alternative"),
                Line::from(
                    "to other Sqlite DB manager tools, usually implemented with graphical user interfaces.",
                ),
                Line::from("It aims to provide a simple yet flexible user experience"),
                Line::from("created by: Jakub Martenek").underlined(),
            ]))
            .centered();

            let quit = Paragraph::new("'q' to quit")
                .block(Block::bordered().title("Q"))
                .centered();

            let open = Paragraph::new("'Space' to open dialog")
                .centered()
                .block(Block::bordered().title("Space"));

            frame.render_widget(banner, layout[1]);
            frame.render_widget(quit, inner_layout[0]);
            frame.render_widget(open, inner_layout[1]);
        }
        Some(ViewState::Create) => {
            let popup = Popupini::new("Enter table name", &app.input, 40, 2);
            // let create_table = Popup::new("                 ")
            //     .title("Create new table")
            //     .border_set(border::ROUNDED);
            // let layout = Layout::default()
            //     .direction(Direction::Vertical)
            //     .constraints(vec![
            //         Constraint::Percentage(35),
            //         Constraint::Max(3),
            //         Constraint::Percentage(35),
            //     ])
            //     .split(frame.size());

            // let create_table = Paragraph::new("Table name")
            //     .centered()
            //     .block(Block::bordered().title("Enter table name"))
            //     .wrap(Wrap { trim: true });
            frame.render_widget(popup, frame.size());
            // todo!(CUSTOM POPUP);
        }
        _ => {}
    }
    if app.display_popup {
        draw_goto_popup(frame);
    }
}

fn draw_background(frame: &mut Frame) {
    let bg = Block::default().style(Style::default().bg(Color::Rgb(16, 31, 65)));
    frame.render_widget(bg, frame.size());
}

fn draw_goto_popup(frame: &mut Frame) {
    let goto_popup = Popup::new(Text::from(vec![
        Line::from("c - create new database"),
        Line::from("r - retrieve database data"),
        Line::from("u - update database"),
        Line::from("d - delete database"),
        Line::from("q - exit"),
    ]))
    .title("Space")
    .border_set(border::ROUNDED)
    .style(Style::default().bg(Color::Rgb(27, 43, 79)));

    frame.render_widget(&goto_popup, frame.size());
}

fn draw_exit_popup(frame: &mut Frame) {}
