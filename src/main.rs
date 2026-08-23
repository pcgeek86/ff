use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::collections::HashSet;

use clap::Parser;
use rayon::prelude::*;
use walkdir::WalkDir;
use serde::Serialize;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

/// Recursively find files by name under a directory.
#[derive(Parser, Debug)]
#[command(name = "ff", version, about)]
struct Args {
    /// Optional positional directory to search under. Overrides --path.
    /// If neither this nor --path is given, the current directory is used.
    #[arg(value_name = "PATH")]
    dir: Option<PathBuf>,

    /// Directory to search under. Overridden by the positional PATH argument.
    #[arg(short, long, value_name = "PATH")]
    path: Option<PathBuf>,

    /// File name to search for. Matches against the full file name.
    #[arg(short, long)]
    name: String,

    /// Match the name as an exact string instead of a case-insensitive substring.
    #[arg(long)]
    exact: bool,

    /// Make the match case-sensitive (default is case-insensitive).
    #[arg(long)]
    case_sensitive: bool,

    /// Print directories as well as files that match the name.
    #[arg(long)]
    include_dirs: bool,

    /// Filter by file extension (e.g., "rs" for .rs files).
    #[arg(short = 'e', long)]
    extension: Option<String>,

    /// Output results in JSON format.
    #[arg(long)]
    json: bool,

    /// Colorize the output.
    #[arg(short = 'c', long)]
    color: bool,

    /// Regex pattern to match file names against.
    #[arg(long)]
    regex: Option<String>,


    /// Maximum depth to recurse into (default is 6).
    #[arg(short, long, default_value = "6")]
    max_depth: u64,
}

impl Args {
    fn resolve_dir(&self) -> PathBuf {
        self.dir
            .clone()
            .or_else(|| self.path.clone())
            .unwrap_or_else(|| PathBuf::from("."))
    }

    fn load_ffignore(&self) -> HashSet<String> {
        let root = self.resolve_dir();
        let ffignore_path = root.join(".ffignore");
        if ffignore_path.exists() {
            if let Ok(content) = fs::read_to_string(ffignore_path) {
                return content
                    .lines()
                    .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
                    .map(|line| line.trim().to_string())
                    .collect();
            }
        }
        HashSet::new()
    }
}

#[derive(Serialize)]
struct MatchResult {
    path: String,
}

enum MatchMode {
    SubstringInsensitive(String),
    SubstringSensitive(String),
    ExactInsensitive(String),
    ExactSensitive(String),
}

impl MatchMode {
    fn new(args: &Args) -> Self {
        let target = args.name.clone();
        if args.exact {
            if args.case_sensitive {
                MatchMode::ExactSensitive(target)
            } else {
                MatchMode::ExactInsensitive(target.to_lowercase())
            }
        } else if args.case_sensitive {
            MatchMode::SubstringSensitive(target)
        } else {
            MatchMode::SubstringInsensitive(target.to_lowercase())
        }
    }

    fn matches(&self, name: &str) -> bool {
        match self {
            MatchMode::SubstringInsensitive(target_lower) => {
                name.to_lowercase().contains(target_lower)
            }
            MatchMode::SubstringSensitive(target) => name.contains(target),
            MatchMode::ExactInsensitive(target_lower) => {
                name.to_lowercase() == *target_lower
            }
            MatchMode::ExactSensitive(target) => name == target,
        }
    }
}

fn main() -> ExitCode {
    let args = Args::parse();

    let root = args.resolve_dir();
    if !root.exists() {
        eprintln!("ff: path does not exist: {}", root.display());
        return ExitCode::from(2);
    }
    if !root.is_dir() {
        eprintln!("ff: path is not a directory: {}", root.display());
        return ExitCode::from(2);
    }

    let ignore_rules = args.load_ffignore();
    let mode = MatchMode::new(&args);

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{msg}: {spinner:.green} [{elapsed_precise}]")
            .unwrap(),
    );
    pb.set_message("Scanning...");

    let mut scanned: usize = 0;
    let results: Vec<PathBuf> = WalkDir::new(root)
        .max_depth(args.max_depth as usize)
        .into_iter()
        .filter_map(|e| {
            if e.is_err() {
                return None;
            }
            let entry = e.unwrap();
            let name = entry.file_name().to_str().unwrap_or("");
            if scanned % 50 == 0 || scanned == 1 {
                pb.set_message("Scanning...");
            }
            scanned += 1;
            if args.hide_hidden && name.starts_with('.') {
                return None;
            }

            let path = entry.path();
            let path_str = path.to_string_lossy();

            // Check ffignore
            for rule in &ignore_rules {
                if path_str.contains(rule) {
                    return None;
                }
            }

            let is_dir = path.is_dir();
            let should_include = (is_dir && args.include_dirs) || (!is_dir && path.is_file());

            if should_include {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    // Extension check
                    if let Some(ref regex_str) = args.regex {
                        if let Ok(re) = Regex::new(regex_str) {
                            if !re.is_match(name) {
                                return None;
                            }
                        } else {
                            eprintln!("Error: Invalid regex pattern: {}", regex_str);
                            return None;
                        }
                    }


                    if mode.matches(name) {
                        return Some(path.to_path_buf());
                    }
                }
            }
            None
        })
        .collect();

    if args.json {
        let mut outputs = Vec::new();
        for path in &results {
            outputs.push(MatchResult {
                path: path.display().to_string(),
            });
        }
        println!("{}", serde_json::to_string(&outputs).unwrap());
    } else {
        results.par_iter().for_each(|path| {
            let mut output = path.display().to_string();
            if args.color {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if mode.matches(name) {
                         output = output.replace(name, &name.yellow().bold().to_string());
                    }
                }
            }
            println!("{}", output);
        });
    }

    println!("Done ({} files)", results.len());

    if results.is_empty() {
        ExitCode::from(1)
    } else {
        ExitCode::from(0)
    }
}