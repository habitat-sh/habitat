use crate::error::{Error,
                   Result};
use habitat_core::{env,
                   package::PackageTarget,
                   util::sys};
use log::{debug,
          trace};
use reqwest::{Certificate as ReqwestCertificate,
              Client as ReqwestClient,
              IntoUrl,
              Proxy,
              RequestBuilder,
              Url,
              header::{CONNECTION,
                       HeaderValue,
                       USER_AGENT}};
use std::{path::Path,
          time::Duration};

// Read and write TCP socket timeout for Hyper/HTTP client calls.
const CLIENT_SOCKET_RW_TIMEOUT_SEC: u64 = 300;

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
        let headers = vec![
            (USER_AGENT, user_agent(product, version)?),
            (
                CONNECTION,
                HeaderValue::from_str("close").expect("Valid Connection header"),
            ),
        ].into_iter().collect();

        let mut client = ReqwestClient::builder().proxy(proxy_for(&endpoint)?)
                                                 .default_headers(headers)
                                                 .timeout(Duration::from_secs(timeout_in_secs))
                                                 .danger_accept_invalid_certs(skip_cert_verify);

        client =
            habitat_core::tls::native_tls_wrapper::certificates_as_der(fs_root_path)?
                                       .into_iter()
                                       .map(|raw| ReqwestCertificate::from_der(&raw))
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
