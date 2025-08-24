# Tree Command - Rust Implementation

A Rust implementation of the `tree` command that displays directory structures in a tree-like format.

## Features

- **Modern CLI**: Built with `clap` for robust command-line argument parsing
- **Modular Architecture**: Well-organized code structure with separate modules
- **gitignore Support**: Respects `.gitignore` patterns when listing files
- **Flexible Output**: Support for various display options and file output
- **Cross-platform**: Works on macOS, Linux, and Windows

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/tree`.

## Usage

```bash
# Basic usage - list current directory
tree

# List specific directory
tree /path/to/directory

# Show all files including hidden ones
tree -a

# Show directories only
tree -d

# Limit depth
tree -L 2

# Output to file
tree -o output.txt

# Respect .gitignore patterns
tree -g

# Show full paths
tree -f

# No indentation lines
tree -i
```

## Options

- `-a, --all`: All files are listed (including hidden files)
- `-d, --dirs-only`: List directories only
- `-i, --no-indent`: Don't print indentation lines
- `-f, --full-path`: Display full file paths
- `-g, --gitignore`: Ignore files specified in .gitignore
- `-L, --max-depth <LEVEL>`: Max display depth of the directory tree
- `-o, --output <FILE>`: Output tree to a file
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Project Structure

```
src/
├── main.rs         # Entry point and main application logic
├── config.rs       # Command-line argument parsing with clap
├── tree.rs         # Core tree generation logic
├── gitignore.rs    # gitignore pattern matching
├── stats.rs        # File and directory statistics
└── error.rs        # Error handling and custom error types
```

## Module Overview

### `config.rs`
- Defines the CLI interface using `clap`
- Handles command-line argument parsing
- Provides the `Config` struct with all application settings

### `tree.rs`
- Contains the main tree generation logic
- Handles directory traversal and file filtering
- Manages output formatting and display

### `gitignore.rs`
- Implements gitignore pattern matching
- Supports wildcards and various gitignore features
- Handles both file and directory patterns

### `stats.rs`
- Tracks file and directory counts
- Provides summary statistics

### `error.rs`
- Defines custom error types using `thiserror`
- Provides structured error handling throughout the application

## Dependencies

- `clap`: Modern command-line argument parser
- `anyhow`: Easy error handling
- `thiserror`: Custom error types

## Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Check code without building
cargo check
```

## License

This project is licensed under the same terms as the original.
