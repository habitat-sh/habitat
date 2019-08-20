use crate::{error::{Error,
                    Result},
            PRODUCT,
            VERSION};
use habitat_common::{command::package::install::{self as install_cmd,
                                                 InstallHookMode,
                                                 InstallMode,
                                                 InstallSource,
                                                 LocalPackageUsage},
                     outputln,
                     ui::UIWriter};
use habitat_core::{env as henv,
                   fs::{self,
                        FS_ROOT_PATH},
                   package::{PackageIdent,
                             PackageInstall},
                   ChannelIdent,
                   AUTH_TOKEN_ENVVAR};
use std::path::Path;

static LOGKEY: &str = "UT";

/// Helper function for use in the Supervisor to handle lower-level
/// arguments needed for installing a package.
pub fn install<T>(ui: &mut T,
                  url: &str,
                  install_source: &InstallSource,
                  channel: &ChannelIdent)
                  -> Result<PackageInstall>
    where T: UIWriter
{
    let fs_root_path = Path::new(&*FS_ROOT_PATH);
    let auth_token = match henv::var(AUTH_TOKEN_ENVVAR) {
        Ok(v) => Some(v),
        Err(_) => None,
    };
    install_cmd::start(ui,
                       url,
                       channel,
                       install_source,
                       PRODUCT,
                       VERSION,
                       fs_root_path,
                       &fs::cache_artifact_path(None::<String>),
                       auth_token.as_ref().map(String::as_str),
                       &InstallMode::default(),
                       &LocalPackageUsage::default(),
                       // Install hooks are run when the supervisor
                       // loads the package in add_service so it is
                       // repetitive to run them here
                       InstallHookMode::Ignore).map_err(Error::from)
}

/// Given an InstallSource, install a new package only if an existing
/// one that can satisfy the package identifier is not already
/// present.
///
/// Return the PackageInstall corresponding to the package that was
/// installed, or was pre-existing.
pub fn satisfy_or_install<T>(ui: &mut T,
                             install_source: &InstallSource,
                             bldr_url: &str,
                             channel: &ChannelIdent)
                             -> Result<PackageInstall>
    where T: UIWriter
{
    match installed(install_source) {
        Some(package) => Ok(package),
        None => install(ui, bldr_url, install_source, channel),
    }.and_then(|installed| {
         if installed.is_runnable() {
             Ok(installed)
         } else {
             outputln!("Can't start non-runnable service: {}", installed.ident());
             Err(Error::PackageNotRunnable(installed.ident().clone()))
         }
     })
}

/// Returns an installed package for the given ident, if one is present.
pub fn installed<T>(ident: T) -> Option<PackageInstall>
    where T: AsRef<PackageIdent>
{
    let fs_root_path = Path::new(&*FS_ROOT_PATH);
    PackageInstall::load(ident.as_ref(), Some(fs_root_path)).ok()
}
