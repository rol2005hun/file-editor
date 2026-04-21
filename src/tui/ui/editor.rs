use crate::tui::app::{App, AppFocus, AppMode};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let editor_title: &str = match app.focus {
        AppFocus::Editor => " Editor (FOCUSED) ",
        _ => " Editor ",
    };

    let bounds: Option<((usize, usize), (usize, usize))> = app.get_selection_bounds();
    let selection_style: Style = Style::default().bg(Color::Rgb(0, 100, 200)).fg(Color::White);

    let mut lines: Vec<Line> = Vec::new();
    for (i, row) in app.document.rows.iter().enumerate() {
        let mut spans: Vec<Span> = Vec::new();

        if app.config.show_line_numbers {
            spans.push(Span::styled(
                format!("{:3}  ", i + 1),
                Style::default().fg(Color::DarkGray),
            ));
        }

        let is_active_line: bool = app.config.highlight_active_line
            && matches!(app.focus, AppFocus::Editor)
            && i == (app.cursor_y as usize);
            
        let active_line_style: Style = if is_active_line && bounds.is_none() {
            Style::default().bg(Color::Rgb(40, 40, 40))
        } else {
            Style::default()
        };

        let chars: Vec<char> = row.chars().collect();
        let row_len: usize = chars.len();

        if let Some((start, end)) = bounds {
            if i >= start.1 && i <= end.1 {
                let sel_start_x: usize = if i == start.1 { start.0 } else { 0 };
                let sel_end_x: usize = if i == end.1 { end.0 } else { row_len };

                let safe_start: usize = std::cmp::min(sel_start_x, row_len);
                let safe_end: usize = std::cmp::min(sel_end_x, row_len);

                let before: String = chars[..safe_start].iter().collect();
                let selected: String = chars[safe_start..safe_end].iter().collect();
                let after: String = chars[safe_end..].iter().collect();

                if !before.is_empty() {
                    spans.push(Span::styled(before, active_line_style));
                }
                if !selected.is_empty() {
                    spans.push(Span::styled(selected, selection_style));
                }
                if !after.is_empty() {
                    spans.push(Span::styled(after, active_line_style));
                }

                if i < end.1 {
                    spans.push(Span::styled(" ", selection_style));
                }
            } else {
                spans.push(Span::styled(row.clone(), active_line_style));
            }
        } else {
            spans.push(Span::styled(row.clone(), active_line_style));
        }

        lines.push(Line::from(spans));
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