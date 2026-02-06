/// Configuration for the downloader
#[derive(Debug, Clone)]
pub struct DownloadConfig {
    /// Number of threads/chunks to use for parallel downloading
    pub num_threads: usize,
    /// Minimum file size (in bytes) to use multi-threaded download
    pub min_file_size: u64,
    /// Output file path
    pub output_path: String,
    /// Whether to clean up temporary chunk files after merging
    pub cleanup_chunks: bool,
}

impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            num_threads: 8,
            min_file_size: 1_024_000, // 1 MB
            output_path: String::new(),
            cleanup_chunks: true,
        }
    }
}

impl DownloadConfig {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the number of threads
    pub fn with_threads(mut self, threads: usize) -> Self {
        self.num_threads = threads.max(1);
        self
    }

    /// Set the minimum file size for multi-threaded download
    pub fn with_min_file_size(mut self, size: u64) -> Self {
        self.min_file_size = size;
        self
    }

    /// Set the output file path
    pub fn with_output_path(mut self, path: String) -> Self {
        self.output_path = path;
        self
    }

    /// Set whether to cleanup chunk files
    pub fn with_cleanup(mut self, cleanup: bool) -> Self {
        self.cleanup_chunks = cleanup;
        self
    }
}
