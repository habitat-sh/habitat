// CLI V4 specific functionality

use clap_v4 as clap;

use clap::Parser;

use hab::SupRunOptions;

use habitat_common::{liveliness_checker,
                     outputln,
                     FeatureFlag};
use habitat_sup::error::{Error,
                         Result};

static LOGKEY: &str = "MN";

#[derive(Debug, Clone, Parser)]
#[command(name = "hab-sup",
            version = habitat_sup::VERSION,
            about = "Patents: https://chef.io/patents\n\"A Habitat is the natural environment for your services\" - Alan Turing",
            author = "\nThe Habitat Maintainers <humans@habitat.sh>",
            arg_required_else_help = true,
            propagate_version = true,
            term_width = 100,
            help_template = "{name} {version} {author-section} {about-section} \
                    \n{usage-heading} {usage}\n\n{all-args}",
        )]
pub(crate) enum HabSup {
    /// Start an interactive Bash-like shell
    #[cfg(any(all(target_os = "linux",
                  any(target_arch = "x86_64", target_arch = "aarch64")),
              all(target_os = "windows", target_arch = "x86_64"),))]
    #[command(aliases = &["b", "ba", "bas"])]
    Bash,

    /// Run the Habitat Supervisor
    #[command(aliases = &["r", "ru"])]
    Run(SupRunOptions),

    /// Start an interactive Bourne-like shell
    #[cfg(any(all(target_os = "linux",
                  any(target_arch = "x86_64", target_arch = "aarch64")),
              all(target_os = "windows", target_arch = "x86_64"),))]
    #[command()]
    Sh,

    /// Gracefully terminate the Habitat Supervisor and all of its running services
    #[command(aliases = &["ter"])]
    Term,
}

pub(crate) async fn start_rsr_imlw_mlw_gsw_smw_rhw_msw(feature_flags: FeatureFlag) -> Result<()> {
    if feature_flags.contains(FeatureFlag::TEST_BOOT_FAIL) {
        outputln!("Simulating boot failure");
        return Err(Error::TestBootFail);
    }
    liveliness_checker::spawn_thread_alive_checker();
    let launcher = crate::cli_common::boot();

    let hab_sup = crate::cli_v4::cli();

    Ok(())
}

pub(crate) fn cli() -> HabSup { HabSup::parse() }
