use walkdir::WalkDir;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use clap::Parser;
use once_cell::sync::Lazy;

/// PHP Hound - An opinionated PHP issue sniffer.
#[derive(Parser)]
#[command(name = "php-hound")]
#[command(about = "Scans PHP files for possible issues.", long_about = None)]

struct Cli {
    // Path to directory to scan.
    #[arg(short, long, default_value = ".")]
    path: String,

    // Paths to ignore.
    #[arg(short, long)]
    ignore: Vec<String>,
}

// Regex to catch potential accidental assignments in if/while/elseif conditions.
static ASSIGNMENT_RX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(if|elseif)\s*\(([^)]*[^=!<>])=([^=][^)]*)\)").unwrap()
});

// Regex to catch ++ or -- in conditionals.
static INCREMENT_RX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(if|elseif)\s*\([^)]*(\+\+|--)[^)]*\)").unwrap()
});

// Regex to catch use of eval().
static EVAL_RX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\beval\s*\(").unwrap()
});

// Regex to catch use of var_dump().
static VARDUMP_RX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\bvar_dump\s*\(").unwrap()
});

fn main() {
    let args = Cli::parse();
    let ignore_paths: Vec<PathBuf> = args.ignore.iter().map(PathBuf::from).collect();

    // Introduction message.
    println!("PHP Hound - An opinionated PHP issue sniffer by Dan Ruscoe.\n");

    for entry in WalkDir::new(&args.path).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        if ignore_paths.iter().any(|ignore| path.starts_with(ignore)) {
            continue;
        }

        if path.extension().map(|e| e == "php").unwrap_or(false) {
            process_php_file(path);
        }
    }
}

// Processes a PHP file and checks each line for issues.
fn process_php_file(path: &Path) {
    let content = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("Failed to read {}: {}", path.display(), err);
            return;
        }
    };

    for (i, line) in content.lines().enumerate() {
        if ASSIGNMENT_RX.is_match(line) {
            println!(
                "Possible accidental assignment in {} at line {}:\n  {}\n",
                path.display(),
                i + 1,
                line.trim()
            );
        }
        if INCREMENT_RX.is_match(line) {
            println!(
                "Increment / decrement in condition in {} at line {}:\n  {}\n",
                path.display(),
                i + 1,
                line.trim()
            );
        }
        if EVAL_RX.is_match(line) {
            println!(
                "Use of eval() in {} at line {}:\n  {}\n",
                path.display(),
                i + 1,
                line.trim()
            );
        }
        if VARDUMP_RX.is_match(line) {
            println!(
                "Use of var_dump() in {} at line {}:\n  {}\n",
                path.display(),
                i + 1,
                line.trim()
            );
        }
    }
}
