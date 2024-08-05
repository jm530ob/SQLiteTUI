use crossterm::style::style;
use ratatui::{
    prelude::*,
    widgets::{block, Block, Borders, Paragraph},
};
use symbols::border;
use tui_popup::Popup;

use crate::app::{App, ViewState};

pub fn draw_ui(app: &App, frame: &mut Frame) {
    match app.current_view {
        Some(ViewState::Main) => {
            let bg = Block::default().style(Style::default().bg(Color::Rgb(16, 31, 65)));

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
                Line::from(" Welcome to Sqlite Tui App ").black().on_white(),
                Line::from("This project is meant to be a lightweight and fast alternative"),
                Line::from(
                    "to other DB manager tools, usually implemented with graphical user interfaces.",
                ),
                Line::from("It aims to provide a simple yet flexible user experience"),
                Line::from("created by: Jakub Martenek").underlined(),
            ]))
            .centered();

            let quit_block: Block = Block::default().title("Q").borders(Borders::ALL);

            let quit = Paragraph::new("'q' to quit")
                .centered()
                .block(quit_block)
                .style(Style::default().bg(Color::Rgb(16, 31, 65)));

            let open_block = Block::default().title("Space").borders(Borders::ALL);

            let open = Paragraph::new("'Space' to open dialog")
                .centered()
                .block(open_block);

            frame.render_widget(bg, frame.size());
            frame.render_widget(banner, layout[1]);
            frame.render_widget(quit, inner_layout[0]);
            frame.render_widget(open, inner_layout[1]);
        }
        _ => {}
    }
}

fn draw_goto(frame: &mut Frame) {
    let goto_popup = Popup::new(Text::from(vec![
        Line::from("c - create new database"),
        Line::from("r - retrieve database data"),
        Line::from("u - update database"),
        Line::from("d - delete database"),
    ]))
    .title("Space")
    .border_set(border::ROUNDED);

    frame.render_widget(&goto_popup, frame.size());
}
