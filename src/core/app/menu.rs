use crate::core::app::{App, AppMode};

impl App {
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
            0 => self.mode = AppMode::PromptFile(String::new()),
            1 => self.mode = AppMode::PromptDir(String::new()),
            2 => {
                let _ = self.document.save();
                self.mode = AppMode::Editor;
            }
            3 => self.mode = AppMode::Settings(0),
            4 => self.should_quit = true,
            _ => {}
        }
    }

    pub fn settings_up(&mut self) {
        if let AppMode::Settings(selection) = self.mode {
            self.mode = AppMode::Settings(if selection > 0 { selection - 1 } else { 3 });
        }
    }

    pub fn settings_down(&mut self) {
        if let AppMode::Settings(selection) = self.mode {
            self.mode = AppMode::Settings(if selection < 3 { selection + 1 } else { 0 });
        }
    }

    pub fn toggle_setting(&mut self) {
        if let AppMode::Settings(selection) = self.mode {
            match selection {
                0 => self.config.show_line_numbers = !self.config.show_line_numbers,
                1 => self.config.show_status_bar = !self.config.show_status_bar,
                2 => self.config.show_help_bar = !self.config.show_help_bar,
                3 => self.config.highlight_active_line = !self.config.highlight_active_line,
                _ => {}
            }
        }
    }
}
