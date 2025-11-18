// Implementation of `hab pkg sign` command

use clap_v4 as clap;

use std::path::PathBuf;

use clap::Parser;

use habitat_core::{crypto,
                   crypto::keys::KeyCache,
                   origin::Origin};

use habitat_common::{cli::clap_validators::FileExistsValueParser,
                     ui::UI};

use crate::{cli_v4::utils::{CacheKeyPath,
                            origin_param_or_env},
            command::pkg::sign,
            error::Result as HabResult};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PkgSignOptions {
    /// Origin key used to create signature
    #[arg(name = "ORIGIN", long = "origin", value_parser = clap::value_parser!(Origin))]
    origin: Option<Origin>,

    /// A path to a source archive file (ex: /home/acme-redis-3.0.7-21120102031201.tar.xz)
    #[arg(name = "SOURCE", value_parser = FileExistsValueParser)]
    source: PathBuf,

    /// The destination path to the signed Habitat Artifact (ex:
    /// /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
    #[arg(name = "DEST")]
    dest: PathBuf,

    #[command(flatten)]
    cache_key_path: CacheKeyPath,
}

impl PkgSignOptions {
    pub(crate) fn do_sign(&self, ui: &mut UI) -> HabResult<()> {
        let origin = origin_param_or_env(&self.origin)?;

        crypto::init()?;
        let key_cache = KeyCache::new::<PathBuf>((&self.cache_key_path).into());
        let key = key_cache.latest_secret_origin_signing_key(&origin)?;
        sign::start(ui,
                    &key,
                    &Into::<PathBuf>::into(self.source.clone()),
                    &self.dest)
    }
}
