use clap_v4 as clap;

use std::path::PathBuf;

use clap::Parser;

use habitat_common::ui::UI;
use habitat_core::{crypto::keys::KeyCache,
                   service::ServiceGroup};

use crate::{cli_v4::utils::CacheKeyPath,
            command::service::key::generate::start,
            error::Result as HabResult};

/// Commands relating to Habitat service keys
#[derive(Clone, Debug, Parser)]
#[command(author = "\nThe Habitat Maintainers <humans@habitat.sh>",
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) enum KeyCommand {
    Generate(KeyGenerate),
}

/// Generates a Habitat service key
#[derive(Clone, Debug, Parser)]
#[command(author = "\nThe Habitat Maintainers <humans@habitat.sh>",
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct KeyGenerate {
    /// Target service group service.group[@organization] (ex: redis.default or
    /// foo.default@bazcorp)
    #[arg(name = "SERVICE_GROUP")]
    service_group: ServiceGroup,

    /// The service organization
    #[structopt(name = "ORG", env = "HABITAT_ORG")]
    org: String,

    #[command(flatten)]
    cache_key_path: CacheKeyPath,
}

impl KeyGenerate {
    pub(crate) async fn do_command(&self, ui: &mut UI) -> HabResult<()> {
        let key_cache = KeyCache::new::<PathBuf>((&self.cache_key_path).into());
        key_cache.setup()?;

        start(ui, &self.org, &self.service_group, &key_cache)
    }
}
