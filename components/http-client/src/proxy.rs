// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use url::{self, Url};
use url::percent_encoding::percent_decode;

use base64;
use hab_core::env;

use error::{Error, Result};

/// Configuration relating to an HTTP Proxy.
///
/// # Examples
///
/// A proxy server with no credentials required:
///
/// ```
/// extern crate habitat_http_client;
/// extern crate url;
///
/// use std::str::FromStr;
/// use url::Url;
/// use habitat_http_client::proxy::ProxyInfo;
///
/// fn main() {
///     let url = Url::from_str("http://proxy.example.com:8001/").unwrap();
///     let proxy = ProxyInfo::new(url, None).unwrap();
///
///     assert_eq!(proxy.scheme(), "http");
///     assert_eq!(proxy.host(), "proxy.example.com");
///     assert_eq!(proxy.port(), 8001);
///     assert!(proxy.authorization_header_value().is_none());
/// }
/// ```
///
/// A proxy server using basic authorization:
///
/// ```
/// extern crate habitat_http_client;
/// extern crate url;
///
/// use std::str::FromStr;
/// use url::Url;
/// use habitat_http_client::proxy::{ProxyBasicAuthorization, ProxyInfo};
///
/// fn main() {
///     let url = Url::from_str("http://proxy.example.com").unwrap();
///     let authz = ProxyBasicAuthorization::new("foo".to_string(), "bar".to_string());
///     let proxy = ProxyInfo::new(url, Some(authz)).unwrap();
///
///     assert_eq!(proxy.scheme(), "http");
///     assert_eq!(proxy.host(), "proxy.example.com");
///     assert_eq!(proxy.port(), 80);
///     assert_eq!(proxy.authorization_header_value().unwrap(), "Basic Zm9vOmJhcg==");
/// }
/// ```
///
#[derive(Debug, PartialEq)]
pub struct ProxyInfo {
    /// Url for the proxy server.
    url: Url,
    /// Optional basic authorization credentials for the proxy server.
    authorization: Option<ProxyBasicAuthorization>,
}

impl ProxyInfo {
    /// Create and return a new `ProxyInfo`.
    ///
    /// # Errors
    ///
    /// * If the proxy scheme is invalid (i.e. not `"http"` or `"https"`)
    /// * If the proxy `Url` has an empty host entry
    /// * If the proxy `Url` has an unknown port number
    pub fn new(url: Url, authorization: Option<ProxyBasicAuthorization>) -> Result<Self> {
        match url.scheme() {
            "http" | "https" => (),
            scheme => {
                let msg = format!("Invalid scheme {} for {}", &scheme, &url);
                return Err(Error::InvalidProxyValue(msg));
            }
        }
        if let None = url.host_str() {
            return Err(Error::UrlParseError(url::ParseError::EmptyHost));
        }
        if let None = url.port_or_known_default() {
            return Err(Error::UrlParseError(url::ParseError::InvalidPort));
        }

        Ok(ProxyInfo {
            url: url,
            authorization: authorization,
        })
    }

    /// Returns the scheme for the proxy server.
    pub fn scheme(&self) -> &str {
        self.url.scheme()
    }

    /// Returns the host entry for the proxy server.
    pub fn host(&self) -> &str {
        // Note that `.unwrap()` can be called here as the constructor function ensures that this
        // entry must not be empty.
        self.url.host_str().unwrap()
    }

    /// Returns the port number for the proxy server.
    pub fn port(&self) -> u16 {
        // Note that `.unwrap()` can be called here as the constructor function ensures that this
        // value must known.
        self.url.port_or_known_default().unwrap()
    }

    /// Returns an `Option` of a `String` representing the value of a `Proxy-Authorization` HTTP
    /// header.
    pub fn authorization_header_value(&self) -> Option<String> {
        match self.authorization {
            Some(ref auth) => Some(auth.header_value()),
            None => None,
        }
    }
}

/// Represents a set of `Basic` proxy authorization credentials.
///
/// # Examples
///
/// ```
/// use habitat_http_client::proxy::ProxyBasicAuthorization;
///
/// let authz = ProxyBasicAuthorization::new("foo".to_string(), "bar".to_string());
///
/// assert_eq!(authz.header_value(), "Basic Zm9vOmJhcg==");
/// ```
///
#[derive(Debug, PartialEq)]
pub struct ProxyBasicAuthorization {
    /// Username for Basic authorization.
    username: String,
    /// Password for Basic authorization.
    password: String,
}

impl ProxyBasicAuthorization {
    /// Creates and returns a new `ProxyBasicAuthorization` with the given username and password.
    pub fn new(username: String, password: String) -> Self {
        ProxyBasicAuthorization {
            username: username,
            password: password,
        }
    }

    /// Returns a `String` containing the value for a `Proxy-Authorization` HTTP header.
    pub fn header_value(&self) -> String {
        format!(
            "Basic {}",
            base64::encode(format!("{}:{}", self.username, self.password).as_bytes())
        )
    }
}

/// Returns a `ProxyInfo` from the `http_proxy` environment variable, if it is set.
///
/// The value of `http_proxy` must be a parseable URL such as `http://proxy.company.com:8001/`
/// otherwise a parsing error will be returned. If the port is not present, than the default port
/// numbers of http/80 and https/443 will be returned. If the `http_proxy` environment variable is
/// not set or is empty, then a `Result` of `None` will be returned.
///
/// References:
///
/// * https://www.gnu.org/software/wget/manual/html_node/Proxies.html
/// * https://wiki.archlinux.org/index.php/proxy_settings
/// * https://msdn.microsoft.com/en-us/library/hh272656(v=vs.120).aspx
/// * http://www.cyberciti.biz/faq/linux-unix-set-proxy-environment-variable/
///
/// # Examples
///
/// Behavior when environment variable is set:
///
/// ```
/// use std;
/// use habitat_http_client::proxy;
///
/// std::env::set_var("http_proxy", "http://proxy.example.com:8001/");
/// let info = proxy::http_proxy().unwrap().unwrap();
///
/// assert_eq!(info.scheme(), "http");
/// assert_eq!(info.host(), "proxy.example.com");
/// assert_eq!(info.port(), 8001);
/// assert!(info.authorization_header_value().is_none());
/// ```
///
/// Behavior when environment variable is set with basic auth credentials:
///
/// ```
/// use std;
/// use habitat_http_client::proxy;
///
/// std::env::set_var("http_proxy", "http://itsme:asecret@proxy.example.com");
/// let info = proxy::http_proxy().unwrap().unwrap();
///
/// assert_eq!(info.scheme(), "http");
/// assert_eq!(info.host(), "proxy.example.com");
/// assert_eq!(info.port(), 80);
/// assert_eq!(info.authorization_header_value().unwrap(), "Basic aXRzbWU6YXNlY3JldA==");
/// ```
///
/// Behavior when both lower case and upper case environment variables are set:
///
/// ```
/// use std;
/// use habitat_http_client::proxy;
///
/// std::env::set_var("HTTP_PROXY", "http://upper.example.com");
/// std::env::set_var("http_proxy", "http://lower.example.com");
/// let info = proxy::http_proxy().unwrap().unwrap();
///
/// assert_eq!(info.host(), "lower.example.com");
/// ```
///
/// Behavior when environment variable is empty:
///
/// ```
/// use std;
/// use habitat_http_client::proxy;
///
/// std::env::set_var("http_proxy", "");
///
/// assert!(proxy::http_proxy().unwrap().is_none());
/// ```
///
/// # Errors
///
/// * If the value of the `http_proxy` environment variable cannot be parsed as a URL
/// * If the URL scheme is not a valid type (currently only supported schemes are http and https)
/// * If the URL is missing a host/domain segment
/// * If the URL is missing a port number and a default cannot be determined
pub fn http_proxy() -> Result<Option<ProxyInfo>> {
    match env::var("http_proxy") {
        Ok(url) => parse_proxy_url(&url),
        _ => {
            match env::var("HTTP_PROXY") {
                Ok(url) => parse_proxy_url(&url),
                _ => Ok(None),
            }
        }
    }
}

/// Returns a `ProxyInfo` from the `https_proxy` environment variable, if it is set.
///
/// The value of `https_proxy` must be a parseable URL such as `https://proxy.company.com:8001/`
/// otherwise a parsing error will be returned. If the port is not present, than the default port
/// numbers of http/80 and https/443 will be returned. If the `https_proxy` environment variable is
/// not set or is empty, then a `Result` of `None` will be returned.
///
/// References:
///
/// * https://www.gnu.org/software/wget/manual/html_node/Proxies.html
/// * https://wiki.archlinux.org/index.php/proxy_settings
/// * https://msdn.microsoft.com/en-us/library/hh272656(v=vs.120).aspx
/// * http://www.cyberciti.biz/faq/linux-unix-set-proxy-environment-variable/
///
/// # Examples
///
/// Behavior when environment variable is set:
///
/// ```
/// use std;
/// use habitat_http_client::proxy;
///
/// std::env::set_var("https_proxy", "http://proxy.example.com:8001/");
/// let info = proxy::https_proxy().unwrap().unwrap();
///
/// assert_eq!(info.scheme(), "http");
/// assert_eq!(info.host(), "proxy.example.com");
/// assert_eq!(info.port(), 8001);
/// assert!(info.authorization_header_value().is_none());
/// ```
///
/// Behavior when environment variable is set with basic auth credentials:
///
/// ```
/// use std;
/// use habitat_http_client::proxy;
///
/// std::env::set_var("https_proxy", "http://itsme:asecret@proxy.example.com");
/// let info = proxy::https_proxy().unwrap().unwrap();
///
/// assert_eq!(info.scheme(), "http");
/// assert_eq!(info.host(), "proxy.example.com");
/// assert_eq!(info.port(), 80);
/// assert_eq!(info.authorization_header_value().unwrap(), "Basic aXRzbWU6YXNlY3JldA==");
/// ```
///
/// Behavior when both lower case and upper case environment variables are set:
///
/// ```
/// use std;
/// use habitat_http_client::proxy;
///
/// std::env::set_var("HTTPS_PROXY", "http://upper.example.com");
/// std::env::set_var("https_proxy", "http://lower.example.com");
/// let info = proxy::https_proxy().unwrap().unwrap();
///
/// assert_eq!(info.host(), "lower.example.com");
/// ```
///
/// Behavior when environment variable is empty:
///
/// ```
/// use std;
/// use habitat_http_client::proxy;
///
/// std::env::set_var("https_proxy", "");
///
/// assert!(proxy::https_proxy().unwrap().is_none());
/// ```
///
/// # Errors
///
/// * If the value of the `https_proxy` environment variable cannot be parsed as a URL
/// * If the URL scheme is not a valid type (currently only supported schemes are http and https)
/// * If the URL is missing a host/domain segment
/// * If the URL is missing a port number and a default cannot be determined
pub fn https_proxy() -> Result<Option<ProxyInfo>> {
    match env::var("https_proxy") {
        Ok(url) => parse_proxy_url(&url),
        _ => {
            match env::var("HTTPS_PROXY") {
                Ok(url) => parse_proxy_url(&url),
                _ => Ok(None),
            }
        }
    }
}

/// Returns a `ProxyInfo` from either the `http_proxy` or `https_proxy` environment variable if
/// either is set and the given domain name is not matched in the `no_proxy` environment variable
/// domain extension set.
///
/// See the [`http_proxy()`] and [`https_proxy()`] functions for more details about the
/// `http_proxy` and `https_proxy` environment variable parsing. This function honors the
/// `no_proxy` environment variable which is assumed to be a comma separated set of domain
/// extensions. If the given domain matches one of these extensions then no proxy information
/// should be returned (i.e. a return of `Ok(None)`).
///
/// # Errors
///
/// * If either the [`http_proxy()`] or [`https_proxy()`] was unsuccessful
///
/// [`http_proxy()`]: #method.http_proxy
/// [`https_proxy()`]: #method.https_proxy
///
/// # Examples
///
/// Behavior when domain matches extension set for http_proxy:
///
/// ```
/// extern crate habitat_http_client;
/// extern crate url;
///
/// use std::str::FromStr;
/// use url::Url;
/// use habitat_http_client::proxy;
///
/// fn main() {
///     std::env::set_var("http_proxy", "http://proxy.example.com:8001/");
///     std::env::set_var("no_proxy", "localhost,127.0.0.1,localaddress,.localdomain.com");
///     let for_domain = Url::from_str("http://server.localdomain.com").unwrap();
///     let info = proxy::proxy_unless_domain_exempted(Some(&for_domain)).unwrap();
///
///     assert!(info.is_none());
/// }
///
/// ```
///
/// Behavior when domain matches extension set for https_proxy:
///
/// ```
/// extern crate habitat_http_client;
/// extern crate url;
///
/// use std::str::FromStr;
/// use url::Url;
/// use habitat_http_client::proxy;
///
/// fn main() {
///     std::env::set_var("https_proxy", "http://proxy.example.com:8001/");
///     std::env::set_var("no_proxy", "localhost,127.0.0.1,localaddress,.localdomain.com");
///     let for_domain = Url::from_str("https://server.localdomain.com").unwrap();
///     let info = proxy::proxy_unless_domain_exempted(Some(&for_domain)).unwrap();
///
///     assert!(info.is_none());
/// }
/// ```
///
/// Behavior when both lower case and uppercase environment variables are set:
///
/// ```
/// extern crate habitat_http_client;
/// extern crate url;
///
/// use std::str::FromStr;
/// use url::Url;
/// use habitat_http_client::proxy;
///
/// fn main() {
///     std::env::set_var("HTTPS_PROXY", "http://upper.example.com");
///     std::env::set_var("https_proxy", "http://lower.example.com");
///     std::env::set_var("NO_PROXY", ".upper.localdomain.com");
///     std::env::set_var("no_proxy", ".lower,localdomain.com");
///     let for_domain = Url::from_str("https://server.lower.localdomain.com").unwrap();
///     let info = proxy::proxy_unless_domain_exempted(Some(&for_domain)).unwrap();
///
///     assert!(info.is_none());
/// }
/// ```
///
/// Behavior when domain does not match extension set:
///
/// ```
/// extern crate habitat_http_client;
/// extern crate url;
///
/// use std::str::FromStr;
/// use url::Url;
/// use habitat_http_client::proxy;
///
/// fn main() {
///     std::env::set_var("https_proxy", "http://itsme:asecret@proxy.example.com:8001/");
///     std::env::set_var("no_proxy", "localhost,127.0.0.1,localaddress,.localdomain.com");
///     let for_domain = Url::from_str("https://www.example.com").unwrap();
///     let info = proxy::proxy_unless_domain_exempted(Some(&for_domain)).unwrap().unwrap();
///
///     assert_eq!(info.scheme(), "http");
///     assert_eq!(info.host(), "proxy.example.com");
///     assert_eq!(info.port(), 8001);
///     assert_eq!(info.authorization_header_value().unwrap(), "Basic aXRzbWU6YXNlY3JldA==");
/// }
/// ```
///
pub fn proxy_unless_domain_exempted(for_domain: Option<&Url>) -> Result<Option<ProxyInfo>> {
    let scheme = match for_domain {
        Some(url) => url.scheme(),
        None => "",
    };
    match env::var("no_proxy") {
        Ok(domains) => process_no_proxy(for_domain, scheme, domains),
        _ => {
            match env::var("NO_PROXY") {
                Ok(domains) => process_no_proxy(for_domain, scheme, domains),
                _ => {
                    match scheme {
                        "https" => https_proxy(),
                        _ => http_proxy(),
                    }
                }
            }
        }
    }
}

fn process_no_proxy(
    for_domain: Option<&Url>,
    scheme: &str,
    domains: String,
) -> Result<Option<ProxyInfo>> {
    let domain = match for_domain {
        Some(url) => url.host_str().unwrap_or(""),
        None => "",
    };
    for extension in domains.split(',') {
        if domain.ends_with(extension) {
            debug!(
                "Domain {} matches domain extension {} from no_proxy='{}'",
                &domain,
                &extension,
                &domains
            );
            return Ok(None);
        }
    }
    match scheme {
        "https" => https_proxy(),
        _ => http_proxy(),
    }
}

fn parse_proxy_url(url: &str) -> Result<Option<ProxyInfo>> {
    let url = try!(Url::parse(&url));
    let auth = match url.password() {
        Some(password) => {
            Some(ProxyBasicAuthorization::new(
                percent_decode(url.username().as_bytes())
                    .decode_utf8_lossy()
                    .into_owned(),
                percent_decode(password.as_bytes())
                    .decode_utf8_lossy()
                    .into_owned(),
            ))
        }
        None => None,
    };
    Ok(Some(try!(ProxyInfo::new(url, auth))))
}
