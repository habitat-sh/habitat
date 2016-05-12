// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std;
use std::env::VarError;
use std::ffi::{OsStr, OsString};

use extern_url;

use error::{Error, Result};

/// Fetches the environment variable `key` from the current process, but only it is not empty.
///
/// This function augments the `std::env::var` function from the standard library, only by
/// returning a `VarError::NotPresent` if the environment variable is set, but the value is empty.
///
/// # Examples
///
/// ```
/// use std;
/// use habitat_core;
///
/// let key = "_I_AM_A_TEAPOT_COMMA_RIGHT_PEOPLE_QUESTION_MARK_";
/// std::env::set_var(key, "");
/// match habitat_core::env::var(key) {
///     Ok(val) => panic!("The environment variable {} is set but empty!", key),
///     Err(e) => println!("The environment variable {} is set, but empty. Not useful!", key),
/// }
/// ```
pub fn var<K: AsRef<OsStr>>(key: K) -> std::result::Result<String, VarError> {
    match std::env::var(key) {
        Ok(val) => {
            if val.is_empty() {
                Err(VarError::NotPresent)
            } else {
                Ok(val)
            }
        }
        Err(e) => Err(e),
    }
}

/// Fetches the environment variable `key` from the current process, but only it is not empty.
///
/// This function augments the `std::env::var_os` function from the standard library, only by
/// returning a `VarError::NotPresent` if the environment variable is set, but the value is empty.
///
/// # Examples
///
/// ```
/// use std;
/// use habitat_core;
///
/// let key = "_I_AM_A_TEAPOT_COMMA_RIGHT_PEOPLE_QUESTION_MARK_";
/// std::env::set_var(key, "");
/// match habitat_core::env::var_os(key) {
///     Some(val) => panic!("The environment variable {} is set but empty!", key),
///     None => println!("The environment variable {} is set, but empty. Not useful!", key),
/// }
/// ```
pub fn var_os<K: AsRef<OsStr>>(key: K) -> std::option::Option<OsString> {
    match std::env::var_os(key) {
        Some(val) => {
            if val.to_string_lossy().as_ref().is_empty() {
                None
            } else {
                Some(val)
            }
        }
        None => None,
    }
}

/// Returns a host/port tuple from the `http_proxy` environment variable, if it is set.
///
/// The value of `http_proxy` must be a parseable URL such as `http://proxy.company.com:8001/`
/// otherwise a parsing error will be returned. If the port is not present, than the default port
/// numbers of http/80 and https/443 will be returned. Currently user authentication is not
/// supported. If the `http_proxy` environment variable is not set or is empty, then a `Result` of
/// `None` will be returned.
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
/// use habitat_core::env;
///
/// std::env::set_var("http_proxy", "http://proxy.example.com:8001/");
/// let (host, port) = env::http_proxy().unwrap().unwrap();
///
/// assert_eq!(&host, "proxy.example.com");
/// assert_eq!(port, 8001);
/// ```
/// Behavior when environment variable is empty:
///
/// ```
/// use std;
/// use habitat_core::env;
///
/// std::env::set_var("http_proxy", "");
///
/// assert_eq!(env::http_proxy().unwrap(), None);
/// ```
///
/// # Errors
///
/// * If the value of the `http_proxy` environment variable cannot be parsed as a URL
/// * If the URL scheme is not a valid type (currently only supported schemes are http and https)
/// * If the URL is missing a host/domain segement
/// * If the URL is missing a port number and a default cannot be determined
pub fn http_proxy() -> Result<Option<(String, u16)>> {
    match self::var("http_proxy") {
        Ok(url) => {
            let url = try!(extern_url::Url::parse(&url));
            match url.scheme() {
                "http" | "https" => (),
                scheme => {
                    let msg = format!("Invalid scheme {} for {}", &scheme, &url);
                    return Err(Error::InvalidProxyValue(msg));
                }
            };
            let host = match url.host_str() {
                Some(val) => val,
                None => return Err(Error::UrlParseError(extern_url::ParseError::EmptyHost)),
            };
            let port = match url.port_or_known_default() {
                Some(val) => val,
                None => return Err(Error::UrlParseError(extern_url::ParseError::InvalidPort)),
            };
            Ok(Some((host.to_string(), port)))
        }
        _ => Ok(None),
    }
}

/// Returns a host/port tuple from the `http_proxy` environment variable if it is set and the given
/// domain name is not matched in the `no_proxy` environment variable domain extension set.
///
/// See the [`http_proxy()`] function for more details about the `http_proxy` environment variable
/// parsing. This function honors the `no_proxy` environment variable which is assumed to be a
/// comma separated set of domain extensions. If the given domain matches one of these extensions
/// then no proxy information should be returned (i.e. a return of `Ok(None)`).
///
/// # Errors
///
/// * If [`http_proxy()`] was unsuccessful
///
/// [`http_proxy()`]: #method.http_proxy
///
/// # Examples
///
/// Behavior when domain matches extension set:
///
/// ```
/// use std;
/// use habitat_core::env;
///
/// std::env::set_var("http_proxy", "http://proxy.example.com:8001/");
/// std::env::set_var("no_proxy", "localhost,127.0.0.1,localaddress,.localdomain.com");
///
/// assert_eq!(env::http_proxy_unless_domain_exempted("server.localdomain.com").unwrap(), None);
/// ```
///
/// Behavior when domain does not match extension set:
///
/// ```
/// use std;
/// use habitat_core::env;
///
/// std::env::set_var("http_proxy", "http://proxy.example.com:8001/");
/// std::env::set_var("no_proxy", "localhost,127.0.0.1,localaddress,.localdomain.com");
/// let (host, port) = env::http_proxy_unless_domain_exempted("www.example.com").unwrap().unwrap();
///
/// assert_eq!(&host, "proxy.example.com");
/// assert_eq!(port, 8001);
/// ```
///
pub fn http_proxy_unless_domain_exempted(domain: &str) -> Result<Option<(String, u16)>> {
    match self::var("no_proxy") {
        Ok(domains) => {
            for extension in domains.split(',') {
                if domain.ends_with(extension) {
                    debug!("Domain {} matches domain extension {} from no_proxy='{}'",
                           &domain,
                           &extension,
                           &domains);
                    return Ok(None);
                }
            }
            http_proxy()
        }
        _ => http_proxy(),
    }
}
