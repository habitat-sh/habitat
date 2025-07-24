// Implementation of `hab plan init` subcommand
use std::str::FromStr;

use clap_v4 as clap;

use clap::Parser;

use habitat_common::{cli::clap_validators::HabPkgIdentValueParser,
                     ui::UI};
use habitat_core::package::PackageIdent;

use crate::error::Result as HabResult;

use crate::cli_v4::utils::{origin_param_or_env,
                           valid_origin};

#[derive(Debug, Clone, Parser)]
#[command(rename_all = "kebab-case",
          help_template = "{name} {version} {author-section} {about-section}\n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PlanInit {
    /// Name for the new app
    #[arg(name = "PKG_NAME")]
    pkg_name: Option<String>,

    /// Origin for the new app
    #[arg(name = "ORIGIN", short = 'o', long = "origin", value_parser = valid_origin)]
    origin: Option<String>,

    /// Create a minimal plan file
    #[arg(name = "MIN", short = 'm', long = "min")]
    min: bool,

    /// Specify explicit Scaffolding for your app (ex: node, ruby)
    #[arg(name = "SCAFFOLDING", short = 's', long = "scaffolding", value_parser = HabPkgIdentValueParser::simple())]
    scaffolding: Option<String>,
}

impl PlanInit {
    pub(crate) fn do_command(&self, ui: &mut UI) -> HabResult<()> {
        let origin = origin_param_or_env(&self.origin)?;
        let scaffolding = if cfg!(windows) {
            self.scaffolding
                .as_deref()
                .map(|s| PackageIdent::from_str(s).unwrap())
        } else {
            crate::scaffolding::scaffold_check(ui, self.scaffolding.as_deref())?
        };

        crate::command::plan::init::start(ui, &origin, self.min, scaffolding, self.pkg_name.clone())
    }
}
