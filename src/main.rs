use std::{
    io::{self, stdin, stdout},
    vec,
};

use crossterm::terminal;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Rect},
    prelude::CrosstermBackend,
    style::Stylize,
    symbols::border,
    text::{Line, Span, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    Frame, Terminal,
};

mod tui;

#[derive(Debug, Default, Clone, Copy)]
struct App {
    counter: u8,
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| {
                self.redner_frame(frame);
            })?;
            self.handle_events();
        }
        Ok(())
    }

    fn redner_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&self) {}
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let header = Title::from("Hey am in center");
        let title = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));

        let block = Block::bordered()
            .title(
                title
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .title(header.alignment(Alignment::Center));

        let p = Paragraph::new("IM INSIDE YOU HOME - also at center")
            .centered()
            .block(block);

        p.render(area, buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}
