use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

use super::constants::Constants;

/// Analyze a file to count total lines and comment lines based on file extension.
/// Returns the file name, total line count, and comment line count.
pub fn analyze_file(file_path: &Path) -> Result<(String, usize, usize)> {
    // Extract file extension and determine language
    let extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    let language = Constants::get_language(&extension);

    // If language is unsupported or has no comment markers, return with zero counts
    let comment_prefixes = match language {
        Some(lang) => Constants::get_comment_markers(lang).unwrap_or(&[]),
        None => &[],
    };

    // Open and read the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut line_count = 0;
    let mut comment_count = 0;

    for line in reader.lines() {
        let line = line?;
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            continue;
        }

        line_count += 1;

        // Check if the line starts with any comment prefix
        if comment_prefixes
            .iter()
            .any(|&prefix| trimmed_line.starts_with(prefix))
        {
            comment_count += 1;
        }
    }

    let file_name = file_path
        .file_name() // Extract the file name
        .and_then(|name| name.to_str()) // Convert OsStr to &str
        .unwrap_or("Unknown file");
    // Return the file name (as a string), total lines, and comment lines
    Ok((file_name.to_string(), line_count, comment_count))
}
