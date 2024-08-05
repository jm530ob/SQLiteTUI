use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};
use symbols::border;
use tui_popup::Popup;

use crate::app::App;

pub fn draw_ui(app: &App, frame: &mut Frame) {
    let p = Paragraph::new("sjdishdshd").block(Block::bordered());

    draw_goto(frame);
    frame.render_widget(p, frame.size());
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
