#![allow(dead_code)]

use super::util::{CacheKeyPath,
                  PkgIdent,
                  RemoteSup};
use crate::cli::file_exists_or_stdin;
use habitat_core::service::ServiceGroup;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(no_version)]
/// Commands relating to a Service's runtime config
pub enum ServiceConfig {
    Apply(ServiceConfigApply),
    Show(ServiceConfigShow),
}

/// Sets a configuration to be shared by members of a Service Group
#[derive(StructOpt)]
#[structopt(name = "apply", no_version, rename_all = "screamingsnake")]
pub struct ServiceConfigApply {
    /// Target service group service.group[@organization] (ex: redis.default or
    /// foo.default@bazcorp)
    #[structopt()]
    service_group:  ServiceGroup,
    /// A version number (positive integer) for this configuration (ex: 42)
    #[structopt()]
    version_number: i64,
    /// Path to local file on disk (ex: /tmp/config.toml, default: <stdin>)
    #[structopt(validator = file_exists_or_stdin)]
    file:           Option<String>,
    /// Name of a user key to use for encryption
    #[structopt(short = "u", long = "user")]
    user:           Option<String>,
    #[structopt(flatten)]
    remote_sup:     RemoteSup,
    #[structopt(flatten)]
    cache_key_path: CacheKeyPath,
}

/// Displays the default configuration options for a service
#[derive(StructOpt)]
#[structopt(name = "show", no_version, rename_all = "screamingsnake")]
pub struct ServiceConfigShow {
    #[structopt(flatten)]
    pkg_ident:  PkgIdent,
    #[structopt(flatten)]
    remote_sup: RemoteSup,
}
