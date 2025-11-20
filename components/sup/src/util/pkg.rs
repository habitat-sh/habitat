use crate::{PRODUCT,
            VERSION,
            error::{Error,
                    Result}};
use hab::{command::pkg::{self,
                         uninstall::{self,
                                     UninstallHookMode,
                                     UninstallSafety}},
          error::Result as HabResult};
use habitat_api_client::BuilderAPIClient;
use habitat_common::{cli_config::CliConfig,
                     command::package::install::{self as install_cmd,
                                                 InstallHookMode,
                                                 InstallMode,
                                                 InstallSource,
                                                 LocalPackageUsage},
                     outputln,
                     ui::{NullUi,
                          UIWriter}};
use habitat_core::{AUTH_TOKEN_ENVVAR,
                   ChannelIdent,
                   env as henv,
                   fs::{self,
                        FS_ROOT_PATH},
                   package::{PackageIdent,
                             PackageInstall,
                             PackageTarget}};
use std::path::Path;

static LOGKEY: &str = "UT";

fn get_auth_token() -> Option<String> {
    henv::var(AUTH_TOKEN_ENVVAR).ok()
                                .or_else(|| CliConfig::cache().auth_token.clone())
}

/// Helper function for use in the Supervisor to handle lower-level
/// arguments needed for installing a package.
pub async fn install<T>(ui: &mut T,
                        url: &str,
                        install_source: &InstallSource,
                        channel: &ChannelIdent)
                        -> Result<PackageInstall>
    where T: UIWriter
{
    let fs_root_path = Path::new(&*FS_ROOT_PATH);
    let auth_token = get_auth_token();
    install_cmd::start(ui,
                       url,
                       channel,
                       install_source,
                       PRODUCT,
                       VERSION,
                       fs_root_path,
                       &fs::cache_artifact_path(None::<String>),
                       auth_token.as_deref(),
                       &InstallMode::default(),
                       &LocalPackageUsage::default(),
                       // Install hooks are run when the supervisor
                       // loads the package in add_service so it is
                       // repetitive to run them here
                       InstallHookMode::Ignore).await
                                               .map_err(Error::from)
}

// `install` but with no ui output and the benefit of thread safety
pub async fn install_no_ui(url: &str,
                           install_source: &InstallSource,
                           channel: &ChannelIdent)
                           -> Result<PackageInstall> {
    install(&mut NullUi::new(), url, install_source, channel).await
}

/// Given an InstallSource, install a new package only if an existing
/// one that can satisfy the package identifier is not already
/// present.
///
/// Return the PackageInstall corresponding to the package that was
/// installed, or was pre-existing.
pub async fn satisfy_or_install<T>(ui: &mut T,
                                   install_source: &InstallSource,
                                   bldr_url: &str,
                                   channel: &ChannelIdent)
                                   -> Result<PackageInstall>
    where T: UIWriter
{
    match installed(install_source) {
        Some(package) => Ok(package),
        None => install(ui, bldr_url, install_source, channel).await,
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

/// Install a package but only consider packages from a channel. Do not consider any locally
/// installed packages.
///
/// This will always return the package at the head of the channel.
pub async fn install_channel_head(url: &str,
                                  ident: impl AsRef<PackageIdent>,
                                  channel: &ChannelIdent)
                                  -> Result<PackageInstall> {
    let fs_root_path = Path::new(&*FS_ROOT_PATH);
    let auth_token = get_auth_token();
    let api_client = BuilderAPIClient::new(url, PRODUCT, VERSION, Some(fs_root_path))?;
    // Get the latest package identifier from the channel
    let channel_latest_ident = api_client.show_package((ident.as_ref(),
                                                        PackageTarget::active_target()),
                                                       channel,
                                                       auth_token.as_deref())
                                         .await?;
    // Ensure the latest package from the channel is installed
    install_no_ui(url, &channel_latest_ident.into(), channel).await
}

pub async fn uninstall_all_but_latest(ident: impl AsRef<PackageIdent>,
                                      number_latest_to_keep: usize)
                                      -> HabResult<usize> {
    uninstall::uninstall_all_but_latest(&mut NullUi::new(),
                                        ident,
                                        number_latest_to_keep,
                                        &FS_ROOT_PATH,
                                        pkg::ExecutionStrategy::Run,
                                        pkg::Scope::PackageAndDependencies,
                                        &[],
                                        UninstallHookMode::default(),
                                        UninstallSafety::Safe).await
}

/// Uninstall a package given a package identifier.
///
/// Note: This will uninstall the package even if the service correlated with the package is
/// loaded by the Supervisor. This is needed for service rollback where the package we are
/// uninstalling is the currently loaded package.
pub async fn uninstall_even_if_loaded(ident: impl AsRef<PackageIdent>) -> HabResult<()> {
    uninstall::uninstall(&mut NullUi::new(),
                         &ident.as_ref(),
                         &FS_ROOT_PATH,
                         pkg::ExecutionStrategy::Run,
                         pkg::Scope::PackageAndDependencies,
                         &[],
                         UninstallHookMode::default(),
                         UninstallSafety::Force).await
}
