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
use retry::{delay,
            retry};
use std::path::PathBuf;

const RETRY_LIMIT: usize = 5;
const INTERNAL_TOOLING_CHANNEL_ENVVAR: &str = "HAB_INTERNAL_BLDR_CHANNEL";

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
/// channel. However, if you'd rather use unstable (particularly if
/// you're developing Habitat), you'll need to set the
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
pub fn command_from_min_pkg(ui: &mut UI,
                            command: impl Into<PathBuf>,
                            ident: &PackageIdent)
                            -> Result<PathBuf> {
    let command = command.into();
    let fs_root_path = FS_ROOT_PATH.as_path();
    let pi = match PackageInstall::load_at_least(ident, Some(fs_root_path)) {
        Ok(pi) => pi,
        Err(hcore::Error::PackageNotFound(_)) => {
            ui.status(Status::Missing, format!("package for {}", &ident))?;

            // JB TODO - Does an auth token need to be plumbed into here?  Not 100% sure.
            retry(delay::NoDelay.take(RETRY_LIMIT), || {
                common::command::package::install::start(ui,
                                                         &default_bldr_url(),
                                                         &internal_tooling_channel(),
                                                         &(ident.clone(),
                                                           PackageTarget::active_target())
                                                                                          .into(),
                                                         PRODUCT,
                                                         VERSION,
                                                         fs_root_path,
                                                         &cache_artifact_path(None::<String>),
                                                         None,
                                                         // TODO fn: pass through and enable
                                                         // offline
                                                         // install mode
                                                         &InstallMode::default(),
                                                         // TODO (CM): pass through and enable
                                                         // no-local-package mode
                                                         &LocalPackageUsage::default(),
                                                         InstallHookMode::default())
            }).map_err(|_| Error::ExecCommandNotFound(command.clone()))?
        }
        Err(e) => return Err(Error::from(e)),
    };

    fs::find_command_in_pkg(&command, &pi, fs_root_path)?.ok_or_else(|| {
                                                             Error::ExecCommandNotFound(command)
                                                         })
}

/// Determine the channel from which to install Habitat-specific
/// packages.
fn internal_tooling_channel() -> ChannelIdent {
    hcore::env::var(INTERNAL_TOOLING_CHANNEL_ENVVAR).map(ChannelIdent::from)
                                                    .unwrap_or_default()
}
