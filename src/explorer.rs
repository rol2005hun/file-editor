use crate::error::Result;
use std::fs;
use std::path::PathBuf;

pub struct Explorer {
    pub current_path: PathBuf,
    pub items: Vec<PathBuf>,
    pub selected: usize,
}

impl Explorer {
    pub fn new() -> Result<Self> {
        let current_path: PathBuf = std::env::current_dir()?;
        let mut exp: Explorer = Self {
            current_path,
            items: Vec::new(),
            selected: 0,
        };
        exp.refresh()?;
        Ok(exp)
    }

    pub fn refresh(&mut self) -> Result<()> {
        self.items.clear();
        if let Some(parent) = self.current_path.parent() {
            self.items.push(parent.to_path_buf());
        }
        let mut dirs: Vec<PathBuf> = Vec::new();
        let mut files: Vec<PathBuf> = Vec::new();

        if let Ok(entries) = fs::read_dir(&self.current_path) {
            for entry in entries.flatten() {
                let path: PathBuf = entry.path();
                if path.is_dir() {
                    dirs.push(path);
                } else {
                    files.push(path);
                }
            }
        }
        dirs.sort();
        files.sort();
        self.items.extend(dirs);
        self.items.extend(files);

        if self.selected >= self.items.len() {
            self.selected = self.items.len().saturating_sub(1);
        }
        Ok(())
    }

    pub fn next(&mut self) {
        if self.selected < self.items.len().saturating_sub(1) {
            self.selected += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn create_file(&mut self, name: &str) -> Result<()> {
        let path: PathBuf = self.current_path.join(name);
        fs::write(&path, "")?;
        self.refresh()?;
        Ok(())
    }

    pub fn create_dir(&mut self, name: &str) -> Result<()> {
        let path: PathBuf = self.current_path.join(name);
        fs::create_dir(&path)?;
        self.refresh()?;
        Ok(())
    }
}