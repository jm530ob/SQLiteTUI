use std::{error::Error, io, time::Duration};

use crossterm::event::{self, poll, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::Block,
    Frame,
};

use crate::{
    components::{modify_table::ModifyTableComponent, tree::TreeComponent, Component, KeyState},
    models,
    //  database::{Db, InputState},
    tui,
    // ui,
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
    pub tree_component: TreeComponent,
    pub modify_table_component: ModifyTableComponent,
    //    pub app_state: Option<AppState>,
    // pub mode: Mode,
    // pub db: Db,
    //  pub display_dialog: bool,
    //    pub display_append: bool,
    //    pub error_message: Option<io::Error>, // go-to dialog
    //    pub input: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_view: Some(ViewState::Main),
            tree_component: TreeComponent::new(),
            modify_table_component: ModifyTableComponent::new(),
            // app_state: None,
            // mode: Mode::Normal,
            // db: Db::new().expect("Could not create DB instance"),
            // display_dialog: false,
            // display_append: false,
            // error_message: None,
            // input: String::new(),k
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        loop {
            terminal.draw(|frame| {
                self.draw(frame, self.is_event())
                    .inspect_err(|e| eprintln!("{e}"));
            });

            if let Some(ViewState::Exiting) = self.current_view {
                break;
            }
        }
        Ok(())
    }

    fn is_event(&self) -> Option<KeyEvent> {
        if poll(Duration::from_millis(0)).ok()? {
            // returns Some(KeyEvent) -> KeyEvent
            match event::read().ok()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    return Some(key_event);
                }
                _ => {}
            }
        }
        None
    }

    pub fn setup(&mut self, args: models::args::Args) {
        self.tree_component.setup(&args);
        // self.modify_table_component.setup(&args);
    }

    fn draw(
        &mut self,
        f: &mut Frame,
        key_event: Option<KeyEvent>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(30), Constraint::Min(1)])
            .split(f.size());

        let mut tree_node_area = chunks[0];
        self.tree_component.draw(f, &mut tree_node_area, self);
        if let Some(ke) = key_event {
            self.tree_component.event(ke);
        }
        //println!("OKI DOKI");
        //tui::clear()?;
        //tree_node.s

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
