use crate::core::document::Document;
use crate::core::app::{App, AppFocus, AppMode};
use crossterm::event::{self, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

pub fn handle_mouse(
    app: &mut App, 
    mouse_event: MouseEvent, 
    explorer_area: Rect, 
    editor_area: Rect, 
    popup_area: Option<Rect>
) {
    match mouse_event.kind {
        MouseEventKind::Down(event::MouseButton::Left) => {
            if !matches!(app.mode, AppMode::Editor) {
                if let Some(p_area) = popup_area {
                    if mouse_event.column >= p_area.x
                        && mouse_event.column < p_area.x + p_area.width
                        && mouse_event.row >= p_area.y
                        && mouse_event.row < p_area.y + p_area.height
                    {
                        if mouse_event.row >= p_area.y + 1
                            && mouse_event.row < p_area.y + p_area.height - 1
                        {
                            let clicked_index = (mouse_event.row - p_area.y - 1) as usize;
                            
                            if let AppMode::Menu = app.mode {
                                if clicked_index < app.menu_items.len() {
                                    app.menu_selection = clicked_index;
                                    app.execute_menu_action();
                                }
                            } else if let AppMode::Settings(_) = app.mode {
                                if clicked_index < 4 {
                                    app.mode = AppMode::Settings(clicked_index);
                                    app.toggle_setting();
                                }
                            }
                        }
                    } else {
                        app.mode = AppMode::Editor;
                    }
                }
                return;
            }

            if mouse_event.column >= explorer_area.x
                && mouse_event.column < explorer_area.x + explorer_area.width
                && mouse_event.row >= explorer_area.y
                && mouse_event.row < explorer_area.y + explorer_area.height
            {
                app.focus = AppFocus::Explorer;

                if mouse_event.row >= explorer_area.y + 1
                    && mouse_event.row < explorer_area.y + explorer_area.height - 1
                {
                    let clicked_index = (mouse_event.row - (explorer_area.y + 1)) as usize;

                    if let Some(path) = app.explorer.items.get(clicked_index) {
                        let path_clone = path.clone();
                        app.explorer.selected = clicked_index;

                        if path_clone.is_dir() {
                            app.explorer.current_path = path_clone;
                            let _ = app.explorer.refresh();
                        } else if let Ok(doc) = Document::open(&path_clone) {
                            app.document = doc;
                            app.cursor_x = 0;
                            app.cursor_y = 0;
                            app.scroll_x = 0;
                            app.scroll_y = 0;
                            app.focus = AppFocus::Editor;
                        }
                    }
                }
            } else if mouse_event.column >= editor_area.x
                && mouse_event.column < editor_area.x + editor_area.width
                && mouse_event.row >= editor_area.y
                && mouse_event.row < editor_area.y + editor_area.height
            {
                app.focus = AppFocus::Editor;
                app.selection_start = None;

                if mouse_event.column >= editor_area.x + 1
                    && mouse_event.column < editor_area.x + editor_area.width - 1
                    && mouse_event.row >= editor_area.y + 1
                    && mouse_event.row < editor_area.y + editor_area.height - 1
                {
                    let click_x = mouse_event.column - (editor_area.x + 1) + app.scroll_x;
                    let click_y = mouse_event.row - (editor_area.y + 1) + app.scroll_y;
                    app.handle_click(click_x, click_y);
                }
            }
        }
        MouseEventKind::Drag(event::MouseButton::Left) => {
            if let AppFocus::Editor = app.focus {
                if mouse_event.column >= editor_area.x + 1
                    && mouse_event.column < editor_area.x + editor_area.width - 1
                    && mouse_event.row >= editor_area.y + 1
                    && mouse_event.row < editor_area.y + editor_area.height - 1
                {
                    if app.selection_start.is_none() {
                        app.selection_start = Some((app.cursor_x, app.cursor_y));
                    }

                    let drag_x = mouse_event.column - (editor_area.x + 1) + app.scroll_x;
                    let drag_y = mouse_event.row - (editor_area.y + 1) + app.scroll_y;
                    app.handle_click(drag_x, drag_y);
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