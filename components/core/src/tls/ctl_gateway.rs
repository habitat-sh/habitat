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
use webpki::DnsNameRef;

const NAME_PREFIX: &str = "ctl-gateway";
const CRT_EXTENSION: &str = "crt.pem";
const KEY_EXTENSION: &str = "key.pem";

#[derive(Error, Debug)]
pub enum Error {
    #[error("ctl gateway TLS file lookup failed when trying to match files {0}")]
    FailedToMatchPattern(String),
    #[error("ctl gateway TLS file lookup failed, err: {0}")]
    RustlsReaders(#[from] RustlsReadersError),
    #[error("ctl gateway TLS file generation failed, err: {0}")]
    CertificateGeneration(#[from] RcgenError),
    #[error("writing the ctl gateway TLS files failed, err: {0}")]
    CertificateWrite(#[from] IoError),
}

pub fn generate_self_signed_certificate_and_key(subject_alternate_name: DnsNameRef,
                                                path: impl AsRef<Path>)
                                                -> Result<(), Error> {
    let mut params =
        CertificateParams::new(vec![Into::<&str>::into(subject_alternate_name).to_string(),
                                    "localhost".to_string(),]);
    let mut distinguished_name = DistinguishedName::new();
    distinguished_name.push(DnType::OrganizationName,
                            "Habitat Supervisor Control Gateway");
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

/// Search for files in `search_directory` that match `file_pattern` and return the last match  
fn get_last_path(search_directory: impl AsRef<Path>, file_pattern: &str) -> Result<PathBuf, Error> {
    let pattern = search_directory.as_ref().join(file_pattern);
    let pattern = pattern.to_string_lossy();
    glob::glob(&pattern).expect("valid pattern")
                        .filter_map(std::result::Result::ok)
                        .filter(|p| p.metadata().map(|m| m.is_file()).unwrap_or(false))
                        .max()
                        .ok_or_else(|| Error::FailedToMatchPattern(pattern.to_string()))
}

pub fn latest_certificates(path: impl AsRef<Path>) -> Result<Vec<Certificate>, Error> {
    let path = get_last_path(path, &format!("{}-*.{}", NAME_PREFIX, CRT_EXTENSION))?;
    Ok(rustls_wrapper::certificates_from_file(&path)?)
}

pub fn latest_private_key(path: impl AsRef<Path>) -> Result<PrivateKey, Error> {
    let path = get_last_path(path, &format!("{}-*.{}", NAME_PREFIX, KEY_EXTENSION))?;
    Ok(rustls_wrapper::private_key_from_file(&path)?)
}

pub fn latest_root_certificate_store(path: impl AsRef<Path>) -> Result<RootCertStore, Error> {
    let path = get_last_path(path, &format!("{}-*.{}", NAME_PREFIX, CRT_EXTENSION))?;
    Ok(rustls_wrapper::root_certificate_store_from_file(&path)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs,
              time::Duration};
    use tempfile::TempDir;
    use webpki::DnsNameRef;

    #[test]
    fn ctl_gateway_generate_and_read_tls_files() {
        let tmpdir = TempDir::new().unwrap();

        generate_self_signed_certificate_and_key(DnsNameRef::try_from_ascii_str("a_test_domain").unwrap(), &tmpdir).unwrap();
        assert_eq!(fs::read_dir(&tmpdir).unwrap().count(), 2);
        let first_path =
            get_last_path(&tmpdir, &format!("{}-*.{}", NAME_PREFIX, CRT_EXTENSION)).unwrap();
        let certificates = latest_certificates(&tmpdir).unwrap();
        assert_eq!(certificates.len(), 1);
        latest_private_key(&tmpdir).unwrap();
        let root_certificate_store = latest_root_certificate_store(&tmpdir).unwrap();
        assert_eq!(root_certificate_store.roots.len(), 1);

        // TLS files are named on second boundaries. Wait enough time to guarantee we get a new
        // name.
        std::thread::sleep(Duration::from_secs(2));

        generate_self_signed_certificate_and_key(DnsNameRef::try_from_ascii_str("another_domain").unwrap(), &tmpdir).unwrap();
        assert_eq!(fs::read_dir(&tmpdir).unwrap().count(), 4);
        let second_path =
            get_last_path(&tmpdir, &format!("{}-*.{}", NAME_PREFIX, CRT_EXTENSION)).unwrap();
        let certificates = latest_certificates(&tmpdir).unwrap();
        assert_eq!(certificates.len(), 1);
        latest_private_key(&tmpdir).unwrap();
        let root_certificate_store = latest_root_certificate_store(&tmpdir).unwrap();
        assert_eq!(root_certificate_store.roots.len(), 1);

        assert!(second_path > first_path);
    }
}
