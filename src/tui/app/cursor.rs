use crate::tui::app::App;

impl App {
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