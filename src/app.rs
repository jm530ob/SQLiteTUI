use std::{error::Error, io};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Direction, Layout},
    Frame,
};

use crate::{
    components::KeyState,
    database::{Db, InputState},
    tui, ui,
};

pub enum Mode {
    Normal,
    Insert,
}

// pub enum AppState {
//     Receiving(ViewState),
//     Editing,
// }

pub enum ViewState {
    Main,
    Create,
    Read,
    Update,
    Delete,
    Exiting,
}

pub struct App {
    pub current_view: Option<ViewState>,
    //    pub app_state: Option<AppState>,
    // pub mode: Mode,
    pub db: Db,
    //  pub display_dialog: bool,
    //    pub display_append: bool,
    //    pub error_message: Option<io::Error>, // go-to dialog
    //    pub input: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_view: Some(ViewState::Main),
            // app_state: None,
            // mode: Mode::Normal,
            db: Db::new().expect("Could not create DB instance"),
            // display_dialog: false,
            // display_append: false,
            // error_message: None,
            // input: String::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        loop {
            if let Ok(event) = event::read() {
                self.handle_event(event);
            }
            terminal.draw(|frame| {
                if let Ok(_) = self.draw(frame) {
                    todo!()
                }
            });

            if let Some(ViewState::Exiting) = self.current_view {
                break;
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
        let chunks = Layout::default().direction(Direction::Horizontal);
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    // for item
    //  fn change_view(&mut self, view: ViewState) {
    //      self.current_view = Some(view);
    //  }

    //  fn change_app_state(&mut self) -> io::Result<()> {
    //      if self.current_view.is_none() {
    //          return Err(io::Error::new(
    //              io::ErrorKind::Other,
    //              "state has to be initialized at this point",
    //          ));
    //      }

    //      match self.current_view.as_ref().unwrap() {
    //          ViewState::Create => self.app_state = Some(AppState::Receiving(ViewState::Create)),
    //          ViewState::Read => self.app_state = Some(AppState::Receiving(ViewState::Read)),
    //          _ => {}
    //      }
    //      Ok(())
    //  }

    fn exit(&mut self) {
        //
        self.current_view = None;
    }
}

#[cfg(test)]
mod tests {
    // #[test]
}
