mod core;
mod gui;
mod tui;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::{Backend, CrosstermBackend}, Terminal};
use std::{env, error::Error, io, sync::{Arc, Mutex}};
use tui::{app::App, events::handle_events, ui::render};

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let app_state: Arc<Mutex<App>> = Arc::new(Mutex::new(App::new()));

    if args.contains(&String::from("--gui")) {
        gui::run_gui(app_state);
        return Ok(());
    }

    enable_raw_mode()?;
    let mut stdout: io::Stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend: CrosstermBackend<io::Stdout> = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;

    let res: io::Result<()> = run_app(&mut terminal, app_state);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app_state: Arc<Mutex<App>>) -> io::Result<()>
where
    std::io::Error: From<B::Error>,
{
    loop {
        let mut app: std::sync::MutexGuard<'_, App> = app_state.lock().unwrap();
        
        if app.should_quit {
            return Ok(());
        }

        terminal.draw(|f| render(f, &mut *app))?;
        handle_events(&mut *app)?;
    }
}