use crate::cli::valid_fully_qualified_ident;
use configopt::{self,
                ConfigOpt};
use habitat_core::{crypto::CACHE_KEY_PATH_ENV_VAR,
                   fs as hab_core_fs,
                   package::PackageIdent};
use lazy_static::lazy_static;
use std::{io,
          net::{SocketAddr,
                ToSocketAddrs},
          path::PathBuf};
use structopt::StructOpt;
use url::Url;

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
#[allow(dead_code)]
pub struct AuthToken {
    /// Authentication token for Builder
    #[structopt(name = "AUTH_TOKEN", short = "z", long = "auth")]
    auth_token: Option<String>,
}

#[derive(ConfigOpt, StructOpt, Deserialize)]
#[structopt(no_version)]
#[allow(dead_code)]
pub struct BldrUrl {
    /// Specify an alternate Builder endpoint. If not specified, the value will be taken from
    /// the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
    // TODO (DM): This should probably use `env` and `default_value`
    #[structopt(name = "BLDR_URL", short = "u", long = "url")]
    bldr_url: Option<Url>,
}

lazy_static! {
    pub static ref CACHE_KEY_PATH_DEFAULT: String =
        hab_core_fs::CACHE_KEY_PATH.to_string_lossy().to_string();
}

#[derive(ConfigOpt, StructOpt, Debug, Deserialize)]
#[configopt(derive(Debug), attrs(serde))]
#[serde(deny_unknown_fields)]
#[structopt(no_version, rename_all = "screamingsnake")]
pub struct CacheKeyPath {
    /// Cache for creating and searching for encryption keys
    #[structopt(long = "cache-key-path",
                env = CACHE_KEY_PATH_ENV_VAR,
                default_value = &*CACHE_KEY_PATH_DEFAULT)]
    cache_key_path: PathBuf,
}

#[derive(ConfigOpt, StructOpt, Deserialize)]
#[structopt(no_version)]
#[allow(dead_code)]
pub struct PkgIdent {
    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[structopt(name = "PKG_IDENT")]
    pkg_ident: PackageIdent,
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
#[allow(dead_code)]
pub struct FullyQualifiedPkgIdent {
    /// A fully qualified package identifier (ex: core/busybox-static/1.42.2/20170513215502)
    #[structopt(name = "PKG_IDENT", validator = valid_fully_qualified_ident)]
    pkg_ident: PackageIdent,
}

#[derive(ConfigOpt, StructOpt, Deserialize)]
#[structopt(no_version)]
#[allow(dead_code)]
pub struct RemoteSup {
    /// Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]
    #[structopt(name = "REMOTE_SUP", long = "remote-sup", short = "r")]
    remote_sup: Option<SocketAddr>,
}

pub fn socket_addr_with_default_port<S: AsRef<str>>(addr: S,
                                                    default_port: u16)
                                                    -> io::Result<SocketAddr> {
    let addr = addr.as_ref();
    let mut iter = if addr.find(':').is_some() {
        addr.to_socket_addrs()
    } else {
        (addr, default_port).to_socket_addrs()
    }?;
    // We expect exactly one address
    iter.next().ok_or_else(|| {
                   io::Error::new(io::ErrorKind::InvalidInput,
                                  "input did not resolve to SocketAddr or error")
               })
}

pub fn socket_addrs_with_default_port<I>(addrs: I, default_port: u16) -> io::Result<Vec<SocketAddr>>
    where I: IntoIterator,
          I::Item: AsRef<str>
{
    addrs.into_iter()
         .map(|a| socket_addr_with_default_port(a, default_port))
         .collect()
}

#[cfg(test)]
mod test {
    use super::{socket_addr_with_default_port,
                socket_addrs_with_default_port};

    #[test]
    fn test_socket_addrs_with_default_port() {
        assert_eq!(socket_addr_with_default_port("127.0.0.1", 89).unwrap(),
                   "127.0.0.1:89".parse().expect(""));
        assert_eq!(socket_addr_with_default_port("1.2.3.4:1500", 89).unwrap(),
                   "1.2.3.4:1500".parse().expect(""));
        assert!(socket_addr_with_default_port("an_invalid_address", 89).is_err());

        let expected = vec!["1.2.3.4:1500".parse().expect(""),
                            "0.0.0.0:5567".parse().expect(""),
                            "127.0.0.1:5567".parse().expect("")];
        assert_eq!(socket_addrs_with_default_port(&["1.2.3.4:1500", "0.0.0.0", "127.0.0.1"], 5567).unwrap(),
                   expected);
        assert!(socket_addrs_with_default_port(&["1.2.3.4:1500",
                                                 "0.0.0.0",
                                                 "an_error",
                                                 "127.0.0.1"],
                                               5567).is_err(),);
    }
}
