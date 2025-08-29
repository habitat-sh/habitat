// Utilities that are used by v4 macros
//
// Note we are duplicating this functionality because trivially using
// `cfg_attr(feature = "v4"),...]` is not easy to make work with existing code. Eventually this
// will be the only `util` left (hope so)

use clap_v4 as clap;

use crate::error::Error;
use clap::{ArgGroup,
           Args,
           Parser};
use lazy_static::lazy_static;
use rustls::pki_types::DnsName;
use url::{ParseError,
          Url};

use habitat_common::{cli_config::CliConfig,
                     types::{GossipListenAddr,
                             ListenCtlAddr,
                             ResolvedListenCtlAddr}};

use habitat_core::{crypto::CACHE_KEY_PATH_ENV_VAR,
                   env as hcore_env,
                   fs::CACHE_KEY_PATH,
                   origin::Origin as CoreOrigin,
                   os::process::ShutdownTimeout,
                   package::PackageIdent,
                   service::ServiceBind,
                   url::{bldr_url_from_env,
                         BLDR_URL_ENVVAR,
                         DEFAULT_BLDR_URL},
                   ChannelIdent,
                   AUTH_TOKEN_ENVVAR};

use hab_common_derive::GenConfig;

use habitat_sup_protocol::types::UpdateCondition;

use crate::error::{Error as HabError,
                   Result as HabResult};

use std::{convert::TryFrom,
          ffi::OsString,
          fmt,
          net::SocketAddr,
          num::ParseIntError,
          path::PathBuf,
          str::FromStr,
          time::Duration};

use serde::{Deserialize,
            Serialize};

use log::error;

use crate::ORIGIN_ENVVAR;

lazy_static! {
    pub(crate) static ref CACHE_KEY_PATH_DEFAULT: String =
        CACHE_KEY_PATH.to_string_lossy().to_string();
    static ref CHANNEL_IDENT_DEFAULT: String = ChannelIdent::default().to_string();
    static ref GROUP_DEFAULT: String = String::from("default");
}

impl GROUP_DEFAULT {
    fn get() -> String { GROUP_DEFAULT.clone() }
}

#[derive(GenConfig)]
#[derive(PartialEq, Debug, Clone, Parser, Serialize, Deserialize)]
pub struct CacheKeyPath {
    /// Cache for creating and searching for encryption keys
    #[arg(long = "cache-key-path",
                env = CACHE_KEY_PATH_ENV_VAR,
                default_value = &*CACHE_KEY_PATH_DEFAULT)]
    #[serde(default = "CacheKeyPath::default_cache_key_path")]
    pub cache_key_path: PathBuf,
}

impl CacheKeyPath {
    pub fn default_cache_key_path() -> PathBuf { CACHE_KEY_PATH.clone() }
}

impl Default for CacheKeyPath {
    fn default() -> Self { Self { cache_key_path: CACHE_KEY_PATH.clone(), } }
}

impl From<PathBuf> for CacheKeyPath {
    fn from(cache_key_path: PathBuf) -> Self { Self { cache_key_path } }
}

impl From<&CacheKeyPath> for PathBuf {
    fn from(cache_key_path: &CacheKeyPath) -> PathBuf { cache_key_path.cache_key_path.clone() }
}

impl FromStr for CacheKeyPath {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(Self { cache_key_path: s.into(), }) }
}

#[derive(Debug, Clone, Parser)]
pub(crate) struct BldrUrl {
    // TODO:agadgil: Use the Url Validator
    /// Specify an alternate Builder endpoint.
    #[arg(name = "BLDR_URL", short = 'u', long = "url")]
    bldr_url: Option<Url>,
}

impl fmt::Display for BldrUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let url = if let Some(url) = &self.bldr_url {
            url.to_string()
        } else {
            match hcore_env::var(BLDR_URL_ENVVAR) {
                Ok(v) => v,
                Err(_) => {
                    // Okay to unwrap it never returns Err!!
                    match CliConfig::load().unwrap().bldr_url {
                        Some(v) => v,
                        None => DEFAULT_BLDR_URL.to_string(),
                    }
                }
            }
        };
        write!(f, "{}", url)
    }
}

impl BldrUrl {
    /// Return the configured Builder URL, falling back to ENV or config.
    pub(crate) fn resolve(&self) -> Result<Url, ParseError> {
        if let Some(ref url) = self.bldr_url {
            Ok(url.clone())
        } else {
            let default = bldr_url_from_env_load_or_default();
            Url::parse(&default)
        }
    }
}

fn bldr_url_from_env_load_or_default() -> String {
    bldr_url_from_env().unwrap_or_else(|| {
                           match CliConfig::load() {
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

#[derive(Debug, Clone, Parser)]
pub(crate) struct AuthToken {
    // TODO: Add Validator for this?
    /// Authentication token for Builder.
    #[arg(name = "AUTH_TOKEN", short = 'z', long = "auth")]
    auth_token: Option<String>,
}

impl AuthToken {
    // This function returns a result. Use this when `auth_token` is required. Either as a command
    // line option or env or from config.
    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn from_cli_or_config(&self) -> HabResult<String> {
        if let Some(auth_token) = &self.auth_token {
            Ok(auth_token.clone())
        } else {
            match hcore_env::var(AUTH_TOKEN_ENVVAR) {
                Ok(v) => Ok(v),
                Err(_) => {
                    CliConfig::load()?.auth_token.ok_or_else(|| {
                                                     HabError::ArgumentError("No auth token \
                                                                              specified"
                                                                                        .into())
                                                 })
                }
            }
        }
    }

    // This function returns an `Option`, so if there is any "error" reading from config or env is
    // not set simply returns a None.
    pub(crate) fn try_from_cli_or_config(&self) -> Option<String> { self.from_cli_or_config().ok() }

    /// Return the token from CLI, ENV, or config, or `Err(Error::ArgumentError)`.
    pub fn resolve(&self) -> Result<String, Error> {
        if let Some(ref tok) = self.auth_token {
            return Ok(tok.clone());
        }
        match std::env::var(AUTH_TOKEN_ENVVAR) {
            Ok(v) => Ok(v),
            Err(_) => {
                let cfg = CliConfig::load()?;
                cfg.auth_token.clone().ok_or_else(|| {
                                          Error::ArgumentError("No auth token specified: please \
                                                                pass `-z/--auth` or set \
                                                                HAB_AUTH_TOKEN"
                                                                               .into())
                                      })
            }
        }
    }
}

#[derive(GenConfig)]
#[derive(Debug, Clone, Parser, Serialize, Deserialize)]
pub(crate) struct RemoteSup {
    /// Address to a remote Supervisor's Control Gateway
    #[arg(name = "REMOTE_SUP",
                long = "remote-sup",
                short = 'r',
                default_missing_value = ListenCtlAddr::default_as_str())]
    remote_sup: Option<ResolvedListenCtlAddr>,
}

impl RemoteSup {
    pub fn inner(&self) -> Option<&ResolvedListenCtlAddr> { self.remote_sup.as_ref() }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "u64", into = "u64")]
pub struct DurationProxy(Duration);

impl From<DurationProxy> for u64 {
    fn from(d: DurationProxy) -> Self { d.0.as_secs() }
}

impl DurationProxy {
    /// Create a default 60 secs duration, used by `serde(default)`
    pub fn from_60() -> Self { Self(Duration::from_secs(60)) }

    /// Create a default 0 secs duration, used by `serde(default)`
    pub fn from_0() -> Self { Self(Duration::from_secs(0)) }

    /// Create a default 300 secs duration, used by `serde(default)`
    pub fn from_300() -> Self { Self(Duration::from_secs(300)) }
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

    // fn from_str(s: &str) -> std::result::Result<DurationProxy, std::num::ParseIntError> {
    // Ok(s.parse::<u64>()?.into()) }
    fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(s.parse::<u64>()?.into()) }
}

impl fmt::Display for DurationProxy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", u64::from(*self)) }
}

/// A wrapper around `SocketAddr`
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "String")]
pub struct SocketAddrProxy(SocketAddr);

impl TryFrom<String> for SocketAddrProxy {
    type Error = Error;

    // fn try_from(value: String) -> HabResult<Self> {
    // let (_, addr) = habitat_common::util::resolve_socket_addr_with_default_port(
    // value,
    // GossipListenAddr::DEFAULT_PORT,
    // )?;
    // Ok(SocketAddrProxy(addr))
    // }
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (_, addr) = habitat_common::util::resolve_socket_addr_with_default_port(
            value,
            GossipListenAddr::DEFAULT_PORT,
        )?;
        Ok(SocketAddrProxy(addr))
    }
}

impl From<&SocketAddrProxy> for SocketAddr {
    fn from(s: &SocketAddrProxy) -> Self { s.0 }
}

impl From<&SocketAddr> for SocketAddrProxy {
    fn from(s: &SocketAddr) -> Self { Self(*s) }
}

impl From<&SocketAddrProxy> for String {
    fn from(s: &SocketAddrProxy) -> Self { toml::to_string(&s.0).unwrap() }
}

impl FromStr for SocketAddrProxy {
    //     type Err = HabError;
    //
    // fn from_str(s: &str) -> Result<SocketAddrProxy, HabError> {
    // let (_, addr) = habitat_common::util::resolve_socket_addr_with_default_port(
    // s,
    // GossipListenAddr::DEFAULT_PORT,
    // )?;
    // Ok((&addr).into())
    // }
    type Err = Error;

    fn from_str(s: &str) -> Result<SocketAddrProxy, Error> {
        let (_, addr) = habitat_common::util::resolve_socket_addr_with_default_port(
            s,
            GossipListenAddr::DEFAULT_PORT,
        )?;
        Ok((&addr).into())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "String", into = "String")]
pub(crate) struct SubjectAlternativeName(String);

impl FromStr for SubjectAlternativeName {
    //  type Err = HabError;
    //
    // fn from_str(s: &str) -> HabResult<Self> { Ok(SubjectAlternativeName(s.to_string())) }
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(SubjectAlternativeName(s.to_string())) }
}

impl std::fmt::Display for SubjectAlternativeName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", AsRef::<str>::as_ref(&self.0))
    }
}

impl SubjectAlternativeName {
    pub fn dns_name(&self) -> Result<DnsName<'_>, Error> {
        DnsName::try_from(self.0.to_owned()).map_err(|_| Error::InvalidDnsName(self.0.to_owned()))
    }
}

habitat_core::impl_try_from_string_and_into_string!(SubjectAlternativeName);

fn health_check_interval_default() -> u64 { 30 }

#[derive(GenConfig)]
#[derive(PartialEq, Debug, Clone, Parser, Deserialize, Serialize)]
#[command(disable_version_flag = true, rename_all = "screamingsnake")]
pub struct SharedLoad {
    /// Receive updates from the specified release channel
    #[arg(long = "channel")]
    pub channel: Option<ChannelIdent>,

    /// Specify an alternate Builder endpoint. If not specified, the value will be taken from
    /// the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
    // TODO (DM): This should probably use `env` and `default_value`
    // TODO (DM): serde nested flattens do no work https://github.com/serde-rs/serde/issues/1547
    #[arg(long = "url", short = 'u')]
    pub bldr_url: Option<Url>,

    /// The service group with shared config and topology
    #[arg(long = "group", default_value = &*GROUP_DEFAULT)]
    #[serde(default = "GROUP_DEFAULT::get")]
    group: String,

    /// Service topology
    #[arg(long = "topology", short = 't')]
    topology: Option<habitat_sup_protocol::types::Topology>,

    /// The update strategy
    #[arg(long = "strategy", short = 's', default_value = "none")]
    #[serde(default)]
    strategy: habitat_sup_protocol::types::UpdateStrategy,

    /// The condition dictating when this service should update
    ///
    /// latest: Runs the latest package that can be found in the configured channel and local
    /// packages.
    ///
    /// track-channel: Always run what is at the head of a given channel. This enables service
    /// rollback where demoting a package from a channel will cause the package to rollback to
    /// an older version of the package. A ramification of enabling this condition is packages
    /// newer than the package at the head of the channel will be automatically uninstalled
    /// during a service rollback.
    #[arg(long = "update-condition",
                default_value = UpdateCondition::Latest.as_str())]
    #[serde(default)]
    update_condition: UpdateCondition,

    /// One or more service groups to bind to a configuration
    #[arg(long = "bind", num_args = 1.., value_delimiter = ' ')]
    #[serde(default)]
    bind: Vec<ServiceBind>,

    /// Governs how the presence or absence of binds affects service startup
    ///
    /// strict: blocks startup until all binds are present.
    #[arg(long = "binding-mode", default_value = "strict")]
    #[serde(default)]
    binding_mode: habitat_sup_protocol::types::BindingMode,

    /// The interval in seconds on which to run health checks
    // We would prefer to use `HealthCheckInterval`. However, `HealthCheckInterval` uses a map based
    // serialization format. We want to allow the user to simply specify a `u64` to be consistent
    // with the CLI, but we cannot change the serialization because the spec file depends on the map
    // based format.
    #[arg(long = "health-check-interval", default_value = "30")]
    #[serde(default = "health_check_interval_default")]
    health_check_interval: u64,

    /// The delay in seconds after sending the shutdown signal to wait before killing the service
    /// process
    ///
    /// The default value can be set in the packages plan file.
    #[arg(long = "shutdown-timeout")]
    #[serde(default)]
    shutdown_timeout: Option<ShutdownTimeout>,

    #[cfg(target_os = "windows")]
    /// Password of the service user
    #[arg(long = "password")]
    password: Option<String>,

    // TODO (DM): This flag can eventually be removed.
    // See https://github.com/habitat-sh/habitat/issues/7339
    /// DEPRECATED
    #[arg(long = "application", short = 'a', hide = true)]
    #[serde(skip)]
    application: Vec<String>,

    // TODO (DM): This flag can eventually be removed.
    // See https://github.com/habitat-sh/habitat/issues/7339
    /// DEPRECATED
    #[arg(long = "environment", short = 'e', hide = true)]
    #[serde(skip)]
    environment: Vec<String>,

    /// Use the package config from this path rather than the package itself
    #[arg(long = "config-from")]
    config_from: Option<PathBuf>,
}

impl Default for SharedLoad {
    fn default() -> Self {
        Self { channel:                  None,
               bldr_url:                 None,
               group:                    GROUP_DEFAULT::get(),
               topology:                 None,
               strategy:                 habitat_sup_protocol::types::UpdateStrategy::None,
               update_condition:         UpdateCondition::Latest,
               bind:                     vec![],
               binding_mode:             habitat_sup_protocol::types::BindingMode::Strict,
               health_check_interval:    30,
               shutdown_timeout:         None,
               #[cfg(windows)]
               password:                 None,
               application:              vec![],
               environment:              vec![],
               config_from:              None, }
    }
}

#[derive(Serialize, Clone, Parser, Debug)]
#[command(
    disable_version_flag = true,
    group(
        ArgGroup::new("upload")
            .required(true)
            .args(&["origin", "public_file"])
    )
)]
pub(crate) struct UploadGroup {
    /// The origin name
    #[arg(value_name = "ORIGIN", value_parser = clap::value_parser!(CoreOrigin), group = "upload")]
    pub origin: Option<CoreOrigin>,

    /// Path to a local public origin key file on disk
    #[arg(value_name = "PUBLIC_FILE", long = "pubfile", group = "upload")]
    pub public_file: Option<PathBuf>,
}

#[derive(Clone, Parser, Deserialize, Serialize, Debug)]
#[command(disable_version_flag = true)]
pub(crate) struct BldrOrigin {
    /// The Builder origin name to target
    #[arg(value_name = "ORIGIN", short = 'o', long = "origin")]
    pub inner: CoreOrigin,
}

#[derive(Clone, Debug, Args)]
#[command(trailing_var_arg = true,
          allow_hyphen_values = true,
          disable_help_flag = true,
          disable_help_subcommand = true,
          disable_version_flag = true)]
pub struct ExternalCommandArgs {
    /// Arguments to the command
    #[arg(value_name = "ARGS", value_parser = clap::builder::OsStringValueParser::new())]
    pub args: Vec<OsString>,
}

// For sending command and args to a sub command where args don't have to be preceeded by --
// For example:
// `hab pkg exec core/bash bash --login` instead of `hab pkg exec core/bash bash -- --login`
#[derive(Clone, Debug, Args)]
#[command(trailing_var_arg = true,
          allow_hyphen_values = true,
          disable_help_subcommand = true)]
pub struct CommandAndArgs {
    /// Command to execute (eg. ls)
    pub cmd: PathBuf,

    /// Arguments to the command
    #[arg(value_name = "ARGS", value_parser = clap::builder::OsStringValueParser::new())]
    pub args: Vec<OsString>,
}

// Resolve an optional origin (from `--origin <ORIGIN>` or `-o`) into `Origin`,
// falling back to HAB_ORIGIN envvar or the `cli.toml` config if none was supplied.
pub(crate) fn origin_param_or_env(opt: &Option<CoreOrigin>) -> Result<CoreOrigin, Error> {
    if let Some(o) = opt {
        // User passed `--origin foo`
        Ok(o.clone())
    } else if let Ok(env_val) = hcore_env::var(ORIGIN_ENVVAR) {
        // Fallback to HAB_ORIGIN env var
        Ok(CoreOrigin::from_str(&env_val).map_err(Error::from)?)
    } else {
        // Last resort: config file
        let cfg = CliConfig::load()?;
        cfg.origin.ok_or_else(|| {
                      Error::ArgumentError("No origin specified; please set --origin, HAB_ORIGIN, \
                                            or configure cli.toml"
                                                                  .into())
                  })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "String", into = "String")]
struct PkgIdentStringySerde(PackageIdent);

impl FromStr for PkgIdentStringySerde {
    type Err = habitat_core::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(Self(s.parse()?)) }
}

impl std::convert::TryFrom<String> for PkgIdentStringySerde {
    type Error = habitat_core::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> { Self::from_str(&s) }
}

impl std::fmt::Display for PkgIdentStringySerde {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", self.0) }
}

impl From<PkgIdentStringySerde> for String {
    fn from(pkg_ident: PkgIdentStringySerde) -> Self { pkg_ident.to_string() }
}

#[derive(Clone, Debug, Args, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PkgIdent {
    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[arg(name = "PKG_IDENT")]
    pkg_ident: PkgIdentStringySerde,
}

impl PkgIdent {
    pub(crate) fn pkg_ident(self) -> PackageIdent { self.pkg_ident.0 }
}

impl FromStr for PkgIdent {
    type Err = habitat_core::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { pkg_ident: PkgIdentStringySerde(s.parse()?), })
    }
}

pub fn shared_load_cli_to_ctl(ident: PackageIdent,
                              shared_load: SharedLoad,
                              force: bool)
                              -> HabResult<habitat_sup_protocol::ctl::SvcLoad> {
    use habitat_common::{ui,
                         ui::UIWriter};
    #[cfg(target_os = "windows")]
    use habitat_core::crypto::dpapi;
    use habitat_sup_protocol::{ctl::{ServiceBindList,
                                     SvcLoad},
                               types::{HealthCheckInterval,
                                       ServiceBind}};

    // TODO (DM): This check can eventually be removed.
    // See https://github.com/habitat-sh/habitat/issues/7339
    if !shared_load.application.is_empty() || !shared_load.environment.is_empty() {
        ui::ui().warn("--application and --environment flags are deprecated and ignored.")
                .ok();
    }

    let binds = if shared_load.bind.is_empty() {
        None
    } else {
        Some(ServiceBindList { binds: shared_load.bind
                                                 .into_iter()
                                                 .map(ServiceBind::from)
                                                 .collect(), })
    };

    let config_from = if let Some(config_from) = shared_load.config_from {
        log::warn!("\nWARNING: Setting '--config-from' should only be used in development, not \
                    production!\n");
        Some(config_from.to_string_lossy().to_string())
    } else {
        None
    };

    #[cfg(target_os = "windows")]
    let svc_encrypted_password = if let Some(password) = shared_load.password {
        Some(dpapi::encrypt(password)?)
    } else {
        None
    };
    #[cfg(not(target_os = "windows"))]
    let svc_encrypted_password = None;

    Ok(SvcLoad { ident: Some(ident.into()),
                 binds,
                 binding_mode: Some(shared_load.binding_mode as i32),
                 bldr_url: Some(habitat_core::url::bldr_url(shared_load.bldr_url)),
                 bldr_channel: shared_load.channel.map(|x| x.to_string()),
                 config_from,
                 force: Some(force),
                 group: Some(shared_load.group),
                 svc_encrypted_password,
                 topology: shared_load.topology.map(i32::from),
                 update_strategy: Some(shared_load.strategy as i32),
                 health_check_interval:
                     Some(HealthCheckInterval { seconds: shared_load.health_check_interval, }),
                 shutdown_timeout: shared_load.shutdown_timeout.map(u32::from),
                 update_condition: Some(shared_load.update_condition as i32) })
}

pub(crate) fn bldr_auth_token_from_args_env_or_load(opt: Option<String>) -> Result<String, Error> {
    if let Some(token) = opt {
        Ok(token)
    } else {
        match hcore_env::var(AUTH_TOKEN_ENVVAR) {
            Ok(v) => Ok(v),
            Err(_) => {
                CliConfig::load()?.auth_token.ok_or_else(|| {
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

pub(crate) fn maybe_bldr_auth_token_from_args_or_load(opt: Option<String>) -> Option<String> {
    bldr_auth_token_from_args_env_or_load(opt).ok()
}

pub(crate) fn is_default<T: Default + PartialEq>(val: &T) -> bool { val == &T::default() }

#[cfg(test)]
mod tests {
    mod auth_token {

        use crate::cli_v4::utils::AuthToken;

        use clap_v4 as clap;

        use clap::Parser;

        habitat_core::locked_env_var!(HAB_AUTH_TOKEN, locked_auth_token);

        #[derive(Debug, Clone, Parser)]
        struct TestAuthToken {
            #[command(flatten)]
            a: AuthToken,
        }

        #[test]
        fn required_env_no_cli_success() {
            let env_var = locked_auth_token();
            env_var.set("env-auth-token");

            let args = ["test-auth-token"];
            let result = TestAuthToken::try_parse_from(args);
            assert!(result.is_ok(), "{:?}", result.err().unwrap());

            let test_auth_token = result.unwrap();
            let auth_token = test_auth_token.a.from_cli_or_config();
            assert!(auth_token.is_ok(), "{:#?}", auth_token.err().unwrap());
        }

        #[test]
        fn required_no_env_cli_success() {
            let env_var = locked_auth_token();
            env_var.unset();

            let args = ["test-auth-token", "--auth", "foo-bar"];
            let result = TestAuthToken::try_parse_from(args);
            assert!(result.is_ok(), "{:?}", result.err().unwrap());
        }

        #[test]
        fn required_no_env_no_cli_error() {
            let env_var = locked_auth_token();
            env_var.unset();

            let args = ["test-auth-token"];
            let result = TestAuthToken::try_parse_from(args);
            assert!(result.is_ok(), "{:?}", result.err().unwrap());

            let test_auth_token = result.unwrap();
            let auth_token = test_auth_token.a.from_cli_or_config();
            assert!(auth_token.is_err(), "{:#?}", auth_token.ok().unwrap());
        }

        #[test]
        fn required_empty_env_no_cli_error() {
            let env_var = locked_auth_token();
            env_var.set("");

            let args = ["test-auth-token"];
            let result = TestAuthToken::try_parse_from(args);
            assert!(result.is_ok(), "{:?}", result.err().unwrap());

            let test_auth_token = result.unwrap();
            let auth_token = test_auth_token.a.from_cli_or_config();
            assert!(auth_token.is_err(), "{:#?}", auth_token.ok().unwrap());
        }
        #[test]
        fn optional_empty_env_no_cli_none() {
            let env_var = locked_auth_token();
            env_var.set("");

            let args = ["test-auth-token"];
            let result = TestAuthToken::try_parse_from(args);
            assert!(result.is_ok(), "{:?}", result.err().unwrap());

            let test_auth_token = result.unwrap();
            let auth_token = test_auth_token.a.try_from_cli_or_config();
            assert!(auth_token.is_none(), "{:#?}", auth_token.unwrap());
        }

        #[test]
        fn tok_optional_from_env_no_cli_some() {
            let env_var = locked_auth_token();
            env_var.set("env-auth-token");

            let args = ["test-auth-token"];
            let result = TestAuthToken::try_parse_from(args);
            assert!(result.is_ok(), "{:?}", result.err().unwrap());

            let test_auth_token = result.unwrap();
            let auth_token = test_auth_token.a.try_from_cli_or_config();
            assert_eq!(Some("env-auth-token".to_string()),
                       auth_token,
                       "{:#?}",
                       auth_token);
        }

        #[test]
        fn optional_no_env_from_cli_some() {
            let env_var = locked_auth_token();
            env_var.set("env-auth-token");

            let args = ["test-auth-token", "--auth", "foo-bar"];
            let result = TestAuthToken::try_parse_from(args);
            assert!(result.is_ok(), "{:?}", result.err().unwrap());

            let test_auth_token = result.unwrap();
            let auth_token = test_auth_token.a.try_from_cli_or_config();
            assert_eq!(Some("foo-bar".to_string()), auth_token, "{:#?}", auth_token);
        }
    }

    mod bldr_url {

        use crate::cli_v4::utils::{BldrUrl,
                                   DEFAULT_BLDR_URL};

        use clap_v4 as clap;

        use clap::Parser;

        habitat_core::locked_env_var!(HAB_BLDR_URL, locked_bldr_url);

        #[derive(Debug, Clone, Parser)]
        struct TestBldrUrl {
            #[command(flatten)]
            u: BldrUrl,
        }

        #[test]
        fn no_env_no_cli_default() {
            let env_var = locked_bldr_url();
            env_var.unset();

            let args = ["test-bldr-url"];
            let result = TestBldrUrl::try_parse_from(args);
            assert!(result.is_ok(), "{:#?}", result.err().unwrap());

            let test_bldr_url = result.unwrap();
            let bldr_url = test_bldr_url.u.to_string();
            assert_eq!(bldr_url.as_str(), DEFAULT_BLDR_URL, "{:#?}", bldr_url);
        }

        #[test]
        fn empty_env_no_cli_default() {
            let env_var = locked_bldr_url();
            env_var.set("");

            let args = ["test-bldr-url"];
            let result = TestBldrUrl::try_parse_from(args);
            assert!(result.is_ok(), "{:#?}", result.err().unwrap());

            let test_bldr_url = result.unwrap();
            let bldr_url = test_bldr_url.u.to_string();
            assert_eq!(bldr_url.as_str(), DEFAULT_BLDR_URL, "{:#?}", bldr_url);
        }

        #[test]
        fn env_cli_passed_value() {
            let test_bldr_url_val = "https://test.bldr.habitat.sh/";
            let cli_bldr_url_val = "https://cli.bldr.habitat.sh/";
            let env_var = locked_bldr_url();
            env_var.set(test_bldr_url_val);

            let args = ["test-bldr-url", "--url", cli_bldr_url_val];
            let result = TestBldrUrl::try_parse_from(args);
            assert!(result.is_ok(), "{:#?}", result.err().unwrap());

            let test_bldr_url = result.unwrap();
            let bldr_url = test_bldr_url.u.to_string();
            assert_eq!(bldr_url.as_str(), cli_bldr_url_val, "{:#?}", bldr_url);
        }

        #[test]
        fn env_no_cli_env_value() {
            let test_bldr_url_val = "https://test.bldr.habitat.sh/";
            let env_var = locked_bldr_url();
            env_var.set(test_bldr_url_val);

            let args = ["test-bldr-url"];
            let result = TestBldrUrl::try_parse_from(args);
            assert!(result.is_ok(), "{:#?}", result.err().unwrap());

            let test_bldr_url = result.unwrap();
            let bldr_url = test_bldr_url.u.to_string();
            assert_eq!(bldr_url.as_str(), test_bldr_url_val, "{:#?}", bldr_url);
        }
    }
}
