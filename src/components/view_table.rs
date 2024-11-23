use ratatui::{layout::Rect, Frame};

use crate::app::{App, Area};

use super::Component;

pub struct ViewTableComponent {
    is_visible: bool,
}

impl ViewTableComponent {
    pub fn new() -> Self {
        return Self { is_visible: false };
    }
}

impl Component for ViewTableComponent {
    fn update(&mut self, _app: &crate::database::Database) {}

    fn draw(&self, frame: &mut Frame, _area: &mut Rect, app: &App) {
        if !self.is_visible && !matches!(app.active, Area::ViewTableComponent) {
            return;
        }

        let db = app.db.as_ref().unwrap();
        if db.table.is_some() {
            let query = db.get_query(&db.conn, db.table.as_ref().unwrap());
            println!("{:?}", query.unwrap());
            assert!(false);
        }
    }

    fn handle_event(
        &mut self,
        key: crossterm::event::KeyEvent,
        active: &mut crate::app::Area,
        db: &mut Option<crate::database::Database>,
    ) -> super::KeyState {
        todo!()
    }

    fn setup(
        &mut self,
        args: &crate::models::args::Args,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn hide(&mut self) {
        self.is_visible = false;
    }

    fn show(&mut self) {
        self.is_visible = true;
    }
}
