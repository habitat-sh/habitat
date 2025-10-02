use clap_v4 as clap;

use clap::Parser;

use habitat_common::ui::UI;
use habitat_core::origin::Origin;

use crate::error::Result as HabResult;

use crate::cli_v4::utils::origin_param_or_env;

#[derive(Debug, Clone, Parser)]
#[command(rename_all = "kebab-case",
          help_template = "{name} {version} {author-section} {about-section}\n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PlanInit {
    /// Name for the new app
    #[arg(name = "PKG_NAME")]
    pkg_name: Option<String>,

    /// Origin for the new app
    #[arg(name = "ORIGIN", short = 'o', long = "origin", value_parser = clap::value_parser!(Origin))]
    origin: Option<Origin>,

    /// Create a minimal plan file
    #[arg(name = "MIN", short = 'm', long = "min")]
    min: bool,

    /// Specify explicit Scaffolding for your app (ex: node, ruby)
    #[arg(name = "SCAFFOLDING", short = 's', long = "scaffolding")]
    scaffolding: Option<String>,
}

impl PlanInit {
    pub(crate) fn do_command(&self, ui: &mut UI) -> HabResult<()> {
        let origin = origin_param_or_env(&self.origin)?;

        // Only call scaffold_check if scaffolding was explicitly provided
        let scaffolding = if self.scaffolding.is_some() {
            crate::scaffolding::scaffold_check(ui, self.scaffolding.as_deref())?
        } else {
            None // Don't auto-detect if no scaffolding was specified
        };

        crate::command::plan::init::start(ui, &origin, self.min, scaffolding, self.pkg_name.clone())
    }
}
