// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

//! Modules which can be serialized and deserialized to and from Google Protobufs.
//!
//! The types in the contained modules are largely generated or are new-type wrappers around
//! generated code. All generated code is placed in [`generated`] which comes from Protobuf
//! definition files in the `protocols` directory at the root of the Supervisor crate.
//!
//! # Defining New Protocols
//!
//! A new generated module is created at `protocols::generated::{T}` where `T` is the name of your
//! Protobuf file placed in the `protocols` directory. For example, given the file
//! `protocols/net.proto`, a new Rust module will be placed at `src/generated/net.rs`.
//!
//! Each time a new Protobuf file is added, you will need to update the `generated` module with
//! an entry of the newly generated module. Given the previous example, you will need to add
//! `pub mod net` to the generated module.
//!
//! WARNING: Do not attempt to create a protocol named after a reserved Rust type, such as `mod`,
//!          as this will lead to undefined behaviour.
//!
//! # Generating Protocols
//!
//! Protocol files are generated when the `protocols` feature is passed to Cargo. You can do this
//! by running `cargo build --features protocols`. This command should be run each time a new
//! protocol file is added or anytime one is changed. Generated files are to be committed to
//! version control. Files are generated on your workstation and committed to version control and
//! *not* by a build server intentionally. This is to ensure we have the source available for
//! all protocol files.

use habitat_core as core;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate prost_derive;

#[macro_use]
extern crate serde_derive;

#[cfg(test)]
extern crate tempfile;

pub mod butterfly;
pub mod codec;
pub mod ctl;
pub mod message;
pub mod net;
pub mod types;

use crate::{core::env as henv,
            net::{ErrCode,
                  NetResult}};
use rand::RngCore;
use std::{fs::File,
          io::Read,
          net::SocketAddr,
          path::{Path,
                 PathBuf}};

// Name of file containing the CtlGateway secret key.
const CTL_SECRET_FILENAME: &str = "CTL_SECRET";
/// Length of characters in CtlGateway secret key.
const CTL_SECRET_LEN: usize = 64;

lazy_static! {
    /// The root path containing all runtime service directories and files
    pub static ref STATE_PATH_PREFIX: PathBuf = {
        Path::new(&*core::fs::FS_ROOT_PATH).join("hab/sup")
    };

    pub static ref DEFAULT_BLDR_URL: String = {
        core::url::default_bldr_url()
    };
}

/// Generate a new secret key used for authenticating clients to the `CtlGateway`.
pub fn generate_secret_key(out: &mut String) {
    let mut rng = rand::rngs::OsRng::new().unwrap();
    let mut result = vec![0u8; CTL_SECRET_LEN];
    rng.fill_bytes(&mut result);
    *out = base64::encode(&result);
}

/// Read the secret key used to authenticate connections to the `CtlGateway` from disk and write
/// it to the given out buffer. An `Ok` return value of `true` indicates a successful read while
/// `false` indicates the file was not found.
pub fn read_secret_key<T>(sup_root: T, out: &mut String) -> NetResult<bool>
    where T: AsRef<Path>
{
    let secret_key_path = sup_root.as_ref().join(CTL_SECRET_FILENAME);
    if secret_key_path.exists() {
        if secret_key_path.is_dir() {
            return Err(net::err(ErrCode::Io,
                                format!("Expected file but found directory when \
                                         reading ctl secret, {}",
                                        secret_key_path.display())));
        }
        File::open(&secret_key_path).and_then(|mut f| f.read_to_string(out))
                                    .map_err(move |e| {
                                        net::err(ErrCode::Io,
                                                 format!("IoError while reading or writing ctl \
                                                          secret, {}, {}",
                                                         secret_key_path.display(),
                                                         e))
                                    })?;
        *out = out.trim_end().into();
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Returns the location of the CtlGateway Secret on disk for the given Supervisor root.
pub fn secret_key_path<T>(sup_root: T) -> PathBuf
    where T: AsRef<Path>
{
    sup_root.as_ref().join(CTL_SECRET_FILENAME)
}

pub fn sup_root(custom_state_path: Option<&PathBuf>) -> PathBuf {
    match custom_state_path {
        Some(custom) => custom.to_path_buf(),
        // TODO: /hab/sup/default is legacy from when we allowed multiple
        // supervisors on the same host with --override-name. The sup dir
        // should really be /hab/sup now, but it would be an awkward change
        // since the assumption of /hab/sup/default is pervasive.
        // See https://github.com/habitat-sh/habitat/issues/5266
        None => STATE_PATH_PREFIX.join("default"),
    }
}

/// Given an Environment variable name, attempts to parse a SocketAddr from it.
/// If the Environment variable is empty or unparseable, returns the default as passed in.
pub fn socket_addr_env_or_default(env_var: &str, default: SocketAddr) -> SocketAddr {
    henv::var(env_var).unwrap_or_default()
                      .parse()
                      .unwrap_or(default)
}

#[cfg(test)]
mod ctl_secret {
    use super::*;
    use std::{fs::File,
              io::Write};
    use tempfile::TempDir;

    #[test]
    fn read_secret_key_file_no_newline() {
        let tmpdir = TempDir::new().unwrap();
        let file_path = tmpdir.path().to_owned().join("CTL_SECRET");
        let mut secret_file = File::create(file_path).unwrap();
        write!(secret_file,
               "w9TuoqTk4Ixaht8ZpJpHQlmPRbvpgz13GaGnvxunJy8iOhZcS7qGqEA7jogq/\
                Itfu4HOdQGmLRY9G5fRUcuw/w==").unwrap();
        let mut out = String::new();
        assert_eq!(read_secret_key(tmpdir, &mut out), Ok(true));
        assert_eq!(out,
                   "w9TuoqTk4Ixaht8ZpJpHQlmPRbvpgz13GaGnvxunJy8iOhZcS7qGqEA7jogq/\
                    Itfu4HOdQGmLRY9G5fRUcuw/w==");
    }

    #[test]
    fn read_secret_key_file_newline_removed() {
        let tmpdir = TempDir::new().unwrap();
        let file_path = tmpdir.path().to_owned().join("CTL_SECRET");
        let mut secret_file = File::create(file_path).unwrap();
        writeln!(secret_file,
                 "w9TuoqTk4Ixaht8ZpJpHQlmPRbvpgz13GaGnvxunJy8iOhZcS7qGqEA7jogq/\
                  Itfu4HOdQGmLRY9G5fRUcuw/w==").unwrap();
        let mut out = String::new();
        assert_eq!(read_secret_key(tmpdir, &mut out), Ok(true));
        assert_eq!(out,
                   "w9TuoqTk4Ixaht8ZpJpHQlmPRbvpgz13GaGnvxunJy8iOhZcS7qGqEA7jogq/\
                    Itfu4HOdQGmLRY9G5fRUcuw/w==");
    }

    #[test]
    fn read_secret_key_file_crlf_removed() {
        let tmpdir = TempDir::new().unwrap();
        let file_path = tmpdir.path().to_owned().join("CTL_SECRET");
        let mut secret_file = File::create(file_path).unwrap();
        writeln!(secret_file,
                 "w9TuoqTk4Ixaht8ZpJpHQlmPRbvpgz13GaGnvxunJy8iOhZcS7qGqEA7jogq/\
                  Itfu4HOdQGmLRY9G5fRUcuw/w==\r").unwrap();
        let mut out = String::new();
        assert_eq!(read_secret_key(tmpdir, &mut out), Ok(true));
        assert_eq!(out,
                   "w9TuoqTk4Ixaht8ZpJpHQlmPRbvpgz13GaGnvxunJy8iOhZcS7qGqEA7jogq/\
                    Itfu4HOdQGmLRY9G5fRUcuw/w==");
    }

    #[test]
    fn read_secret_key_file_nonexistent() {
        let tmpdir = TempDir::new().unwrap();
        let mut out = String::new();
        assert_eq!(read_secret_key(tmpdir, &mut out), Ok(false));
        let empty = String::new();
        assert_eq!(empty, out);
    }
}
