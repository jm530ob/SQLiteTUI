use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph, Widget, WidgetRef};

pub struct Popupini<'a, T: Into<Text<'a>>> {
    pub title: &'a str,
    pub content: T,
    pub width: u16,
    pub height: u16,
}

// implement this block for type Popupini that has  generic lietime and assigne it to 'a (could be 'whatever)
impl<'a, T> Popupini<'a, T>
where
    T: Into<Text<'a>>,
{
    pub fn new(title: &'a str, content: T, width: u16, height: u16) -> Self {
        Self {
            title,
            content,
            width,
            height,
        }
    }
}

// implement this block enhanced by Widget trait for a Popupini type that has some generic lifetime
// and I also don't care about that generic lifetime in this block
impl<'a, T> Widget for Popupini<'a, T>
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
// impl<'a, T> From<T> for Popupini<'a, T>
// where
//     T: Into<Text<'a>>,
// {
//     fn from() -> Self {}
// }
