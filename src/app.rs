use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::{tui, ui};

pub enum ViewState {
    Main,
    Exit,
    Create,
    Read,
    Update,
    Delete,
}

pub struct App {
    pub current_view: Option<ViewState>,
    /// Display Go-To dialog
    pub display_goto: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_view: Some(ViewState::Main),
            display_goto: false,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        loop {
            if let Ok(event) = event::read() {
                self.handle_event(event);
            }
            terminal.draw(|frame| ui::draw_ui(self, frame))?;
        }
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
        if self.display_goto {
            match key_event.code {
                KeyCode::Char('c') => self.change_view(ViewState::Create),
                KeyCode::Char('r') => self.change_view(ViewState::Read),
                KeyCode::Char('u') => self.change_view(ViewState::Update),
                KeyCode::Char('d') => self.change_view(ViewState::Delete),
                _ => {}
            }
            self.display_goto = false;
            return;
        };

        match key_event.code {
            KeyCode::Char(' ') => self.display_goto = true,
            _ => {}
        }

        match self.current_view {
            Some(ViewState::Main) => match key_event.code {
                KeyCode::Char('q') => self.exit(),
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
