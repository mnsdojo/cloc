use std::path::{Path, PathBuf};
pub mod analyzer;
pub mod color;
pub mod constants;
pub mod github;

use std::{env, io};

/// Checks if the URL is a GitHub URL and contains more than 3 parts in the path
pub fn is_github_url(url: &str) -> bool {
    url.starts_with("https://github.com/") && url.split('/').count() > 3
}

/// Analyzes a single file by calling the 'analyze_file' function from the 'analyzer' module

pub fn analyze_single_file(file_path: &Path) -> Result<(), io::Error> {
    match analyzer::analyze_file(file_path) {
        Ok((file_name, line_count, comment_count)) => {
            println!("File: {}", file_name);
            println!("Total lines: {}", line_count);
            println!("Comment lines: {}", comment_count);
        }
        Err(e) => {
            eprintln!("Error analyzing file {}: {}", file_path.display(), e);
            return Err(e);
        }
    }
    Ok(())
}

/// Prints an error message and returns an IO error
pub fn exit_with_error(message: &str) -> Result<(), io::Error> {
    eprintln!("{}", message); // Print the error message to standard error
    Err(io::Error::new(io::ErrorKind::Other, message)) // Return an IO error with the message
}

/// Checks if the provided path is a valid file
pub fn is_valid_file(path: &Path) -> Result<bool, String> {
    if path.is_file() {
        Ok(true)
    } else {
        Err(format!("{} is not a valid file.", path.display()))
    }
}

/// Checks if the provided path is a valid directory
pub fn is_valid_directory(path: &Path) -> Result<bool, String> {
    if path.is_dir() {
        Ok(true)
    } else {
        Err(format!("{} is not a valid directory.", path.display()))
    }
}

pub fn parse_flags() -> PathBuf {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <project directory>", args[0]);
        std::process::exit(1);
    }
    PathBuf::from(&args[1])
}
