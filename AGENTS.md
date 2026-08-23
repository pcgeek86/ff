# Functional Program Requirements: ff (File Finder)

## Core Features
- **Recursive Search:** The primary function is to traverse directories recursively to find files matching specific criteria.
- **Name Matching:** Users can search for files by partial or exact names using the `--name` flag.
- **Extension Filtering:** Support for searching files ending in specific extensions (e.g., `.rs`, `.txt`) using the `-e` flag.
- **Directory Specification:** Users should be able to define a starting directory for the search.
- **Multi-path Searching:** Support for searching multiple directories simultaneously. *(In development)*
- **Fuzzy/Regex Searching:** Support for fuzzy matching and regular expressions to find files matching complex criteria.

## Filtering & Exclusion
- **Hidden Files:** A `--hide-hidden` flag must be available to exclude files starting with a dot (`.`).
- **Ignored Patterns:** Integration with `.ffignore` files to respect excluded directories/files.
- **Depth Limitation:** A `--max-depth` (or `-m`) flag to limit how deep the recursion goes.
- **Case Sensitivity:** A `--case-sensitive` flag to toggle behavior between matching "File" and "file".

## Output & UI
- **Format Options:** Support for plain text output and structured JSON output for better CLI piping.
- **Colorization:** Support for ANSI color codes to highlight file paths and status.
- **Progress Indicator:** A progress bar for long-running searches using `indicatif`.

## Technical Constraints
- **Language:** Rust.
- **Concurrency:** Use efficient iterators (like `walkdir`) for fast scanning.
- **CLI Parsing:** Use `clap` for robust argument handling.

## Agent Instructions
- Keep yourself up to date anytime that new features are added or modified in this application.
- AGENTS.md is the source of truth for functional requirements.
- For information on creating and updating scoop manifests, refer to the documentation here: https://github.com/ScoopInstaller/Scoop/wiki/App-Manifests
- All release files must have the version number in the file name.
- The README must always be up to date with comprehensive examples showing how to use the application.
- Scoop manifests must be valid JSON — do not publish invalid JSON manifests. Validate the manifest with `scoop validate` or a JSON linter before publishing.
- When generating hash values for Scoop manifests, use `Get-FileHash -Algorithm SHA512 | ConvertTo-Json` to obtain the full-length hash string in `sha512:<hash>` format.