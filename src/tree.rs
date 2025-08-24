use crate::config::Config;
use crate::error::TreeResult;
use crate::gitignore::GitignoreManager;
use crate::stats::FileStats;
use std::fs;
use std::io::Write;
use std::path::Path;

pub struct TreeGenerator<'a> {
    config: &'a Config,
    gitignore: GitignoreManager,
}

impl<'a> TreeGenerator<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            gitignore: GitignoreManager::new(),
        }
    }

    pub fn generate(&mut self) -> TreeResult<()> {
        let mut tree_output: String = String::new();

        for path in &self.config.paths {
            let mut path_stats: FileStats = FileStats::new();

            if self.config.gitignore {
                self.gitignore.load_patterns(path);
            }

            let path_tree: String = self.visit_dir(path, path, 0, &mut path_stats, "")?;

            tree_output.push_str(&path_tree);
            tree_output.push_str(&format!(
                "\n{} directories, {} files\n",
                path_stats.dirs, path_stats.files
            ));
        }

        self.output_result(&tree_output)?;
        Ok(())
    }

    fn visit_dir(
        &self,
        dir: &Path,
        base_dir: &Path,
        level: usize,
        stats: &mut FileStats,
        prefix: &str,
    ) -> TreeResult<String> {
        let mut output: String = String::new();

        // Check max depth
        if let Some(max_depth) = self.config.max_depth {
            if level > max_depth {
                return Ok(output);
            }
        }

        // Print directory name at level 0
        if level == 0 {
            let display_path: std::path::PathBuf = if self.config.full_path {
                dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf())
            } else {
                dir.to_path_buf()
            };
            output.push_str(&format!("{}/\n", display_path.display()));
        }

        let entries: fs::ReadDir = fs::read_dir(dir)?;
        let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();

        // Sort entries by name
        entries.sort_by(|a: &fs::DirEntry, b: &fs::DirEntry| a.file_name().cmp(&b.file_name()));

        // Filter out entries based on config
        entries.retain(|entry: &fs::DirEntry| self.should_include_entry(entry, base_dir));

        // Iterate through sorted entries
        for (index, entry) in entries.iter().enumerate() {
            let path: std::path::PathBuf = entry.path();
            let file_name: std::ffi::OsString = entry.file_name();
            let is_dir: bool = path.is_dir();
            let is_last: bool = index == entries.len() - 1;

            // Calculate new prefix for child items
            let (connector, new_prefix) = if self.config.no_indent {
                ("", "")
            } else if is_last {
                ("└── ", "    ")
            } else {
                ("├── ", "│   ")
            };

            // Create display name
            let display_name: String = self.format_display_name(&path, &file_name, is_dir);

            // Add current entry to output
            output.push_str(&format!("{}{}{}\n", prefix, connector, display_name));

            // Update statistics and recurse if directory
            if is_dir {
                stats.dirs += 1;
                let child_prefix: String = format!("{}{}", prefix, new_prefix);
                let child_output: String =
                    self.visit_dir(&path, base_dir, level + 1, stats, &child_prefix)?;
                output.push_str(&child_output);
            } else {
                stats.files += 1;
            }
        }

        Ok(output)
    }

    fn should_include_entry(&self, entry: &fs::DirEntry, base_dir: &Path) -> bool {
        let path: std::path::PathBuf = entry.path();
        let file_name: std::ffi::OsString = entry.file_name();
        let is_dir: bool = path.is_dir();

        // Skip hidden files unless -a flag is provided
        if !self.config.all && file_name.to_string_lossy().starts_with('.') {
            return false;
        }

        // Skip files if -d flag is provided
        if self.config.dirs_only && !is_dir {
            return false;
        }

        // Skip .git directory if gitignore option is used
        if self.config.gitignore && path == base_dir.join(".git") {
            return false;
        }

        // Check gitignore patterns
        if self.config.gitignore && self.gitignore.matches(&path, base_dir) {
            return false;
        }

        true
    }

    fn format_display_name(
        &self,
        path: &Path,
        file_name: &std::ffi::OsStr,
        is_dir: bool,
    ) -> String {
        if self.config.full_path {
            let full_path: std::path::PathBuf = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
            full_path.to_string_lossy().to_string()
        } else {
            let name: String = if is_dir {
                format!("{}/", file_name.to_string_lossy())
            } else {
                file_name.to_string_lossy().to_string()
            };
            name
        }
    }

    fn output_result(&self, content: &str) -> TreeResult<()> {
        if let Some(output_path) = &self.config.output {
            let mut file: fs::File = fs::File::create(output_path)?;
            file.write_all(content.as_bytes())?;
            println!("Tree output generated successfully.");
        } else {
            print!("{}", content);
        }
        Ok(())
    }
}
