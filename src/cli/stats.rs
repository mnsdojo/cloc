use std::collections::HashMap;

pub struct Stats {
    pub total_files: usize,
    pub code_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
    pub total_lines: usize,
    pub language_stats: HashMap<String, usize>,
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            total_files: 0,
            code_lines: 0,
            comment_lines: 0,
            blank_lines: 0,
            total_lines: 0,
            language_stats: HashMap::new(),
        }
    }
}
