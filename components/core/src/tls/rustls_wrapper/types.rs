//! Types for reading certificates, private keys, and root certificate stores from the CLI
//! TODO (DM): Ideally these would be defined in `hab::cli::hab::util::tls.rs` however the ctl
//! gateway client currently needs access to these types so they must be defined in a common crate
//! and we simply reexport them in `hab::cli::hab::util::tls.rs`.

use crate::{error::Error,
            tls::{ctl_gateway,
                  rustls_wrapper}};
use rustls::{Certificate,
             PrivateKey as RustlsPrivateKey,
             RootCertStore};
use serde::{Deserialize,
            Serialize};
use std::{path::PathBuf,
          str::FromStr};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "&str", into = "String")]
pub struct CertificateChain {
    path:         PathBuf,
    certificates: Vec<Certificate>,
}

impl CertificateChain {
    pub fn certificates(self) -> Vec<Certificate> { self.certificates }
}

impl FromStr for CertificateChain {
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

impl std::convert::TryFrom<&str> for CertificateChain {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> { Self::from_str(s) }
}

impl std::fmt::Display for CertificateChain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path.to_string_lossy())
    }
}

impl From<CertificateChain> for String {
    fn from(pkg_ident: CertificateChain) -> Self { pkg_ident.to_string() }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "&str", into = "String")]
pub struct PrivateKey {
    path:        PathBuf,
    private_key: RustlsPrivateKey,
}

impl PrivateKey {
    pub fn private_key(self) -> RustlsPrivateKey { self.private_key }
}

impl FromStr for PrivateKey {
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

impl std::convert::TryFrom<&str> for PrivateKey {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> { Self::from_str(s) }
}

impl std::fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path.to_string_lossy())
    }
}

impl From<PrivateKey> for String {
    fn from(pkg_ident: PrivateKey) -> Self { pkg_ident.to_string() }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "&str", into = "String")]
pub struct RootCertificateStore {
    path:                   PathBuf,
    root_certificate_store: RootCertStore,
}

impl RootCertificateStore {
    pub fn root_certificate_store(self) -> RootCertStore { self.root_certificate_store }
}

impl FromStr for RootCertificateStore {
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

impl std::convert::TryFrom<&str> for RootCertificateStore {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> { Self::from_str(s) }
}

impl std::fmt::Display for RootCertificateStore {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path.to_string_lossy())
    }
}

impl From<RootCertificateStore> for String {
    fn from(pkg_ident: RootCertificateStore) -> Self { pkg_ident.to_string() }
}
