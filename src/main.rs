mod core;
mod gui;
mod tui;

use core::app::{App, AppFocus};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::Rect,
    Terminal,
};
use std::{
    env,
    error::Error,
    io::{self, Write},
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tui::events::handle_events;
use tui::ui::render;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        let _ = disable_raw_mode();
        default_panic(info);
    }));

    let args: Vec<String> = env::args().collect();
    let mut is_gui = false;
    let mut target_path: Option<String> = None;
    let mut ask_mode = true;

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
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim() == "2" {
            is_gui = true;
        }
    }

    let mut initial_app = App::new();
    if let Some(path_str) = target_path {
        let p = PathBuf::from(&path_str);
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

    // Arc and Mutex to allow shared mutable access to the app state across threads
    let app_state = Arc::new(Mutex::new(initial_app));

    if is_gui {
        gui::run_gui(app_state);
        return Ok(());
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, Arc::clone(&app_state));

    let _ = execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    );
    let _ = terminal.show_cursor();
    let _ = disable_raw_mode();

    if let Err(err) = res {
        eprintln!("TUI Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app_state: Arc<Mutex<App>>) -> io::Result<()>
where
    std::io::Error: From<B::Error>,
{
    loop {
        let mut explorer_area = Rect::default();
        let mut editor_area = Rect::default();
        let mut popup_area = None;

        {
            let mut app = app_state.lock().unwrap();
            if app.should_quit {
                return Ok(());
            }

            terminal.draw(|f| {
                let (ex, ed, po) = render(f, &mut *app);
                explorer_area = ex;
                editor_area = ed;
                popup_area = po;
            })?;
        }

        let mut app_to_handle = app_state.lock().unwrap();
        handle_events(&mut *app_to_handle, explorer_area, editor_area, popup_area)?;
    }
}
