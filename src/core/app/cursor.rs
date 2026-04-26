use crate::core::app::App;

impl App {
    pub fn handle_click(&mut self, click_x: u16, click_y: u16) {
        let offset_x = if self.config.show_line_numbers { 5 } else { 0 };
        
        self.cursor_x = click_x.saturating_sub(offset_x);
        self.cursor_y = click_y;

        if (self.cursor_y as usize) >= self.document.rows.len() {
            self.cursor_y = self.document.rows.len().saturating_sub(1) as u16;
        }

        let row_len = self.document.rows[self.cursor_y as usize].len() as u16;
        if self.cursor_x > row_len {
            self.cursor_x = row_len;
        }
    }

    pub fn move_cursor(&mut self, dx: i16, dy: i16) {
        self.selection_start = None; 

        let mut new_y = self.cursor_y as i16 + dy;
        if new_y < 0 {
            new_y = 0;
        } else if (new_y as usize) >= self.document.rows.len() {
            new_y = self.document.rows.len().saturating_sub(1) as i16;
        }
        self.cursor_y = new_y as u16;

        let row_len = self.document.rows[self.cursor_y as usize].len() as i16;
        let mut new_x = self.cursor_x as i16 + dx;
        
        if new_x < 0 {
            new_x = 0;
        } else if new_x > row_len {
            new_x = row_len;
        }
        self.cursor_x = new_x as u16;
    }
}