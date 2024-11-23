use std::path::PathBuf;
pub mod analyzer;
pub mod color;
pub mod constants;
pub mod stats;

use std::env;

pub fn parse_flags() -> PathBuf {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <project directory>", args[0]);
        std::process::exit(1);
    }
    PathBuf::from(&args[1])
}
