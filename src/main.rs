use std::{process::exit, time::Instant};

use cli::color::Color as _;

fn print_banner() {
    println!(
        "{}",
        r#"
   ________    ________  ________
  / ____/ /   / __ \  \/ / ____/
 / /   / /   / / / /\  / /     
/ /___/ /___/ /_/ / / / /___   
\____/_____/\____/ /_/\____/   
"#
        .cyan()
    );

    println!(
        "{}",
        "Count Lines of Code - A Fast Code Analytics Tool".yellow()
    );
    println!("{}", "----------------------------------------".blue());
    println!();
}

fn print_scanning_message(path: &str) {
    println!("{} {}", "📊 Analyzing:".green(), path);
    println!("{}", "----------------------------------------".blue());
}
mod cli;
fn main() {
    print_banner();
    let start_time = Instant::now();
    let project_path = cli::parse_flags();
    print_scanning_message(&project_path.to_string_lossy());
    let project_path = cli::parse_flags();
    if project_path.is_file() {
        if let Err(e) = cli::analyze_single_file(&project_path) {
            eprintln!("Error: {}", e);
            exit(1); // Exit with error code
        }
    } else if project_path.is_dir() {
    } else if cli::is_github_url(&project_path.to_string_lossy()) {
    }
}
