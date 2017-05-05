use error::Result;
use std::fs;
use std::path::PathBuf;

/// Encapsulates the local filesystem directory in which in-process
/// build job logs will be collected prior to being sent to long-term
/// storage.
#[derive(Clone, Debug)]
pub struct LogDirectory {
    path: PathBuf,
}

impl LogDirectory {
    /// Create a new `LogDirectory` wrapping `path`.
    pub fn new(path: &str) -> Self {
        LogDirectory { path: PathBuf::from(path) }
    }

    /// Ensures that the specified log directory can be used.
    ///
    /// # Panics
    /// * when the path does not exist
    /// * when the path is not a directory
    /// * when the path is not writable
    pub fn validate(&self) -> Result<()> {
        let meta =
            fs::metadata(&self.path).expect(format!("Build log directory {:?} doesn't exist!",
                                                    self.path)
                                                    .as_str());
        if !meta.is_dir() {
            panic!("Build log directory {:?} is not a directory!", self.path);
        }
        if meta.permissions().readonly() {
            panic!("Build log directory {:?} is not writable!", self.path);
        }
        Ok(())
    }

    /// Returns the path to a particular job's log file within the
    /// `LogDirectory`. The file may not exist yet.
    pub fn log_file_path(&self, job_id: u64) -> PathBuf {
        self.path.join(format!("{}.log", job_id))
    }
}
