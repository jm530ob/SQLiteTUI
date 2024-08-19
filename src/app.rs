use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use serde::de::value::Error;

use crate::{
    database::{Db, InputState},
    tui, ui,
};

pub enum AppState {
    Receiving(ViewState),
    Editing,
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
    pub app_state: Option<AppState>,
    pub db: Db,
    pub display_dialog: bool,
    pub error_message: Option<io::Error>, // go-to dialog
    pub input: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_view: Some(ViewState::Main),
            app_state: None,
            db: Db::new().expect("Could not create DB instance"),
            display_dialog: false,
            error_message: None,
            input: String::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        loop {
            if let Ok(event) = event::read() {
                self.handle_event(event);
            }
            terminal.draw(|frame| ui::draw_ui(self, frame))?;

            if let Some(ViewState::Exiting) = self.current_view {
                break;
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if self.display_dialog {
            match key_event.code {
                KeyCode::Char('c') => self.change_view(ViewState::Create),
                KeyCode::Char('r') => self.change_view(ViewState::Read),
                KeyCode::Char('u') => self.change_view(ViewState::Update),
                KeyCode::Char('d') => self.change_view(ViewState::Delete),
                KeyCode::Char('q') => self.current_view = Some(ViewState::Exiting),

                _ => {}
            }
            self.display_dialog = false;
            return;
        };

        match key_event.code {
            KeyCode::Char(' ') => self.display_dialog = true,
            KeyCode::Esc => self.error_message = None,
            _ => {}
        }

        match self.current_view {
            Some(ViewState::Main) => match key_event.code {
                KeyCode::Char('q') => self.exit(),
                _ => {}
            },
            Some(ViewState::Create) => match key_event.code {
                KeyCode::Char(ch) => self.input.push(ch),
                KeyCode::Backspace => {
                    self.input.pop();
                }
                KeyCode::Enter => {
                    if !self.input.is_empty() {
                        self.db.db_name = Some(self.input.clone());
                    }
                    if let Err(err) = self.db.create_db() {
                        self.error_message = Some(err);
                    }
                    self.change_app_state();
                    // self.db.select_table(&self.current_view).unwrap();
                    self.current_view = None;
                    self.input.clear();
                }

                _ => {}
            },
            Some(ViewState::Read) => match key_event.code {
                KeyCode::Char(ch) => self.input.push(ch),
                KeyCode::Backspace => {
                    self.input.pop();
                }
                KeyCode::Enter => {
                    if !self.input.is_empty() {
                        self.db.db_name = Some(self.input.clone());
                    }
                    if let Err(err) = self.db.open_db_if_exists() {
                        self.error_message = Some(err);
                    }

                    self.change_app_state();
                    // self.db.select_table(&self.current_view).unwrap();

                    self.current_view = None;
                    self.input.clear();
                }
                _ => {}
            },
            Some(ViewState::Update) => match key_event.code {
                _ => {}
            },
            Some(ViewState::Delete) => match key_event.code {
                _ => {}
            },
            _ => {}
        }

        match self.app_state {
            Some(AppState::Receiving(ViewState::Create)) => {
                match self.db.input_state {
                    InputState::Table => match key_event.code {
                        KeyCode::Char(ch) => {
                            self.db.table_name.push(ch);
                        }
                        KeyCode::Backspace => {
                            self.db.table_name.pop();
                        }
                        KeyCode::Tab => {
                            self.db.toggle_input_state();
                        }
                        _ => {}
                    },
                    InputState::Attributes => match key_event.code {
                        KeyCode::Char(ch) => {
                            self.db.attributes.push(ch);
                        }
                        KeyCode::Backspace => {
                            self.db.attributes.pop();
                        }
                        KeyCode::Tab => {
                            self.db.toggle_input_state();
                        }
                        _ => {}
                    },
                }
                // self.db.records.push(10.into());
            }
            Some(AppState::Receiving(ViewState::Read)) => {}
            Some(AppState::Editing) => {}
            _ => {}
        }
    }

    // for item
    fn change_view(&mut self, view: ViewState) {
        self.current_view = Some(view);
    }

    pub fn change_app_state(&mut self) -> io::Result<()> {
        if self.current_view.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "state has to be initialized at this point",
            ));
        }

        match self.current_view.as_ref().unwrap() {
            ViewState::Create => self.app_state = Some(AppState::Receiving(ViewState::Create)),
            ViewState::Read => self.app_state = Some(AppState::Receiving(ViewState::Read)),
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        //
        self.current_view = None;
    }
    //fn ok($mu)
}

#[cfg(test)]
mod tests {
    // #[test]
}
