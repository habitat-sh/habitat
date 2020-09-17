//! Types for reading certificates, private keys, and root certificate stores from the CLI

use habitat_core::tls;
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
    type Err = habitat_core::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = PathBuf::from(s);
        let certificates = if path.is_dir() {
            // TODO: If it is a directory generate a self signed cert and use that or use an
            // existing cert with a given naming scheme
            todo!()
        } else {
            tls::rustls_reader::certificates_from_file(&path)?
        };
        Ok(Self { path, certificates })
    }
}

impl std::convert::TryFrom<&str> for CertificateChain {
    type Error = habitat_core::Error;

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
    type Err = habitat_core::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = PathBuf::from(s);
        let private_key = if path.is_dir() {
            // TODO: If it is a directory generate a self signed cert and use that or use an
            // existing cert with a given naming scheme
            todo!()
        } else {
            tls::rustls_reader::private_key_from_file(&path)?
        };
        Ok(Self { path, private_key })
    }
}

impl std::convert::TryFrom<&str> for PrivateKey {
    type Error = habitat_core::Error;

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
    type Err = habitat_core::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = PathBuf::from(s);
        let root_certificate_store = if path.is_dir() {
            // TODO: If it is a directory generate a self signed cert and use that or use an
            // existing cert with a given naming scheme
            todo!()
        } else {
            tls::rustls_reader::root_certificate_store_from_file(&path)?
        };
        Ok(Self { path,
                  root_certificate_store })
    }
}

impl std::convert::TryFrom<&str> for RootCertificateStore {
    type Error = habitat_core::Error;

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
