//! Utility functions to standardize reading certificates, private keys, and root certificate stores
//! using `rustls`

use rustls::{RootCertStore,
             pki_types::{CertificateDer,
                         PrivatePkcs8KeyDer,
                         pem::PemObject}};
use std::{fs::File,
          io::{self,
               BufReader,
               Read},
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

/// Reads X.509 certificates in PEM format from a file.
///
/// # Format Requirements
///
/// The file must contain one or more X.509 certificates in PEM format:
///
/// ```text
/// -----BEGIN CERTIFICATE-----
/// <base64-encoded certificate data>
/// -----END CERTIFICATE-----
/// ```
///
/// Multiple certificates can be concatenated in the same file.
///
/// # Arguments
///
/// * `path` - Path to the PEM file containing certificates
///
/// # Returns
///
/// A vector of parsed certificates, or an error if the file cannot be read
/// or the certificates are invalid.
///
/// # Errors
///
/// Returns [`Error::FailedToReadFile`] if the file cannot be read, or
/// [`Error::FailedToReadCerts`] if the certificates cannot be parsed.
pub fn certificates_from_file(path: impl AsRef<Path>)
                              -> Result<Vec<CertificateDer<'static>>, Error> {
    let mut buf = buf_from_file(path.as_ref())?;
    let mut pem_data = Vec::new();
    buf.read_to_end(&mut pem_data)
       .map_err(|e| Error::FailedToReadFile(path.as_ref().into(), e))?;

    CertificateDer::pem_slice_iter(&pem_data).collect::<Result<Vec<_>, _>>()
                                             .map_err(|_| {
                                                 Error::FailedToReadCerts(path.as_ref().into())
                                             })
}

/// Reads PKCS8 private keys in PEM format from a file.
///
/// # Format Requirements
///
/// The file must contain one or more private keys in PKCS8 PEM format:
///
/// ```text
/// -----BEGIN PRIVATE KEY-----
/// <base64-encoded PKCS8 key data>
/// -----END PRIVATE KEY-----
/// ```
///
/// Multiple keys can be present in the same file.
///
/// # Arguments
///
/// * `path` - Path to the PEM file containing PKCS8 private keys
///
/// # Returns
///
/// A vector of parsed private keys, or an error if the file cannot be read
/// or the keys are invalid.
///
/// # Errors
///
/// Returns [`Error::FailedToReadFile`] if the file cannot be read, or
/// [`Error::FailedToReadPrivateKeys`] if the keys cannot be parsed.
///
/// # Note
///
/// Only PKCS8 format is supported. Keys in other formats (e.g., PKCS1, SEC1)
/// must be converted to PKCS8 first.
fn private_keys_from_file(path: impl AsRef<Path>)
                          -> Result<Vec<PrivatePkcs8KeyDer<'static>>, Error> {
    let mut buf = buf_from_file(path.as_ref())?;
    let mut pem_data = Vec::new();
    buf.read_to_end(&mut pem_data)
       .map_err(|e| Error::FailedToReadFile(path.as_ref().into(), e))?;

    PrivatePkcs8KeyDer::pem_slice_iter(&pem_data).collect::<Result<Vec<_>, _>>()
                                                 .map_err(|_| {
                                                     Error::FailedToReadPrivateKeys(path.as_ref()
                                                                                        .into())
                                                 })
}

/// Reads a single PKCS8 private key from a PEM file.
///
/// # Format Requirements
///
/// The file must contain at least one private key in PKCS8 PEM format.
/// If multiple keys are present, only the last one is returned.
///
/// # Arguments
///
/// * `path` - Path to the PEM file containing a PKCS8 private key
///
/// # Returns
///
/// The last private key found in the file, or an error if no keys are present
/// or the file cannot be read.
///
/// # Errors
///
/// Returns [`Error::NoPrivateKey`] if no keys are found in the file,
/// [`Error::FailedToReadFile`] if the file cannot be read, or
/// [`Error::FailedToReadPrivateKeys`] if the keys cannot be parsed.
pub fn private_key_from_file(path: impl AsRef<Path>) -> Result<PrivatePkcs8KeyDer<'static>, Error> {
    private_keys_from_file(path.as_ref())?.pop()
                                          .ok_or_else(|| Error::NoPrivateKey(path.as_ref().into()))
}

/// Reads X.509 certificates from a PEM file and creates a root certificate store.
///
/// # Format Requirements
///
/// The file must contain one or more X.509 certificates in PEM format that
/// will be used as trusted root certificates.
///
/// # Arguments
///
/// * `path` - Path to the PEM file containing root certificates
///
/// # Returns
///
/// A [`RootCertStore`] containing the parsed certificates, or an error if
/// certificates cannot be read or parsed.
///
/// # Errors
///
/// Returns [`Error::FailedToReadFile`] if the file cannot be read,
/// [`Error::FailedToReadCerts`] if the certificates cannot be parsed, or
/// [`Error::FailedToReadCertificatesFromRootCertificateStore`] if any
/// certificates fail to be added to the store.
pub fn root_certificate_store_from_file(path: impl AsRef<Path>) -> Result<RootCertStore, Error> {
    let mut root_certificate_store = RootCertStore::empty();
    let certs = certificates_from_file(path.as_ref())?;
    let (_, failed) = root_certificate_store.add_parsable_certificates(certs);
    if failed > 0 {
        Err(Error::FailedToReadCertificatesFromRootCertificateStore(failed, path.as_ref().into()))
    } else {
        Ok(root_certificate_store)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rcgen::{CertificateParams,
                KeyPair};
    use std::io::Write;
    use tempfile::NamedTempFile;

    // Valid PEM certificate for testing
    const TEST_CERT_PEM: &str = "-----BEGIN CERTIFICATE-----
MIIB/jCCAYWgAwIBAgIIdJclisc/elQwCgYIKoZIzj0EAwMwRTELMAkGA1UEBhMC
VVMxFDASBgNVBAoMC0FmZmlybVRydXN0MSAwHgYDVQQDDBdBZmZpcm1UcnVzdCBQ
cmVtaXVtIEVDQzAeFw0xMDAxMjkxNDIwMjRaFw00MDEyMzExNDIwMjRaMEUxCzAJ
BgNVBAYTAlVTMRQwEgYDVQQKDAtBZmZpcm1UcnVzdDEgMB4GA1UEAwwXQWZmaXJt
VHJ1c3QgUHJlbWl1bSBFQ0MwdjAQBgcqhkjOPQIBBgUrgQQAIgNiAAQNMF4bFZ0D
0KF5Nbc6PJJ6yhUczWLznCZcBz3lVPqj1swS6vQUX+iOGasvLkjmrBhDeKzQN8O9
ss0s5kfiGuZjuD0uL3jET9v0D6RoTFVya5UdThhClXjMNzyR4ptlKymjQjBAMB0G
A1UdDgQWBBSaryl6wBE1NSZRMADDav5A1a7WPDAPBgNVHRMBAf8EBTADAQH/MA4G
A1UdDwEB/wQEAwIBBjAKBggqhkjOPQQDAwNnADBkAjAXCfOHiFBar8jAQr9HX/Vs
aobgxCd05DhT1wV/GzTjxi+zygk8N53X57hG8f2h4nECMEJZh0PUUd+60wkyWs6I
flc9nF9Ca/UHLbXwgpP5WW+uZPpY5Yse42O+tYHNbwKMeQ==
-----END CERTIFICATE-----";

    const TEST_CERT_PEM_2: &str = "-----BEGIN CERTIFICATE-----
MIIDQTCCAimgAwIBAgITBmyfz5m/jAo54vB4ikPmljZbyjANBgkqhkiG9w0BAQsF
ADA5MQswCQYDVQQGEwJVUzEPMA0GA1UEChMGQW1hem9uMRkwFwYDVQQDExBBbWF6
b24gUm9vdCBDQSAxMB4XDTE1MDUyNjAwMDAwMFoXDTM4MDExNzAwMDAwMFowOTEL
MAkGA1UEBhMCVVMxDzANBgNVBAoTBkFtYXpvbjEZMBcGA1UEAxMQQW1hem9uIFJv
b3QgQ0EgMTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBALJ4gHHKeNXj
ca9HgFB0fW7Y14h29Jlo91ghYPl0hAEvrAIthtOgQ3pOsqTQNroBvo3bSMgHFzZM
9O6II8c+6zf1tRn4SWiw3te5djgdYZ6k/oI2peVKVuRF4fn9tBb6dNqcmzU5L/qw
IFAGbHrQgLKm+a/sRxmPUDgH3KKHOVj4utWp+UhnMJbulHheb4mjUcAwhmahRWa6
VOujw5H5SNz/0egwLX0tdHA114gk957EWW67c4cX8jJGKLhD+rcdqsq08p8kDi1L
93FcXmn/6pUCyziKrlA4b9v7LWIbxcceVOF34GfID5yHI9Y/QCB/IIDEgEw+OyQm
jgSubJrIqg0CAwEAAaNCMEAwDwYDVR0TAQH/BAUwAwEB/zAOBgNVHQ8BAf8EBAMC
AYYwHQYDVR0OBBYEFIQYzIU07LwMlJQuCFmcx7IQTgoIMA0GCSqGSIb3DQEBCwUA
A4IBAQCY8jdaQZChGsV2USggNiMOruYou6r4lK5IpDB/G/wkjUu0yKGX9rbxenDI
U5PMCCjjmCXPI6T53iHTfIUJrU6adTrCC2qJeHZERxhlbI1Bjjt/msv0tadQ1wUs
N+gDS63pYaACbvXy8MWy7Vu33PqUXHeeE6V/Uq2V8viTO96LXFvKWlJbYK8U90vv
o/ufQJVtMVT8QtPHRh8jrdkPSHCa2XV4cdFyQzR1bldZwgJcJmApzyMZFo6IQ6XU
5MsI+yMRQ+hDKXJioaldXgjUkK642M4UwtBV8ob2xJNDd2ZhwLnoQdeXeGADbkpy
rqXRfboQnoZsG4q5WTP468SQvvG5
-----END CERTIFICATE-----";

    #[test]
    fn test_certificates_from_file_single_cert() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", TEST_CERT_PEM).unwrap();
        file.flush().unwrap();

        let certs = certificates_from_file(file.path()).unwrap();
        assert_eq!(certs.len(), 1);
    }

    #[test]
    fn test_certificates_from_file_multiple_certs() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}\n{}", TEST_CERT_PEM, TEST_CERT_PEM_2).unwrap();
        file.flush().unwrap();

        let certs = certificates_from_file(file.path()).unwrap();
        assert_eq!(certs.len(), 2);
    }

    #[test]
    fn test_certificates_from_file_empty() {
        let file = NamedTempFile::new().unwrap();

        let result = certificates_from_file(file.path());
        // Empty file should successfully parse to an empty vector
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_certificates_from_file_invalid() {
        let mut file = NamedTempFile::new().unwrap();
        // Malformed PEM - missing END tag
        write!(file, "-----BEGIN CERTIFICATE-----\nMIIB/jCCAYWgAw").unwrap();
        file.flush().unwrap();

        let result = certificates_from_file(file.path());
        // Malformed PEM structure should result in error
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::FailedToReadCerts(_)));
    }

    #[test]
    fn test_certificates_from_file_nonexistent() {
        let result = certificates_from_file("/nonexistent/path/to/file.pem");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::FailedToReadFile(_, _)));
    }

    #[test]
    fn test_private_key_from_file_valid() {
        // Generate a valid PKCS8 private key using rcgen
        let key_pair = KeyPair::generate().unwrap();
        let pkcs8_pem = key_pair.serialize_pem();

        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", pkcs8_pem).unwrap();
        file.flush().unwrap();

        let result = private_key_from_file(file.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_private_key_from_file_multiple_keys() {
        // Generate two valid PKCS8 private keys
        let key_pair_1 = KeyPair::generate().unwrap();
        let pkcs8_pem_1 = key_pair_1.serialize_pem();
        let key_pair_2 = KeyPair::generate().unwrap();
        let pkcs8_pem_2 = key_pair_2.serialize_pem();

        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}\n{}", pkcs8_pem_1, pkcs8_pem_2).unwrap();
        file.flush().unwrap();

        // Should return the last key
        let result = private_key_from_file(file.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_private_key_from_file_empty() {
        let file = NamedTempFile::new().unwrap();

        let result = private_key_from_file(file.path());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::NoPrivateKey(_)));
    }

    #[test]
    fn test_private_key_from_file_invalid() {
        let mut file = NamedTempFile::new().unwrap();
        // Malformed PEM - missing END tag
        write!(file, "-----BEGIN PRIVATE KEY-----\nMIIEvQIBADA").unwrap();
        file.flush().unwrap();

        let result = private_key_from_file(file.path());
        assert!(result.is_err());
        // Malformed PEM should result in FailedToReadPrivateKeys
        assert!(matches!(result.unwrap_err(), Error::FailedToReadPrivateKeys(_)));
    }

    #[test]
    fn test_private_key_from_file_nonexistent() {
        let result = private_key_from_file("/nonexistent/path/to/key.pem");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::FailedToReadFile(_, _)));
    }

    #[test]
    fn test_root_certificate_store_from_file_valid() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", TEST_CERT_PEM).unwrap();
        file.flush().unwrap();

        let result = root_certificate_store_from_file(file.path());
        assert!(result.is_ok());
        let store = result.unwrap();
        assert!(!store.is_empty());
    }

    #[test]
    fn test_root_certificate_store_from_file_multiple() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}\n{}", TEST_CERT_PEM, TEST_CERT_PEM_2).unwrap();
        file.flush().unwrap();

        let result = root_certificate_store_from_file(file.path());
        assert!(result.is_ok());
        let store = result.unwrap();
        assert!(!store.is_empty());
    }

    #[test]
    fn test_root_certificate_store_from_file_empty() {
        let file = NamedTempFile::new().unwrap();

        let result = root_certificate_store_from_file(file.path());
        // Empty file results in empty store, which is OK
        assert!(result.is_ok());
    }

    #[test]
    fn test_root_certificate_store_from_file_invalid() {
        let mut file = NamedTempFile::new().unwrap();
        // Malformed PEM - missing END tag
        write!(file, "-----BEGIN CERTIFICATE-----\nMIIB/jCCAYWgAw").unwrap();
        file.flush().unwrap();

        let result = root_certificate_store_from_file(file.path());
        // Malformed PEM should result in error
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::FailedToReadCerts(_)));
    }

    #[test]
    fn test_root_certificate_store_from_file_nonexistent() {
        let result = root_certificate_store_from_file("/nonexistent/path/to/file.pem");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::FailedToReadFile(_, _)));
    }

    #[test]
    fn test_end_to_end_cert_and_key() {
        // Generate a certificate and key pair
        let mut params = CertificateParams::new(vec!["localhost".to_string()]).unwrap();
        params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
        let key_pair = KeyPair::generate().unwrap();
        let cert = params.self_signed(&key_pair).unwrap();

        // Write certificate to file
        let mut cert_file = NamedTempFile::new().unwrap();
        write!(cert_file, "{}", cert.pem()).unwrap();
        cert_file.flush().unwrap();

        // Write key to file
        let mut key_file = NamedTempFile::new().unwrap();
        write!(key_file, "{}", key_pair.serialize_pem()).unwrap();
        key_file.flush().unwrap();

        // Test reading certificate
        let certs = certificates_from_file(cert_file.path()).unwrap();
        assert_eq!(certs.len(), 1);

        // Test reading private key
        let key = private_key_from_file(key_file.path()).unwrap();
        assert!(!key.secret_pkcs8_der().is_empty());

        // Test root certificate store
        let store = root_certificate_store_from_file(cert_file.path()).unwrap();
        assert!(!store.is_empty());
    }
}
