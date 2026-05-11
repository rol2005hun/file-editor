use crate::core::error::{EditorError, Result};
use std::fs;
use std::path::Path;

pub struct Document {
    pub rows: Vec<String>,
    pub path: Option<String>,
}

impl TryFrom<String> for Document {
    type Error = EditorError;

    fn try_from(content: String) -> std::result::Result<Self, Self::Error> {
        let mut rows: Vec<String> = content.lines().map(|line| line.to_string()).collect();
        if rows.is_empty() {
            rows.push(String::new());
        }
        Ok(Document { rows, path: None })
    }
}

impl Document {
    pub fn new() -> Self {
        Document {
            rows: vec![String::new()],
            path: None,
        }
    }

    pub fn open<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let path_str = filename.as_ref().to_string_lossy().into_owned();
        let content = fs::read_to_string(&filename)?;
        let mut doc = Document::try_from(content)?;
        doc.path = Some(path_str);
        Ok(doc)
    }

    pub fn save(&self) -> Result<()> {
        if let Some(path) = &self.path {
            let content = self.rows.join("\n");
            fs::write(path, content)?;
        }
        Ok(())
    }

    pub fn insert(&mut self, x: usize, y: usize, c: char) {
        if y < self.rows.len() {
            let row = &mut self.rows[y];
            if x <= row.len() {
                row.insert(x, c);
            } else {
                row.push(c);
            }
        }
    }

    pub fn delete(&mut self, x: usize, y: usize) {
        if y < self.rows.len() {
            let row = &mut self.rows[y];
            if x > 0 && x <= row.len() {
                row.remove(x - 1);
            }
        }
    }

    pub fn insert_newline(&mut self, x: usize, y: usize) {
        if y < self.rows.len() {
            let row = &mut self.rows[y];
            let new_row = if x < row.len() {
                row.split_off(x)
            } else {
                String::new()
            };
            self.rows.insert(y + 1, new_row);
        }
    }
}
