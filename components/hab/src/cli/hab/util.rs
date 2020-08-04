use crate::cli::valid_fully_qualified_ident;
use configopt::{self,
                ConfigOpt};
use habitat_common::types::ListenCtlAddr;
use habitat_core::{crypto::CACHE_KEY_PATH_ENV_VAR,
                   fs as hab_core_fs,
                   package::PackageIdent};
use lazy_static::lazy_static;
use std::{fmt,
          io,
          net::{SocketAddr,
                ToSocketAddrs},
          num::ParseIntError,
          path::PathBuf,
          str::FromStr,
          time::Duration};
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
#[configopt(derive(Serialize))]
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
    pub cache_key_path: PathBuf,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "&str", into = "String")]
struct PkgIdentStringySerde(PackageIdent);

impl FromStr for PkgIdentStringySerde {
    type Err = habitat_core::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(Self(s.parse()?)) }
}

impl std::convert::TryFrom<&str> for PkgIdentStringySerde {
    type Error = habitat_core::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> { Self::from_str(s) }
}

impl std::fmt::Display for PkgIdentStringySerde {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", self.0) }
}

impl From<PkgIdentStringySerde> for String {
    fn from(pkg_ident: PkgIdentStringySerde) -> Self { pkg_ident.to_string() }
}

#[derive(Clone, ConfigOpt, Debug, StructOpt, Deserialize, Serialize)]
#[configopt(derive(Clone, Serialize, Debug), attrs(serde))]
#[structopt(no_version)]
#[serde(transparent)]
pub struct PkgIdent {
    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[structopt(name = "PKG_IDENT")]
    pkg_ident: PkgIdentStringySerde,
}

impl PkgIdent {
    pub fn pkg_ident(self) -> PackageIdent { self.pkg_ident.0 }
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
#[allow(dead_code)]
pub struct FullyQualifiedPkgIdent {
    /// A fully qualified package identifier (ex: core/busybox-static/1.42.2/20170513215502)
    #[structopt(name = "PKG_IDENT", validator = valid_fully_qualified_ident)]
    pkg_ident: PackageIdent,
}

#[derive(ConfigOpt, StructOpt, Deserialize, Debug)]
#[configopt(derive(Clone, Debug))]
#[structopt(no_version)]
pub struct RemoteSup {
    /// Address to a remote Supervisor's Control Gateway
    #[structopt(name = "REMOTE_SUP",
                long = "remote-sup",
                short = "r",
                default_value = ListenCtlAddr::default_as_str(),
                parse(try_from_str = ListenCtlAddr::resolve_listen_ctl_addr))]
    #[serde(default)]
    remote_sup: ListenCtlAddr,
}

impl RemoteSup {
    pub fn to_listen_ctl_addr(&self) -> ListenCtlAddr { self.remote_sup }
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

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "u64", into = "u64")]
pub struct DurationProxy(Duration);

impl From<DurationProxy> for u64 {
    fn from(d: DurationProxy) -> Self { d.0.as_secs() }
}

impl From<u64> for DurationProxy {
    fn from(n: u64) -> Self { Self(Duration::from_secs(n)) }
}

impl From<DurationProxy> for Duration {
    fn from(d: DurationProxy) -> Self { d.0 }
}

impl From<Duration> for DurationProxy {
    fn from(d: Duration) -> Self { Self(d) }
}

impl FromStr for DurationProxy {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(s.parse::<u64>()?.into()) }
}

impl fmt::Display for DurationProxy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", u64::from(*self)) }
}

#[cfg(test)]
mod test {
    use super::socket_addr_with_default_port;

    #[test]
    fn test_socket_addr_with_default_port() {
        assert_eq!(socket_addr_with_default_port("127.0.0.1", 89).unwrap(),
                   "127.0.0.1:89".parse().expect(""));
        assert_eq!(socket_addr_with_default_port("1.2.3.4:1500", 89).unwrap(),
                   "1.2.3.4:1500".parse().expect(""));
        assert!(socket_addr_with_default_port("an_invalid_address", 89).is_err());
    }
}
