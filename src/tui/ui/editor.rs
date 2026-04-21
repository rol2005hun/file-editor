use crate::tui::app::{App, AppFocus, AppMode};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let editor_title: &str = match app.focus {
        AppFocus::Editor => " Editor (FOCUSED) ",
        _ => " Editor ",
    };

    let mut lines: Vec<Line> = Vec::new();
    for (i, row) in app.document.rows.iter().enumerate() {
        let row_text: String = if app.config.show_line_numbers {
            format!("{:3}  {}", i + 1, row)
        } else {
            row.clone()
        };

        let mut style: Style = Style::default();
        
        if app.config.highlight_active_line && matches!(app.focus, AppFocus::Editor) {
            if i == app.cursor_y as usize {
                style = style.bg(Color::Rgb(40, 40, 40));
            }
        }

        lines.push(Line::from(row_text).style(style));
    }

    let text: Text = Text::from(lines);
    let editor: Paragraph = Paragraph::new(text)
        .block(Block::default().title(editor_title).borders(Borders::ALL))
        .scroll((app.scroll_y, app.scroll_x));
    f.render_widget(editor, area);

    if let AppMode::Editor = app.mode {
        if let AppFocus::Editor = app.focus {
            let is_y_visible: bool = app.cursor_y >= app.scroll_y
                && app.cursor_y < app.scroll_y + app.editor_area.height.saturating_sub(2);
            let is_x_visible: bool = app.cursor_x >= app.scroll_x
                && app.cursor_x < app.scroll_x + app.editor_area.width.saturating_sub(2);

            if is_y_visible && is_x_visible {
                let mut offset_x: u16 = 0;
                if app.config.show_line_numbers {
                    offset_x = 5;
                }
                let display_x: u16 = app.editor_area.x + 1 + app.cursor_x - app.scroll_x + offset_x;
                let display_y: u16 = app.editor_area.y + 1 + app.cursor_y - app.scroll_y;
                f.set_cursor_position((display_x, display_y));
            }
        }
    }
}