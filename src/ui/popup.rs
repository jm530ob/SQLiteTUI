use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph, Widget};

#[derive(Debug)]
pub struct Popup<'a, T: Into<Text<'a>>> {
    pub title: &'a str,
    pub content: T,
    pub width: u16,
    pub height: u16,
}

impl<'a, T> Popup<'a, T>
where
    T: Into<Text<'a>>,
{
    pub fn new(content: T) -> Self {
        Self {
            title: "",
            content,
            width: 0,
            height: 0,
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }
}

impl<'a, T> Widget for Popup<'a, T>
where
    T: Into<Text<'a>>,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rect = Rect::new(
            (area.width - self.width) / 2,
            (area.height - self.height) / 2,
            self.width,
            self.height,
        );
        let p = Paragraph::new(self.content).block(Block::bordered().title(self.title));
        p.render(rect, buf);
    }
}
