use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "tree")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "List contents of directories in a tree-like format")]
#[command(long_about = None)]
pub struct Config {
    /// All files are listed
    #[arg(short, long)]
    pub all: bool,

    /// List directories only
    #[arg(short, long)]
    pub dirs_only: bool,

    /// Don't print indentation lines
    #[arg(short = 'i', long)]
    pub no_indent: bool,

    /// Display full file paths
    #[arg(short, long)]
    pub full_path: bool,

    /// Ignore files specified in .gitignore
    #[arg(short, long)]
    pub gitignore: bool,

    /// Max display depth of the directory tree
    #[arg(short = 'L', long, value_name = "LEVEL")]
    pub max_depth: Option<usize>,

    /// Output tree to a file
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Paths to list (default: current directory)
    #[arg(value_name = "PATH")]
    pub paths: Vec<PathBuf>,
}

impl Config {
    pub fn parse_args() -> Self {
        let mut config: Config = Self::parse();

        // If no paths provided, use current directory
        if config.paths.is_empty() {
            config.paths.push(PathBuf::from("."));
        }

        config
    }
}
