//! Types for reading certificates, private keys, and root certificate stores from the CLI. These
//! types act as a bridge from the CLI to interior types. These types implement `FromStr` (opposed
//! to `From<Path>`) which `structopt` uses to parse the user input.
//!
//! TODO (DM): Ideally these would be defined in `hab::cli::hab::util::tls.rs` however the ctl
//! gateway client currently needs access to these types so they must be defined in a common crate
//! and we simply reexport them in `hab::cli::hab::util::tls.rs`.

use crate::{error::Error,
            tls::{ctl_gateway,
                  rustls_wrapper}};
use rustls::{pki_types::{CertificateDer,
                         PrivatePkcs8KeyDer},
             RootCertStore};
use serde::{Deserialize,
            Serialize};
use std::{path::PathBuf,
          str::FromStr};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct CertificateChainCli {
    path:         PathBuf,
    certificates: Vec<CertificateDer<'static>>,
}

impl CertificateChainCli {
    pub fn into_inner(self) -> Vec<CertificateDer<'static>> { self.certificates }
}

impl FromStr for CertificateChainCli {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = PathBuf::from(s);
        let certificates = if path.is_dir() {
            ctl_gateway::latest_certificates(&path)?
        } else {
            rustls_wrapper::certificates_from_file(&path)?
        };
        Ok(Self { path, certificates })
    }
}

impl std::fmt::Display for CertificateChainCli {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct PrivateKeyCli {
    path:        PathBuf,
    private_key: PrivatePkcs8KeyDer<'static>,
}

impl PrivateKeyCli {
    pub fn into_inner(self) -> PrivatePkcs8KeyDer<'static> { self.private_key }
}

impl Clone for PrivateKeyCli {
    fn clone(&self) -> Self {
        Self { path:        self.path.clone(),
               private_key: self.private_key.clone_key(), }
    }
}

impl FromStr for PrivateKeyCli {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = PathBuf::from(s);
        let private_key = if path.is_dir() {
            ctl_gateway::latest_private_key(&path)?
        } else {
            rustls_wrapper::private_key_from_file(&path)?
        };
        Ok(Self { path, private_key })
    }
}

impl std::fmt::Display for PrivateKeyCli {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path.to_string_lossy())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct RootCertificateStoreCli {
    path:                   PathBuf,
    root_certificate_store: RootCertStore,
}

impl RootCertificateStoreCli {
    pub fn into_inner(self) -> RootCertStore { self.root_certificate_store }
}

impl FromStr for RootCertificateStoreCli {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = PathBuf::from(s);
        let root_certificate_store = if path.is_dir() {
            ctl_gateway::latest_root_certificate_store(&path)?
        } else {
            rustls_wrapper::root_certificate_store_from_file(&path)?
        };
        Ok(Self { path,
                  root_certificate_store })
    }
}

impl std::fmt::Display for RootCertificateStoreCli {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path.to_string_lossy())
    }
}

crate::impl_try_from_string_and_into_string!(CertificateChainCli,
                                             PrivateKeyCli,
                                             RootCertificateStoreCli);
