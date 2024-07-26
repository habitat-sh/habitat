// Implementation of `hab pkg upload` command

use clap_v4 as clap;

use std::path::PathBuf;

use clap::{ArgAction,
           Parser};

use habitat_common::ui::UI;

use habitat_core::{crypto::keys::KeyCache,
                   ChannelIdent};

use habitat_api_client::BuildOnUpload;

use crate::{cli_v4::utils::{AuthToken,
                            BldrUrl,
                            CacheKeyPath},
            command::pkg::upload,
            error::Result as HabResult};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true)]
pub(crate) struct PkgUploadOptions {
    #[command(flatten)]
    bldr_url: BldrUrl,

    #[command(flatten)]
    auth_token: AuthToken,

    /// Optional additional release channel to upload package to. Packages are always uploaded
    /// to `unstable`, regardless of the value of this option
    #[structopt(name = "CHANNEL", short = 'c', long = "channel")]
    channel: Option<ChannelIdent>,

    /// Skips checking availability of package and force uploads, potentially overwriting a
    /// stored copy of a package. (default: false)
    #[structopt(name = "FORCE", long = "force", action = ArgAction::SetTrue)]
    force: bool,

    /// Disable auto-build for all packages in this upload
    #[structopt(name = "NO_BUILD", long = "no-build", action = ArgAction::SetTrue)]
    no_build: bool,

    /// One or more filepaths to a Habitat Artifact (ex:
    /// /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
    #[structopt(name = "HART_FILE", required = true)]
    hart_file: Vec<PathBuf>,

    #[structopt(flatten)]
    cache_key_path: CacheKeyPath,
}

impl PkgUploadOptions {
    pub(crate) async fn do_upload(&self, ui: &mut UI) -> HabResult<()> {
        let auth_token = self.auth_token.from_cli_or_config()?;

        let auto_build = if self.no_build {
            BuildOnUpload::Disable
        } else {
            BuildOnUpload::PackageDefault
        };

        let key_cache = KeyCache::new::<PathBuf>((&self.cache_key_path).into());

        for hart_file in &self.hart_file {
            upload::start(ui,
                          &self.bldr_url.to_string(),
                          &self.channel,
                          &auth_token,
                          hart_file,
                          self.force,
                          auto_build,
                          &key_cache).await?;
        }
        Ok(())
    }
}
