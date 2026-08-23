# ff (File Finder)

`ff` is a fast, recursive file search tool written in Rust. It allows you to find files based on names, extensions, and other criteria with a simple CLI interface.

## Installation

To install `ff` from source, navigate to the project root and run:

```bash
cargo install --path .
```

## Usage

The basic syntax for using `ff` is:

```bash
ff [OPTIONS] [DIRECTORY]
```

### Basic Search
To search for all files in the current directory and its subdirectories:
```bash
ff
```

### Searching by Name
To search for files containing a specific string:
```bash
ff --name "search_term"
```

### Filtering by Extension
To search for files with a specific extension (e.g., `.rs`):
```bash
ff -e .rs
```

### Specifying a Directory
To search within a specific directory:
```bash
ff /path/to/directory
```

### Advanced Filtering
- **Exclude Hidden Files**: Skip files starting with a dot.
  ```bash
  ff --hide-hidden
  ```
- **Limit Recursion Depth**: Limit how deep the search goes.
  ```bash
  ff --max-depth 2
  ```
- **Case Sensitivity**: Toggle between case-sensitive and case-insensitive matching.
  ```bash
  ff --case-sensitive
  ```

### Output Options
- **JSON Output**: Output results in structured JSON format (useful for piping).
  ```bash
  ff --json
  ```
- **Colorization**: `ff` supports ANSI color codes by default to highlight results.

## Examples

- Find all `.txt` files in `/home/user/docs` ignoring hidden files:
  ```bash
  ff /home/user/docs -e .txt --hide-hidden
  ```

- Case-insensitive search for "config" files in the current directory:
  ```bash
  ff --name "config" --case-insensitive
  ```

- Find all files in the current directory up to 3 levels deep:
  ```bash
  ff -m 3
  ```
