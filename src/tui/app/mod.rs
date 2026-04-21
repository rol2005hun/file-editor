pub mod cursor;
pub mod editor;
pub mod menu;

use crate::core::config::AppConfig;
use crate::core::document::Document;
use crate::core::explorer::Explorer;
use ratatui::layout::Rect;

pub enum AppMode {
    Editor,
    Menu,
    Settings(usize),
    PromptFile(String),
    PromptDir(String),
}

pub enum AppFocus {
    Explorer,
    Editor,
}

pub struct App {
    pub document: Document,
    pub explorer: Explorer,
    pub config: AppConfig,
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub scroll_x: u16,
    pub scroll_y: u16,
    pub selection_start: Option<(u16, u16)>,
    pub history: Vec<(Vec<String>, u16, u16)>,
    pub editor_area: Rect,
    pub explorer_area: Rect,
    pub mode: AppMode,
    pub focus: AppFocus,
    pub menu_items: Vec<String>,
    pub menu_selection: usize,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let explorer: Explorer = Explorer::new().unwrap_or_else(|_| Explorer {
            current_path: std::path::PathBuf::from("."),
            items: Vec::new(),
            selected: 0,
        });

        Self {
            document: Document::new(),
            explorer,
            config: AppConfig::new(),
            cursor_x: 0,
            cursor_y: 0,
            scroll_x: 0,
            scroll_y: 0,
            selection_start: None,
            history: Vec::new(),
            editor_area: Rect::default(),
            explorer_area: Rect::default(),
            mode: AppMode::Editor,
            focus: AppFocus::Explorer,
            menu_items: vec![
                String::from("New File"),
                String::from("New Directory"),
                String::from("Save"),
                String::from("Settings"),
                String::from("Exit"),
            ],
            menu_selection: 0,
            should_quit: false,
        }
    }
}