use ratatui::prelude::{Buffer, Rect};
use ratatui::widgets::{Block, Paragraph, Widget};

pub struct Popupini<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub width: u16,
    pub height: u16,
}

impl<'a> Popupini<'a> {
    pub fn new(title: &'a str, content: &'a str, width: u16, height: u16) -> Self {
        Self {
            title,
            content,
            width,
            height,
        }
    }
}

impl<'a> Widget for Popupini<'a> {
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
