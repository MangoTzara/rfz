use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC`
        KeyCode::Esc => {
            app.quit();
        }
        // Counter handlers
        KeyCode::Up => {
            app.decrement_counter();
        }
        KeyCode::Down => {
            app.increment_counter();
        }
        KeyCode::Enter => {
            app.quit();
        }
        KeyCode::Backspace => app.delete(),
        KeyCode::Char(c) => {
            app.update_query(c);
        }
        _ => {}
    }
    Ok(())
}
