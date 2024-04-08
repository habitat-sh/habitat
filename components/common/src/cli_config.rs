//! This file is called `cli_config` and corresponds to the `cli.toml` file. However, it can be used
//! for more than simply CLI configuration. If the opportunity arose it would be useful to rename
//! this to convey that it is general configuration.
use crate::types::ResolvedListenCtlAddr;
use habitat_core::{fs::{am_i_root,
                        FS_ROOT_PATH},
                   origin::Origin,
                   tls::rustls_wrapper::{CertificateChainCli,
                                         PrivateKeyCli,
                                         RootCertificateStoreCli}};
use log::debug;
use rustls::{ClientConfig as TlsClientConfig,
             Error as TLSError};
use serde::{Deserialize,
            Serialize};
use std::{fs,
          io,
          path::{Path,
                 PathBuf}};

const CLI_CONFIG_PATH_POSTFIX: &str = "hab/etc/cli.toml";

lazy_static::lazy_static! {
static ref CLI_CONFIG_PATH: PathBuf = cli_config_path();
static ref CLI_CONFIG_PATH_PARENT: &'static Path = CLI_CONFIG_PATH
                                    .parent()
                                    .expect("cli config path parent");

/// A cached reading of the config file. This avoids the need to continually read from disk.
/// However, it also means changes to the file will not be picked up after the program has
/// started. Ideally, we would repopulate this struct on file change or on some configured
/// interval.
/// https://github.com/habitat-sh/habitat/issues/7243
static ref CACHED_CLI_CONFIG: CliConfig = CliConfig::load().unwrap_or_default();
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("'{}' io failure, err: {0}", CLI_CONFIG_PATH.display())]
    Io(#[from] io::Error),
    #[error("deserializing '{}' failed, err: {0}", CLI_CONFIG_PATH.display())]
    Deserialize(#[from] toml::de::Error),
    #[error("serializing '{}' failed, err: {0}", CLI_CONFIG_PATH.display())]
    Serialize(#[from] toml::ser::Error),
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CliConfig {
    pub auth_token:                 Option<String>,
    pub origin:                     Option<Origin>,
    pub ctl_secret:                 Option<String>,
    pub listen_ctl:                 Option<ResolvedListenCtlAddr>,
    pub ctl_client_certificate:     Option<CertificateChainCli>,
    pub ctl_client_key:             Option<PrivateKeyCli>,
    pub ctl_server_ca_certificate:  Option<RootCertificateStoreCli>,
    pub ctl_server_name_indication: Option<String>,
    pub bldr_url:                   Option<String>,
}

impl CliConfig {
    fn from_file<T: AsRef<Path>>(path: T) -> Result<Self, Error> {
        let raw = fs::read_to_string(path)?;
        Ok(toml::from_str(&raw)?)
    }

    /// Get a reference to the `CliConfig` cached at startup
    pub fn cache() -> &'static Self { &CACHED_CLI_CONFIG }

    /// Load an up to date `CliConfig` from disk
    pub fn load() -> Result<Self, Error> {
        if CLI_CONFIG_PATH.exists() {
            debug!("Loading CLI config from {}", CLI_CONFIG_PATH.display());
            Ok(CliConfig::from_file(&*CLI_CONFIG_PATH)?)
        } else {
            debug!("No CLI config found, loading defaults");
            Ok(CliConfig::default())
        }
    }

    /// Save the `CliConfig` to disk
    pub fn save(&self) -> Result<(), Error> {
        fs::create_dir_all(*CLI_CONFIG_PATH_PARENT)?;
        let raw = toml::ser::to_string(self)?;
        debug!("Raw config toml:\n---\n{}\n---", &raw);
        fs::write(&*CLI_CONFIG_PATH, raw)?;
        Ok(())
    }

    pub fn maybe_tls_client_config(self) -> Result<Option<TlsClientConfig>, TLSError> {
        let client_certificates = self.ctl_client_certificate
                                      .map(CertificateChainCli::into_inner);
        let client_key = self.ctl_client_key.map(PrivateKeyCli::into_inner);
        let server_ca_certificates = self.ctl_server_ca_certificate
                                         .map(RootCertificateStoreCli::into_inner);
        if let Some(server_certificates) = server_ca_certificates {
            let tls_config = TlsClientConfig::builder().with_safe_defaults()
                                                       .with_root_certificates(server_certificates);
            if let Some(client_key) = client_key {
                debug!("Configuring ctl-gateway TLS with client certificate");
                let config =
                    tls_config.with_client_auth_cert(client_certificates.unwrap_or_default(),
                                                     client_key)?;
                Ok(Some(config))
            } else {
                Ok(Some(tls_config.with_no_client_auth()))
            }
        } else {
            Ok(None)
        }
    }
}

fn cli_config_path() -> PathBuf {
    if !am_i_root() {
        if let Some(home) = dirs::home_dir() {
            return home.join(format!(".{}", CLI_CONFIG_PATH_POSTFIX));
        }
    }
    PathBuf::from(&*FS_ROOT_PATH).join(CLI_CONFIG_PATH_POSTFIX)
}
