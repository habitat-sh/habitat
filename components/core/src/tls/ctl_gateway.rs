//! Utilities for generating and reading the self-signed certificate for use with the control
//! gateway.

use crate::{crypto::keys::NamedRevision,
            tls::rustls_wrapper::{self,
                                  Error as RustlsReadersError}};
use rcgen::{Certificate as RcgenCertificate,
            CertificateParams,
            DistinguishedName,
            DnType,
            RcgenError,
            PKCS_ECDSA_P256_SHA256};
use rustls::{Certificate,
             PrivateKey,
             RootCertStore};
use std::{fs::{self,
               File},
          io::{Error as IoError,
               Write},
          path::{Path,
                 PathBuf}};
use thiserror::Error;

const NAME_PREFIX: &str = "ctl-gateway";
const CRT_EXTENSION: &str = "crt.pem";
const KEY_EXTENSION: &str = "key.pem";

#[derive(Error, Debug)]
pub enum Error {
    #[error("ctl gateway certificate lookup failed when trying to match files {0}")]
    FailedToMatchPattern(String),
    #[error("ctl gateway certificate lookup failed, err: {0}")]
    RustlsReaders(#[from] RustlsReadersError),
    #[error("ctl gateway certificate generation failed, err: {0}")]
    CertificateGeneration(#[from] RcgenError),
    #[error("writing the ctl gateway certificate failed, err: {0}")]
    CertificateWrite(#[from] IoError),
}

pub fn generate_self_signed_certificate_and_key(subject_alternate_name: &str,
                                                path: impl AsRef<Path>)
                                                -> Result<(), Error> {
    let mut params = CertificateParams::new(vec![subject_alternate_name.to_string(),
                                                 "localhost".to_string(),
                                                 "127.0.0.1".to_string(),
                                                 "0.0.0.0".to_string()]);
    let mut distinguished_name = DistinguishedName::new();
    distinguished_name.push(DnType::CommonName, "Habitat Supervisor Control Gateway");
    params.distinguished_name = distinguished_name;
    params.alg = &PKCS_ECDSA_P256_SHA256;

    let certificate = RcgenCertificate::from_params(params)?;
    let crt = certificate.serialize_pem()?;
    let key = certificate.serialize_private_key_pem();

    fs::create_dir_all(&path)?;
    let named_revision = NamedRevision::new(NAME_PREFIX.to_string());

    let crt_path = path.as_ref()
                       .join(format!("{}.{}", named_revision, CRT_EXTENSION));
    let mut crt_file = File::create(crt_path)?;
    crt_file.write_all(crt.as_bytes())?;

    let key_path = path.as_ref()
                       .join(format!("{}.{}", named_revision, KEY_EXTENSION));
    let mut key_file = File::create(key_path)?;
    key_file.write_all(key.as_bytes())?;

    Ok(())
}

fn get_latest_path(path: impl AsRef<Path>, pattern: &str) -> Result<PathBuf, Error> {
    let pattern = path.as_ref().join(pattern);
    let pattern = pattern.to_string_lossy();
    glob::glob(&pattern).expect("valid pattern")
                        .filter_map(std::result::Result::ok)
                        .filter(|p| p.metadata().map(|m| m.is_file()).unwrap_or(false))
                        .max()
                        .ok_or_else(|| Error::FailedToMatchPattern(pattern.to_string()))
}

pub fn latest_certificates(path: impl AsRef<Path>) -> Result<Vec<Certificate>, Error> {
    let path = get_latest_path(path, &format!("{}-*.{}", NAME_PREFIX, CRT_EXTENSION))?;
    Ok(rustls_wrapper::certificates_from_file(&path)?)
}

pub fn latest_private_key(path: impl AsRef<Path>) -> Result<PrivateKey, Error> {
    let path = get_latest_path(path, &format!("{}-*.{}", NAME_PREFIX, KEY_EXTENSION))?;
    Ok(rustls_wrapper::private_key_from_file(&path)?)
}

pub fn latest_root_certificate_store(path: impl AsRef<Path>) -> Result<RootCertStore, Error> {
    let path = get_latest_path(path, &format!("{}-*.{}", NAME_PREFIX, CRT_EXTENSION))?;
    Ok(rustls_wrapper::root_certificate_store_from_file(&path)?)
}
