use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::app::{App, Screen};

pub struct Ui;

impl Ui {
    pub fn render(frame: &mut Frame, app: &App) {
        let mut layout;
        match app.screen {
            Some(Screen::Main) => {
                layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(frame.size());

                let p = Paragraph::new("text")
                    .alignment(Alignment::Center)
                    .block(Block::bordered());

                let p2 = Paragraph::new("text2")
                    .alignment(Alignment::Center)
                    .block(Block::bordered());

                frame.render_widget(p, layout[0]);
                // frame.render_widget(p2, layout[1]);
            }
            _ => {}
        }
    }
}
