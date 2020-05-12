//! Abstractions for dealing with the main behaviors we need when
//! dealing with container images, while remaining unconcerned about
//! which underlying tool is actually performing those tasks.

use crate::error::Result;
use habitat_core::fs::find_command;
use std::{path::{Path,
                 PathBuf},
          process::{Command,
                    ExitStatus}};

#[derive(Debug, Fail)]
enum EngineError {
    #[fail(display = "Container image build failed with exit code: {}", _0)]
    BuildFailed(ExitStatus),
    #[fail(display = "Could not find the container engine executable '{}' on the PATH",
           _0)]
    ExecutableNotFound(String),
    #[fail(display = "Could not determine container image ID for: {}", _0)]
    ImageIdNotFound(String),
    #[fail(display = "Removing local container images failed with exit code: {}",
           _0)]
    RemoveFailed(ExitStatus),
    #[fail(display = "Container image push failed with exit code: {}", _0)]
    PushFailed(ExitStatus),
}

pub struct Engine {
    binary: PathBuf,
}

impl Engine {
    pub fn new() -> Result<Self> {
        let binary_name = "docker";
        let binary =
            find_command(binary_name).ok_or_else(|| {
                                         EngineError::ExecutableNotFound(binary_name.to_string())
                                     })?;
        Ok(Engine { binary })
    }

    /// Retrieve the ID of the given image, which is expected to exist.
    pub fn image_id(&self, image_reference: &str) -> Result<String> {
        let mut cmd = self.image_id_command(image_reference);
        debug!("Running: {:?}", cmd);
        let output = cmd.output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        match stdout.lines().next() {
            Some(id) => Ok(id.to_string()),
            None => Err(EngineError::ImageIdNotFound(image_reference.to_string()).into()),
        }
    }

    /// Delete the referenced image in the local image store.
    pub fn remove_image(&self, image_reference: &str) -> Result<()> {
        Engine::run(self.image_removal_command(image_reference),
                    EngineError::RemoveFailed)
    }

    /// Pushes the specified container image to a remote repository, using
    /// configuration stored in `config_dir`.
    // TODO (CM): accept repository URL information
    // TODO (CM): worth taking credential / repo information and
    // handling the config directory stuff internally?
    pub fn push_image(&self, image_reference: &str, config_dir: &Path) -> Result<()> {
        Engine::run(self.image_push_command(image_reference, config_dir),
                    EngineError::PushFailed)
    }

    /// Actually create the image.
    ///
    /// `build_context` will serve as the build context directory, and
    /// a suitable `Dockerfile` is expected to be present in it. The
    /// image will be tagged with each of `tags`.
    ///
    /// `memory` governs how much memory is provided to the build
    /// process.
    ///
    /// Returns the ID of the image that was built.
    pub fn build<S: AsRef<str>>(&self,
                                build_context: &Path,
                                tags: &[S],
                                memory: Option<&str>)
                                -> Result<String> {
        Engine::run(self.build_command(build_context, tags, memory),
                    EngineError::BuildFailed)?;

        let identifier = tags.first()
                             .expect("There should always be at least one tag")
                             .as_ref();
        self.image_id(identifier)
    }

    ////////////////////////////////////////////////////////////////////////

    /// General helper function for actually executing all these commands
    fn run<F>(mut cmd: Command, err_fn: F) -> Result<()>
        where F: Fn(ExitStatus) -> EngineError
    {
        debug!("Running: {:?}", &cmd);
        let exit_status = cmd.spawn()?.wait()?;
        if !exit_status.success() {
            return Err(err_fn(exit_status).into());
        }
        Ok(())
    }

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
    fn build_command<S: AsRef<str>>(&self,
                                    build_context: &Path,
                                    tags: &[S],
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
            cmd.arg("--tag").arg(tag.as_ref());
        }
        cmd.arg(".");
        cmd
    }
}
