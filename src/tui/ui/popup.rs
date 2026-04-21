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
        AppMode::Settings(selection) => {
            let area: Rect = centered_rect(40, 30, f.area());
            f.render_widget(Clear, area);

            let s1: &str = if app.config.show_line_numbers { "[X] Show Line Numbers" } else { "[ ] Show Line Numbers" };
            let s2: &str = if app.config.show_status_bar { "[X] Show Status Bar" } else { "[ ] Show Status Bar" };
            let s3: &str = if app.config.show_help_bar { "[X] Show Help Bar" } else { "[ ] Show Help Bar" };
            let s4: &str = if app.config.highlight_active_line { "[X] Highlight Active Line" } else { "[ ] Highlight Active Line" };

            let options: Vec<String> = vec![s1.to_string(), s2.to_string(), s3.to_string(), s4.to_string()];
            let mut items: Vec<ListItem> = Vec::new();

            for (i, opt) in options.iter().enumerate() {
                if i == selection {
                    items.push(ListItem::new(Line::from(format!("> {}", opt))).style(Style::default().fg(Color::Yellow)));
                } else {
                    items.push(ListItem::new(Line::from(format!("  {}", opt))));
                }
            }

            let list: List = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(" Settings (Esc to back) "));
            f.render_widget(list, area);
        }
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
                .block(Block::default().borders(Borders::ALL).title(" Main Menu "));
            f.render_widget(list, area);
        }
        AppMode::PromptFile(ref input) => {
            let area: Rect = centered_rect(40, 20, f.area());
            f.render_widget(Clear, area);
            let block: Paragraph = Paragraph::new(input.as_str())
                .block(Block::default().borders(Borders::ALL).title(" New File Name "));
            f.render_widget(block, area);
        }
        AppMode::PromptDir(ref input) => {
            let area: Rect = centered_rect(40, 20, f.area());
            f.render_widget(Clear, area);
            let block: Paragraph = Paragraph::new(input.as_str())
                .block(Block::default().borders(Borders::ALL).title(" New Directory Name "));
            f.render_widget(block, area);
        }
        AppMode::Editor => {}
    }
}