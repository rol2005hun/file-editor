mod error;
mod model;
mod monitor;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{error::Error, io};
use monitor::{cpu::CpuMonitor, ResourceCollector};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut cpu_monitor = CpuMonitor::new();

    let res = run_app(&mut terminal, &mut cpu_monitor);

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
    monitor: &mut CpuMonitor,
) -> io::Result<()> {
    loop {
        let stats = monitor.fetch_stats().unwrap_or_else(|_| crate::model::ResourceStats {
            label: "Error".to_string(),
            value: 0.0,
        });

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());

            let display_text = format!("{}: {}%", stats.label, stats.value);
            let block = Paragraph::new(display_text)
                .block(Block::default().title("Resource Monitor").borders(Borders::ALL));
            f.render_widget(block, chunks[0]);
        })?;

        if event::poll(std::time::Duration::from_millis(1000))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
    }
}