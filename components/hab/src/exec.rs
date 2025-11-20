use crate::cli_v4::utils::maybe_bldr_auth_token_from_args_or_load;

use crate::{common::{self,
                     command::package::install::{InstallHookMode,
                                                 InstallMode,
                                                 LocalPackageUsage},
                     ui::{Status,
                          UIWriter,
                          UI}},
            error::{Error,
                    Result},
            hcore::{self,
                    fs::{self,
                         cache_artifact_path,
                         FS_ROOT_PATH},
                    package::{PackageIdent,
                              PackageInstall,
                              PackageTarget},
                    url::default_bldr_url,
                    ChannelIdent},
            PRODUCT,
            VERSION};
use habitat_common::error::Error as CommonError;
use retry::delay;
use std::path::PathBuf;

const RETRY_LIMIT: usize = 5;
const INTERNAL_TOOLING_CHANNEL_ENVVAR: &str = "HAB_INTERNAL_BLDR_CHANNEL";

pub async fn command_from_min_pkg(ui: &mut UI,
                                  command: impl Into<PathBuf>,
                                  ident: &PackageIdent)
                                  -> Result<PathBuf> {
    command_from_min_pkg_with_optional_channel(ui, command, ident, None).await
}

// TODO: This function is only called from the *launcher* and since *launcher* is not yet a first
// class citizen in MacOS we get a compile time warning. When *launcher* and *sup* are properly
// supported, this should go
#[cfg(not(target_os = "macos"))]
pub async fn command_from_min_pkg_with_channel(ui: &mut UI,
                                               command: impl Into<PathBuf>,
                                               ident: &PackageIdent,
                                               channel: ChannelIdent)
                                               -> Result<PathBuf> {
    command_from_min_pkg_with_optional_channel(ui, command, ident, Some(channel)).await
}

/// Returns the absolute path to the given command from a package no
/// older than the given package identifier.
///
/// If the package is not locally installed, the package will be
/// installed before recomputing.  There are a maximum number of times
/// a re-installation will be attempted before returning an error.
///
/// # Notes on Package Installation
///
/// By default, Habitat will install packages from the `stable`
/// channel or will use a channel specific to a particular context (eg
/// `hab sup run --channel`) passed with the `channel` parameter of
/// this function. However, if you'd rather use a different channel
/// (particularly if you're developing Habitat), you'll need to set the
/// `INTERNAL_TOOLING_CHANNEL_ENVVAR` appropriately.
///
/// Note that this environment variable *only* applies to packages
/// installed through this function. As a result, this function should
/// only be used to install Habitat packages (i.e. things Habitat
/// itself needs to run), and not arbitrary user packages. This allows
/// users to fine-tune where packages come from.
///
/// Also note that due to the "minimum package" logic, this overriding
/// of the internal tooling channel logic is really only called for
/// when first encountering a given package; thereafter, we can use
/// whatever is on disk already. This provides another mechanism by
/// which you can influence what packages are used: simply install a
/// newer one.
///
/// # Failures
///
/// * If the package is installed but the command cannot be found in the package
/// * If an error occurs when loading the local package from disk
/// * If the maximum number of installation retries has been exceeded
async fn command_from_min_pkg_with_optional_channel(ui: &mut UI,
                                                    command: impl Into<PathBuf>,
                                                    ident: &PackageIdent,
                                                    channel: Option<ChannelIdent>)
                                                    -> Result<PathBuf> {
    let command = command.into();
    let fs_root_path = FS_ROOT_PATH.as_path();
    let pi = match PackageInstall::load_at_least(ident, Some(fs_root_path)) {
        Ok(pi) => pi,
        Err(hcore::Error::PackageNotFound(_)) => {
            ui.status(Status::Missing, format!("package for {}", &ident))?;

            let channel = internal_tooling_channel(channel);

            let auth_token = maybe_bldr_auth_token_from_args_or_load(None);

            // JB TODO - Does an auth token need to be plumbed into here?  Not 100% sure.
            retry::retry_future!(delay::NoDelay.take(RETRY_LIMIT), async {
                common::command::package::install::start(ui,
                                                         &default_bldr_url(),
                                                         &channel,
                                                         &(ident.clone(),
                                                           PackageTarget::active_target())
                                                                                          .into(),
                                                         PRODUCT,
                                                         VERSION,
                                                         fs_root_path,
                                                         &cache_artifact_path(None::<String>),
                                                         auth_token.as_deref(),
                                                         // TODO fn: pass through and enable
                                                         // offline
                                                         // install mode
                                                         &InstallMode::default(),
                                                         // TODO (CM): pass through and enable
                                                         // no-local-package mode
                                                         &LocalPackageUsage::default(),
                                                         InstallHookMode::default()).await
            }).await
              .map_err(|e| CommonError::PackageFailedToInstall(ident.clone(), Box::new(e.error)))?
        }
        Err(e) => return Err(Error::from(e)),
    };

    fs::find_command_in_pkg(&command, &pi, fs_root_path)?.ok_or({
                                                             Error::ExecCommandNotFound(command)
                                                         })
}

/// Determine the channel from which to install Habitat-specific packages.
fn internal_tooling_channel(channel: Option<ChannelIdent>) -> ChannelIdent {
    hcore::env::var(INTERNAL_TOOLING_CHANNEL_ENVVAR).ok()
                                                    .map(ChannelIdent::from)
                                                    .or(channel)
                                                    .or_else(|| {
                                                        option_env!("HAB_RC_CHANNEL").map(ChannelIdent::from)
                                                    })
                                                    .unwrap_or_default()
}
