// Implemenatation of `hab pkg bulkupload`

use clap_v4 as clap;

use std::path::PathBuf;

use clap::{ArgAction,
           Parser};

use habitat_common::{cli::clap_validators::DirExistsValueParser,
                     ui::UI};

use habitat_core::{crypto::keys::KeyCache,
                   ChannelIdent};

use habitat_api_client::BuildOnUpload;

use crate::{cli_v4::utils::{AuthToken,
                            BldrUrl},
            command::pkg::bulkupload,
            error::Result as HabResult};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PkgBulkUploadOptions {
    #[command(flatten)]
    bldr_url: BldrUrl,

    #[command(flatten)]
    auth_token: AuthToken,

    /// Optional additional release channel to upload package to. Packages are always uploaded
    /// to `unstable`, regardless of the value of this option
    #[arg(name = "CHANNEL", short = 'c', long = "channel")]
    channel: Option<ChannelIdent>,

    /// Skip checking availability of package and force uploads, potentially overwriting a
    /// stored copy of a package
    #[arg(name = "FORCE", long = "force", action = ArgAction::SetTrue)]
    force: bool,

    // TODO: This option is to be removed?
    /// Enable auto-build for all packages in this upload. Only applicable to SaaS Builder
    #[arg(name = "AUTO_BUILD", long = "auto-build", hide = true, action = ArgAction::SetTrue)]
    auto_build: bool,

    /// Skip the confirmation prompt and automatically create origins that do not exist in the
    /// target Builder
    #[arg(name = "AUTO_CREATE_ORIGINS", long = "auto-create-origins", action = ArgAction::SetTrue)]
    auto_create_channels: bool,

    // TODO: Add Path Exists validator.
    /// Directory Path from which artifacts will be uploaded
    #[arg(name = "UPLOAD_DIRECTORY", value_parser = DirExistsValueParser)]
    upload_directory: PathBuf,
}

impl PkgBulkUploadOptions {
    pub(super) async fn do_bulkupload(&self, ui: &mut UI) -> HabResult<()> {
        let artifact_path = self.upload_directory.join("artifacts");
        let key_path = self.upload_directory.join("keys");
        let key_cache = KeyCache::new(key_path);
        key_cache.setup()?;

        let auto_build = if self.auto_build {
            BuildOnUpload::PackageDefault
        } else {
            BuildOnUpload::Disable
        };

        let auth_token = self.auth_token.from_cli_or_config()?;

        bulkupload::start(ui,
                          &self.bldr_url.to_string(),
                          &self.channel,
                          &auth_token,
                          &artifact_path,
                          self.force,
                          auto_build,
                          self.auto_create_channels,
                          &key_cache).await
    }
}
