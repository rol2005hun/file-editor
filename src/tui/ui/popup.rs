use crate::tui::app::{App, AppMode};
use crate::tui::ui::layout_helpers::centered_rect;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) {
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
        AppMode::Editor => {}
    }
}