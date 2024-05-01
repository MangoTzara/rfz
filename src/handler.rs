use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC`
        KeyCode::Esc => {
            app.quit(true);
        }
        // Counter handlers
        KeyCode::Up | KeyCode::BackTab => app.decrement_counter(),
        KeyCode::Down | KeyCode::Tab => app.increment_counter(),
        KeyCode::Enter => app.quit(false),
        KeyCode::Backspace => app.delete(),
        KeyCode::Char(_c) => app.update_query(key_event),
        _ => {}
    }
    Ok(())
}
