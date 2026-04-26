use crate::core::app::{App, AppFocus, AppMode};
use arboard::Clipboard;
use crossterm::event::KeyCode;

pub fn handle_ctrl_shortcut(app: &mut App, code: KeyCode) {
    if let AppMode::Editor = app.mode {
        if let AppFocus::Editor = app.focus {
            match code {
                KeyCode::Char('s') => { let _ = app.document.save(); }
                KeyCode::Char('z') => app.undo(),
                KeyCode::Char('c') => {
                    if let Ok(mut clipboard) = Clipboard::new() {
                        if let Some(selected_text) = app.get_selected_text() {
                            let _ = clipboard.set_text(selected_text);
                        } else if let Some(line) = app.document.rows.get(app.cursor_y as usize) {
                            let _ = clipboard.set_text(line.clone());
                        }
                    }
                }
                KeyCode::Char('v') => {
                    if let Ok(mut clipboard) = Clipboard::new() {
                        if let Ok(text) = clipboard.get_text() {
                            app.paste(&text);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}