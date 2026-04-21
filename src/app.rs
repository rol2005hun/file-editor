use crate::document::Document;
use ratatui::layout::Rect;

pub struct App {
    pub document: Document,
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub editor_area: Rect,
}

impl App {
    pub fn new(document: Document) -> Self {
        Self {
            document,
            cursor_x: 0,
            cursor_y: 0,
            editor_area: Rect::default(),
        }
    }

    pub fn handle_click(&mut self, x: u16, y: u16) {
        if x >= self.editor_area.x + 1
            && x < self.editor_area.x + self.editor_area.width - 1
            && y >= self.editor_area.y + 1
            && y < self.editor_area.y + self.editor_area.height - 1
        {
            self.cursor_x = x - (self.editor_area.x + 1);
            self.cursor_y = y - (self.editor_area.y + 1);
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