use std::{process::exit, time::Instant};

use cli::color::Color as _;

mod cli;
fn main() {
    print_banner();

    let start_time = Instant::now();
    let project_path = cli::parse_flags();

    print_scanning_message(&project_path.to_string_lossy());

    let result = if project_path.is_file() {
        cli::analyze_single_file(&project_path) // Analyze the single file
    } else {
        eprintln!(
            "{} '{}' is not a valid file",
            "❌ Error:".red(),
            project_path.to_string_lossy()
        );
        exit(1);
    };

    match result {
        Ok(stats) => {
            let duration = start_time.elapsed().as_secs_f64();
            print_summary(&stats, duration);

            // Exit with success
            println!("\n{} Analysis completed successfully!", "✨".green());
            exit(0);
        }
        Err(e) => {
            eprintln!("\n{} {}", "❌ Error:".red(), e);
            exit(1);
        }
    }
}

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

fn print_summary(stats: &cli::stats::Stats, duration: f64) {
    println!("\n{}", "📈 Analysis Results:".green());
    println!("{}", "----------------------------------------".blue());

    println!(
        "{:<20} {:>10}",
        "Total Files:".yellow(),
        stats.total_files.to_string().cyan()
    );
    println!(
        "{:<20} {:>10}",
        "Lines of Code:".yellow(),
        stats.code_lines.to_string().cyan()
    );
    println!(
        "{:<20} {:>10}",
        "Comments:".yellow(),
        stats.comment_lines.to_string().cyan()
    );
    println!(
        "{:<20} {:>10}",
        "Blank Lines:".yellow(),
        stats.blank_lines.to_string().cyan()
    );
    println!(
        "{:<20} {:>10}",
        "Total Lines:".yellow(),
        stats.total_lines.to_string().cyan()
    );

    println!("\n{}", "📊 Language Breakdown:".green());
    println!("{}", "----------------------------------------".blue());

    for (lang, count) in &stats.language_stats {
        println!("{:<20} {:>10}", lang.yellow(), count.to_string().cyan());
    }

    println!("\n{}", "⚡ Performance:".green());
    println!("{}", "----------------------------------------".blue());
    println!("{:<20} {:>10.2}s", "Analysis Time:".yellow(), duration);
}
