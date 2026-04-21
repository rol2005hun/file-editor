use crate::tui::app::{App, AppFocus};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let explorer_title: &str = match app.focus {
        AppFocus::Explorer => " Explorer (FOCUSED) ",
        _ => " Explorer ",
    };

    let mut list_items: Vec<ListItem> = Vec::new();
    for (i, path) in app.explorer.items.iter().enumerate() {
        let name: std::borrow::Cow<'_, str> =
            path.file_name().unwrap_or_default().to_string_lossy();
        let display_name: String = if path.is_dir() {
            format!("[{}]", name)
        } else {
            name.to_string()
        };

        let mut style: Style = Style::default();
        if let AppFocus::Explorer = app.focus {
            if i == app.explorer.selected {
                style = style.bg(Color::DarkGray).fg(Color::White);
            }
        }
        list_items.push(ListItem::new(Line::from(display_name)).style(style));
    }

    let sidebar: List = List::new(list_items)
        .block(Block::default().title(explorer_title).borders(Borders::ALL));
    f.render_widget(sidebar, area);
}