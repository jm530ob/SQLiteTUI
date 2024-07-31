use std::io;

use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph, Widget},
    Frame,
};

use crate::app::App;

pub struct Ui {
    pub frame: &Frame,
}

impl Ui {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for Ui {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        Paragraph::new("Par1").block(Block::bordered()).al
    }
}
