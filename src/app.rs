use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use serde::de::value::Error;

use crate::{database, tui, ui};

pub enum LogicalState {
    Editing,
    Viewing,
}

pub enum ViewState {
    Main,
    Create,
    Read,
    Update,
    Delete,
}

pub struct App {
    pub database: Option<String>,
    pub current_view: Option<ViewState>,
    pub display_popup: bool, // go-to dialog
    pub input: String,
    pub error_message: Option<io::Error>,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_view: Some(ViewState::Main),
            display_popup: false,
            input: String::new(),
            database: None,
            error_message: None,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        loop {
            if let Ok(event) = event::read() {
                self.handle_event(event);
            }
            terminal.draw(|frame| ui::draw_ui(self, frame))?;

            if let None = self.current_view {
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
        if self.display_popup {
            match key_event.code {
                KeyCode::Char('c') => self.change_view(ViewState::Create),
                KeyCode::Char('r') => self.change_view(ViewState::Read),
                KeyCode::Char('u') if self.database != None => self.change_view(ViewState::Update),
                KeyCode::Char('d') if self.database != None => self.change_view(ViewState::Delete),
                KeyCode::Char('q') => self.current_view = None,

                _ => {}
            }
            self.display_popup = false;
            return;
        };

        match key_event.code {
            KeyCode::Char(' ') => self.display_popup = true,
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
                    self.database = Some(self.input.clone());
                    if let Some(name) = &self.database {
                        if let Err(err) = database::create_db(name) {
                            self.error_message = Some(err);
                        }
                    }
                    self.input.clear();
                    self.change_view(ViewState::Update);
                }

                _ => {}
            },
            Some(ViewState::Read) => match key_event.code {
                KeyCode::Char(ch) => self.input.push(ch),
                KeyCode::Backspace => {
                    self.input.pop();
                }
                KeyCode::Enter => {
                    if let Err(err) = database::open_db_if_exists(&self.input) {
                        self.error_message = Some(err);
                    } else {
                        self.database = Some(self.input.clone());
                    }
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

            // Some(ViewState::Goto) => match key_event.code {
            //     _ => {}
            // },
            _ => {}
        }
    }

    fn change_view(&mut self, view: ViewState) {
        self.current_view = Some(view);
    }

    fn exit(&mut self) {
        //
        self.current_view = None;
    }
}
