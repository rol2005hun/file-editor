mod app;
mod document;
mod error;

use app::{App, AppMode};
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

    let document: Document = Document::open("src/main.rs").unwrap_or_else(|_| Document::new());
    let mut app: App = App::new(document);

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

            let sidebar: Paragraph = Paragraph::new("Fajlkezelo")
                .block(Block::default().title("Explorer").borders(Borders::ALL));
            f.render_widget(sidebar, horizontal_chunks[0]);

            let content: String = app.document.rows.join("\n");
            let text: Text = Text::raw(content);
            let editor: Paragraph = Paragraph::new(text)
                .block(Block::default().title("Editor").borders(Borders::ALL));
            f.render_widget(editor, app.editor_area);

            let status_mode: &str = match app.mode {
                AppMode::Editor => "SZERKESZTO",
                AppMode::Menu => "MENU",
            };

            let path_display: String = app
                .document
                .path
                .clone()
                .unwrap_or_else(|| "Nevtelen".to_string());
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

            if let AppMode::Menu = app.mode {
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
                    .block(Block::default().borders(Borders::ALL).title("Fomenuk"));
                f.render_widget(list, area);
            } else {
                f.set_cursor_position((
                    app.editor_area.x + 1 + app.cursor_x,
                    app.editor_area.y + 1 + app.cursor_y,
                ));
            }
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match app.mode {
                            AppMode::Editor => match key.code {
                                KeyCode::Esc => app.toggle_menu(),
                                KeyCode::Char(c) => app.insert_char(c),
                                KeyCode::Backspace => app.delete_char(),
                                KeyCode::Left => app.move_cursor(-1, 0),
                                KeyCode::Right => app.move_cursor(1, 0),
                                KeyCode::Up => app.move_cursor(0, -1),
                                KeyCode::Down => app.move_cursor(0, 1),
                                _ => {}
                            },
                            AppMode::Menu => match key.code {
                                KeyCode::Esc => app.toggle_menu(),
                                KeyCode::Up => app.menu_up(),
                                KeyCode::Down => app.menu_down(),
                                KeyCode::Enter => app.execute_menu_action(),
                                _ => {}
                            },
                        }
                    }
                }
                Event::Mouse(mouse_event) => {
                    if mouse_event.kind == MouseEventKind::Down(event::MouseButton::Left) {
                        app.handle_click(mouse_event.column, mouse_event.row);
                    }
                }
                _ => {}
            }
        }
    }
}