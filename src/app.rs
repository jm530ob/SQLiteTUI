use crossterm::event::{self, read, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::{tui::Tui, ui::Ui};
use std::{collections::HashMap, io};

pub enum ItemType {
    Key,
    Value,
}

pub enum Screen {
    Main,
    Editing(ItemType),
    Exiting,
}

pub struct App {
    pub json_key: String,
    pub json_value: String,
    pub json_pair: HashMap<String, String>,
    pub screen: Option<Screen>,
}

impl App {
    pub fn default() -> Self {
        Self {
            json_key: "".to_owned(),
            json_value: "".to_owned(),
            json_pair: HashMap::new(),
            screen: Some(Screen::Main),
        }
    }

    pub fn save_json_pair(&mut self) {
        self.json_pair
            .insert(self.json_key.clone(), self.json_value.clone());
        self.json_key.clear();
        self.json_value.clear();
    }

    pub fn print_json(&self) {
        let res = serde_json::to_string(&self.json_pair).unwrap();
        println!("{:?}", res);
    }

    pub fn toggle_screens(&mut self) {
        match self.screen {
            Some(Screen::Main) => self.screen = Some(Screen::Editing(ItemType::Key)),
            Some(Screen::Editing(_)) => self.screen = Some(Screen::Main),
            _ => self.screen = None,
        }
    }

    pub fn process_item(&mut self) {
        if let Some(Screen::Editing(ItemType::Key)) = self.screen {
            self.screen = Some(Screen::Editing(ItemType::Value));
        } else {
            self.save_json_pair();
            self.screen = Some(Screen::Main);
        }
    }

    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        loop {
            if let Ok(event) = read() {
                match event {
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        self.handle_key_event(key_event)
                    }
                    _ => {}
                }
            }
            terminal.draw(|frame| Ui::render(frame, self))?;
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match &self.screen {
            Some(Screen::Main) => match key_event.code {
                KeyCode::Char('q') => self.screen = Some(Screen::Exiting),
                KeyCode::Char('e') => self.screen = Some(Screen::Editing(ItemType::Key)),
                KeyCode::Tab => self.toggle_screens(),
                _ => {}
            },
            Some(Screen::Editing(item)) => match key_event.code {
                // Draw inner UI
                KeyCode::Char(ch) => match item {
                    ItemType::Key => {
                        self.json_key.push(ch);
                    }
                    ItemType::Value => {
                        self.json_value.push(ch);
                    }
                },
                KeyCode::Backspace => match item {
                    ItemType::Key => {
                        self.json_key.pop();
                    }
                    ItemType::Value => {
                        self.json_value.pop();
                    }
                },
                KeyCode::Enter => self.process_item(),

                _ => {}
            },
            Some(Screen::Exiting) => {
                match key_event.code {
                    KeyCode::Char('y') => {
                        self.screen = None;
                        self.print_json();
                    }
                    KeyCode::Char('n') => {
                        self.screen = Some(Screen::Main);
                    }
                    _ => {}
                }
                // TODO: Draw inner UI
            }
            _ => {}
        }
    }
}
