mod core;
mod gui;
mod tui;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::{Backend, CrosstermBackend}, Terminal};
use std::{env, error::Error, io::{self, Write}, path::PathBuf, sync::{Arc, Mutex}};
use tui::{app::{App, AppFocus}, events::handle_events, ui::render};

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    let mut is_gui: bool = false;
    let mut target_path: Option<String> = None;
    let mut ask_mode: bool = true;

    for arg in args.iter().skip(1) {
        if arg == "--gui" {
            is_gui = true;
            ask_mode = false;
        } else if arg == "--tui" {
            is_gui = false;
            ask_mode = false;
        } else if !arg.starts_with("--") {
            target_path = Some(arg.clone());
        }
    }

    if ask_mode {
        print!("Which mode would you like to run? [1] TUI, [2] GUI: ");
        io::stdout().flush()?;
        let mut input: String = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim() == "2" {
            is_gui = true;
        }
    }

    let mut initial_app: App = App::new();

    if let Some(path_str) = target_path {
        let p: PathBuf = PathBuf::from(&path_str);
        if p.is_dir() {
            initial_app.explorer.current_path = p;
            let _ = initial_app.explorer.refresh();
        } else if p.is_file() {
            if let Some(parent) = p.parent() {
                initial_app.explorer.current_path = parent.to_path_buf();
                let _ = initial_app.explorer.refresh();
            }
            if let Ok(doc) = crate::core::document::Document::open(&p) {
                initial_app.document = doc;
                initial_app.focus = AppFocus::Editor;
            }
        }
    }

    // Arc and Mutex for shared state between TUI and GUI
    let app_state: Arc<Mutex<App>> = Arc::new(Mutex::new(initial_app));

    if is_gui {
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