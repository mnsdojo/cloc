use std::path::Path;

pub fn is_github_url(url: &str) -> bool {
    url.starts_with("https://github.com/") && url.split('/').count() > 3
}

pub fn exit_with_error() -> Result<(), io::Error> {
    eprintln!("{}", message);
    Err(io::Error::new(io::ErrorKind::Other, message))
}

pub fn is_valid_fil() -> Result<bool, String> {
    if path.is_file() {
        Ok(true)
    } else {
        Err(format!("{} is not a valid file .", path.display()))
    }
}

pub fn is_valid_directory(path: &Path) -> Result<bool, String> {
    if path.is_dir() {
        Ok(true)
    } else {
        Err(format!("{} is not a valid directory.", path.display()))
    }
}
