use std::{
    collections::HashMap,
    io::Error,
    path::{Path, PathBuf},
};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Margin,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

use crate::{models, tui, utils::scroll_state::ScrollState};

use super::KeyState;

pub struct TreeComponent {
    // Database: tables
    pub abs_paths: Vec<PathBuf>,
    pub databases: HashMap<String, Vec<String>>,
    pub scroll_state: ScrollState,
}
impl TreeComponent {
    pub fn new() -> Self {
        Self {
            abs_paths: vec![],
            databases: HashMap::new(),
            scroll_state: ScrollState::new(),
        }
    }
}
impl super::Component for TreeComponent {
    // observer

    fn setup(&mut self, args: &models::args::Args) -> Result<(), Box<dyn std::error::Error>> {
        for path in &args.paths {
            let path = Path::new(path);
            if path.ends_with(".db") {
                self.abs_paths.push(path.canonicalize()?);
                // println!("{:?}", path);
            } else {
                return Err(Box::new(Error::new(
                    std::io::ErrorKind::NotFound,
                    "Path has to be a database!",
                )));
            }
        }
        Ok(())
        // retrieve .db files
    }

    fn draw(
        &self,
        frame: &mut ratatui::prelude::Frame,
        area: &mut ratatui::prelude::Rect,
        app: &crate::app::App,
    ) {
        let items = vec![
            Line::from("Item 1"),
            Line::from("Item 2"),
            Line::from("Item 3"),
        ];

        let paragraph = Paragraph::new(items.clone())
            .scroll((
                self.scroll_state.vertical_scroll as u16,
                self.scroll_state.horizontal_scroll as u16,
            ))
            .block(Block::new().borders(Borders::RIGHT)); // to show a background for the scrollbar
        frame.render_widget(paragraph, *area);
    }

    fn handle_event(&mut self, key_event: KeyEvent) -> super::KeyState {
        if matches!(self.scroll_state.scroll(key_event), KeyState::Consumed) {
            return KeyState::Consumed;
        };

        //if matches!(key_event.code, KeyCode::Esc) {
            // tui::clear().expect("msg");
        //}
        KeyState::NotConsumed
    }

    fn hide(&mut self) {
        todo!()
    }

    fn show(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::super::Component;
    use std::{collections::HashMap, path::Path};

    #[test]
    fn path_exists() {
        // let mut tree = super::TreeComponent {
        //     databases: HashMap::new(),
        // };

        let path = Path::new("test.db");
        if path.ends_with(".db") {
            //assert_eq!(path.canonicalize()?)
        }
        assert!(path.canonicalize().unwrap().try_exists().unwrap());
        // println!("{:?}", path.canonicalize().unwrap()); // -- --nocapture
    }
}
