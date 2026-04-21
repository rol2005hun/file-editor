mod document;
mod error;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
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

    let mut document: Document = Document::open("Cargo.toml").unwrap_or_else(|_| Document::new());

    let res: io::Result<()> = run_app(&mut terminal, &mut document);

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

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    document: &mut Document,
) -> io::Result<()>
where
    std::io::Error: From<B::Error>,
{
    loop {
        terminal.draw(|f: &mut ratatui::Frame| {
            let chunks: std::rc::Rc<[ratatui::layout::Rect]> = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.area());

            let content: String = document.rows.join("\n");
            let text: Text = Text::raw(content);

            let block: Paragraph = Paragraph::new(text)
                .block(Block::default().title("TUI Editor").borders(Borders::ALL));
            
            f.render_widget(block, chunks[0]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Esc = key.code {
                    return Ok(());
                }
            }
        }
    }
}