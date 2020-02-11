use super::util::CacheKeyPath;
use crate::cli::file_exists_or_stdin;
use habitat_core::{package::PackageIdent,
                   service::ServiceGroup};
use std::net::SocketAddr;
use structopt::StructOpt;

#[derive(StructOpt)]
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
        /// Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]
        #[structopt(name = "REMOTE_SUP", long = "remote-sup", short = "r")]
        remote_sup:     Option<SocketAddr>,
        #[structopt(flatten)]
        cache_key_path: CacheKeyPath,
    },
    /// Displays the default configuration options for a service
    Show {
        /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
        #[structopt(name = "PKG_IDENT")]
        pkg_ident:  PackageIdent,
        /// Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]
        #[structopt(name = "REMOTE_SUP", long = "remote-sup", short = "r")]
        remote_sup: Option<SocketAddr>,
    },
}
