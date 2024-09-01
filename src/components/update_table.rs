use ratatui::Frame;

use crate::app::App;

pub struct AlterTableComponent {}

impl super::Component for AlterTableComponent {
    fn draw(&self, frame: &mut Frame, app: &App) -> Box<dyn std::error::Error> {
        todo!()
    }

    fn is_focused(&self) -> bool {
        todo!()
    }

    fn hide(&self) {
        todo!()
    }

    fn show(&self) {
        todo!()
    }
}
