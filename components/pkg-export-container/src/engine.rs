//! Abstractions for dealing with the main behaviors we need when
//! dealing with container images, while remaining unconcerned about
//! which underlying tool is actually performing those tasks.
//!
//! This allows us to swap out the `docker` CLI for `buildah` if we
//! want to create containers as a non-root user, for instance.

use crate::error::Result;
use clap::{Arg,
           ArgMatches};
use habitat_core::fs::find_command;
use std::{convert::TryFrom,
          path::{Path,
                 PathBuf},
          process::{Command,
                    ExitStatus},
          result::Result as StdResult,
          str::FromStr};

#[cfg(not(windows))]
mod buildah;
mod docker;

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
    #[fail(display = "Unknown Container Engine '{}' was specified.", _0)]
    UnknownEngine(String),
    #[fail(display = "Cannot use `--engine=buildah` with `--multi-layer` due to https://github.com/containers/buildah/issues/2215. Please use `--engine=docker` or remove `--multi-layer`.")]
    BuildahIncompatibleWithMultiLayer,
}

/// Due to a bug in Buildah, any layers that we create in a
/// multi-layer build won't get reused, which eliminates any benefit
/// we might get from them.
///
/// Until that bug is fixed, we'll prevent using Buildah to create
/// multi-layer images, as the confusion arising from generating
/// multiple layers but not being able to reuse any of them is
/// something that's better to avoid.
///
/// When https://github.com/containers/buildah/issues/2215 is fixed,
/// we can update our Buildah dependency and remove this check.
pub fn fail_if_buildah_and_multilayer(matches: &ArgMatches) -> Result<()> {
    if matches.value_of("ENGINE") == Some("buildah") && matches.is_present("MULTI_LAYER") {
        return Err(EngineError::BuildahIncompatibleWithMultiLayer.into());
    }
    Ok(())
}

/// Things that can build containers!
#[derive(Clone, Copy, Debug)]
enum EngineKind {
    Docker,
    #[cfg(not(windows))]
    Buildah,
}

impl FromStr for EngineKind {
    type Err = EngineError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        match s {
            "docker" => Ok(EngineKind::Docker),
            #[cfg(not(windows))]
            "buildah" => Ok(EngineKind::Buildah),
            _ => Err(EngineError::UnknownEngine(s.to_string())),
        }
    }
}

/// Define the CLAP CLI argument for specifying a container build
/// engine to use.
#[rustfmt::skip] // otherwise the long_help formatting goes crazy
pub fn cli_arg<'a, 'b>() -> Arg<'a, 'b> {
    let arg =
        Arg::with_name("ENGINE").value_name("ENGINE")
        .long("engine")
        .required(true)
        .env("HAB_PKG_EXPORT_CONTAINER_ENGINE")
        .takes_value(true)
        .multiple(false)
        .default_value("docker")
        .help("The name of the container creation engine to use.");

    // TODO (CM): Find a way to tie this more closely to the
    // Engine enum values.
    if cfg!(windows) {
        // Since there is effectively no choice of engine for
        // Windows, we hide the CLI option and don't document it
        // any further.
        arg.possible_values(&["docker"]).hidden(true)
    } else {
        arg.long_help(
"Using the `docker` engine allows you to use Docker to create
your container images. You must ensure that a Docker daemon
is running on the host where this command is executed, and
that the user executing the command has permission to access
the Docker socket.

Using the `buildah` engine allows you to create container images
as an unprivileged user, and without having to use a Docker
daemon. This is the recommended engine for use in CI systems and
other environments where security is of particular concern.
Please see https://buildah.io for more details.

Both engines create equivalent container images.
",
        )
            .possible_values(&["docker", "buildah"])
    }
}

impl TryFrom<&ArgMatches<'_>> for Box<dyn Engine> {
    type Error = failure::Error;

    fn try_from(value: &ArgMatches) -> StdResult<Self, Self::Error> {
        let engine_kind =
            clap::value_t!(value, "ENGINE", EngineKind).expect("ENGINE is a required option");
        match engine_kind {
            EngineKind::Docker => Ok(Box::new(docker::DockerEngine::new()?)),
            #[cfg(not(windows))]
            EngineKind::Buildah => Ok(Box::new(buildah::BuildahEngine::new()?)),
        }
    }
}

pub trait Engine {
    /// A command that takes a container image reference and returns
    /// the ID of that image on the first line of standard output.
    fn image_id_command(&self, image_reference: &str) -> Command;

    /// A command that removes the specified local container image;
    fn image_removal_command(&self, image_reference: &str) -> Command;

    /// A command that pushes the specified container image, using
    /// configuration stored in `config_dir`.
    // TODO (CM): accept repository URL information
    // TODO (CM): worth taking credential / repo information and
    // handling the config directory stuff internally?
    fn image_push_command(&self, image_reference: &str, config_dir: &Path) -> Command;

    fn build_command(&self, build_context: &Path, tags: &[String], memory: Option<&str>)
                     -> Command;

    /// Retrieve the ID of the given image, which is expected to exist.
    fn image_id(&self, image_reference: &str) -> Result<String> {
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
    fn remove_image(&self, image_reference: &str) -> Result<()> {
        run(self.image_removal_command(image_reference),
            EngineError::RemoveFailed)
    }

    /// Pushes the specified container image to a remote repository, using
    /// configuration stored in `config_dir`.
    // TODO (CM): accept repository URL information
    // TODO (CM): worth taking credential / repo information and
    // handling the config directory stuff internally?
    fn push_image(&self, image_reference: &str, config_dir: &Path) -> Result<()> {
        run(self.image_push_command(image_reference, config_dir),
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
    fn build(&self, build_context: &Path, tags: &[String], memory: Option<&str>) -> Result<String> {
        run(self.build_command(build_context, tags, memory),
            EngineError::BuildFailed)?;

        let identifier = tags.first()
                             .expect("There should always be at least one tag");
        self.image_id(identifier)
    }
}

/// General helper function for actually executing all these commands.
///
/// Not part of the trait because nobody need to be calling this from
/// outside.
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

fn resolve_engine_binary(binary_name: &str) -> StdResult<PathBuf, EngineError> {
    find_command(binary_name).ok_or_else(|| {
                                 EngineError::ExecutableNotFound(binary_name.to_string())
                             })
}
