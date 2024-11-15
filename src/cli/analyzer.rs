use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Result},
    path::Path,
};

use super::stats::Stats;
use super::constants::Constants;

/// Analyze a file to count total lines and comment lines based on file extension.
/// Returns the file name, total line count, and comment line count.
pub fn analyze_file(file_path: &Path, stats: &mut Stats) -> Result<()> {
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

        // update the overall stats
        stats.total_files += 1;
        stats.total_lines = line_count;
        stats.comment_lines = comment_count;
        stats.code_lines = line_count - comment_count;
    }

    // Return the file name (as a string), total lines, and comment lines

    // Get the language and update language stats
    if let Some(lang) = language {
        *stats.language_stats.entry(lang.to_string()).or_insert(0) += 1;
    }

    Ok(())
}

/// Analyze a folder and its contents, including subdirectories.
/// Skips excluded files/folders and analyzes supported file types.
pub fn analyze_folder(folder_path: &Path, stats: &mut Stats) -> Result<()> {
    // Validate if the path is a valid directory using the utility function
    if !folder_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Not a valid directory",
        ));
    }

    // Iterate over the directory contents
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let entry_path = entry.path();

        // Skip excluded files and directories
        if Constants::EXCLUDED_FILES_AND_FOLDERS
            .iter()
            .any(|&exclude| {
                entry_path
                    .file_name()
                    .map(|name| name.to_string_lossy().contains(exclude))
                    .unwrap_or(false)
            })
        {
            continue;
        }

        if entry_path.is_dir() {
            // Recursively analyze subdirectories
            analyze_folder(&entry_path, stats)?;
        } else if entry_path.is_file() {
            // Only analyze supported files
            if let Some(ext) = entry_path.extension() {
                if let Some(_) = Constants::get_language(&ext.to_str().unwrap_or_default()) {
                    if !Constants::is_binary_extension(&ext.to_str().unwrap_or_default()) {
                        // Analyze the valid file
                        analyze_file(&entry_path, stats)?;
                    }
                }
            }
        }
    }

    Ok(())
}
