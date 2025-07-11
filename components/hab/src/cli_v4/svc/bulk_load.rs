use clap_v4 as clap;

use clap::Parser;
use std::path::PathBuf;

/// Load services using the service config files from the specified paths
#[derive(Clone, Debug, Parser)]
#[command(author = "\nThe Habitat Maintainers <humans@habitat.sh>",
          name = "bulkload",
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub struct BulkLoadCommand {
    /// Paths to files or directories of service config files
    #[arg(long = "svc-config-paths",
          default_value = "/hab/sup/default/config/svc")]
    pub svc_config_paths: Vec<PathBuf>,
}
