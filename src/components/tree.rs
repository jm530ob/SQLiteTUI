use regex::{Captures, Regex};

use std::{
    collections::HashMap,
    i16,
    io::Error,
    path::{Path, PathBuf},
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MediaKeyCode, ModifierKeyCode};
use ratatui::{
    layout::Margin,
    style::{Color, Modifier, Style, Styled, Stylize},
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

use crate::{
    app::{App, Area},
    models, tui,
    utils::scroll_state::ScrollState,
};

use super::{Component, KeyState};

pub struct TreeComponent {
    // Database: tables
    pub visible: bool,
    pub count: u16,
    pub paths_total: u16,
    pub abs_paths: Vec<PathBuf>,
    pub databases: HashMap<String, Vec<String>>,
    pub scroll_state: ScrollState,
}
impl TreeComponent {
    pub fn new() -> Self {
        Self {
            visible: true,
            count: 0,
            paths_total: 0,
            abs_paths: vec![],
            databases: HashMap::new(),
            scroll_state: ScrollState::new(),
        }
    }

    fn to_relative_path<'a>(&self, path: &'a str) -> &'a str {
        let re = Regex::new(r".([^/]+)$").unwrap();
        let caps = re.captures(path).unwrap();
        caps.get(0).unwrap().as_str()
    }
}
impl super::Component for TreeComponent {
    // observer

    fn setup(&mut self, args: &models::args::Args) -> Result<(), Box<dyn std::error::Error>> {
        for path in &args.paths {
            let path = Path::new(path);

            self.abs_paths.push(path.canonicalize()?);
            self.paths_total += 1;
        }
        Ok(())
        /* else {
                return Err(Box::new(Error::new(
                    std::io::ErrorKind::NotFound,
                    "Path has to be a database!",
                )));
            }
        */
        // retrieve .db files
    }

    fn draw(
        &self,
        frame: &mut ratatui::prelude::Frame,
        area: &mut ratatui::prelude::Rect,
        app: &crate::app::App,
    ) {
        if !self.visible {
            return;
        }

        let mut count: i16 = -1;

        let items = &self
            .abs_paths
            .iter()
            .map(|path| {
                count += 1;
                if count == self.count as i16 {
                    Line::from(self.to_relative_path(path.to_str().unwrap()))
                        .style(Style::new().bg(Color::Rgb(55, 53, 63)))
                } else {
                    Line::from(self.to_relative_path(path.to_str().unwrap()))
                }
            })
            .collect::<Vec<Line>>();

        let paragraph = Paragraph::new(items.clone())
            .scroll((
                self.scroll_state.vertical_scroll as u16,
                self.scroll_state.horizontal_scroll as u16,
            ))
            .block(Block::new().borders(Borders::RIGHT)); // to show a background for the scrollbar
        frame.render_widget(paragraph, *area);
    }

    fn handle_event(&mut self, key_event: KeyEvent, active: &Area) -> super::KeyState {
        if !self.visible {
            return KeyState::NotConsumed;
        }

        if !matches!(active, Area::TreeComponent) {
            return KeyState::NotConsumed;
        }
        if matches!(self.scroll_state.scroll(key_event), KeyState::Consumed) {
            return KeyState::Consumed;
        };

        match key_event.code {
            KeyCode::Char('j') => {
                if self.count + 1 < self.paths_total {
                    self.count += 1;
                } else {
                    self.count = 0;
                }
            }
            KeyCode::Char('k') => {
                if self.count != 0 {
                    self.count = self.count.saturating_sub(1);
                } else {
                    self.count = self.paths_total - 1;
                }
            }
            _ => {}
        }

        KeyState::NotConsumed
    }

    fn hide(&mut self) {}

    fn show(&mut self) {}
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
