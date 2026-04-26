pub mod keyboard;
pub mod mouse;
pub mod shortcuts;

use crate::core::app::App;
use crossterm::event::{self, Event, KeyEventKind, KeyModifiers};
use ratatui::layout::Rect;
use std::io;

pub fn handle_events(
    app: &mut App, 
    explorer_area: Rect, 
    editor_area: Rect, 
    popup_area: Option<Rect>
) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        match event::read()? {
            Event::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    let is_ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
                    if is_ctrl {
                        shortcuts::handle_ctrl_shortcut(app, key.code);
                        return Ok(());
                    }
                    keyboard::handle_key(app, key.code);
                }
            }
            Event::Mouse(mouse_event) => {
                mouse::handle_mouse(app, mouse_event, explorer_area, editor_area, popup_area);
            }
            _ => {}
        }
    }
    Ok(())
}