use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct GitignoreManager {
    patterns: Vec<String>,
}

impl GitignoreManager {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    pub fn load_patterns(&mut self, dir: &Path) {
        self.patterns = self.read_gitignore(dir);
    }

    pub fn matches(&self, path: &Path, base_dir: &Path) -> bool {
        if self.patterns.is_empty() {
            return false;
        }

        let filename = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        // Relative path from the base directory
        let relative_path = path
            .strip_prefix(base_dir)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        self.patterns
            .iter()
            .any(|pattern: &String| self.matches_pattern(&filename, &relative_path, pattern, path.is_dir()))
    }

    fn read_gitignore(&self, dir: &Path) -> Vec<String> {
        let gitignore_path: std::path::PathBuf = dir.join(".gitignore");
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
            .filter(|line| {
                // Skip comments and empty lines
                !line.trim().is_empty() && !line.trim().starts_with('#')
            })
            .collect()
    }

    fn matches_pattern(
        &self,
        filename: &str,
        relative_path: &str,
        pattern: &str,
        is_dir: bool,
    ) -> bool {
        // Handle negation patterns
        let is_negation = pattern.starts_with('!');
        let pattern = if is_negation { &pattern[1..] } else { pattern };

        // Handle absolute path patterns starting with /
        let is_absolute_pattern = pattern.starts_with('/');
        let pattern = pattern.trim_start_matches('/');

        // Handle directory-only patterns ending with /
        let is_directory_pattern = pattern.ends_with('/');
        let pattern = pattern.trim_end_matches('/');

        // Check exact filename or path matches
        if is_absolute_pattern {
            // For absolute patterns, match against relative path
            if relative_path == pattern {
                return !is_negation;
            }
        } else {
            // For relative patterns, match filename or path
            if is_directory_pattern && is_dir {
                if filename == pattern || relative_path.contains(&format!("/{}/", pattern)) {
                    return !is_negation;
                }
            } else {
                // Wildcard matching for filename
                if self.matches_filename_pattern(filename, pattern)
                    || relative_path.contains(&format!("/{}", pattern))
                {
                    return !is_negation;
                }
            }
        }

        false
    }

    fn matches_filename_pattern(&self, filename: &str, pattern: &str) -> bool {
        let pattern_chars: Vec<char> = pattern.chars().collect();
        let filename_chars: Vec<char> = filename.chars().collect();

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
}
