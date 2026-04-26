use crate::tui::app::App;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, State};

#[derive(Serialize)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
}

#[tauri::command]
pub fn get_explorer_items(state: State<'_, Arc<Mutex<App>>>) -> Vec<FileNode> {
    let app: std::sync::MutexGuard<'_, App> = state.lock().unwrap();
    let mut nodes: Vec<FileNode> = Vec::new();

    for path in &app.explorer.items {
        let is_parent: bool = Some(path.as_path()) == app.explorer.current_path.parent();
        let name: String = if is_parent {
            String::from("..")
        } else {
            path.file_name().unwrap_or_default().to_string_lossy().into_owned()
        };

        nodes.push(FileNode {
            name,
            path: path.to_string_lossy().into_owned(),
            is_dir: path.is_dir(),
        });
    }
    nodes
}

#[tauri::command]
pub fn open_path(state: State<'_, Arc<Mutex<App>>>, path: String) -> Result<(), String> {
    let mut app: std::sync::MutexGuard<'_, App> = state.lock().unwrap();
    let p: PathBuf = PathBuf::from(path);
    app.explorer.current_path = p;
    app.explorer.refresh().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_file(path: String, content: String) -> Result<(), String> {
    fs::write(path, content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_file(state: State<'_, Arc<Mutex<App>>>, name: String) -> Result<(), String> {
    let mut app: std::sync::MutexGuard<'_, App> = state.lock().unwrap();
    app.explorer.create_file(&name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_dir(state: State<'_, Arc<Mutex<App>>>, name: String) -> Result<(), String> {
    let mut app: std::sync::MutexGuard<'_, App> = state.lock().unwrap();
    app.explorer.create_dir(&name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn exit_app(app_handle: AppHandle) {
    app_handle.exit(0);
}

#[tauri::command]
pub fn search_in_file(state: tauri::State<'_, std::sync::Arc<std::sync::Mutex<App>>>, pattern: String) -> Vec<usize> {
    let app = state.lock().unwrap();
    app.find_line(&pattern)
}