use super::{resolve_engine_binary,
            Engine,
            EngineError};
use std::{path::{Path,
                 PathBuf},
          process::Command,
          result::Result};

#[derive(Debug)]
pub(super) struct DockerEngine {
    binary: PathBuf,
}

impl DockerEngine {
    pub fn new() -> Result<Self, EngineError> {
        let binary = resolve_engine_binary("docker")?;
        Ok(DockerEngine { binary })
    }
}

impl Engine for DockerEngine {
    /// `docker images -q mycompany/coolapp`
    fn image_id_command(&self, image_reference: &str) -> Command {
        let mut cmd = Command::new(&self.binary);
        cmd.args(&["images", "-q", image_reference]);
        cmd
    }

    /// `docker rmi mycompany/coolapp`
    fn image_removal_command(&self, image_reference: &str) -> Command {
        let mut cmd = Command::new(&self.binary);
        cmd.args(&["rmi", image_reference]);
        cmd
    }

    /// `docker --config /path/to/local/config push mycompany/mycoolapp`
    fn image_push_command(&self, image_reference: &str, config_dir: &Path) -> Command {
        let mut cmd = Command::new(&self.binary);
        cmd.args(&["--config",
                   &config_dir.to_string_lossy(),
                   "push",
                   image_reference]);
        cmd
    }

    /// `docker build --force-rm --memory MEMORY [--tag TAG] .`
    fn build_command(&self,
                     build_context: &Path,
                     tags: &[String],
                     memory: Option<&str>)
                     -> Command {
        let mut cmd = Command::new(&self.binary);
        cmd.current_dir(build_context);
        cmd.arg("build");
        cmd.arg("--force-rm");

        if let Some(mem) = memory {
            cmd.arg("--memory").arg(mem);
        }
        for tag in tags {
            cmd.arg("--tag").arg(&tag);
        }
        cmd.arg(".");
        cmd
    }
}
