pub mod editor;
pub mod footer;
pub mod layout_helpers;
pub mod popup;
pub mod sidebar;

use crate::core::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub fn render(f: &mut Frame, app: &mut App) -> (Rect, Rect, Option<Rect>) {
    let mut vertical_constraints = vec![Constraint::Min(0)];

    if app.config.show_status_bar {
        vertical_constraints.push(Constraint::Length(3));
    }
    if app.config.show_help_bar {
        vertical_constraints.push(Constraint::Length(3));
    }

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vertical_constraints)
        .split(f.area());

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(vertical_chunks[0]);

    let explorer_area = horizontal_chunks[0];
    let editor_area = horizontal_chunks[1];

    sidebar::render(f, app, explorer_area);
    editor::render(f, app, editor_area);

    let mut current_bottom_chunk = 1;

    if app.config.show_status_bar {
        footer::render_status(f, app, vertical_chunks[current_bottom_chunk]);
        current_bottom_chunk += 1;
    }
    
    if app.config.show_help_bar {
        footer::render_help(f, app, vertical_chunks[current_bottom_chunk]);
    }
    
    let popup_area = popup::render(f, app);

    (explorer_area, editor_area, popup_area)
}