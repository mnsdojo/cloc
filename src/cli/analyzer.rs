use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::Path,
};

use super::constants::Constants;
use super::stats::Stats;

pub fn analyze_file(file_path: &Path, stats: &mut Stats) -> io::Result<()> {
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
    let mut code_count = 0;
    let mut comment_count = 0;
    let mut blank_count = 0;

    for line in reader.lines() {
        let line = line?;
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            blank_count += 1;
            stats.blank_lines += 1;
            continue;
        }

        line_count += 1;
        stats.total_lines += 1;

        // Check if the line starts with any comment prefix
        if comment_prefixes
            .iter()
            .any(|&prefix| trimmed_line.starts_with(prefix))
        {
            comment_count += 1;
            stats.comment_lines += 1;
        } else {
            code_count += 1;
            stats.code_lines += 1;
        }
    }

    // Update the overall stats
    stats.total_files += 1;

    // Update language-specific statistics
    if let Some(lang) = language {
        let lang_stats = stats.language_stats.entry(lang.to_string()).or_default();

        lang_stats.total_lines += line_count;
        lang_stats.code_lines += code_count;
        lang_stats.comment_lines += comment_count;
        lang_stats.blank_lines += blank_count;
        lang_stats.file_count += 1;
    }

    Ok(())
}

/// Analyze a folder and its contents, including subdirectories.
/// Skips excluded files/folders and analyzes supported file types.
pub fn analyze_folder(folder_path: &Path, stats: &mut Stats) -> io::Result<()> {
    // Validate if the path is a valid directory
    if !folder_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Not a valid directory",
        ));
    }
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if let Some(name) = entry_path.file_name() {
            if Constants::EXCLUDED_FILES_AND_FOLDERS
                .iter()
                .any(|&exclude| name.to_string_lossy().contains(exclude))
            {
                continue;
            }
        }

        if entry_path.is_dir() {
            analyze_folder(&entry_path, stats)?;
        } else if entry_path.is_file() {
            if let Some(ext) = entry_path.extension() {
                if !ext.to_str().unwrap_or("").is_empty() {
                    let x = &ext.to_str().unwrap_or_default();
                    if Constants::get_language(x).is_some() && !Constants::is_binary_extension(x) {
                        analyze_file(&entry_path, stats)?;
                    }
                }
            }
        }
    }

    Ok(())
}
