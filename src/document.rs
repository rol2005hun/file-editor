use crate::error::{EditorError, Result};
use std::fs;
use std::path::Path;

pub struct Document {
    pub rows: Vec<String>,
}

impl TryFrom<&str> for Document {
    type Error = EditorError;

    fn try_from(content: &str) -> std::result::Result<Self, Self::Error> {
        let rows: Vec<String> = content.lines().map(|line: &str| line.to_string()).collect();
        Ok(Document { rows })
    }
}

impl Document {
    pub fn open<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let content: String = fs::read_to_string(filename)?;
        Document::try_from(content.as_str())
    }

    pub fn new() -> Self {
        Document {
            rows: vec![String::new()],
        }
    }

    pub fn insert(&mut self, x: usize, y: usize, c: char) {
        if y < self.rows.len() {
            let row: &mut String = &mut self.rows[y];
            if x <= row.len() {
                row.insert(x, c);
            }
        }
    }

    pub fn delete(&mut self, x: usize, y: usize) {
        if y < self.rows.len() {
            let row: &mut String = &mut self.rows[y];
            if x > 0 && x <= row.len() {
                row.remove(x - 1);
            }
        }
    }
}