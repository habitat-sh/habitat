//! Utility functions to standardize reading certificates, private keys, and root certificate stores
//! using `rustls`

use rustls::{internal::pemfile,
             Certificate,
             PrivateKey,
             RootCertStore};
use std::{fs,
          io::{self,
               Cursor},
          path::{Path,
                 PathBuf}};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to read file {0}, err: {1}")]
    FailedToReadFile(PathBuf, io::Error),

    #[error("failed to read PEM certificates")]
    FailedToReadCerts,
    #[error("failed to read PEM certificates from file {0}")]
    FailedToReadCertsFromFile(PathBuf),

    #[error("failed to read PEM, RSA private keys")]
    FailedToReadPrivateKeys,
    #[error("failed to read PEM, RSA private keys from file {0}")]
    FailedToReadPrivateKeysFromFile(PathBuf),
    #[error("no PEM, RSA private key")]
    NoPrivateKey,
    #[error("no PEM, RSA private keys in file {0}")]
    NoPrivateKeyFromFile(PathBuf),

    #[error("failed to read PEM root certificate store")]
    FailedToReadRootCertificateStore,
    #[error("failed to read PEM root certificate store {0}")]
    FailedToReadRootCertificateStoreFromFile(PathBuf),
    #[error("failed to read {0} certificates from PEM root certificate store")]
    FailedToReadCertificatesFromRootStore(usize),
    #[error("failed to read {0} certificates from PEM root certificate store file {1}")]
    FailedToReadCertificatesFromRootStoreFile(usize, PathBuf),
}

pub fn certificates_from_buf(buf: &[u8]) -> Result<Vec<Certificate>, Error> {
    let mut cursor = Cursor::new(buf);
    pemfile::certs(&mut cursor).map_err(|_| Error::FailedToReadCerts)
}

pub fn certificates_from_file(path: impl AsRef<Path>) -> Result<Vec<Certificate>, Error> {
    let buf =
        fs::read(path.as_ref()).map_err(|e| Error::FailedToReadFile(path.as_ref().into(), e))?;
    certificates_from_buf(&buf).map_err(|_| Error::FailedToReadCertsFromFile(path.as_ref().into()))
}

pub fn private_keys_from_buf(buf: &[u8]) -> Result<Vec<PrivateKey>, Error> {
    let mut cursor = Cursor::new(buf);
    pemfile::rsa_private_keys(&mut cursor).map_err(|_| Error::FailedToReadPrivateKeys)
}

pub fn private_keys_from_file(path: impl AsRef<Path>) -> Result<Vec<PrivateKey>, Error> {
    let buf =
        fs::read(path.as_ref()).map_err(|e| Error::FailedToReadFile(path.as_ref().into(), e))?;
    private_keys_from_buf(&buf).map_err(|_| {
                                   Error::FailedToReadPrivateKeysFromFile(path.as_ref().into())
                               })
}

pub fn private_key_from_buf(buf: &[u8]) -> Result<PrivateKey, Error> {
    private_keys_from_buf(buf)?.into_iter()
                               .nth(0)
                               .ok_or_else(|| Error::NoPrivateKey)
}

pub fn private_key_from_file(path: impl AsRef<Path>) -> Result<PrivateKey, Error> {
    private_keys_from_file(path.as_ref())?.into_iter()
                                          .nth(0)
                                          .ok_or_else(|| {
                                              Error::NoPrivateKeyFromFile(path.as_ref().into())
                                          })
}

pub fn root_certificate_store_from_buf(buf: &[u8]) -> Result<RootCertStore, Error> {
    let mut root_certificate_store = RootCertStore::empty();
    let mut cursor = Cursor::new(buf);
    let (_, failed) = root_certificate_store.add_pem_file(&mut cursor)
                                            .map_err(|_| Error::FailedToReadPrivateKeys)?;
    if failed > 0 {
        return Err(Error::FailedToReadCertificatesFromRootStore(failed));
    }
    Ok(root_certificate_store)
}

pub fn root_certificate_store_from_file(path: impl AsRef<Path>) -> Result<RootCertStore, Error> {
    let buf =
        fs::read(path.as_ref()).map_err(|e| Error::FailedToReadFile(path.as_ref().into(), e))?;
    root_certificate_store_from_buf(&buf).map_err(|e| {
        use Error::{FailedToReadCertificatesFromRootStore as ReadCertificates,
                    FailedToReadCertificatesFromRootStoreFile as ReadCertificatesFromFile,
                    FailedToReadRootCertificateStore as RootCertStore,
                    FailedToReadRootCertificateStoreFromFile as RootCertStoreFromFile};
        match e {
            RootCertStore => RootCertStoreFromFile(path.as_ref().into()),
            ReadCertificates(failed) => ReadCertificatesFromFile(failed, path.as_ref().into()),
            _ => e,
        }
    })
}
