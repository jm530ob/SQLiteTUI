use ratatui::prelude::*;
use ratatui::widgets::{Block, Clear, Paragraph, Widget};

#[derive(Debug)]
pub struct Popup<'a> {
    pub content: Text<'a>,
    pub block: Block<'a>,
    pub style: Style,
    pub width: u16,
    pub height: u16,
    // pub _marker: PhantomData<&'a ()>,
}

impl<'a> Popup<'a> {
    pub fn new() -> Self {
        Self {
            content: Text::default(),
            style: Style::default(),
            block: Block::bordered(),
            width: 0,
            height: 0,
        }
    }

    pub fn content(mut self, content: Text<'a>) -> Self {
        self.content = content;
        self
    }

    pub fn w(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    pub fn h(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = block;
        self
    }
}

impl<'a> Widget for Popup<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rect = Rect::new(
            (area.width - self.width) / 2,
            (area.height - self.height) / 2,
            self.width,
            self.height,
        );
        Clear.render(rect, buf);

        let block = self.block.style(self.style);
        let paragraph = Paragraph::new(Text::from(self.content)).block(block);

        paragraph.render(rect, buf);
    }
}
