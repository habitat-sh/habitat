use clap_v4 as clap;

use clap::Parser;

use habitat_core::service::ServiceGroup;

use crate::cli_v4::utils::CacheKeyPath;

/// Commands relating to Habitat service keys
#[derive(Clone, Debug, Parser)]
#[command(author = "\nThe Habitat Maintainers <humans@habitat.sh>",
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub enum KeyCommand {
    Generate(KeyGenerate),
}

/// Generates a Habitat service key
#[derive(Clone, Debug, Parser)]
#[command(author = "\nThe Habitat Maintainers <humans@habitat.sh>",
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub struct KeyGenerate {
    /// Target service group service.group[@organization] (ex: redis.default or
    /// foo.default@bazcorp)
    #[arg(name = "SERVICE_GROUP")]
    service_group: ServiceGroup,

    /// The service organization
    #[structopt(name = "ORG")]
    org: Option<String>,

    #[command(flatten)]
    cache_key_path: CacheKeyPath,
}
