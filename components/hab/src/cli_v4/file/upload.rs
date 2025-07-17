use crate::{cli_v4::utils::{CacheKeyPath,
                            RemoteSup},
            command::file::sub_file_put,
            error::Result as HabResult};
use clap::Parser;
use clap_v4 as clap;
use habitat_common::ui::UI;
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          rename_all = "kebab-case",
          help_template = "{name} {version} {author-section} {about-section}\n{usage-heading} \
                           {usage}\n\n{all-args}\n",
          about = "Uploads a file to be shared between members of a Service Group")]
pub(crate) struct FileUploadOptions {
    /// Target service group service.group[@organization] (ex: redis.default or
    /// foo.default@bazcorp)
    #[arg(value_name = "SERVICE_GROUP")]
    service_group: String,

    /// A version number (positive integer) for this file (ex: 42)
    #[arg(value_name = "VERSION_NUMBER", value_parser = clap::value_parser!(u64))]
    upload_version: u64,

    /// Path to local file on disk
    #[arg(value_name = "FILE", value_parser = clap::value_parser!(PathBuf))]
    file: PathBuf,

    #[command(flatten)]
    cache_key_path: CacheKeyPath,

    /// Supervisor control address (overrides HAB_SUP_CTL_ADDR)
    #[command(flatten)]
    remote_sup: RemoteSup,

    /// Name of the user key
    #[arg(short = 'u', long, value_name = "USER")]
    user: Option<String>,
}

impl FileUploadOptions {
    pub(crate) async fn do_upload(&self, ui: &mut UI) -> HabResult<()> {
        sub_file_put(&self.service_group,
                     self.upload_version,
                     &self.file,
                     self.user.clone(),
                     self.remote_sup.inner().cloned(),
                     (&self.cache_key_path).into(),
                     ui).await
    }
}
