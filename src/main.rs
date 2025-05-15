use walkdir::WalkDir;
use regex::Regex;
use std::fs;
use std::path::Path;
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

    // Regex to catch potential accidental assignments in if/while/elseif conditions.
    let assignment_rx = Regex::new(r"(if|elseif)\s*\(([^)]*[^=!<>])=([^=][^)]*)\)").unwrap();

    // Regex to catch ++ or -- in conditionals.
    let increment_rx = Regex::new(r"(if|elseif)\s*\([^)]*(\+\+|--)[^)]*\)").unwrap();

    // Regex to catch use of eval().
    let eval_rx = Regex::new(r"\beval\s*\(").unwrap();

    // Regex to catch use of var_dump().
    let vardump_rx = Regex::new(r"\bvar_dump\s*\(").unwrap();

    // Introduction message.
    println!("PHP Hound - An opinionated PHP issue sniffer by Dan Ruscoe.\n");

    for entry in WalkDir::new(&args.path).into_iter().filter_map(Result::ok) {
        if entry.path().extension().map(|e| e == "php").unwrap_or(false) {
            process_php_file(entry.path(), &assignment_rx, &increment_rx, &eval_rx, &vardump_rx);
        }
    }
}

// Processes a PHP file and checks each line for issues.
fn process_php_file(path: &Path, assignment_rx: &Regex, increment_rx: &Regex, eval_rx: &Regex, vardump_rx: &Regex) {
    let content = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("Failed to read {}: {}", path.display(), err);
            return;
        }
    };

    for (i, line) in content.lines().enumerate() {
        if assignment_rx.is_match(line) {
            println!(
                "Possible accidental assignment in {} at line {}:\n  {}\n",
                path.display(),
                i + 1,
                line.trim()
            );
        }
        if increment_rx.is_match(line) {
            println!(
                "Increment / decrement in condition in {} at line {}:\n  {}\n",
                path.display(),
                i + 1,
                line.trim()
            );
        }
        if eval_rx.is_match(line) {
            println!(
                "Use of eval() in {} at line {}:\n  {}\n",
                path.display(),
                i + 1,
                line.trim()
            );
        }
        if vardump_rx.is_match(line) {
            println!(
                "Use of var_dump() in {} at line {}:\n  {}",
                path.display(),
                i + 1,
                line.trim()
            );
        }
    }
}
