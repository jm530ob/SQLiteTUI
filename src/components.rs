use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

use crate::{
    app::{App, Area},
    database::Database,
    models,
};

pub mod select_table;
pub mod tree;
pub mod view_table;

/// Represents the state of a key event after being handled by a component.
/// Restricts unwanted behaviour when multiple widgets are open.
/// - Used in App
pub enum KeyState {
    Consumed,
    NotConsumed,
}

/// Common interface of all widgets.
pub trait Component {
    /// Draws the component on the given frame within the specified area.
    ///
    /// # Parameters
    /// - `frame`: The rendering target for the component.
    /// - `area`: The layout area where the component should be drawn.
    /// - `app`: The application state used for rendering.
    fn draw(&self, frame: &mut Frame, area: &mut Rect, app: &App);

    /// Updates the component.
    ///
    /// # Parameters
    /// - `db`: Reference to the database.
    fn update(&mut self, db: &Database);

    /// Handles a key event, updating the active area or database state.
    ///
    /// # Parameters
    /// - `key`: The key event to handle.
    /// - `active`: A mutable reference to the currently active area.
    /// - `db`: A mutable optional reference to the database.
    ///
    /// # Returns
    /// A `KeyState` indicating whether the event was consumed or not.
    fn handle_event(
        &mut self,
        key: KeyEvent,
        active: &mut Area,
        db: &mut Option<Database>,
    ) -> KeyState;

    /// Sets up the component.
    ///
    /// # Parameters
    /// - `args`: The parsed command-line arguments.
    ///
    /// # Returns
    /// A `Result`.
    fn setup(&mut self, args: &models::args::Args) -> Result<(), Box<dyn std::error::Error>>;

    /// Hides the component, making it inactive.
    fn hide(&mut self);

    /// Makes the component visible.
    fn show(&mut self);
}
