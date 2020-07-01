use crate::error::{Error,
                   Result};
use habitat_core::{env,
                   fs::cache_ssl_path,
                   package::{PackageIdent,
                             PackageInstall,
                             PackageTarget},
                   util::sys};
use native_tls::Certificate;
use reqwest::{header::{HeaderMap,
                       HeaderValue,
                       CONNECTION,
                       USER_AGENT},
              Certificate as ReqwestCertificate,
              Client as ReqwestClient,
              IntoUrl,
              Proxy,
              RequestBuilder,
              Url};
use std::{fs,
          iter::FromIterator,
          path::{Path,
                 PathBuf},
          str::FromStr,
          time::Duration};

// Read and write TCP socket timeout for Hyper/HTTP client calls.
const CLIENT_SOCKET_RW_TIMEOUT_SEC: u64 = 300;

const CACERTS_PKG_IDENT: &str = "core/cacerts";
const CACERT_PEM: &str = include_str!(concat!(env!("OUT_DIR"), "/cacert.pem"));

/// A generic wrapper around a Reqwest HTTP client intended for API-like usage.
///
/// When an `ApiClient` is created, it has a constant URL base which is assumed to be some API
/// endpoint. This allows the underlying client to load and use any relevant HTTP proxy
/// support and to provide reasonable User-Agent HTTP headers, etc.
#[derive(Debug)]
pub struct ApiClient {
    /// The base URL for the client.
    endpoint: Url,
    /// An instance of a `reqwest::Client`
    inner:    ReqwestClient,
}

impl ApiClient {
    /// Creates and returns a new `ApiClient` instance.
    ///
    /// Builds a new Reqwest HTTP client with appropriate SSL configuration and HTTP/HTTPS proxy
    /// support.
    ///
    /// # Errors
    ///
    /// * If the underlying Reqwest client cannot be created
    /// * If a suitable SSL context cannot be established
    /// * If an HTTP/S proxy cannot be correctly setup
    /// * If a `User-Agent` HTTP header string cannot be constructed
    pub fn new<T>(endpoint: T,
                  product: &str,
                  version: &str,
                  fs_root_path: Option<&Path>)
                  -> Result<Self>
        where T: IntoUrl
    {
        let endpoint = endpoint.into_url().map_err(Error::ReqwestError)?;

        let timeout_in_secs = match env::var("HAB_CLIENT_SOCKET_TIMEOUT") {
            Ok(t) => {
                match t.parse::<u64>() {
                    Ok(n) => n,
                    Err(_) => CLIENT_SOCKET_RW_TIMEOUT_SEC,
                }
            }
            Err(_) => CLIENT_SOCKET_RW_TIMEOUT_SEC,
        };
        debug!("Client socket timeout: {} secs", timeout_in_secs);

        let skip_cert_verify = env::var("HAB_SSL_CERT_VERIFY_NONE").is_ok();
        debug!("Skip cert verification: {}", skip_cert_verify);

        // We set the Connection header to close so that the underlying socket
        // will be closed upon completion of the current request and response.
        // This is done in order to compensate for a bug in reqwest on Windows
        // which creates a new socket on each creation of a client that is not
        // closed until the process exits. Until the process exits, these connections
        // remain in CLOSE_WAIT. Since this ApiClient is created fresh from CLI
        // commands, we are not taking advantage of keep-alive anyways so setting
        // the Connection header to close should not have adverse effects.
        let headers = HeaderMap::from_iter(vec![
            (USER_AGENT, user_agent(product, version)?),
            (
                CONNECTION,
                HeaderValue::from_str("close").expect("Valid Connection header"),
            ),
        ].into_iter());

        let mut client = ReqwestClient::builder().proxy(proxy_for(&endpoint)?)
                                                 .default_headers(headers)
                                                 .timeout(Duration::from_secs(timeout_in_secs))
                                                 .danger_accept_invalid_certs(skip_cert_verify);

        client =
            certificates(fs_root_path)?.iter()
                                       .map(Certificate::to_der)
                                       .collect::<std::result::Result<Vec<_>, _>>()?
                                       .into_iter()
                                       .map(|raw| ReqwestCertificate::from_der(&*raw))
                                       .collect::<std::result::Result<Vec<_>, _>>()?
                                       .into_iter()
                                       .fold(client, |client, cert| {
                                           client.add_root_certificate(cert)
                                       });

        Ok(ApiClient { inner: client.build()?,
                       endpoint })
    }

    /// Builds an HTTP GET request for a given path.
    pub fn get(&self, path: &str) -> RequestBuilder { self.get_with_custom_url(path, |_| {}) }

    /// Builds an HTTP GET request for a given path with the ability to customize the target URL.
    pub fn get_with_custom_url<F>(&self, path: &str, mut customize_url: F) -> RequestBuilder
        where F: FnMut(&mut Url)
    {
        let mut url = self.url_for(path);
        customize_url(&mut url);
        debug!("GET {} with {:?}", &url, &self);
        self.inner.get(url)
    }

    /// Builds an HTTP HEAD request for a given path.
    pub fn head(&self, path: &str) -> RequestBuilder { self.head_with_custom_url(path, |_| {}) }

    /// Builds an HTTP HEAD request for a given path with the ability to customize the target URL.
    pub fn head_with_custom_url<F>(&self, path: &str, mut customize_url: F) -> RequestBuilder
        where F: FnMut(&mut Url)
    {
        let mut url = self.url_for(path);
        customize_url(&mut url);
        debug!("HEAD {} with {:?}", &url, &self);
        self.inner.head(url)
    }

    /// Builds an HTTP PATCH request for a given path.
    pub fn patch(&self, path: &str) -> RequestBuilder { self.patch_with_custom_url(path, |_| {}) }

    /// Builds an HTTP PATCH request for a given path with the ability to customize the target URL.
    pub fn patch_with_custom_url<F>(&self, path: &str, mut customize_url: F) -> RequestBuilder
        where F: FnMut(&mut Url)
    {
        let mut url = self.url_for(path);
        customize_url(&mut url);
        debug!("PATH {} with {:?}", &url, &self);
        self.inner.patch(url)
    }

    /// Builds an HTTP POST request for a given path.
    pub fn post(&self, path: &str) -> RequestBuilder { self.post_with_custom_url(path, |_| {}) }

    /// Builds an HTTP POST request for a given path with the ability to customize the target URL.
    pub fn post_with_custom_url<F>(&self, path: &str, mut customize_url: F) -> RequestBuilder
        where F: FnMut(&mut Url)
    {
        let mut url = self.url_for(path);
        customize_url(&mut url);
        debug!("POST {} with {:?}", &url, &self);
        self.inner.post(url)
    }

    /// Builds an HTTP PUT request for a given path.
    pub fn put(&self, path: &str) -> RequestBuilder { self.put_with_custom_url(path, |_| {}) }

    /// Builds an HTTP PUT request for a given path with the ability to customize the target URL.
    pub fn put_with_custom_url<F>(&self, path: &str, mut customize_url: F) -> RequestBuilder
        where F: FnMut(&mut Url)
    {
        let mut url = self.url_for(path);
        customize_url(&mut url);
        debug!("PUT {} with {:?}", &url, &self);
        self.inner.put(url)
    }

    /// Builds an HTTP DELETE request for a given path.
    pub fn delete(&self, path: &str) -> RequestBuilder { self.delete_with_custom_url(path, |_| {}) }

    /// Builds an HTTP DELETE request for a given path with the ability to customize the target URL.
    pub fn delete_with_custom_url<F>(&self, path: &str, mut customize_url: F) -> RequestBuilder
        where F: FnMut(&mut Url)
    {
        let mut url = self.url_for(path);
        customize_url(&mut url);
        debug!("DELETE {} with {:?}", &url, &self);
        self.inner.delete(url)
    }

    fn url_for(&self, path: &str) -> Url {
        let mut url = self.endpoint.clone();

        if path.is_empty() {
            return url;
        }

        if url.path().ends_with('/') || path.starts_with('/') {
            url.set_path(&format!("{}{}", self.endpoint.path(), path));
        } else {
            url.set_path(&format!("{}/{}", self.endpoint.path(), path));
        }

        url
    }
}

fn proxy_for(url: &Url) -> reqwest::Result<Proxy> {
    trace!("Checking proxy for url: {:?}", url);

    if let Some(proxy_url) = env_proxy::for_url(url).to_string() {
        match url.scheme() {
            "http" => {
                debug!("Setting http_proxy to {}", proxy_url);
                Proxy::http(&proxy_url)
            }
            "https" => {
                debug!("Setting https proxy to {}", proxy_url);
                Proxy::https(&proxy_url)
            }
            _ => unimplemented!(),
        }
    } else {
        debug!("No proxy configured for url: {:?}", url);
        Ok(Proxy::custom(|_| None::<Url>))
    }
}

/// Returns an HTTP User-Agent string type for use by Reqwest when making HTTP requests.
///
/// The general form for Habitat-related clients are of the following form:
///
/// ```text
/// <PRODUCT>/<VERSION> (<TARGET>; <KERNEL_RELEASE>)
/// ```
///
/// where:
///
/// * `<PRODUCT>`: is the provided product name
/// * `<VERSION>`: is the provided version string which may also include a release number
/// * `<TARGET>`: is the machine architecture and the kernel separated by a dash in lower case
/// * `<KERNEL_RELEASE>`: is the kernel release string from `uname`
///
/// For example:
///
/// ```text
/// hab/0.6.0/20160606153031 (x86_64-darwin; 14.5.0)
/// ```
///
/// # Errors
///
/// * If system information cannot be obtained via `uname`
fn user_agent(product: &str, version: &str) -> Result<HeaderValue> {
    let uname = sys::uname()?;
    let ua = format!("{}/{} ({}; {})",
                     product.trim(),
                     version.trim(),
                     PackageTarget::active_target(),
                     uname.release.trim().to_lowercase());
    debug!("User-Agent: {}", &ua);
    Ok(HeaderValue::from_str(&ua).expect("Valid User Agent header"))
}

/// We need a set of root certificates when connected to SSL/TLS web endpoints.
///
/// The following strategy is used to locate a set of certificates that are used
/// IN ADDITION to any system certificates that may be available (e.g., in /etc/ssl/certs or
/// specified by a `SSL_CERT_FILE` environment variable):
///
/// 1. If the `core/cacerts` Habitat package is installed locally, then use the latest release's
///    `cacert.pem` file.
/// 2. If there is no 'core/cacerts packages, then a copy of `cacert.pem` will be written in an SSL
///    cache directory (by default `/hab/cache/ssl` for a root user and `$HOME/.hab/cache/ssl` for
///    a non-root user) and this will be used. The contents of this file will be inlined in this
///    crate at build time as a fallback, which means that if the program using this code is
///    operating in a minimal environment which may not contain any system certificates, it can
///    still operate.
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

fn installed_cacerts(fs_root_path: Option<&Path>) -> Result<Option<PathBuf>> {
    let cacerts_ident = PackageIdent::from_str(CACERTS_PKG_IDENT)?;

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

fn populate_cache(cache_path: &Path) -> Result<()> {
    let cached_certs = cache_path.join("cert.pem");
    if !cached_certs.exists() {
        debug!("Adding embedded cert file to Habitat SSL cache path {} as fallback",
               cached_certs.display());
        fs::create_dir_all(&cache_path)?;
        fs::write(cached_certs, CACERT_PEM)?;
    }
    Ok(())
}

fn process_cache_dir(cache_path: &Path, mut certificates: &mut Vec<Certificate>) {
    match fs::read_dir(cache_path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_file() {
                            process_cert_file(&mut certificates, &path);
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
    match certs_from_file(&file_path) {
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

fn certs_from_pem_file(buf: &[u8]) -> Result<Vec<Certificate>> {
    if buf.is_empty() {
        return Ok(Vec::new());
    }
    // Try to decode the first certificate as a pem file. This is necessary because
    // `pem::parse_many` does not return an error. It simply parses what it can and ignores the
    // rest.
    Certificate::from_pem(buf)?;
    pem::parse_many(buf).iter()
                        .map(|cert| Ok(Certificate::from_der(&cert.contents)?))
                        .collect()
}

fn certs_from_file(file_path: &Path) -> Result<Vec<Certificate>> {
    let buf = fs::read(file_path)?;
    // Try and interpret the file as a pem cert. If that fails try and interpret it as a der cert.
    certs_from_pem_file(&buf).or_else(|_| Ok(vec![Certificate::from_der(&buf)?]))
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
        assert_eq!(certs_from_file(file.path()).unwrap().len(), 1);

        // From single pem
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", PEM_CERT).unwrap();
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
        assert_eq!(certs_from_file(file.path()).unwrap().len(), 2);

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
