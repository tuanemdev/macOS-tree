use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process;

#[derive(Debug)]
struct Config {
    paths: Vec<PathBuf>,
    all: bool,
    dirs_only: bool,
    no_indent: bool,
    max_depth: Option<usize>,
    version: bool,
    help: bool,
    output_path: Option<PathBuf>,
    full_path: bool,
    gitignore: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            paths: vec![PathBuf::from(".")],
            all: false,
            dirs_only: false,
            no_indent: false,
            max_depth: None,
            version: false,
            help: false,
            output_path: None,
            full_path: false,
            gitignore: false,
        }
    }
}

struct FileStats {
    dirs: usize,
    files: usize,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let config: Config = parse_args();

    if config.version {
        println!("tree {}", VERSION);
        return;
    }

    if config.help {
        print_help();
        return;
    }

    match generate_tree(&config) {
        Ok(_) => {
            if config.output_path.is_some() {
                println!("Tree output generated successfully.");
            }
        }
        Err(err) => {
            eprintln!("Error generating tree: {}", err);
            process::exit(1);
        }
    }
}

fn generate_tree(config: &Config) -> io::Result<()> {
    let mut tree_output: String = String::new();

    for path in &config.paths {
        let mut path_stats: FileStats = FileStats { dirs: 0, files: 0 };
        let path_tree: String = visit_dir(path, config, 0, &mut path_stats, "")?;

        tree_output.push_str(&path_tree);
        tree_output.push_str(&format!(
            "\n{} directories, {} files\n",
            path_stats.dirs, path_stats.files
        ));
    }

    // If output path is specified, write to file
    if let Some(output_path) = &config.output_path {
        let mut file: fs::File = fs::File::create(output_path)?;
        file.write_all(tree_output.as_bytes())?;
    } else {
        // If no output path, print to console
        print!("{}", tree_output);
    }

    Ok(())
}

fn parse_args() -> Config {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut config: Config = Config::default();
    let mut index: usize = 0;

    while index < args.len() {
        match args[index].as_str() {
            "-a" | "--all" => config.all = true,
            "-d" | "--dirs-only" => config.dirs_only = true,
            "-i" | "--no-indent" => config.no_indent = true,
            "-f" | "--full-path" => config.full_path = true,
            "-g" | "--gitignore" => config.gitignore = true,
            "-l" | "--max-depth" => {
                if index + 1 < args.len() {
                    index += 1;
                    match args[index].parse::<usize>() {
                        Ok(depth) => config.max_depth = Some(depth),
                        Err(_) => {
                            eprintln!("Invalid depth value: {}", args[index]);
                            process::exit(1);
                        }
                    }
                } else {
                    eprintln!("Missing value for --max-depth");
                    process::exit(1);
                }
            }
            "-o" | "--output" => {
                if index + 1 < args.len() {
                    index += 1;
                    config.output_path = Some(PathBuf::from(&args[index]));
                } else {
                    eprintln!("Missing value for --output");
                    process::exit(1);
                }
            }
            "-v" | "--version" => config.version = true,
            "-h" | "--help" => config.help = true,
            _ => {
                if args[index].starts_with('-') {
                    eprintln!("Unknown option: {}", args[index]);
                    print_help();
                    process::exit(1);
                } else {
                    config.paths.push(PathBuf::from(&args[index]));
                }
            }
        }
        index += 1;
    }

    // If specific paths were provided, clear the default path
    if config.paths.len() > 1 {
        config.paths.remove(0);
    }

    config
}

fn print_help() {
    println!("Tree Command v{}", VERSION);
    println!("Usage: tree [OPTIONS] [PATH...]");
    println!("List contents of directories in a tree-like format.");
    println!("\nOptions:");
    println!("  -a, --all                 All files are listed");
    println!("  -d, --dirs-only           List directories only");
    println!("  -i, --no-indent           Don't print indentation lines");
    println!("  -f, --full-path           Display full file paths");
    println!("  -g, --gitignore           Ignore files specified in .gitignore");
    println!("  -l, --max-depth level     Max display depth of the directory tree");
    println!("  -o, --output file         Output tree to a file");
    println!("  -v, --version             Print version information");
    println!("  -h, --help                Print this help message");
}

fn visit_dir(
    dir: &Path,
    config: &Config,
    level: usize,
    stats: &mut FileStats,
    prefix: &str,
) -> io::Result<String> {
    let mut output: String = String::new();

    // Check max depth
    if let Some(max_depth) = config.max_depth {
        if level > max_depth {
            return Ok(output);
        }
    }

    // Read .gitignore patterns if gitignore option is used
    let gitignore_patterns: Vec<String> = if config.gitignore {
        read_gitignore(dir)
    } else {
        Vec::new()
    };

    // Print directory name at level 0
    if level == 0 {
        // Use full path if -f/--full-path is set
        let display_path: PathBuf = if config.full_path {
            dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf())
        } else {
            dir.to_path_buf()
        };
        output.push_str(&format!("{}/\n", display_path.display()));
        stats.dirs += 1;
    }

    let entries: fs::ReadDir = fs::read_dir(dir)?;
    let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();

    // Sort entries by name
    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    // Iterate through sorted entries
    for (index, entry) in entries.iter().enumerate() {
        let path: PathBuf = entry.path();
        let file_name: std::ffi::OsString = entry.file_name();
        let is_dir: bool = path.is_dir();

        // Skip hidden files unless -a flag is provided
        if !config.all && file_name.to_string_lossy().starts_with('.') {
            continue;
        }

        // Skip files if -d flag is provided
        if config.dirs_only && !is_dir {
            continue;
        }

        // Skip .git directory if gitignore option is used
        if config.gitignore && file_name == ".git" {
            continue;
        }

        // Check gitignore patterns
        if config.gitignore && matches_pattern(&path, dir, &gitignore_patterns) {
            continue;
        }

        let is_last: bool = index == entries.len() - 1;

        // Calculate new prefix for child items
        let (connector, new_prefix) = if config.no_indent {
            ("", "")
        } else if is_last {
            ("└── ", "    ")
        } else {
            ("├── ", "│   ")
        };

        // Create display name
        let display_name: String = if config.full_path {
            // Use canonicalized full path if requested
            let full_path: PathBuf = path.canonicalize().unwrap_or_else(|_| path.clone());
            full_path.to_string_lossy().to_string()
        } else {
            // Use just the filename
            let name: String = if is_dir {
                format!("{}/", file_name.to_string_lossy())
            } else {
                file_name.to_string_lossy().to_string()
            };
            name
        };

        // Add current entry to output
        output.push_str(&format!("{}{}{}\n", prefix, connector, display_name));

        // Update statistics
        if is_dir {
            stats.dirs += 1;
            // Recursively visit subdirectories
            let child_prefix: String = format!("{}{}", prefix, new_prefix);
            let child_output: String = visit_dir(&path, config, level + 1, stats, &child_prefix)?;
            output.push_str(&child_output);
        } else {
            stats.files += 1;
        }
    }

    Ok(output)
}

// pattern matching
fn matches_pattern(path: &Path, base_dir: &Path, patterns: &[String]) -> bool {
    let filename: String = path
        .file_name()
        .map(|n: &std::ffi::OsStr| n.to_string_lossy().to_string())
        .unwrap_or_default();

    // Relative path from the base directory
    let relative_path: String = path
        .strip_prefix(base_dir)
        .map(|p: &Path| p.to_string_lossy().to_string())
        .unwrap_or_default();

    patterns.iter().any(|pattern| {
        // Trim whitespace and handle negation
        let pattern: &str = pattern.trim();
        if pattern.is_empty() || pattern.starts_with('#') {
            return false;
        }

        // Handle negation patterns
        let is_negation: bool = pattern.starts_with('!');
        let pattern: &str = if is_negation { &pattern[1..] } else { pattern };

        // Handle absolute path patterns starting with /
        let is_absolute_pattern: bool = pattern.starts_with('/');
        let pattern: &str = pattern.trim_start_matches('/');

        // Handle directory-only patterns ending with /
        let is_directory_pattern: bool = pattern.ends_with('/');
        let pattern: &str = pattern.trim_end_matches('/');

        // Check exact filename or path matches
        if is_absolute_pattern {
            // For absolute patterns, match against relative path
            if relative_path == pattern || relative_path.ends_with(&format!("/{}", pattern)) {
                return !is_negation;
            }
        } else {
            // For relative patterns, match filename or path
            if is_directory_pattern && path.is_dir() {
                if filename == pattern || relative_path.contains(&format!("/{}/", pattern)) {
                    return !is_negation;
                }
            } else {
                // Wildcard matching for filename
                if matches_filename_pattern(&filename, pattern)
                    || relative_path.contains(&format!("/{}", pattern))
                {
                    return !is_negation;
                }
            }
        }

        false
    })
}

// Helper function for filename pattern matching
fn matches_filename_pattern(filename: &str, pattern: &str) -> bool {
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let filename_chars: Vec<char> = filename.chars().collect();

    // Custom wildcard matching function
    fn wildcard_match(pattern: &[char], text: &[char]) -> bool {
        fn match_helper(p: &[char], t: &[char], p_idx: usize, t_idx: usize) -> bool {
            // Base cases
            if p_idx == p.len() {
                return t_idx == t.len();
            }

            if t_idx == t.len() {
                // Only * can match empty string at end
                return p[p_idx..].iter().all(|&c| c == '*');
            }

            // Wildcard handling
            match p[p_idx] {
                '*' => {
                    // Try matching 0 or more characters
                    (t_idx..=t.len()).any(|i| match_helper(p, t, p_idx + 1, i))
                }
                '?' => {
                    // Match any single character
                    match_helper(p, t, p_idx + 1, t_idx + 1)
                }
                c => {
                    // Exact character match
                    c == t[t_idx] && match_helper(p, t, p_idx + 1, t_idx + 1)
                }
            }
        }

        match_helper(pattern, text, 0, 0)
    }

    wildcard_match(&pattern_chars, &filename_chars)
}

// Read .gitignore file
fn read_gitignore(dir: &Path) -> Vec<String> {
    let gitignore_path: PathBuf = dir.join(".gitignore");
    if !gitignore_path.exists() {
        return Vec::new();
    }

    let file: fs::File = match fs::File::open(&gitignore_path) {
        Ok(file) => file,
        Err(_) => return Vec::new(),
    };

    let reader: BufReader<fs::File> = BufReader::new(file);
    reader
        .lines()
        .filter_map(Result::ok)
        .filter(|line: &String| {
            // Skip comments and empty lines
            !line.trim().is_empty() && !line.trim().starts_with('#')
        })
        .collect()
}
