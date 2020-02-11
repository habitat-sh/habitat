use super::util::CacheKeyPath;
use crate::cli::file_exists;
use habitat_core::service::ServiceGroup;
use std::net::SocketAddr;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(no_version)]
/// Commands relating to Habitat files
pub enum File {
    /// Uploads a file to be shared between members of a Service Group
    Upload {
        /// Target service group service.group[@organization] (ex: redis.default or
        /// foo.default@bazcorp)
        #[structopt(name = "SERVICE_GROUP")]
        service_group:  ServiceGroup,
        /// A version number (positive integer) for this configuration (ex: 42)
        #[structopt(name = "VERSION_NUMBER")]
        version_number: i64,
        /// Path to local file on disk
        #[structopt(name = "FILE", validator = file_exists)]
        file:           String,
        /// Name of the user key
        #[structopt(name = "USER", short = "u", long = "user")]
        user:           Option<String>,
        /// Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]
        #[structopt(name = "REMOTE_SUP", long = "remote-sup", short = "r")]
        remote_sup:     Option<SocketAddr>,
        #[structopt(flatten)]
        cache_key_path: CacheKeyPath,
    },
}
