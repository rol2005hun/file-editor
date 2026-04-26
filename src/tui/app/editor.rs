use crate::tui::app::App;

impl App {
    pub fn prune_history(&mut self) {
        while let Some(_) = self.history.get(50) {
            self.history.remove(0);
        }
    }

    pub fn find_line(&self, pattern: &str) -> Vec<usize> {
        self.document.rows.iter()
            .enumerate()
            .filter(|(_, row)| row.contains(pattern))
            .map(|(i, _)| i)
            .collect()
    }

    pub fn save_state(&mut self) {
        let state: (Vec<String>, u16, u16) = (self.document.rows.clone(), self.cursor_x, self.cursor_y);
        self.history.push(state);
        self.prune_history();
    }

    pub fn undo(&mut self) {
        if let Some((rows, cx, cy)) = self.history.pop() {
            self.document.rows = rows;
            self.cursor_x = cx;
            self.cursor_y = cy;
            self.adjust_scroll();
        }
    }

    pub fn get_selection_bounds(&self) -> Option<((usize, usize), (usize, usize))> {
        if let Some((start_x, start_y)) = self.selection_start {
            let start: (usize, usize) = (start_x as usize, start_y as usize);
            let end: (usize, usize) = (self.cursor_x as usize, self.cursor_y as usize);
            
            if start.1 < end.1 || (start.1 == end.1 && start.0 <= end.0) {
                Some((start, end))
            } else {
                Some((end, start))
            }
        } else {
            None
        }
    }

    pub fn get_selected_text(&self) -> Option<String> {
        let (start, end) = self.get_selection_bounds()?;
        
        if start.1 == end.1 {
            let row: &String = self.document.rows.get(start.1)?;
            let chars: Vec<char> = row.chars().collect();
            let end_idx: usize = std::cmp::min(end.0, chars.len());
            let start_idx: usize = std::cmp::min(start.0, chars.len());
            let text: String = chars[start_idx..end_idx].iter().collect();
            Some(text)
        } else {
            let mut text: String = String::new();
            for y in start.1..=end.1 {
                if let Some(row) = self.document.rows.get(y) {
                    let chars: Vec<char> = row.chars().collect();
                    if y == start.1 {
                        let start_idx: usize = std::cmp::min(start.0, chars.len());
                        let part: String = chars[start_idx..].iter().collect();
                        text.push_str(&part);
                        text.push('\n');
                    } else if y == end.1 {
                        let end_idx: usize = std::cmp::min(end.0, chars.len());
                        let part: String = chars[..end_idx].iter().collect();
                        text.push_str(&part);
                    } else {
                        text.push_str(row);
                        text.push('\n');
                    }
                }
            }
            Some(text)
        }
    }

    pub fn paste(&mut self, text: &str) {
        self.save_state();
        let lines: Vec<&str> = text.split('\n').collect();
        if lines.is_empty() {
            return;
        }

        let cy: usize = self.cursor_y as usize;
        let cx: usize = self.cursor_x as usize;
        let cx_safe: usize = std::cmp::min(cx, self.document.rows[cy].len());

        if lines.len() == 1 {
            let clean_line: &str = lines[0].trim_end_matches('\r');
            self.document.rows[cy].insert_str(cx_safe, clean_line);
            self.cursor_x += clean_line.len() as u16;
        } else {
            let mut current_row: String = self.document.rows.remove(cy);
            let remainder: String = current_row.drain(cx_safe..).collect();
            
            let first_line: &str = lines[0].trim_end_matches('\r');
            current_row.push_str(first_line);
            self.document.rows.insert(cy, current_row);

            for (i, line) in lines[1..lines.len() - 1].iter().enumerate() {
                let clean_line: &str = line.trim_end_matches('\r');
                self.document.rows.insert(cy + 1 + i, clean_line.to_string());
            }

            let last_line: &str = lines.last().unwrap().trim_end_matches('\r');
            let mut final_row: String = last_line.to_string();
            let final_len: usize = final_row.len();
            final_row.push_str(&remainder);
            self.document.rows.insert(cy + lines.len() - 1, final_row);

            self.cursor_y += (lines.len() - 1) as u16;
            self.cursor_x = final_len as u16;
        }
        self.adjust_scroll();
    }

    pub fn insert_char(&mut self, c: char) {
        if c == ' ' {
            self.save_state();
        }
        self.document
            .insert(self.cursor_x as usize, self.cursor_y as usize, c);
        self.cursor_x += 1;
        self.adjust_scroll();
    }

    pub fn delete_char(&mut self) {
        self.save_state();
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
        self.save_state();
        self.document
            .insert_newline(self.cursor_x as usize, self.cursor_y as usize);
        self.cursor_x = 0;
        self.cursor_y += 1;
        self.adjust_scroll();
    }
}