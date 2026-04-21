use crate::tui::app::{App, AppFocus, AppMode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout: std::rc::Rc<[Rect]> = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

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

    app.editor_area = horizontal_chunks[1];

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
    f.render_widget(sidebar, horizontal_chunks[0]);

    let editor_title: &str = match app.focus {
        AppFocus::Editor => " Editor (FOCUSED) ",
        _ => " Editor ",
    };

    let content: String = app.document.rows.join("\n");
    let text: Text = Text::raw(content);
    let editor: Paragraph = Paragraph::new(text)
        .block(Block::default().title(editor_title).borders(Borders::ALL))
        .scroll((app.scroll_y, app.scroll_x));
    f.render_widget(editor, app.editor_area);

    let status_mode: &str = match app.mode {
        AppMode::Editor => "EDITOR",
        AppMode::Menu => "MENU",
        AppMode::PromptFile(_) | AppMode::PromptDir(_) => "INPUT",
    };

    let path_display: String = app
        .document
        .path
        .clone()
        .unwrap_or_else(|| "Untitled".to_string());
    let status_text: String = format!(
        "{} | {} | Ln {}, Col {}",
        status_mode,
        path_display,
        app.cursor_y + 1,
        app.cursor_x + 1
    );
    let status_bar: Paragraph = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status_bar, vertical_chunks[1]);

    let help_text: String = match app.mode {
        AppMode::Editor => match app.focus {
            AppFocus::Explorer => "Tab: Focus Editor | Enter: Open | n: New File | d: New Dir".to_string(),
            AppFocus::Editor => "Ctrl+S: Save | Ctrl+C: Copy Line | Ctrl+V: Paste | Tab: Focus Explorer".to_string(),
        },
        AppMode::Menu => "Enter: Select | Esc: Back | Up/Down: Navigate".to_string(),
        AppMode::PromptFile(_) | AppMode::PromptDir(_) => "Enter: Submit | Esc: Cancel".to_string(),
    };

    let help_bar: Paragraph = Paragraph::new(help_text)
        .block(Block::default().title(" Shortcuts / Tutorial ").borders(Borders::ALL));
    f.render_widget(help_bar, vertical_chunks[2]);

    match app.mode {
        AppMode::Menu => {
            let area: Rect = centered_rect(40, 40, f.area());
            f.render_widget(Clear, area);

            let items: Vec<ListItem> = app
                .menu_items
                .iter()
                .enumerate()
                .map(|(i, m)| {
                    if i == app.menu_selection {
                        ListItem::new(Line::from(format!("> {}", m)))
                            .style(Style::default().fg(Color::Yellow))
                    } else {
                        ListItem::new(Line::from(format!("  {}", m)))
                    }
                })
                .collect();

            let list: List = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Main Menu"));
            f.render_widget(list, area);
        }
        AppMode::PromptFile(ref input) => {
            let area: Rect = centered_rect(40, 20, f.area());
            f.render_widget(Clear, area);
            let block: Paragraph = Paragraph::new(input.as_str())
                .block(Block::default().borders(Borders::ALL).title("New File Name"));
            f.render_widget(block, area);
        }
        AppMode::PromptDir(ref input) => {
            let area: Rect = centered_rect(40, 20, f.area());
            f.render_widget(Clear, area);
            let block: Paragraph = Paragraph::new(input.as_str())
                .block(Block::default().borders(Borders::ALL).title("New Directory Name"));
            f.render_widget(block, area);
        }
        AppMode::Editor => {
            if let AppFocus::Editor = app.focus {
                let is_y_visible: bool = app.cursor_y >= app.scroll_y
                    && app.cursor_y < app.scroll_y + app.editor_area.height.saturating_sub(2);
                let is_x_visible: bool = app.cursor_x >= app.scroll_x
                    && app.cursor_x < app.scroll_x + app.editor_area.width.saturating_sub(2);

                if is_y_visible && is_x_visible {
                    let display_x: u16 = app.editor_area.x + 1 + app.cursor_x - app.scroll_x;
                    let display_y: u16 = app.editor_area.y + 1 + app.cursor_y - app.scroll_y;
                    f.set_cursor_position((display_x, display_y));
                }
            }
        }
    }
}