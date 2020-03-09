use super::{ExecutionStrategy,
            Scope};
use crate::{command::pkg::uninstall_impl,
            error::Result};
use habitat_common::ui::UI;
use habitat_core::package::PackageIdent;
use std::path::Path;

pub async fn start(ui: &mut UI,
                   ident: &PackageIdent,
                   fs_root_path: &Path,
                   execution_strategy: ExecutionStrategy,
                   scope: Scope,
                   excludes: &[PackageIdent])
                   -> Result<()> {
    uninstall_impl::uninstall(ui,
                              ident,
                              fs_root_path,
                              execution_strategy,
                              scope,
                              excludes,
                              false).await
}
