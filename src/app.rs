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
                self.draw(frame, self.is_event());
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
        if let Some(key_event) = key_event {
            if matches!(self.tree_component.handle_event(key_event), KeyState::Consumed) {
                return Ok(());
            }
        }

        if key_event.is_some() {
            match key_event.unwrap().code {
                KeyCode::Esc => {
                    self.current_view = Some(ViewState::Exiting);
                    return Ok(());
                },
                _ => {}
            }
        }

        Ok(())
    }
    fn exit(&mut self) {
        //
        self.current_view = None;
    }
}

#[cfg(test)]
mod tests {
    // #[test]
}
