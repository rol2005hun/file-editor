use crate::core::document::Document;
use crate::core::explorer::Explorer;
use ratatui::layout::Rect;

pub enum AppMode {
    Editor,
    Menu,
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
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub scroll_x: u16,
    pub scroll_y: u16,
    pub editor_area: Rect,
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
            cursor_x: 0,
            cursor_y: 0,
            scroll_x: 0,
            scroll_y: 0,
            editor_area: Rect::default(),
            mode: AppMode::Editor,
            focus: AppFocus::Explorer,
            menu_items: vec![
                "New File".to_string(),
                "Save".to_string(),
                "Settings (Soon)".to_string(),
                "Exit".to_string(),
            ],
            menu_selection: 0,
            should_quit: false,
        }
    }

    pub fn toggle_menu(&mut self) {
        match self.mode {
            AppMode::Editor => self.mode = AppMode::Menu,
            AppMode::Menu => self.mode = AppMode::Editor,
            _ => self.mode = AppMode::Editor,
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
                self.scroll_x = 0;
                self.scroll_y = 0;
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
        self.cursor_x = x;
        self.cursor_y = y;

        if (self.cursor_y as usize) >= self.document.rows.len() {
            self.cursor_y = self.document.rows.len().saturating_sub(1) as u16;
        }

        let row_len: u16 = self.document.rows[self.cursor_y as usize].len() as u16;
        if self.cursor_x > row_len {
            self.cursor_x = row_len;
        }
        self.adjust_scroll();
    }

    pub fn paste(&mut self, text: &str) {
        for c in text.chars() {
            if c == '\n' {
                self.insert_newline();
            } else if c != '\r' {
                self.insert_char(c);
            }
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.document
            .insert(self.cursor_x as usize, self.cursor_y as usize, c);
        self.cursor_x += 1;
        self.adjust_scroll();
    }

    pub fn delete_char(&mut self) {
        if self.cursor_x > 0 {
            self.document
                .delete(self.cursor_x as usize, self.cursor_y as usize);
            self.cursor_x -= 1;
            self.adjust_scroll();
        } else if self.cursor_y > 0 {
            let current_row: String = self.document.rows.remove(self.cursor_y as usize);
            self.cursor_y -= 1;
            let prev_row: &mut String = &mut self.document.rows[self.cursor_y as usize];
            self.cursor_x = prev_row.len() as u16;
            prev_row.push_str(&current_row);
            self.adjust_scroll();
        }
    }

    pub fn insert_newline(&mut self) {
        self.document
            .insert_newline(self.cursor_x as usize, self.cursor_y as usize);
        self.cursor_x = 0;
        self.cursor_y += 1;
        self.adjust_scroll();
    }

    pub fn move_cursor(&mut self, dx: i16, dy: i16) {
        let new_x: i16 = self.cursor_x as i16 + dx;
        let new_y: i16 = self.cursor_y as i16 + dy;

        if new_y >= 0 && (new_y as usize) < self.document.rows.len() {
            self.cursor_y = new_y as u16;
        }

        let row_len: i16 = self.document.rows[self.cursor_y as usize].len() as i16;
        if new_x >= 0 && new_x <= row_len {
            self.cursor_x = new_x as u16;
        } else if new_x > row_len {
            self.cursor_x = row_len as u16;
        }

        self.adjust_scroll();
    }

    pub fn adjust_scroll(&mut self) {
        if self.cursor_y < self.scroll_y {
            self.scroll_y = self.cursor_y;
        } else if self.cursor_y >= self.scroll_y + self.editor_area.height.saturating_sub(2) {
            self.scroll_y = self.cursor_y - self.editor_area.height.saturating_sub(3);
        }

        if self.cursor_x < self.scroll_x {
            self.scroll_x = self.cursor_x;
        } else if self.cursor_x >= self.scroll_x + self.editor_area.width.saturating_sub(2) {
            self.scroll_x = self.cursor_x - self.editor_area.width.saturating_sub(3);
        }
    }
}