use crate::document::Document;
use ratatui::layout::Rect;

pub enum AppMode {
    Editor,
    Menu,
}

pub struct App {
    pub document: Document,
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub editor_area: Rect,
    pub mode: AppMode,
    pub menu_items: Vec<String>,
    pub menu_selection: usize,
    pub should_quit: bool,
}

impl App {
    pub fn new(document: Document) -> Self {
        Self {
            document,
            cursor_x: 0,
            cursor_y: 0,
            editor_area: Rect::default(),
            mode: AppMode::Editor,
            menu_items: vec![
                "Uj fajl".to_string(),
                "Mentes".to_string(),
                "Beallitasok (Hamarosan)".to_string(),
                "Kilepes".to_string(),
            ],
            menu_selection: 0,
            should_quit: false,
        }
    }

    pub fn toggle_menu(&mut self) {
        match self.mode {
            AppMode::Editor => self.mode = AppMode::Menu,
            AppMode::Menu => self.mode = AppMode::Editor,
        }
    }

    pub fn menu_up(&mut self) {
        if self.menu_selection > 0 {
            self.menu_selection -= 1;
        } else {
            self.menu_selection = self.menu_items.len() - 1;
        }
    }

    pub fn menu_down(&mut self) {
        if self.menu_selection < self.menu_items.len() - 1 {
            self.menu_selection += 1;
        } else {
            self.menu_selection = 0;
        }
    }

    pub fn execute_menu_action(&mut self) {
        match self.menu_selection {
            0 => {
                self.document = Document::new();
                self.cursor_x = 0;
                self.cursor_y = 0;
                self.mode = AppMode::Editor;
            }
            1 => {
                let _ = self.document.save();
                self.mode = AppMode::Editor;
            }
            2 => {
                self.mode = AppMode::Editor;
            }
            3 => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn handle_click(&mut self, x: u16, y: u16) {
        if let AppMode::Editor = self.mode {
            if x >= self.editor_area.x + 1
                && x < self.editor_area.x + self.editor_area.width - 1
                && y >= self.editor_area.y + 1
                && y < self.editor_area.y + self.editor_area.height - 1
            {
                self.cursor_x = x - (self.editor_area.x + 1);
                self.cursor_y = y - (self.editor_area.y + 1);
            }
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.document
            .insert(self.cursor_x as usize, self.cursor_y as usize, c);
        self.cursor_x += 1;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_x > 0 {
            self.document
                .delete(self.cursor_x as usize, self.cursor_y as usize);
            self.cursor_x -= 1;
        }
    }

    pub fn move_cursor(&mut self, dx: i16, dy: i16) {
        let new_x: i16 = self.cursor_x as i16 + dx;
        let new_y: i16 = self.cursor_y as i16 + dy;
        if new_x >= 0 {
            self.cursor_x = new_x as u16;
        }
        if new_y >= 0 {
            self.cursor_y = new_y as u16;
        }
    }
}