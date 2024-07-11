#![allow(dead_code)]
use super::util::{CacheKeyPath,
                  RemoteSup};
use crate::cli::file_exists;
use habitat_core::service::ServiceGroup;

use clap::Parser;

#[derive(Parser)]
/// Commands relating to Habitat files
pub enum File {
    Upload(FileUpload),
}

/// Uploads a file to be shared between members of a Service Group
#[derive(Parser)]
pub struct FileUpload {
    /// Target service group service.group[@organization] (ex: redis.default or
    /// foo.default@bazcorp)
    #[clap(name = "SERVICE_GROUP")]
    service_group:  ServiceGroup,
    /// A version number (positive integer) for this configuration (ex: 42)
    #[clap(name = "VERSION_NUMBER")]
    version_number: i64,
    /// Path to local file on disk
    #[clap(name = "FILE", validator = file_exists)]
    file:           String,
    /// Name of the user key
    #[clap(name = "USER", short = "u", long = "user")]
    user:           Option<String>,

    #[command(flatten)]
    remote_sup: RemoteSup,

    #[command(flatten)]
    cache_key_path: CacheKeyPath,
}
