use crate::tui::app::App;

impl App {
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
}