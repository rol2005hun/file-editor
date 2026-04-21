use crate::core::document::Document;
use crate::tui::app::{App, AppFocus, AppMode};
use arboard::Clipboard;
use crossterm::event::{
    self, Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind,
};
use std::io;

pub fn handle_events(app: &mut App) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        match event::read()? {
            Event::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    let is_ctrl: bool = key.modifiers.contains(KeyModifiers::CONTROL);

                    if is_ctrl {
                        if let AppMode::Editor = app.mode {
                            if let AppFocus::Editor = app.focus {
                                match key.code {
                                    KeyCode::Char('s') => {
                                        let _ = app.document.save();
                                    }
                                    KeyCode::Char('c') => {
                                        if let Ok(mut clipboard) = Clipboard::new() {
                                            if let Some(line) =
                                                app.document.rows.get(app.cursor_y as usize)
                                            {
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
                        return Ok(());
                    }

                    let mut mode_transition: Option<AppMode> = None;

                    match app.mode {
                        AppMode::PromptFile(ref mut input) => match key.code {
                            KeyCode::Esc => mode_transition = Some(AppMode::Editor),
                            KeyCode::Enter => {
                                let text: String = input.clone();
                                let _ = app.explorer.create_file(&text);
                                mode_transition = Some(AppMode::Editor);
                            }
                            KeyCode::Char(c) => input.push(c),
                            KeyCode::Backspace => {
                                input.pop();
                            }
                            _ => {}
                        },
                        AppMode::PromptDir(ref mut input) => match key.code {
                            KeyCode::Esc => mode_transition = Some(AppMode::Editor),
                            KeyCode::Enter => {
                                let text: String = input.clone();
                                let _ = app.explorer.create_dir(&text);
                                mode_transition = Some(AppMode::Editor);
                            }
                            KeyCode::Char(c) => input.push(c),
                            KeyCode::Backspace => {
                                input.pop();
                            }
                            _ => {}
                        },
                        AppMode::Editor => {
                            if key.code == KeyCode::Tab {
                                app.focus = match app.focus {
                                    AppFocus::Explorer => AppFocus::Editor,
                                    AppFocus::Editor => AppFocus::Explorer,
                                };
                            } else {
                                match app.focus {
                                    AppFocus::Explorer => match key.code {
                                        KeyCode::Esc => app.toggle_menu(),
                                        KeyCode::Up => app.explorer.previous(),
                                        KeyCode::Down => app.explorer.next(),
                                        KeyCode::Enter => {
                                            if let Some(path) =
                                                app.explorer.items.get(app.explorer.selected)
                                            {
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
                                        KeyCode::Char('n') => {
                                            mode_transition =
                                                Some(AppMode::PromptFile(String::new()));
                                        }
                                        KeyCode::Char('d') => {
                                            mode_transition =
                                                Some(AppMode::PromptDir(String::new()));
                                        }
                                        _ => {}
                                    },
                                    AppFocus::Editor => match key.code {
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
                        AppMode::Menu => match key.code {
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
            }
            Event::Mouse(mouse_event) => {
                match mouse_event.kind {
                    MouseEventKind::Down(event::MouseButton::Left) => {
                        if let AppFocus::Editor = app.focus {
                            if mouse_event.column >= app.editor_area.x + 1
                                && mouse_event.column
                                    < app.editor_area.x + app.editor_area.width - 1
                                && mouse_event.row >= app.editor_area.y + 1
                                && mouse_event.row < app.editor_area.y + app.editor_area.height - 1
                            {
                                let click_x: u16 = mouse_event.column.saturating_sub(app.editor_area.x + 1) + app.scroll_x;
                                let click_y: u16 = mouse_event.row.saturating_sub(app.editor_area.y + 1) + app.scroll_y;
                                app.handle_click(click_x, click_y);
                            }
                        }
                    }
                    MouseEventKind::ScrollDown => {
                        if let AppFocus::Editor = app.focus {
                            app.scroll_y = app.scroll_y.saturating_add(1);
                        }
                    }
                    MouseEventKind::ScrollUp => {
                        if let AppFocus::Editor = app.focus {
                            app.scroll_y = app.scroll_y.saturating_sub(1);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    Ok(())
}