use super::util::{CacheKeyPath,
                  ConfigOptCacheKeyPath,
                  ConfigOptPkgIdent,
                  ConfigOptRemoteSup,
                  FileOrStdin,
                  PkgIdent,
                  RemoteSup,
                  HABITAT_USER_ENVVAR};
use configopt::ConfigOpt;
use habitat_core::service::ServiceGroup;
use structopt::StructOpt;

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version, name = "config", aliases = &["co", "con", "conf", "confi"])]
/// Commands relating to a Service's runtime config
pub enum SvcConfig {
    #[structopt(aliases = &["ap", "app", "appl"])]
    Apply(SvcConfigApply),
    /// Displays the default configuration options for a service
    #[structopt(aliases = &["sh", "sho"])]
    Show {
        #[structopt(flatten)]
        pkg_ident:  PkgIdent,
        #[structopt(flatten)]
        remote_sup: RemoteSup,
    },
}

/// Sets a configuration to be shared by members of a Service Group
#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version, name = "apply", rename_all = "screamingsnake")]
pub struct SvcConfigApply {
    /// Target service group service.group[@organization] (ex: redis.default or
    /// foo.default@bazcorp)
    #[structopt()]
    pub service_group:  ServiceGroup,
    /// A version number (positive integer) for this configuration (ex: 42)
    #[structopt()]
    pub version_number: u64,
    /// Path to a local file on disk or "-" to read from stdin.
    #[structopt(default_value = "-")]
    pub file:           FileOrStdin,
    /// Name of a user key to use for encryption
    #[structopt(short = "u", long = "user", env = HABITAT_USER_ENVVAR)]
    pub user:           Option<String>,
    #[structopt(flatten)]
    pub remote_sup:     RemoteSup,
    #[structopt(flatten)]
    pub cache_key_path: CacheKeyPath,
}
