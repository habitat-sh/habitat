// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

extern crate habitat_core as hab_core;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate log;
extern crate openssl;

pub mod error;

use std::sync::Arc;
use std::path::Path;

use hab_core::env::http_proxy_unless_domain_exempted;
use hyper::client::Client;
use hyper::client::pool::{Config, Pool};
use hyper::http::h1::Http11Protocol;
use hyper::net::{HttpsConnector, Openssl};
use hyper::Url;
use openssl::ssl::{SslContext, SslMethod, SSL_OP_NO_SSLV2, SSL_OP_NO_SSLV3, SSL_OP_NO_COMPRESSION};

pub use error::{Error, Result};

/// Builds a new hyper HTTP client with appropriate SSL configuration and HTTP/HTTPS proxy support.
///
/// ## Linux Platforms
///
/// We need a set of root certificates when connected to SSL/TLS web endpoints and this usually
/// boild down to using an all-in-one certificate file (such as a `cacert.pem` file) or a directory
/// of files which are certificates. The strategy to location or use a reasonable set of
/// certificates is as follows:
///
/// 1. If the `SSL_CERT_FILE` environment variable is set, then use its value for the certificates.
///    Interally this is triggering default OpenSSL behavior for this environment variable.
/// 2. If the `SSL_CERT_DIR` environment variable is set, then use its value for the directory
///    containing certificates. Like the `SSL_CERT_FILE` case above, this triggers default OpenSSL
///    behavior for this environment variable.
/// 3. If the `core/cacerts` Habitat package is installed locally, then use the latest release's
///    `cacert.pem` file.
/// 4. If none of the conditions above are met, then a `cacert.pem` will be written in an SSL cache
///    directory (by default `/hab/cache/ssl` for a root user and `$HOME/.hab/cache/ssl` for a
///    non-root user) and this will be used. The contents of this file will be inlined in this
///    crate at build time as a fallback insurance policy, meaning that if the a program using this
///    code is operating in a minimal environment which may not contain system certificates, it can
///    still operate. Once a `core/cacerts` Habitat package is present, the logic would fall back
///    to preferring the package version to the cached/inline file version.
///
/// ## Mac Platforms
///
/// The Mac platoform uses a Security Framework to store and find root certificates and the hyper
/// library will default to using this on the Mac. Therefore the behavior on the Mac remains
/// unchanged and will use the system's certificates.
///
pub fn new_hyper_client(for_domain: Option<&Url>, fs_root_path: Option<&Path>) -> Result<Client> {
    let for_domain = match for_domain {
        Some(url) => url.host_str().unwrap_or(""),
        None => "",
    };
    match try!(http_proxy_unless_domain_exempted(for_domain)) {
        Some((proxy_host, proxy_port)) => {
            debug!("Using proxy {}:{}...", &proxy_host, &proxy_port);
            Ok(Client::with_http_proxy(proxy_host, proxy_port))
        }
        None => {
            let ctx = try!(ssl_ctx(fs_root_path));
            let connector = HttpsConnector::new(Openssl { context: Arc::new(ctx) });
            let pool = Pool::with_connector(Config::default(), connector);
            Ok(Client::with_protocol(Http11Protocol::with_connector(pool)))
        }
    }
}

fn ssl_ctx(fs_root_path: Option<&Path>) -> Result<SslContext> {
    // The spirit of this implementation is directly from Hyper's default OpensslClient function:
    // https://github.com/hyperium/hyper/blob/v0.9.5/src/net.rs#L653-L661
    let mut ctx = try!(SslContext::new(SslMethod::Sslv23));
    try!(ssl::set_ca(&mut ctx, fs_root_path));
    ctx.set_options(SSL_OP_NO_SSLV2 | SSL_OP_NO_SSLV3 | SSL_OP_NO_COMPRESSION);
    try!(ctx.set_cipher_list("ALL!EXPORT!EXPORT40!EXPORT56!aNULL!LOW!RC4@STRENGTH"));
    Ok(ctx)
}

#[cfg(target_os = "linux")]
mod ssl {
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;
    use std::str::FromStr;

    use hab_core::env;
    use hab_core::fs::cache_ssl_path;
    use hab_core::package::{PackageIdent, PackageInstall};
    use openssl::ssl::SslContext;

    use error::Result;

    const CACERTS_PKG_IDENT: &'static str = "core/cacerts";
    const CACERT_PEM: &'static str = include_str!(concat!(env!("OUT_DIR"), "/cacert.pem"));

    pub fn set_ca(ctx: &mut SslContext, fs_root_path: Option<&Path>) -> Result<()> {
        let cacerts_ident = try!(PackageIdent::from_str(CACERTS_PKG_IDENT));

        if let Ok(_) = env::var("SSL_CERT_FILE") {
            try!(ctx.set_default_verify_paths());
        } else if let Ok(_) = env::var("SSL_CERT_DIR") {
            try!(ctx.set_default_verify_paths());
        } else if let Ok(pkg_install) = PackageInstall::load(&cacerts_ident, fs_root_path) {
            let pkg_certs = pkg_install.installed_path().join("ssl/cert.pem");
            debug!("Setting CA file for SSL context to: {}",
                   pkg_certs.display());
            try!(ctx.set_CA_file(pkg_certs));
        } else {
            let cached_certs = cache_ssl_path(fs_root_path).join("cert.pem");
            if !cached_certs.exists() {
                try!(fs::create_dir_all(cache_ssl_path(None)));
                debug!("Creating cached cacert.pem at: {}", cached_certs.display());
                let mut file = try!(File::create(&cached_certs));
                try!(file.write_all(CACERT_PEM.as_bytes()));
            }
            debug!("Setting CA file for SSL context to: {}",
                   cached_certs.display());
            try!(ctx.set_CA_file(cached_certs));
        }
        Ok(())
    }
}

#[cfg(not(target_os = "linux"))]
mod ssl {
    use std::path::Path;

    use openssl::ssl::SslContext;

    use error::Result;

    pub fn set_ca(ctx: &mut SslContext, _fs_root_path: Option<&Path>) -> Result<()> {
        try!(ctx.set_default_verify_paths());
        Ok(())
    }
}
