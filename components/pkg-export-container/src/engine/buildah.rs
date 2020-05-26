use super::{resolve_engine_binary,
            Engine,
            EngineError};
use std::{io::Write,
          path::{Path,
                 PathBuf},
          process::Command,
          result::Result};
use tempfile::TempPath;

/// Contents of the signature policy file used by Buildah (normally
/// present at /etc/containers/policy.json.)
///
/// Our policy will be to default to accepting everything (which is
/// also the default given by RPM installations of buildah).
///
/// See https://www.mankier.com/5/containers-policy.json for further
/// information.
const SIGNATURE_POLICY: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),
                                                    "/defaults/containers-policy.json"));

#[derive(Debug)]
pub(super) struct BuildahEngine {
    binary: PathBuf,

    /// Path to a signature policy file that we control, not
    /// `/etc/containers/policy.json`.
    ///
    /// The file will be removed when this struct is dropped.
    policy: TempPath,
}

#[derive(Debug, Fail)]
enum BuildahError {
    #[fail(display = "Could not create signature policy file for Buildah: {}", _0)]
    SignaturePolicyError(std::io::Error),
}

impl From<BuildahError> for EngineError {
    fn from(b: BuildahError) -> EngineError { EngineError::EngineSpecificError(b.into()) }
}

impl BuildahEngine {
    pub fn new() -> Result<Self, EngineError> {
        let binary = resolve_engine_binary("buildah")?;
        let policy = Self::signature_policy()?;
        Ok(BuildahEngine { binary, policy })
    }

    /// Write out a permissive default signature policy to a temporary
    /// file, and return the path to that file.
    ///
    /// The file will be removed when that `TempPath` is dropped.
    fn signature_policy() -> Result<TempPath, BuildahError> {
        let mut policy =
            tempfile::NamedTempFile::new().map_err(BuildahError::SignaturePolicyError)?;
        policy.write_all(SIGNATURE_POLICY.as_bytes())
              .map_err(BuildahError::SignaturePolicyError)?;
        Ok(policy.into_temp_path())
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

        // Have to override the policy file location because we don't
        // control /etc/containers/policy.json
        cmd.arg("--signature-policy");
        cmd.arg(&self.policy);

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
