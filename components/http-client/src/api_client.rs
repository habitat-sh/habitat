use std::{fs::{self,
               File},
          io::{Read,
               Write},
          iter::FromIterator,
          path::{Path,
                 PathBuf},
          str::FromStr,
          time::Duration};

use reqwest::{header::{HeaderMap,
                       HeaderValue,
                       USER_AGENT},
              Certificate,
              Client as ReqwestClient,
              ClientBuilder,
              IntoUrl,
              Proxy,
              RequestBuilder};

use habitat_core::{env,
                   fs::cache_ssl_path,
                   package::{PackageIdent,
                             PackageInstall,
                             PackageTarget},
                   util::sys};

use url::Url;

use crate::error::{Error,
                   Result};

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
    inner: ReqwestClient,
}

impl ApiClient {
    /// Creates and returns a new `ApiClient` instance.
    ///
    /// Builds a new Reqwest HTTP client with appropriate SSL configuration and HTTP/HTTPS proxy
    /// support.
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

        let header_values = vec![(USER_AGENT, user_agent(product, version)?)];
        let headers = HeaderMap::from_iter(header_values.into_iter());

        let mut client = client_with_proxy(&endpoint).default_headers(headers)
                                                     .timeout(Duration::from_secs(timeout_in_secs))
                                                     .danger_accept_invalid_certs(skip_cert_verify);

        let certs = certificates(fs_root_path)?;
        for cert in certs {
            client = client.add_root_certificate(cert);
        }

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

fn client_with_proxy(url: &Url) -> ClientBuilder {
    trace!("Checking proxy for url: {:?}", url);

    let mut client = ReqwestClient::builder();

    if let Some(proxy_url) = env_proxy::for_url(&url).to_string() {
        if url.scheme() == "http" {
            debug!("Setting http_proxy to {}", proxy_url);
            match Proxy::http(&proxy_url) {
                Ok(p) => {
                    client = client.proxy(p);
                }
                Err(e) => warn!("Invalid proxy, err: {:?}", e),
            }
        }

        if url.scheme() == "https" {
            debug!("Setting https proxy to {}", proxy_url);
            match Proxy::https(&proxy_url) {
                Ok(p) => {
                    client = client.proxy(p);
                }
                Err(e) => warn!("Invalid proxy, err: {:?}", e),
            }
        }
    } else {
        debug!("No proxy configured for url: {:?}", url);
    }

    client
}

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
///    will also get loaded into the root certs list. Both PEM and DER formats are supported (the
///    extensions should be '.pem' or '.der' respectively)
fn certificates(fs_root_path: Option<&Path>) -> Result<Vec<Certificate>> {
    let mut certificates = Vec::new();

    // MacOS is not yet fully consistent with other platforms,
    // as it cannot handle PEM files with multiple certs.
    // We can enable this when the following issue is resolved:
    // https://github.com/sfackler/rust-native-tls/issues/132
    if cfg!(not(target_os = "macos")) {
        let cacerts_ident = PackageIdent::from_str(CACERTS_PKG_IDENT)?;

        if let Ok(pkg_install) = PackageInstall::load(&cacerts_ident, fs_root_path) {
            let pkg_certs = pkg_install.installed_path().join("ssl/cert.pem");
            debug!("Found installed certs: {}", pkg_certs.display());
            let cert = cert_from_file(&pkg_certs)?;
            certificates.push(cert);
        } else {
            debug!("No installed cacerts package found");

            let cached_certs = cache_ssl_path(fs_root_path).join("cert.pem");
            if !cached_certs.exists() {
                fs::create_dir_all(cache_ssl_path(fs_root_path))?;
                debug!("Creating cached cacert.pem: {}", cached_certs.display());
                let mut file = File::create(&cached_certs)?;
                file.write_all(CACERT_PEM.as_bytes())?;
            }
        }
    }

    if let Ok(entries) = fs::read_dir(cache_ssl_path(fs_root_path)) {
        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if let Some(ext) = path.extension() {
                if path.is_file() && ((ext == "pem") || (ext == "der")) {
                    debug!("Found cached cert: {}", path.display());
                    let cert = cert_from_file(&path)?;
                    certificates.push(cert);
                }
            }
        }
    }

    Ok(certificates)
}

fn cert_from_file(file_path: &PathBuf) -> Result<Certificate> {
    let mut buf = Vec::new();
    File::open(file_path)?.read_to_end(&mut buf)?;

    let ext = file_path.extension().unwrap(); // unwrap Ok
    if ext == "pem" {
        Certificate::from_pem(&buf).map_err(Error::ReqwestError)
    } else {
        Certificate::from_der(&buf).map_err(Error::ReqwestError)
    }
}
