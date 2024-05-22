//! Utility functions to standardize reading certificates, private keys, and root certificate stores
//! using `rustls`

use rustls::{Certificate,
             PrivateKey,
             RootCertStore};
use std::{fs::File,
          io::{self,
               BufReader},
          path::{Path,
                 PathBuf}};
use thiserror::Error;

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

pub fn certificates_from_file(path: impl AsRef<Path>) -> Result<Vec<Certificate>, Error> {
    let mut buf = buf_from_file(path.as_ref())?;
    rustls_pemfile::certs(&mut buf).map(|c| {
                                       c.map_err(|_| Error::FailedToReadCerts(path.as_ref().into()))
                                        .map(|c| Certificate(c.as_ref().to_vec()))
                                   })
                                   .collect()
}

fn private_keys_from_file(path: impl AsRef<Path>) -> Result<Vec<PrivateKey>, Error> {
    let mut buf = buf_from_file(path.as_ref())?;
    rustls_pemfile::pkcs8_private_keys(&mut buf).map(|k| {
        k.map_err(|_| Error::FailedToReadPrivateKeys(path.as_ref().into()))
         .map(|k| PrivateKey(k.secret_pkcs8_der().to_vec()))
    })
    .collect()
}

pub fn private_key_from_file(path: impl AsRef<Path>) -> Result<PrivateKey, Error> {
    private_keys_from_file(path.as_ref())?.pop()
                                          .ok_or_else(|| Error::NoPrivateKey(path.as_ref().into()))
}

pub fn root_certificate_store_from_file(path: impl AsRef<Path>) -> Result<RootCertStore, Error> {
    let mut buf = buf_from_file(path.as_ref())?;
    let mut root_certificate_store = RootCertStore::empty();
    let certs = &rustls_pemfile::certs(&mut buf).map(|c| {
                     c.map_err(|_| Error::FailedToReadRootCertificateStore(path.as_ref().into()))
                      .map(|c| c.as_ref().to_vec())
                 })
                 .collect::<Result<Vec<_>, _>>()?;
    let (_, failed) = root_certificate_store.add_parsable_certificates(certs);
    if failed > 0 {
        Err(Error::FailedToReadCertificatesFromRootCertificateStore(failed, path.as_ref().into()))
    } else {
        Ok(root_certificate_store)
    }
}
