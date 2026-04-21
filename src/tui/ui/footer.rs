use crate::tui::app::{App, AppFocus, AppMode};
use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_status(f: &mut Frame, app: &App, area: Rect) {
    let status_mode: &str = match app.mode {
        AppMode::Editor => "EDITOR",
        AppMode::Menu => "MENU",
        AppMode::Settings(_) => "SETTINGS",
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
    f.render_widget(status_bar, area);
}

pub fn render_help(f: &mut Frame, app: &App, area: Rect) {
    let help_text: String = match app.mode {
        AppMode::Editor => match app.focus {
            AppFocus::Explorer => "Tab: Focus Editor | Enter: Open | N: New File | D: New Dir".to_string(),
            AppFocus::Editor => "Ctrl+S: Save | Ctrl+C: Copy Line | Ctrl+V: Paste | Tab: Focus Explorer".to_string(),
        },
        AppMode::Menu => "Enter: Select | Esc: Back | Up/Down: Navigate".to_string(),
        AppMode::Settings(_) => "Enter: Toggle | Esc: Back | Up/Down: Navigate".to_string(),
        AppMode::PromptFile(_) | AppMode::PromptDir(_) => "Enter: Submit | Esc: Cancel".to_string(),
    };

    let help_bar: Paragraph = Paragraph::new(help_text)
        .block(Block::default().title(" Shortcuts / Tutorial ").borders(Borders::ALL));
    f.render_widget(help_bar, area);
}