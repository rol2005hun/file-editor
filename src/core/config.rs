pub struct AppConfig {
    pub show_line_numbers: bool,
    pub show_help_bar: bool,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            show_line_numbers: true,
            show_help_bar: true,
        }
    }
}