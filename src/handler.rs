use crate::app::{App, AppResult, InputMode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.mode {
        InputMode::Normal => match key_event.code {
            // Switch mode
            KeyCode::Tab | KeyCode::Char('/') => {
                app.switch_mode();
            }
            KeyCode::Esc => app.clear_search(),
            // Exit application on `q`
            KeyCode::Char('q') => {
                app.quit();
            }
            // Exit application on `Ctrl-C`
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                }
            }
            // List handlers
            KeyCode::Char('j') => {
                app.select_next();
            }
            KeyCode::Char('k') => {
                app.select_previous();
            }
            KeyCode::Char('o') => {
                app.switch_owned_only();
            }
            // Other handlers you could add here.
            _ => {}
        },
        InputMode::Search => match key_event.code {
            // Switch mode
            KeyCode::Tab | KeyCode::Esc | KeyCode::Enter => {
                app.switch_mode();
            }
            // List handlers
            KeyCode::Char(c) => {
                app.push_search(c);
            }
            // Delete
            KeyCode::Backspace => {
                app.pull_search();
            }
            // Other handlers you could add here.
            _ => {}
        },
    }
    Ok(())
}
