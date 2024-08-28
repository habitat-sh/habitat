mod uninstall_impl;

use super::{ExecutionStrategy,
            Scope};
use crate::error::Result;
use habitat_common::ui::UI;
use habitat_core::package::PackageIdent;
use std::path::Path;

pub use uninstall_impl::{uninstall,
                         uninstall_all_but_latest,
                         UninstallHookMode,
                         UninstallSafety};

#[derive(Clone, Copy)]
pub enum UninstallMode {
    Single,
    KeepLatest(usize),
}

impl From<Option<usize>> for UninstallMode {
    fn from(keep_latest: Option<usize>) -> Self {
        match keep_latest {
            Some(keep_latest) => Self::KeepLatest(keep_latest),
            None => Self::Single,
        }
    }
}

// TODO: Remove after feature `v2` is removed
#[cfg(feature = "v2")]
use clap::ArgMatches;

#[cfg(feature = "v2")]
impl<'a> From<&'a ArgMatches<'a>> for UninstallMode {
    fn from(m: &ArgMatches) -> Self {
        m.value_of("KEEP_LATEST")
         .and_then(|s| s.parse().ok())
         .map(Self::KeepLatest)
         .unwrap_or(Self::Single)
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn start(ui: &mut UI,
                   ident: &PackageIdent,
                   fs_root_path: &Path,
                   execution_strategy: ExecutionStrategy,
                   mode: UninstallMode,
                   scope: Scope,
                   excludes: &[PackageIdent],
                   uninstall_hook_mode: UninstallHookMode)
                   -> Result<()> {
    match mode {
        UninstallMode::Single => {
            uninstall(ui,
                      ident,
                      fs_root_path,
                      execution_strategy,
                      scope,
                      excludes,
                      uninstall_hook_mode,
                      UninstallSafety::Safe).await
        }
        UninstallMode::KeepLatest(number_latest_to_keep) => {
            uninstall_all_but_latest(ui,
                                     ident,
                                     number_latest_to_keep,
                                     fs_root_path,
                                     execution_strategy,
                                     scope,
                                     excludes,
                                     uninstall_hook_mode,
                                     UninstallSafety::Safe).await?;
            Ok(())
        }
    }
}
