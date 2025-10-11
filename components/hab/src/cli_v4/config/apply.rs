use crate::{cli_v4::utils::{CacheKeyPath,
                            RemoteSup},
            command::config::sub_svc_set,
            error::Result as HabResult};
use clap::Parser;
use clap_v4 as clap;
use habitat_common::{cli::clap_validators::FileExistsOrStdinValueParser,
                     ui::UI};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          rename_all = "kebab-case",
          help_template = "{name} {version} {author-section} {about-section}\n{usage-heading} \
                           {usage}\n\n{all-args}\n",
          about = "Sets a configuration to be shared by members of a Service Group")]
pub(crate) struct ConfigApplyOptions {
    #[command(flatten)]
    cache_key_path: CacheKeyPath,

    /// Supervisor control address (overrides HAB_SUP_CTL_ADDR)
    #[command(flatten)]
    remote_sup: RemoteSup,

    /// Name of a user key to use for encryption
    #[arg(short = 'u', long, value_name = "USER")]
    user: Option<String>,

    /// Target service group service.group[@organization] (ex: redis.default or
    /// foo.default@bazcorp)
    #[arg(value_name = "SERVICE_GROUP")]
    service_group: String,

    /// A version number (positive integer) for this configuration (ex: 42)
    #[arg(value_name = "VERSION_NUMBER", value_parser = clap::value_parser!(u64))]
    config_version: u64,

    /// Path to local file on disk (ex: /tmp/config.toml, "-" for stdin)
    #[arg(value_parser = FileExistsOrStdinValueParser, value_name = "FILE", default_value = "-")]
    file: String,
}

impl ConfigApplyOptions {
    pub(crate) async fn do_apply(&self, ui: &mut UI) -> HabResult<()> {
        let service_group = self.service_group
                                .parse()
                                .expect("Invalid service group identifier");

        sub_svc_set(ui,
                    service_group,
                    &self.file,
                    self.config_version,
                    self.user.clone(),
                    self.remote_sup.inner(),
                    (&self.cache_key_path).into()).await
    }
}
