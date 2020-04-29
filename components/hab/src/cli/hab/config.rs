use super::util::{CacheKeyPath,
                  ConfigOptCacheKeyPath,
                  ConfigOptPkgIdent,
                  ConfigOptRemoteSup,
                  PkgIdent,
                  RemoteSup};
use crate::cli::file_exists_or_stdin;
use configopt::ConfigOpt;
use habitat_core::service::ServiceGroup;
use structopt::StructOpt;

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Commands relating to a Service's runtime config
pub enum ServiceConfig {
    /// Sets a configuration to be shared by members of a Service Group
    Apply {
        /// Target service group service.group[@organization] (ex: redis.default or
        /// foo.default@bazcorp)
        #[structopt(name = "SERVICE_GROUP")]
        service_group:  ServiceGroup,
        /// A version number (positive integer) for this configuration (ex: 42)
        #[structopt(name = "VERSION_NUMBER")]
        version_number: i64,
        /// Path to local file on disk (ex: /tmp/config.toml, default: <stdin>)
        #[structopt(name = "FILE", validator = file_exists_or_stdin)]
        file:           Option<String>,
        /// Name of a user key to use for encryption
        #[structopt(name = "USER", short = "u", long = "user")]
        user:           Option<String>,
        #[structopt(flatten)]
        remote_sup:     RemoteSup,
        #[structopt(flatten)]
        cache_key_path: CacheKeyPath,
    },
    /// Displays the default configuration options for a service
    Show {
        #[structopt(flatten)]
        pkg_ident:  PkgIdent,
        #[structopt(flatten)]
        remote_sup: RemoteSup,
    },
}
