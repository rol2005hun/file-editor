mod app;
mod document;
mod error;

use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use document::Document;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    text::Text,
    widgets::{Block, Borders, Paragraph},
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()>
where
    std::io::Error: From<B::Error>,
{
    loop {
        terminal.draw(|f: &mut ratatui::Frame| {
            let vertical_chunks: std::rc::Rc<[ratatui::layout::Rect]> = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(f.area());

            let horizontal_chunks: std::rc::Rc<[ratatui::layout::Rect]> = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(vertical_chunks[0]);

            app.editor_area = horizontal_chunks[1];

            let sidebar: Paragraph = Paragraph::new("src/\n  main.rs\n  app.rs\n  document.rs\n  error.rs")
                .block(Block::default().title("Explorer").borders(Borders::ALL));
            f.render_widget(sidebar, horizontal_chunks[0]);

            let content: String = app.document.rows.join("\n");
            let text: Text = Text::raw(content);
            let editor: Paragraph = Paragraph::new(text)
                .block(Block::default().title("Editor").borders(Borders::ALL));
            f.render_widget(editor, app.editor_area);

            let status_text: String = format!(
                "NORMAL | src/main.rs | Ln {}, Col {}",
                app.cursor_y + 1,
                app.cursor_x + 1
            );
            let status_bar: Paragraph = Paragraph::new(status_text)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(status_bar, vertical_chunks[1]);

            f.set_cursor(
                app.editor_area.x + 1 + app.cursor_x,
                app.editor_area.y + 1 + app.cursor_y,
            );
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Char(c) => app.insert_char(c),
                    KeyCode::Backspace => app.delete_char(),
                    KeyCode::Left => app.move_cursor(-1, 0),
                    KeyCode::Right => app.move_cursor(1, 0),
                    KeyCode::Up => app.move_cursor(0, -1),
                    KeyCode::Down => app.move_cursor(0, 1),
                    _ => {}
                },
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