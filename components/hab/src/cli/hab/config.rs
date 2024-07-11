#![allow(dead_code)]

use super::util::{CacheKeyPath,
                  PkgIdent,
                  RemoteSup};
use crate::cli::file_exists_or_stdin;
use habitat_core::service::ServiceGroup;

use clap::Parser;

#[derive(Parser)]
/// Commands relating to a Service's runtime config
pub enum ServiceConfig {
    Apply(ServiceConfigApply),
    Show(ServiceConfigShow),
}

/// Sets a configuration to be shared by members of a Service Group
#[derive(Parser)]
pub struct ServiceConfigApply {
    /// Target service group service.group[@organization] (ex: redis.default or
    /// foo.default@bazcorp)
    #[clap(long)]
    service_group: ServiceGroup,

    /// A version number (positive integer) for this configuration (ex: 42)
    #[clap(long)]
    version_number: i64,

    /// Path to local file on disk (ex: /tmp/config.toml, default: <stdin>)
    #[structopt(validator = file_exists_or_stdin)]
    file: Option<String>,

    /// Name of a user key to use for encryption
    #[clap(short = "u", long = "user")]
    user: Option<String>,

    #[command(flatten)]
    remote_sup: RemoteSup,

    #[command(flatten)]
    cache_key_path: CacheKeyPath,
}

/// Displays the default configuration options for a service
#[derive(Parser)]
pub struct ServiceConfigShow {
    #[command(flatten)]
    pkg_ident: PkgIdent,

    #[command(flatten)]
    remote_sup: RemoteSup,
}
