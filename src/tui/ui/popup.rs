use crate::core::app::{App, AppMode};
use crate::tui::ui::layout_helpers::centered_rect;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Padding},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) -> Option<Rect> {
    match app.mode {
        AppMode::Settings(selection) => {
            let area = centered_rect(40, 30, f.area());
            f.render_widget(Clear, area);

            let s1 = if app.config.show_line_numbers { "[X] Show Line Numbers" } else { "[ ] Show Line Numbers" };
            let s2 = if app.config.show_status_bar { "[X] Show Status Bar" } else { "[ ] Show Status Bar" };
            let s3 = if app.config.show_help_bar { "[X] Show Help Bar" } else { "[ ] Show Help Bar" };
            let s4 = if app.config.highlight_active_line { "[X] Highlight Active Line" } else { "[ ] Highlight Active Line" };

            let options = vec![s1, s2, s3, s4];
            let mut items = Vec::new();

            for (i, opt) in options.iter().enumerate() {
                if i == selection {
                    items.push(ListItem::new(Line::from(format!("> {}", opt))).style(Style::default().fg(Color::Yellow)));
                } else {
                    items.push(ListItem::new(Line::from(format!("  {}", opt))));
                }
            }

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(" Settings (Esc to back) "));
            f.render_widget(list, area);
            Some(area)
        }
        AppMode::Menu => {
            let area = centered_rect(40, 40, f.area());
            f.render_widget(Clear, area);

            let items: Vec<ListItem> = app
                .menu_items
                .iter()
                .enumerate()
                .map(|(i, m)| {
                    if i == app.menu_selection {
                        ListItem::new(Line::from(format!("> {}", m))).style(Style::default().fg(Color::Yellow))
                    } else {
                        ListItem::new(Line::from(format!("  {}", m)))
                    }
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(" Main Menu "));
            f.render_widget(list, area);
            Some(area)
        }
        AppMode::PromptFile(ref input) => {
            let area = centered_rect(40, 6, f.area());
            f.render_widget(Clear, area);
            
            let block = Block::default()
                .borders(Borders::ALL)
                .title(" New File Name ")
                .padding(Padding::horizontal(1));

            let paragraph = Paragraph::new(input.as_str()).block(block);
            f.render_widget(paragraph, area);
            Some(area)
        }
        AppMode::PromptDir(ref input) => {
            let area = centered_rect(40, 6, f.area());
            f.render_widget(Clear, area);
            
            let block = Block::default()
                .borders(Borders::ALL)
                .title(" New Directory Name ")
                .padding(Padding::horizontal(1));

            let paragraph = Paragraph::new(input.as_str()).block(block);
            f.render_widget(paragraph, area);
            Some(area)
        }
        AppMode::Editor => None,
    }
}