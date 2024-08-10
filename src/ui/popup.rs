use std::marker::PhantomData;

use ratatui::prelude::*;
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, Paragraph, Widget};

#[derive(Debug)]
pub struct Popup<'a, T, U>
where
    T: Into<Text<'a>>,
    U: Into<Title<'a>>,
{
    pub content: T,
    pub title: U,
    pub width: u16,
    pub height: u16,
    pub _marker: PhantomData<&'a ()>,
}

impl<'a, T, U> Popup<'a, T, U>
where
    T: Into<Text<'a>>,
    U: Into<Title<'a>>,
{
    pub fn new(title: U, content: T) -> Self {
        Self {
            title,
            content,
            width: 0,
            height: 0,
            _marker: PhantomData,
        }
    }

    // pub fn title(mut self, title: U) -> Self {
    //     self.title = title;
    //     self
    // }

    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }
}

impl<'a, T, U> Widget for Popup<'a, T, U>
where
    T: Into<Text<'a>> + From<&'a str>,
    U: Into<Title<'a>>,
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
