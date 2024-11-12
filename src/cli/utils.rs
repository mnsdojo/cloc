use std::io;
use std::path::Path;

/// Checks if the URL is a GitHub URL and contains more than 3 parts in the path
pub fn is_github_url(url: &str) -> bool {
    url.starts_with("https://github.com/") && url.split('/').count() > 3
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
