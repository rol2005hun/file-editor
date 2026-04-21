pub struct AppConfig {
    pub show_line_numbers: bool,
    pub show_status_bar: bool,
    pub show_help_bar: bool,
    pub highlight_active_line: bool,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            show_line_numbers: true,
            show_status_bar: true,
            show_help_bar: true,
            highlight_active_line: true,
        }
    }
}