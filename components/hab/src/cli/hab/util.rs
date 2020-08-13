use crate::{cli::valid_fully_qualified_ident,
            config,
            error::Error};
use configopt::{self,
                ConfigOpt};
use habitat_common::types::ListenCtlAddr;
use habitat_core::{crypto::CACHE_KEY_PATH_ENV_VAR,
                   env as henv,
                   fs as hab_core_fs,
                   origin::Origin,
                   package::PackageIdent,
                   url::{bldr_url_from_env,
                         DEFAULT_BLDR_URL},
                   AUTH_TOKEN_ENVVAR};
use lazy_static::lazy_static;
use std::{ffi::OsString,
          fmt,
          io,
          net::{SocketAddr,
                ToSocketAddrs},
          num::ParseIntError,
          path::PathBuf,
          str::FromStr,
          time::Duration};
use structopt::{clap::AppSettings,
                StructOpt};
use url::{ParseError,
          Url};

#[derive(ConfigOpt, StructOpt)]
#[configopt(derive(Serialize))]
#[structopt(no_version)]
pub struct AuthToken {
    /// Authentication token for Builder.
    // TODO (JM): This should probably use `env`
    #[structopt(name = "AUTH_TOKEN", short = "z", long = "auth")]
    pub value: Option<String>,
}

#[derive(ConfigOpt, StructOpt, Deserialize)]
#[configopt(derive(Serialize))]
#[structopt(no_version)]
pub struct BldrUrl {
    /// Specify an alternate Builder endpoint. If not specified, the value will be
    /// taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
    // TODO (DM): This should probably use `env` and `default_value`
    #[structopt(name = "BLDR_URL", short = "u", long = "url")]
    pub value: Option<Url>,
}

#[derive(ConfigOpt, StructOpt, Deserialize, Serialize)]
#[structopt(no_version)]
#[configopt(derive(Serialize))]
pub struct BldrOrigin {
    /// The Builder origin name to target
    #[structopt(name = "ORIGIN", short = "o", long = "origin")]
    pub inner: Origin,
}

fn bldr_url_from_env_load_or_default() -> String {
    bldr_url_from_env().unwrap_or_else(|| {
                           match config::load() {
                               Ok(config) => {
                                   config.bldr_url
                                         .unwrap_or_else(|| DEFAULT_BLDR_URL.to_string())
                               }
                               Err(e) => {
                                   error!("Found a cli.toml but unable to load it. Resorting to \
                                           default BLDR_URL: {}",
                                          e);
                                   DEFAULT_BLDR_URL.to_string()
                               }
                           }
                       })
}

pub fn bldr_url_from_args_env_load_or_default(opt: Option<Url>) -> Result<Url, ParseError> {
    if let Some(url) = opt {
        Ok(url)
    } else {
        Url::parse(&bldr_url_from_env_load_or_default())
    }
}

pub fn bldr_auth_token_from_args_env_or_load(opt: Option<String>) -> Result<String, Error> {
    if let Some(token) = opt {
        Ok(token)
    } else {
        match henv::var(AUTH_TOKEN_ENVVAR) {
            Ok(v) => Ok(v),
            Err(_) => {
                config::load()?.auth_token.ok_or_else(|| {
                                              Error::ArgumentError("No auth token specified. \
                                                                    Please check that you have \
                                                                    specified a valid Personal \
                                                                    Access Token with:  -z, \
                                                                    --auth <AUTH_TOKEN>"
                                                                                        .into())
                                          })
            }
        }
    }
}

pub fn maybe_bldr_auth_token_from_args_or_load(opt: Option<String>) -> Option<String> {
    bldr_auth_token_from_args_env_or_load(opt).ok()
}

lazy_static! {
    pub static ref CACHE_KEY_PATH_DEFAULT: String =
        hab_core_fs::CACHE_KEY_PATH.to_string_lossy().to_string();
}

#[derive(ConfigOpt, StructOpt, Debug, Deserialize)]
#[configopt(derive(Serialize, Debug), attrs(serde))]
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
#[configopt(derive(Serialize, Clone, Debug))]
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

// Collect trailing arguments to pass to an external command
//
// This disables help and version flags for the subcommand. Making it easy to check the help or
// version of the external command. See `ExternalCommandArgsWithHelpAndVersion` for more details.
#[derive(ConfigOpt, StructOpt)]
#[configopt(derive(Serialize))]
#[structopt(no_version, rename_all = "screamingsnake",
            settings = &[AppSettings::TrailingVarArg,
                         AppSettings::AllowLeadingHyphen,
                         AppSettings::DisableHelpFlags,
                         AppSettings::DisableHelpSubcommand,
                         AppSettings::DisableVersion
                        ])]
pub struct ExternalCommandArgs {
    /// Arguments to the command
    #[structopt(parse(from_os_str), takes_value = true, multiple = true)]
    pub args: Vec<OsString>,
}

// Collect trailing arguments to pass to an external command
//
// This is useful when you have a subcommand that has more arguments than just "external command
// args" because it allows showing the help of the subcommand. Consider:
//
// 1. hab pkg exec --help
// 2. hab pkg exec core/redis ls --help
// 3. hab pkg exec core/redis ls -- --help
//
// If we were to use `ExternalCommandArgs` #1 would produce an error due to missing args instead of
// displaying the help because the help is disabled. #2 is ambiguous. Should it show the help of the
// subcommand or of `ls`? In this case it will show the help of the subcommand. If we want to see
// the help of `ls` we can use #3.
#[derive(ConfigOpt, StructOpt)]
#[configopt(derive(Serialize))]
#[structopt(no_version, rename_all = "screamingsnake",
            settings = &[AppSettings::TrailingVarArg,
                         AppSettings::AllowLeadingHyphen,
                        ])]
pub struct ExternalCommandArgsWithHelpAndVersion {
    /// Arguments to the command
    #[structopt(parse(from_os_str), takes_value = true, multiple = true)]
    pub args: Vec<OsString>,
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
