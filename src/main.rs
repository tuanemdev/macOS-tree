use std::env;
use std::fs;
use std::io;
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
        }
    }
}

struct FileStats {
    dirs: usize,
    files: usize,
}

fn main() {
    let config: Config = parse_args();

    if config.version {
        println!("tree 1.0.0");
        return;
    }

    if config.help {
        print_help();
        return;
    }

    let mut total_stats: FileStats = FileStats { dirs: 0, files: 0 };

    for path in &config.paths {
        match visit_dir(path, &config, 0, &mut total_stats, "") {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
                process::exit(1);
            }
        }
    }

    println!(
        "\n{} directories, {} files",
        total_stats.dirs, total_stats.files
    );
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
            "-L" | "--max-depth" => {
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
        config.paths.remove(0); // Remove the default "." path
    }

    config
}

fn print_help() {
    println!("Usage: tree [OPTIONS] [PATH...]");
    println!("List contents of directories in a tree-like format.");
    println!("\nOptions:");
    println!("  -a, --all             All files are listed");
    println!("  -d, --dirs-only       List directories only");
    println!("  -i, --no-indent       Don't print indentation lines");
    println!("  -L, --max-depth LEVEL Max display depth of the directory tree");
    println!("  -v, --version         Print version information");
    println!("  -h, --help            Print this help message");
}

fn visit_dir(
    dir: &Path,
    config: &Config,
    level: usize,
    stats: &mut FileStats,
    prefix: &str,
) -> io::Result<()> {
    // Check max depth
    if let Some(max_depth) = config.max_depth {
        if level > max_depth {
            return Ok(());
        }
    }

    // Print directory name at level 0
    if level == 0 {
        println!("{}/", dir.display());
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

        let is_last: bool = index == entries.len() - 1;

        // Calculate new prefix for child items
        let (connector, new_prefix) = if config.no_indent {
            ("", "")
        } else if is_last {
            ("└── ", "    ")
        } else {
            ("├── ", "│   ")
        };

        // Print the current entry with a slash for directories
        let display_name: String = if is_dir {
            format!("{}/", file_name.to_string_lossy())
        } else {
            file_name.to_string_lossy().to_string()
        };

        println!("{}{}{}", prefix, connector, display_name);

        // Update statistics
        if is_dir {
            stats.dirs += 1;
            // Recursively visit subdirectories
            let child_prefix: String = format!("{}{}", prefix, new_prefix);
            visit_dir(&path, config, level + 1, stats, &child_prefix)?;
        } else {
            stats.files += 1;
        }
    }

    Ok(())
}
