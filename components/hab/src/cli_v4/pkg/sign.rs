// Implementation of `hab pkg sign` command

use clap_v4 as clap;

use std::path::PathBuf;

use clap::Parser;

use habitat_core::{crypto,
                   crypto::keys::KeyCache,
                   origin::Origin};

use habitat_common::{cli_config::CliConfig,
                     ui::UI};

use crate::{cli_v4::utils::CacheKeyPath,
            command::pkg::sign,
            error::{Error as HabError,
                    Result as HabResult}};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true)]
pub(crate) struct PkgSignOptions {
    /// Origin key used to create signature
    #[arg(name = "ORIGIN", long = "origin", env=crate::ORIGIN_ENVVAR)]
    origin: Option<Origin>,

    /// A path to a source archive file (ex: /home/acme-redis-3.0.7-21120102031201.tar.xz)
    #[structopt(name = "SOURCE")]
    source: PathBuf,

    /// The destination path to the signed Habitat Artifact (ex:
    /// /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
    #[structopt(name = "DEST")]
    dest: PathBuf,

    #[structopt(flatten)]
    cache_key_path: CacheKeyPath,
}

impl PkgSignOptions {
    pub(crate) fn do_sign(&self, ui: &mut UI) -> HabResult<()> {
        let origin = match &self.origin {
            Some(origin) => origin.clone(),
            None => {
                CliConfig::load()?.origin.ok_or_else(|| {
                                              HabError::CryptoCLI("No origin specified".to_string())
                                          })?
            }
        };

        crypto::init()?;
        let key_cache = KeyCache::new::<PathBuf>((&self.cache_key_path).into());
        let key = key_cache.latest_secret_origin_signing_key(&origin)?;
        sign::start(ui, &key, &self.source, &self.dest)
    }
}
