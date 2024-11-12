 mod constants;

use std::{env, path::Path, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <project directory>", args[0]);
        process::exit(1);
    }
    let project_path = Path::new(&args[1]);

    if project_path.is_file() {
        println!("Processing single file: {}", project_path.display());
    } else if project_path.is_dir() {
    } else if is_github_url(&args[1]) {
    }
}
fn is_github_url(url: &str) -> bool {
    url.starts_with("https://github.com/")
}



