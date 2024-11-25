use regex::Regex;

use std::path::{Path, PathBuf};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::{app::Area, database::Database, models, utils::scroll_state::ScrollState};

use super::KeyState;

pub struct TreeComponent {
    pub visible: bool,
    pub count: u16,
    pub paths_total: u16,
    pub abs_paths: Vec<PathBuf>,
    pub scroll_state: ScrollState,
}
impl TreeComponent {
    pub fn new() -> Self {
        Self {
            visible: true,
            count: 0,
            paths_total: 0,
            abs_paths: vec![],
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
    fn setup(&mut self, args: &models::args::Args) -> Result<(), Box<dyn std::error::Error>> {
        for path in &args.paths {
            let path = Path::new(path);

            self.abs_paths.push(path.canonicalize()?);
            self.paths_total += 1;
        }
        Ok(())
    }

    fn update(&mut self, _db: &Database) {}

    fn draw(
        &self,
        frame: &mut ratatui::prelude::Frame,
        area: &mut ratatui::prelude::Rect,
        _app: &crate::app::App,
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
                    Line::from(self.to_relative_path(path.to_str().unwrap())).style(
                        Style::new()
                            .bg(Color::Rgb(42, 39, 42))
                            // .fg(Color::Rgb(186, 187, 192))
                            .bold(),
                    )
                } else {
                    Line::from(self.to_relative_path(path.to_str().unwrap()))
                    //.style(Style::new().fg(Color::Rgb(186, 187, 192)))
                }
            })
            .collect::<Vec<Line>>();

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(90), Constraint::Min(1)])
            .split(*area);

        let paragraph = Paragraph::new(items.clone())
            .scroll((
                self.scroll_state.vertical_scroll as u16,
                self.scroll_state.horizontal_scroll as u16,
            ))
            .block(
                Block::new()
                    .borders(Borders::RIGHT)
                    .border_style(Style::new().fg(Color::Rgb(68, 68, 68))),
            );

        let footer = Paragraph::new(Line::from(vec![
            Span::raw("Press "),
            Span::styled("<Ctrl-o>", Style::default().fg(Color::Rgb(255, 255, 0))),
            Span::raw(" to focus"),
        ]))
        .centered();

        frame.render_widget(paragraph, layout[0]);
        frame.render_widget(footer, layout[1])
    }

    fn handle_event(
        &mut self,
        key_event: KeyEvent,
        active: &mut Area,
        db: &mut Option<Database>,
    ) -> super::KeyState {
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
                if self.paths_total == 0 {
                    return KeyState::Consumed;
                }

                if self.count + 1 < self.paths_total {
                    self.count += 1;
                } else {
                    self.count = 0;
                }
                return KeyState::Consumed;
            }
            KeyCode::Char('k') => {
                if self.paths_total == 0 {
                    return KeyState::Consumed;
                }

                if self.count != 0 {
                    self.count = self.count.saturating_sub(1);
                } else {
                    self.count = self.paths_total.saturating_sub(1);
                }
                return KeyState::Consumed;
            }
            KeyCode::Enter => {
                *db = Some(
                    Database::new(
                        self.abs_paths
                            .get(self.count as usize)
                            .unwrap()
                            .to_str()
                            .unwrap(),
                    )
                    .unwrap(),
                );
                *active = Area::SelectTableComponent;
                return KeyState::Consumed;
            }
            _ => KeyState::NotConsumed,
        };

        KeyState::NotConsumed
    }

    fn hide(&mut self) {
        self.visible = false;
    }

    fn show(&mut self) {}
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn path_exists() {
        let path = Path::new("test.db");
        assert!(path.canonicalize().unwrap().try_exists().unwrap());
    }
}
