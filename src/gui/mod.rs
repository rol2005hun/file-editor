pub mod commands;

use crate::tui::app::App;
use std::sync::{Arc, Mutex};

pub fn run_gui(state: Arc<Mutex<App>>) {
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::get_explorer_items,
            commands::open_path,
            commands::read_file,
            commands::save_file,
            commands::create_file,
            commands::create_dir,
            commands::exit_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}