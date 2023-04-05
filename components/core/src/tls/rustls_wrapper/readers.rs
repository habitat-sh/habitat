//! Utility functions to standardize reading certificates, private keys, and root certificate stores
//! using `rustls`
use crate::fs::cache_ssl_path;
use log::debug;
use rustls::{Certificate,
             PrivateKey,
             RootCertStore};
use std::{fs::{self,
               File},
          io::{self,
               BufReader},
          path::{Path,
                 PathBuf},
          str::FromStr};
use thiserror::Error;

#[cfg(not(target_os = "macos"))]
use crate::package::{PackageIdent,
                     PackageInstall};

#[cfg(not(target_os = "macos"))]
const CACERTS_PKG_IDENT: &str = "core/cacerts";
#[cfg(not(target_os = "macos"))]
const CACERT_PEM: &str = include_str!(concat!(env!("OUT_DIR"), "/cacert.pem"));

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to read file {0}, err: {1}")]
    FailedToReadFile(PathBuf, io::Error),

    #[error("failed to read PEM certificates from file {0}")]
    FailedToReadCerts(PathBuf),

    #[error("failed to read PEM, PKCS8 private keys from file {0}")]
    FailedToReadPrivateKeys(PathBuf),
    #[error("no PEM, PKCS8 private keys in file {0}")]
    NoPrivateKey(PathBuf),

    #[error("failed to read PEM root certificate store {0}")]
    FailedToReadRootCertificateStore(PathBuf),
    #[error("failed to read {0} certificates from PEM root certificate store file {1}")]
    FailedToReadCertificatesFromRootCertificateStore(usize, PathBuf),
}

fn buf_from_file(path: impl AsRef<Path>) -> Result<BufReader<File>, Error> {
    let file =
        File::open(path.as_ref()).map_err(|e| Error::FailedToReadFile(path.as_ref().into(), e))?;
    Ok(BufReader::new(file))
}

pub fn certificates(fs_root_path: Option<&Path>) -> Result<Vec<Certificate>, Error> {
    let mut certificates = Vec::new();
    let cert_cache_dir = cache_ssl_path(fs_root_path);

    // MacOS is not yet fully consistent with other platforms,
    // as it cannot handle PEM files with multiple certs.
    // We can enable this when the following issue is resolved:
    // https://github.com/sfackler/rust-native-tls/issues/132
    #[cfg(not(target_os = "macos"))]
    {
        match installed_cacerts(fs_root_path)? {
            Some(cert_path) => process_cert_file(&mut certificates, &cert_path),
            None => populate_cache(&cert_cache_dir)?,
        }
    }

    process_cache_dir(&cert_cache_dir, &mut certificates);
    Ok(certificates)
}

#[cfg(not(target_os = "macos"))]
fn installed_cacerts(fs_root_path: Option<&Path>) -> Result<Option<PathBuf>, Error> {
    let cacerts_ident = PackageIdent::from_str(CACERTS_PKG_IDENT).unwrap();

    if let Ok(pkg_install) = PackageInstall::load(&cacerts_ident, fs_root_path) {
        let cert_path = pkg_install.installed_path().join("ssl/cert.pem");
        debug!("Found an installed Habitat core/cacerts package at: {}",
               cert_path.display());
        Ok(Some(cert_path))
    } else {
        debug!("No installed Habitat core/cacerts package found");
        Ok(None)
    }
}

fn process_cert_file(certificates: &mut Vec<Certificate>, file_path: &Path) {
    debug!("Processing cert file: {}", file_path.display());
    match certificates_from_file(file_path) {
        Ok(mut certs) => {
            debug!("Found {} certs in: {}", certs.len(), file_path.display());
            certificates.append(&mut certs)
        }
        Err(err) => {
            debug!("Unable to process cert file: {}, err={}",
                   file_path.display(),
                   err)
        }
    }
}

// fn certs_from_pem_file(buf: &[u8]) -> Result<Vec<Certificate>, Error> {
//     if buf.is_empty() {
//         return Ok(Vec::new());
//     }
//     // Try to decode the first certificate as a pem file. This is necessary because
//     // `pem::parse_many` does not return an error. It simply parses what it can and ignores the
//     // rest.
//     Certificate::from_pem(buf)?;
//     pem::parse_many(buf)?.iter()
//                          .map(|cert| Ok(Certificate::from_der(cert.contents())?))
//                          .collect()
// }

// fn certs_from_file(file_path: &Path) -> Result<Vec<Certificate>, Error> {
//     let buf = fs::read(file_path)?;
//     // Try and interpret the file as a pem cert. If that fails try and interpret it as a der
// cert.     certs_from_pem_file(&buf).or_else(|_| Ok(vec![Certificate::from_der(&buf)?]))
// }

#[cfg(not(target_os = "macos"))]
fn populate_cache(cache_path: &Path) -> Result<(), Error> {
    let cached_certs = cache_path.join("cert.pem");
    if !cached_certs.exists() {
        debug!("Adding embedded cert file to Habitat SSL cache path {} as fallback",
               cached_certs.display());
        fs::create_dir_all(cache_path).unwrap();
        fs::write(cached_certs, CACERT_PEM).unwrap();
    }
    Ok(())
}

fn process_cache_dir(cache_path: &Path, certificates: &mut Vec<Certificate>) {
    match fs::read_dir(cache_path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_file() {
                            process_cert_file(certificates, &path);
                        }
                    }
                    Err(err) => debug!("Unable to read cache entry, err = {}", err),
                }
            }
        }
        Err(err) => debug!("Unable to read cache directory, err = {}", err),
    }
}

pub fn certificates_from_file(path: impl AsRef<Path>) -> Result<Vec<Certificate>, Error> {
    let mut buf = buf_from_file(path.as_ref())?;
    let certs = rustls_pemfile::certs(&mut buf).map_err(|_| {
                                                   Error::FailedToReadCerts(path.as_ref().into())
                                               })?;
    Ok(certs.into_iter().map(Certificate).collect())
}

fn private_keys_from_file(path: impl AsRef<Path>) -> Result<Vec<PrivateKey>, Error> {
    let mut buf = buf_from_file(path.as_ref())?;
    let private_keys = rustls_pemfile::pkcs8_private_keys(&mut buf).map_err(|_| {
                                             Error::FailedToReadPrivateKeys(path.as_ref().into())
                                         })?;
    Ok(private_keys.into_iter().map(PrivateKey).collect())
}

pub fn private_key_from_file(path: impl AsRef<Path>) -> Result<PrivateKey, Error> {
    private_keys_from_file(path.as_ref())?.pop()
                                          .ok_or_else(|| Error::NoPrivateKey(path.as_ref().into()))
}

pub fn root_certificate_store_from_file(path: impl AsRef<Path>) -> Result<RootCertStore, Error> {
    let mut buf = buf_from_file(path.as_ref())?;
    let mut root_certificate_store = RootCertStore::empty();
    let certs =
        &rustls_pemfile::certs(&mut buf).map_err(|_| {
                                            Error::FailedToReadRootCertificateStore(path.as_ref()
                                                                                        .into())
                                        })?;
    let (_, failed) = root_certificate_store.add_parsable_certificates(certs);
    if failed > 0 {
        Err(Error::FailedToReadCertificatesFromRootCertificateStore(failed, path.as_ref().into()))
    } else {
        Ok(root_certificate_store)
    }
}
