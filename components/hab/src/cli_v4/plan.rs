use clap_v4 as clap;

use clap::Parser;

use habitat_common::ui::UI;

use crate::error::Result as HabResult;

mod init;
use init::PlanInit;

mod render;
use render::PlanRender;

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          rename_all = "kebab-case",
          help_template = "{name} {version} {author-section} {about-section}\n{usage-heading} \
                           {usage}\n\n{all-args}\n",
          about = "Sets a configuration to be shared by members of a Service Group")]
pub(crate) enum PlanCommand {
    /// Generates common package specific configuration files.
    ///
    /// Executing without argument will create a `habitat` directory in your current folder for the
    /// plan. If `PKG_NAME` is specified it will create a folder with that name. Environment
    /// variables (those starting with 'pkg_') that are set will be used in the generated plan
    Init(PlanInit),

    /// Renders plan config files.
    Render(PlanRender),
}

impl PlanCommand {
    pub(crate) async fn do_command(&self, ui: &mut UI) -> HabResult<()> {
        match self {
            Self::Init(plan_init) => plan_init.do_command(ui),
            Self::Render(plan_render) => plan_render.do_command(ui),
        }
    }
}
