mod app;
mod document;
mod error;
mod explorer;

use app::{App, AppFocus, AppMode};
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use document::Document;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Terminal,
};
use std::{error::Error, io};

fn main() -> std::result::Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout: io::Stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend: CrosstermBackend<io::Stdout> = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;

    let mut app: App = App::new();

    let res: io::Result<()> = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()>
where
    std::io::Error: From<B::Error>,
{
    loop {
        if app.should_quit {
            return Ok(());
        }

        terminal.draw(|f: &mut ratatui::Frame| {
            let vertical_chunks: std::rc::Rc<[Rect]> = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
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
                        .block(Block::default().borders(Borders::ALL).title(" Main Menu "));
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
                        let display_x: u16 = app.editor_area.x + 1 + app.cursor_x - app.scroll_x;
                        let display_y: u16 = app.editor_area.y + 1 + app.cursor_y - app.scroll_y;
                        f.set_cursor_position((display_x, display_y));
                    }
                }
            }
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        let mut mode_transition: Option<AppMode> = None;

                        match app.mode {
                            AppMode::PromptFile(ref mut input) => match key.code {
                                KeyCode::Esc => mode_transition = Some(AppMode::Editor),
                                KeyCode::Enter => {
                                    let text: String = input.clone();
                                    let _ = app.explorer.create_file(&text);
                                    mode_transition = Some(AppMode::Editor);
                                }
                                KeyCode::Char(c) => input.push(c),
                                KeyCode::Backspace => {
                                    input.pop();
                                }
                                _ => {}
                            },
                            AppMode::PromptDir(ref mut input) => match key.code {
                                KeyCode::Esc => mode_transition = Some(AppMode::Editor),
                                KeyCode::Enter => {
                                    let text: String = input.clone();
                                    let _ = app.explorer.create_dir(&text);
                                    mode_transition = Some(AppMode::Editor);
                                }
                                KeyCode::Char(c) => input.push(c),
                                KeyCode::Backspace => {
                                    input.pop();
                                }
                                _ => {}
                            },
                            AppMode::Editor => {
                                if key.code == KeyCode::Tab {
                                    app.focus = match app.focus {
                                        AppFocus::Explorer => AppFocus::Editor,
                                        AppFocus::Editor => AppFocus::Explorer,
                                    };
                                } else {
                                    match app.focus {
                                        AppFocus::Explorer => match key.code {
                                            KeyCode::Up => app.explorer.previous(),
                                            KeyCode::Down => app.explorer.next(),
                                            KeyCode::Enter => {
                                                if let Some(path) = app.explorer
                                                    .items
                                                    .get(app.explorer.selected)
                                                {
                                                    if path.is_dir() {
                                                        app.explorer.current_path = path.clone();
                                                        let _ = app.explorer.refresh();
                                                    } else if let Ok(doc) = Document::open(path) {
                                                        app.document = doc;
                                                        app.cursor_x = 0;
                                                        app.cursor_y = 0;
                                                        app.scroll_x = 0;
                                                        app.scroll_y = 0;
                                                        app.focus = AppFocus::Editor;
                                                    }
                                                }
                                            }
                                            KeyCode::Char('n') => {
                                                mode_transition =
                                                    Some(AppMode::PromptFile(String::new()));
                                            }
                                            KeyCode::Char('d') => {
                                                mode_transition =
                                                    Some(AppMode::PromptDir(String::new()));
                                            }
                                            _ => {}
                                        },
                                        AppFocus::Editor => match key.code {
                                            KeyCode::Esc => app.toggle_menu(),
                                            KeyCode::Char(c) => app.insert_char(c),
                                            KeyCode::Backspace => app.delete_char(),
                                            KeyCode::Enter => app.insert_newline(),
                                            KeyCode::Left => app.move_cursor(-1, 0),
                                            KeyCode::Right => app.move_cursor(1, 0),
                                            KeyCode::Up => app.move_cursor(0, -1),
                                            KeyCode::Down => app.move_cursor(0, 1),
                                            _ => {}
                                        },
                                    }
                                }
                            }
                            AppMode::Menu => match key.code {
                                KeyCode::Esc => app.toggle_menu(),
                                KeyCode::Up => app.menu_up(),
                                KeyCode::Down => app.menu_down(),
                                KeyCode::Enter => app.execute_menu_action(),
                                _ => {}
                            },
                        }

                        if let Some(new_mode) = mode_transition {
                            app.mode = new_mode;
                        }
                    }
                }
                Event::Mouse(mouse_event) => {
                    if mouse_event.kind == MouseEventKind::Down(event::MouseButton::Left) {
                        if let AppFocus::Editor = app.focus {
                            if mouse_event.column >= app.editor_area.x + 1
                                && mouse_event.column
                                    < app.editor_area.x + app.editor_area.width - 1
                                && mouse_event.row >= app.editor_area.y + 1
                                && mouse_event.row < app.editor_area.y + app.editor_area.height - 1
                            {
                                let click_x: u16 = mouse_event.column.saturating_sub(app.editor_area.x + 1) + app.scroll_x;
                                let click_y: u16 = mouse_event.row.saturating_sub(app.editor_area.y + 1) + app.scroll_y;
                                app.handle_click(click_x, click_y);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}