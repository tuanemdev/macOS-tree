#[derive(Debug, Default)]
pub struct FileStats {
    pub dirs: usize,
    pub files: usize,
}

impl FileStats {
    pub fn new() -> Self {
        Self::default()
    }
}
