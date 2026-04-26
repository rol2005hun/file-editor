use crate::core::document::Document;
use crate::core::app::{App, AppFocus, AppMode};
use crossterm::event::KeyCode;

pub fn handle_key(app: &mut App, code: KeyCode) {
    let mut mode_transition = None;

    match app.mode {
        AppMode::Settings(_) => match code {
            KeyCode::Esc => mode_transition = Some(AppMode::Menu),
            KeyCode::Up => app.settings_up(),
            KeyCode::Down => app.settings_down(),
            KeyCode::Enter => app.toggle_setting(),
            _ => {}
        },
        AppMode::PromptFile(ref mut input) => match code {
            KeyCode::Esc => mode_transition = Some(AppMode::Editor),
            KeyCode::Enter => {
                let text = input.clone();
                let _ = app.explorer.create_file(&text);
                mode_transition = Some(AppMode::Editor);
            }
            KeyCode::Char(c) => input.push(c),
            KeyCode::Backspace => { input.pop(); }
            _ => {}
        },
        AppMode::PromptDir(ref mut input) => match code {
            KeyCode::Esc => mode_transition = Some(AppMode::Editor),
            KeyCode::Enter => {
                let text = input.clone();
                let _ = app.explorer.create_dir(&text);
                mode_transition = Some(AppMode::Editor);
            }
            KeyCode::Char(c) => input.push(c),
            KeyCode::Backspace => { input.pop(); }
            _ => {}
        },
        AppMode::Editor => {
            if code == KeyCode::Tab {
                app.focus = match app.focus {
                    AppFocus::Explorer => AppFocus::Editor,
                    AppFocus::Editor => AppFocus::Explorer,
                };
            } else {
                match app.focus {
                    AppFocus::Explorer => match code {
                        KeyCode::Esc => app.toggle_menu(),
                        KeyCode::Up => app.explorer.previous(),
                        KeyCode::Down => app.explorer.next(),
                        KeyCode::Enter => {
                            if let Some(path) = app.explorer.items.get(app.explorer.selected) {
                                if path.is_dir() {
                                    app.explorer.current_path = path.clone();
                                    let _ = app.explorer.refresh();
                                } else if let Ok(doc) = Document::open(path) {
                                    app.document = doc;
                                    app.cursor_x = 0;
                                    app.cursor_y = 0;
                                    app.scroll_x = 0;
                                    app.scroll_y = 0;
                                    app.focus = AppFocus::Editor;
                                }
                            }
                        }
                        KeyCode::Char('n') => mode_transition = Some(AppMode::PromptFile(String::new())),
                        KeyCode::Char('d') => mode_transition = Some(AppMode::PromptDir(String::new())),
                        _ => {}
                    },
                    AppFocus::Editor => match code {
                        KeyCode::Esc => app.toggle_menu(),
                        KeyCode::Char(c) => app.insert_char(c),
                        KeyCode::Backspace => app.delete_char(),
                        KeyCode::Enter => app.insert_newline(),
                        KeyCode::Left => app.move_cursor(-1, 0),
                        KeyCode::Right => app.move_cursor(1, 0),
                        KeyCode::Up => app.move_cursor(0, -1),
                        KeyCode::Down => app.move_cursor(0, 1),
                        _ => {}
                    },
                }
            }
        }
        AppMode::Menu => match code {
            KeyCode::Esc => app.toggle_menu(),
            KeyCode::Up => app.menu_up(),
            KeyCode::Down => app.menu_down(),
            KeyCode::Enter => app.execute_menu_action(),
            _ => {}
        },
    }

    if let Some(new_mode) = mode_transition {
        app.mode = new_mode;
    }
}