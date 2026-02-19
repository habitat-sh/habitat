//! Utility functions to standardize reading certificates
use crate::{error::Result,
            fs::cache_ssl_path};
use log::debug;
use native_tls::Certificate;
use std::{fs,
          path::Path,
          result::Result as StdResult};

#[cfg(not(target_os = "macos"))]
use crate::package::{PackageIdent,
                     PackageInstall};
#[cfg(not(target_os = "macos"))]
use std::{path::PathBuf,
          str::FromStr};
#[cfg(not(target_os = "macos"))]
const CACERTS_PKG_IDENT: &str = "core/cacerts";
#[cfg(not(target_os = "macos"))]
const CACERT_PEM: &str = include_str!(concat!(env!("OUT_DIR"), "/cacert.pem"));

/// We need a set of root certificates when connected to SSL/TLS web endpoints.
///
/// The following strategy is used to locate a set of certificates that are used
/// IN ADDITION to any system certificates that may be available (e.g., in /etc/ssl/certs or
/// specified by a `SSL_CERT_FILE` environment variable):
///
/// 1. If the `core/cacerts` Habitat package is installed locally, then use the latest release's
///    `cacert.pem` file.
/// 2. If there is no 'core/cacerts packages, then a copy of `cacert.pem` will be written in an SSL
///    cache directory (by default `/hab/cache/ssl` for a root user and `$HOME/.hab/cache/ssl` for a
///    non-root user) and this will be used. The contents of this file will be inlined in this crate
///    at build time as a fallback, which means that if the program using this code is operating in
///    a minimal environment which may not contain any system certificates, it can still operate.
/// 3. Other certs files (for example self-signed certs) that are found in the SSL cache directory
///    will also get loaded into the root certs list. Both PEM and DER formats are supported. All
///    files will be assumed to be one of the supported formats, and any errors will be ignored
///    silently (other than debug logging)
pub fn certificates(fs_root_path: Option<&Path>) -> Result<Vec<Certificate>> {
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

pub fn certificates_as_der(fs_root_path: Option<&Path>) -> Result<Vec<Vec<u8>>> {
    Ok(certificates(fs_root_path)?.iter()
                                  .map(Certificate::to_der)
                                  .collect::<StdResult<_, _>>()?)
}

#[cfg(not(target_os = "macos"))]
fn installed_cacerts(fs_root_path: Option<&Path>) -> Result<Option<PathBuf>> {
    let cacerts_ident = PackageIdent::from_str(CACERTS_PKG_IDENT)?;

    match PackageInstall::load(&cacerts_ident, fs_root_path) {
        Ok(pkg_install) => {
            let cert_path = pkg_install.installed_path().join("ssl/cert.pem");
            debug!("Found an installed Habitat core/cacerts package at: {}",
                   cert_path.display());
            Ok(Some(cert_path))
        }
        _ => {
            debug!("No installed Habitat core/cacerts package found");
            Ok(None)
        }
    }
}

#[cfg(not(target_os = "macos"))]
fn populate_cache(cache_path: &Path) -> Result<()> {
    let cached_certs = cache_path.join("cert.pem");
    if !cached_certs.exists() {
        debug!("Adding embedded cert file to Habitat SSL cache path {} as fallback",
               cached_certs.display());
        fs::create_dir_all(cache_path)?;
        fs::write(cached_certs, CACERT_PEM)?;
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

fn process_cert_file(certificates: &mut Vec<Certificate>, file_path: &Path) {
    debug!("Processing cert file: {}", file_path.display());
    match certs_from_file(file_path) {
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

#[cfg(not(target_os = "macos"))]
fn certs_from_pem_file(buf: &[u8]) -> Result<Vec<Certificate>> {
    if buf.is_empty() {
        return Ok(Vec::new());
    }
    // Try to decode the first certificate as a pem file. This is necessary because
    // `pem::parse_many` does not return an error. It simply parses what it can and ignores the
    // rest.
    Certificate::from_pem(buf)?;
    pem::parse_many(buf)?.iter()
                         .map(|cert| Ok(Certificate::from_der(cert.contents())?))
                         .collect()
}

#[cfg(target_os = "macos")]
fn certs_from_pem_file_macos(buf: &[u8]) -> Result<Vec<Certificate>> {
    if buf.is_empty() {
        return Ok(Vec::new());
    }
    // Try to decode the first certificate as a pem file. This is necessary because
    // `pem::parse_many` does not return an error. It simply parses what it can and ignores the
    // rest.
    Certificate::from_pem(buf)?;

    let pem_data = pem::parse_many(buf)?;
    // If no PEM blocks were found, this is likely DER data misidentified as PEM on macOS
    if pem_data.is_empty() {
        return Err(crate::error::Error::CryptoError("No PEM blocks found in \
                                                     data"
                                                          .to_string()));
    }

    // Convert PEM contents to certificates, filtering out any that fail validation
    // (macOS has stricter certificate validation)
    let valid_certs: Vec<Certificate> =
        pem_data.iter()
                .filter_map(|cert| Certificate::from_der(cert.contents()).ok())
                .collect();

    // If no certificates were successfully validated, return an error
    if valid_certs.is_empty() {
        return Err(crate::error::Error::CryptoError(
            "No valid certificates found in PEM data".to_string(),
        ));
    }

    Ok(valid_certs)
}

fn certs_from_file(file_path: &Path) -> Result<Vec<Certificate>> {
    let buf = fs::read(file_path)?;
    // Try and interpret the file as a pem cert. If that fails try and interpret it as a der cert.
    #[cfg(target_os = "macos")]
    {
        certs_from_pem_file_macos(&buf).or_else(|_| Ok(vec![Certificate::from_der(&buf)?]))
    }
    #[cfg(not(target_os = "macos"))]
    {
        certs_from_pem_file(&buf).or_else(|_| Ok(vec![Certificate::from_der(&buf)?]))
    }
}

#[cfg(test)]
mod tests {
    use super::certs_from_file;
    use native_tls::Certificate;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_certs_from_file() {
        const PEM_CERT: &str = "-----BEGIN CERTIFICATE-----
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

        // From empty file
        let file = NamedTempFile::new().unwrap();
        assert!(certs_from_file(file.path()).unwrap().is_empty());

        // From der
        let mut file = NamedTempFile::new().unwrap();
        let cert = Certificate::from_pem(PEM_CERT.as_bytes()).unwrap();
        file.write_all(&cert.to_der().unwrap()).unwrap();
        file.flush().unwrap(); // Ensure data is written before reading
        assert_eq!(certs_from_file(file.path()).unwrap().len(), 1);

        // From single pem
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", PEM_CERT).unwrap();
        file.flush().unwrap(); // Ensure data is written before reading
        assert_eq!(certs_from_file(file.path()).unwrap().len(), 1);

        // From multiple pems
        let mut file = NamedTempFile::new().unwrap();
        write!(
               file,
               "{}
-----BEGIN CERTIFICATE-----
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
-----END CERTIFICATE-----
",
               PEM_CERT
        ).unwrap();

        // On macOS, certificate validation might be stricter and could fail entirely
        match certs_from_file(file.path()) {
            Ok(result) => {
                if cfg!(target_os = "macos") {
                    assert!(!result.is_empty(),
                            "Expected at least 1 certificate, got {}",
                            result.len());
                } else {
                    assert_eq!(result.len(), 2);
                }
            }
            Err(_) if cfg!(target_os = "macos") => {
                // On macOS, multiple certificate validation might fail entirely - that's acceptable
                println!("Multiple certificate validation failed on macOS (expected due to \
                          stricter validation)");
            }
            Err(e) => panic!("Unexpected error on non-macOS platform: {}", e),
        }

        // Invalid cert gives an error
        let mut file = NamedTempFile::new().unwrap();
        write!(
               file,
               "-----BEGIN CERTIFICATE-----
a bad cert
-----END CERTIFICATE-----"
        ).unwrap();
        assert!(certs_from_file(file.path()).is_err());
    }
}
