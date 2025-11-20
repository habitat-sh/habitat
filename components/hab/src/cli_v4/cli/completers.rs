use crate::{cli_v4::Hab,
            error::Result as HabResult};
use clap::{CommandFactory,
           Parser};
use clap_complete::{Shell,
                    generate};
use clap_v4 as clap;
use habitat_common::FeatureFlag;
use std::io;

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n",
          about = "Creates command-line completers for your shell")]
pub(crate) struct CliCompletersOptions {
    /// The name of the shell you want to generate the command-completion
    #[arg(short = 's', long = "shell", value_name = "SHELL")]
    shell: Shell,
}

impl CliCompletersOptions {
    pub(crate) fn do_completers(&self, _feature_flags: FeatureFlag) -> HabResult<()> {
        let mut cmd = Hab::command();

        generate(self.shell, &mut cmd, "hab", &mut io::stdout());

        Ok(())
    }
}
