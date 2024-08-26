// Implementation of `hab pkg uninstall` command
use clap_v4 as clap;

use clap::{ArgAction,
           Parser};

use habitat_core::{fs::FS_ROOT_PATH,
                   package::PackageIdent};

use habitat_common::ui::UI;

use crate::{command::pkg::{uninstall,
                           uninstall::UninstallHookMode,
                           ExecutionStrategy,
                           Scope},
            error::Result as HabResult};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PkgUninstallOptions {
    #[arg(name = "PKG_IDENT")]
    pkg_ident: PackageIdent,

    /// Just show what would be uninstalled, don't actually do it
    #[arg(name = "DRYRUN", short = 'd', long = "dryrun", action = ArgAction::SetTrue)]
    dryrun: bool,

    /// Only keep this number of latest packages uninstalling all others.
    #[arg(name = "KEEP_LATEST", long = "keep-latest")]
    keep_latest: Option<usize>,

    /// Identifier of one or more packages that should not be uninstalled. (ex: core/redis,
    /// core/busybox-static/1.42.2/21120102031201)
    #[arg(name = "EXCLUDE", long = "exclude")]
    exclude: Vec<PackageIdent>,

    /// Don't uninstall dependencies
    #[arg(name = "NO_DEPS", long = "no-deps")]
    no_deps: bool,

    /// Do not run any uninstall hooks
    #[arg(name = "IGNORE_UNINSTALL_HOOK", long = "ignore-uninstall-hook")]
    ignore_uninstall_hook: bool,
}

impl PkgUninstallOptions {
    pub(crate) async fn do_uninstall(&self, ui: &mut UI) -> HabResult<()> {
        let exec_strategy = if self.dryrun {
            ExecutionStrategy::DryRun
        } else {
            ExecutionStrategy::Run
        };

        let uninstall_mode = self.keep_latest.into();

        let scope = if self.no_deps {
            Scope::Package
        } else {
            Scope::PackageAndDependencies
        };

        let uninstall_hook_mode = if self.ignore_uninstall_hook {
            UninstallHookMode::Ignore
        } else {
            UninstallHookMode::default()
        };

        uninstall::start(ui,
                         &self.pkg_ident,
                         &FS_ROOT_PATH,
                         exec_strategy,
                         uninstall_mode,
                         scope,
                         &self.exclude,
                         uninstall_hook_mode).await
    }
}
