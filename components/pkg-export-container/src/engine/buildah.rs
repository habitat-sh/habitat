use super::{resolve_engine_binary,
            Engine,
            EngineError};
use std::{path::{Path,
                 PathBuf},
          process::Command,
          result::Result};

#[derive(Debug)]
pub(super) struct BuildahEngine {
    binary: PathBuf,
}

impl BuildahEngine {
    pub fn new() -> Result<Self, EngineError> {
        let binary = resolve_engine_binary("buildah")?;
        Ok(BuildahEngine { binary })
    }
}

impl Engine for BuildahEngine {
    /// `buildah images -q mycompany/coolapp`
    fn image_id_command(&self, image_reference: &str) -> Command {
        let mut cmd = Command::new(&self.binary);
        cmd.args(&["images", "-q", image_reference]);
        cmd
    }

    /// `buildah rmi mycompany/coolapp`
    fn image_removal_command(&self, image_reference: &str) -> Command {
        let mut cmd = Command::new(&self.binary);
        cmd.args(&["rmi", image_reference]);
        cmd
    }

    /// `buildah push --authfile=/path/to/local/config.json push mycompany/mycoolapp`
    fn image_push_command(&self, image_reference: &str, config_dir: &Path) -> Command {
        let mut cmd = Command::new(&self.binary);
        cmd.args(&["push",
                   "--authfile",
                   &config_dir.join("config.json").to_string_lossy(),
                   image_reference]);
        cmd
    }

    fn build_command(&self,
                     build_context: &Path,
                     tags: &[String],
                     memory: Option<&str>)
                     -> Command {
        let mut cmd = Command::new(&self.binary);
        cmd.current_dir(build_context);

        cmd.arg("build-using-dockerfile")
           .arg("--layers")
           .arg("--force-rm");

        // Need this (Buildah's default format is OCI) because
        // apparently DockerHub has problems with OCI images.
        //
        // https://github.com/docker/hub-feedback/issues/1871
        //
        // (This is only really a problem when *pushing* images, but
        // since DockerHub is the 800 lb gorilla, we'll defer to it
        // for now.)
        cmd.args(&["--format", "docker"]);

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
