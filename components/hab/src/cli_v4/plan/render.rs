use std::path::PathBuf;

use clap_v4 as clap;

use clap::Parser;

use habitat_common::{cli::clap_validators::FileExistsValueParser,
                     ui::UI};

use crate::error::Result as HabResult;

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          rename_all = "kebab-case",
          help_template = "{name} {version} {author-section} {about-section}\n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PlanRender {
    /// Path to default.toml
    #[arg(name = "DEFAULT_TOML",
          short = 'd',
          long = "default-toml",
          default_value = "./default.toml")]
    default_toml: PathBuf,

    /// Path to config to render
    #[arg(name = "TEMPLATE_PATH", value_parser = FileExistsValueParser)]
    template_path: PathBuf,

    /// Path to user.toml, defaults to none
    #[arg(name = "USER_TOML", short = 'u', long = "user-toml")]
    user_toml: Option<PathBuf>,

    /// Path to json file with mock data for template, defaults to none
    #[arg(name = "MOCK_DATA", short = 'm', long = "mock-data")]
    mock_data: Option<PathBuf>,

    /// Prints config to STDOUT
    #[arg(name = "PRINT", short = 'p', long = "print")]
    print: bool,

    /// Path to render templates
    #[arg(name = "RENDER_DIR",
          short = 'r',
          long = "render-dir",
          default_value = "./results")]
    render_dir: PathBuf,

    /// Don't write anything to disk, ignores --render-dir
    #[arg(name = "NO_RENDER", short = 'n', long = "no-render")]
    no_render: bool,

    /// Don't print any helper messages.  When used with `--print` will only print config file
    #[arg(name = "QUIET", short = 'q', long = "no-verbose", long = "quiet")]
    quiet: bool,
}

impl PlanRender {
    pub(crate) fn do_command(&self, ui: &mut UI) -> HabResult<()> {
        crate::command::plan::render::start(ui,
                                            &self.template_path,
                                            &self.default_toml,
                                            self.user_toml.as_deref(),
                                            self.mock_data.as_deref(),
                                            self.print,
                                            !self.no_render,
                                            &self.render_dir,
                                            self.quiet)
    }
}
