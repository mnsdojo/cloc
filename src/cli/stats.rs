use std::collections::HashMap;

#[derive(Default)]
pub struct Stats {
    pub total_files: usize,
    pub code_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
    pub total_lines: usize,
    pub language_stats: HashMap<String, LanguageStats>, // Storing detailed stats per language
}

#[derive(Default)]
pub struct LanguageStats {
    pub total_lines: usize,
    pub code_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
    pub file_count: usize, // Number of files for that language
}
