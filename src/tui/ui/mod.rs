pub mod editor;
pub mod footer;
pub mod layout_helpers;
pub mod popup;
pub mod sidebar;

use crate::tui::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub fn render(f: &mut Frame, app: &mut App) {
    let vertical_chunks: std::rc::Rc<[Rect]> = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(0),
                Constraint::Length(3),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.area());

    let horizontal_chunks: std::rc::Rc<[Rect]> = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(vertical_chunks[0]);

    app.explorer_area = horizontal_chunks[0];
    app.editor_area = horizontal_chunks[1];

    sidebar::render(f, app, app.explorer_area);
    editor::render(f, app, app.editor_area);
    footer::render_status(f, app, vertical_chunks[1]);
    footer::render_help(f, app, vertical_chunks[2]);
    popup::render(f, app);
}