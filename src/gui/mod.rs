use crate::tui::app::App;
use std::sync::{Arc, Mutex};
use tauri::State;

#[tauri::command]
pub fn get_document_content(state: State<'_, Arc<Mutex<App>>>) -> String {
    let app: std::sync::MutexGuard<'_, App> = state.lock().unwrap();
    app.document.rows.join("\n")
}

pub fn run_gui(state: Arc<Mutex<App>>) {
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![get_document_content])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}