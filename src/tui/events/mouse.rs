use crate::core::document::Document;
use crate::tui::app::{App, AppFocus};
use crossterm::event::{self, MouseEvent, MouseEventKind};

pub fn handle_mouse(app: &mut App, mouse_event: MouseEvent) {
    match mouse_event.kind {
        MouseEventKind::Down(event::MouseButton::Left) => {
            if mouse_event.column >= app.explorer_area.x
                && mouse_event.column < app.explorer_area.x + app.explorer_area.width
                && mouse_event.row >= app.explorer_area.y
                && mouse_event.row < app.explorer_area.y + app.explorer_area.height
            {
                app.focus = AppFocus::Explorer;

                if mouse_event.row >= app.explorer_area.y + 1
                    && mouse_event.row < app.explorer_area.y + app.explorer_area.height - 1
                {
                    let clicked_index: usize =
                        (mouse_event.row - (app.explorer_area.y + 1)) as usize;

                    if clicked_index < app.explorer.items.len() {
                        app.explorer.selected = clicked_index;

                        let path: std::path::PathBuf = app.explorer.items[clicked_index].clone();

                        if path.is_dir() {
                            app.explorer.current_path = path;
                            let _ = app.explorer.refresh();
                        } else if let Ok(doc) = Document::open(&path) {
                            app.document = doc;
                            app.cursor_x = 0;
                            app.cursor_y = 0;
                            app.scroll_x = 0;
                            app.scroll_y = 0;
                            app.focus = AppFocus::Editor;
                        }
                    }
                }
            } else if mouse_event.column >= app.editor_area.x
                && mouse_event.column < app.editor_area.x + app.editor_area.width
                && mouse_event.row >= app.editor_area.y
                && mouse_event.row < app.editor_area.y + app.editor_area.height
            {
                app.focus = AppFocus::Editor;

                if mouse_event.column >= app.editor_area.x + 1
                    && mouse_event.column < app.editor_area.x + app.editor_area.width - 1
                    && mouse_event.row >= app.editor_area.y + 1
                    && mouse_event.row < app.editor_area.y + app.editor_area.height - 1
                {
                    let click_x: u16 = mouse_event
                        .column
                        .saturating_sub(app.editor_area.x + 1)
                        + app.scroll_x;
                    let click_y: u16 = mouse_event
                        .row
                        .saturating_sub(app.editor_area.y + 1)
                        + app.scroll_y;
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