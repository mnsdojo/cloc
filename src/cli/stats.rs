use std::collections::HashMap;

pub struct Stats {
    pub total_files: usize,
    pub code_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
    pub total_lines: usize,
    pub language_stats: HashMap<String, LanguageStats>, // Storing detailed stats per language
}

pub struct LanguageStats {
    pub total_lines: usize,
    pub code_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
    pub file_count: usize, // Number of files for that language
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

impl Default for LanguageStats {
    fn default() -> Self {
        LanguageStats {
            total_lines: 0,
            code_lines: 0,
            comment_lines: 0,
            blank_lines: 0,
            file_count: 0,
        }
    }
}

