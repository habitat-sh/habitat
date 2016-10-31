// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

extern crate habitat_core as hab_core;
extern crate httparse;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate log;
extern crate openssl;
extern crate rustc_serialize;
extern crate url;

pub mod api_client;
pub mod error;
pub mod net;
pub mod proxy;

pub use api_client::ApiClient;
pub use error::{Error, Result};

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
                try!(fs::create_dir_all(cache_ssl_path(fs_root_path)));
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
