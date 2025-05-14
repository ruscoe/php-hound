use walkdir::WalkDir;
use regex::Regex;
use std::fs;
use clap::Parser;

/// PHP Hound - An opinionated PHP issue sniffer.
#[derive(Parser)]
#[command(name = "php-hound")]
#[command(about = "Scans PHP files for possible issues.", long_about = None)]

struct Cli {
    /// Path to directory to scan
    #[arg(short, long, default_value = ".")]
    path: String,
}

fn main() {
    let args = Cli::parse();

    // Scan for potential accidental assignments in if/while/elseif conditions.
    let re = Regex::new(r"(if|while|elseif)\s*\(([^)]*[^=!<>])=([^=][^)]*)\)").unwrap();

    for entry in WalkDir::new(&args.path).into_iter().filter_map(Result::ok) {
        if entry.path().extension().map(|e| e == "php").unwrap_or(false) {
            let path = entry.path();
            if let Ok(content) = fs::read_to_string(path) {
                for (i, line) in content.lines().enumerate() {
                    if re.is_match(line) {
                        println!(
                            "Possible accidental assignment in {} at line {}:\n  {}",
                            path.display(),
                            i + 1,
                            line.trim()
                        );
                    }
                }
            }
        }
    }
}
