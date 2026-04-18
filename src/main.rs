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
use monitor::{cpu::CpuMonitor, ram::RamMonitor, ResourceCollector};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut cpu_monitor = CpuMonitor::new();
    let mut ram_monitor = RamMonitor::new();

    let res = run_app(&mut terminal, &mut cpu_monitor, &mut ram_monitor);

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
    cpu_monitor: &mut CpuMonitor,
    ram_monitor: &mut RamMonitor,
) -> io::Result<()> {
    loop {
        let cpu_stats = cpu_monitor.fetch_stats().unwrap_or_else(|_| crate::model::ResourceStats {
            label: "Error".to_string(),
            value: 0.0,
        });

        let ram_stats = ram_monitor.fetch_stats().unwrap_or_else(|_| crate::model::ResourceStats {
            label: "Error".to_string(),
            value: 0.0,
        });

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let cpu_text = format!("{}: {}%", cpu_stats.label, cpu_stats.value);
            let cpu_block = Paragraph::new(cpu_text)
                .block(Block::default().title("CPU").borders(Borders::ALL));
            f.render_widget(cpu_block, chunks[0]);

            let ram_text = format!("{}: {}%", ram_stats.label, ram_stats.value);
            let ram_block = Paragraph::new(ram_text)
                .block(Block::default().title("RAM").borders(Borders::ALL));
            f.render_widget(ram_block, chunks[1]);
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