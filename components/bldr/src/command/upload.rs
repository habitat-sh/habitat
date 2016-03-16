// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! Uploads a package to a [Depot](../depot).
//!
//! # Examples
//!
//! ```bash
//! $ bldr upload chef/redis -u http://localhost:9632
//! ```
//!
//! Will upload a package to the Depot.
//!
//! # Notes
//!
//! This should be extended to cover uploading specific packages, and finding them by ways more
//! complex than just latest version.
//!

use std::path::PathBuf;

use hyper::status::StatusCode;

use error::{BldrResult, BldrError, ErrorKind};
use config::Config;
use package::archive::PackageArchive;
use depot;

static LOGKEY: &'static str = "CU";

/// Upload a package from the cache to a Depot. The latest version/release of the package
/// will be uploaded if not specified.
///
/// # Failures
///
/// * Fails if it cannot find a package
/// * Fails if the package doesn't have a `.bldr` file in the cache
/// * Fails if it cannot upload the file
pub fn package(config: &Config) -> BldrResult<()> {
    let url = config.url().as_ref().unwrap();
    let mut archive_path = PathBuf::from(config.archive().clone());
    archive_path.pop();
    let mut pa = PackageArchive::new(PathBuf::from(config.archive()));
    outputln!("Checking that all dependencies are in the depot...");
    let tdeps = try!(pa.tdeps());
    for dep in tdeps.iter() {
        match depot::client::show_package(url, dep) {
            Ok(_) => outputln!("{} is present", dep),
            Err(BldrError{ err: ErrorKind::RemotePackageNotFound(_), .. }) => {
                let dep_path = PathBuf::from(format!("{}/{}",
                                                     archive_path.to_string_lossy(),
                                                     dep.archive_name().unwrap()));
                if dep_path.is_file() {
                    outputln!("Uploading {} from {}", dep, dep_path.to_string_lossy());
                    let mut dpa = PackageArchive::new(dep_path);
                    match upload(&mut dpa, &url) {
                        Ok(()) => {}
                        Err(e @ BldrError{ err: ErrorKind::HTTP(_), .. }) => return Err(e),
                        Err(e @ BldrError{ err: ErrorKind::PackageArchiveMalformed(_), .. }) => {
                            return Err(e)
                        }
                        Err(_) => {
                            // This is because of a bug where the depot and client code seem to not
                            // correctly deal with conflicts.
                            outputln!("Trying the next package anyway...");
                        }
                    }
                } else {
                    outputln!("Cannot find an archive for {} at {}",
                              dep.archive_name().unwrap(),
                              archive_path.to_string_lossy());
                    return Err(bldr_error!(ErrorKind::FileNotFound(archive_path.to_string_lossy()
                                                                           .into_owned())));
                }
            }
            Err(e) => return Err(e),
        }
    }
    let ident = try!(pa.ident());
    match depot::client::show_package(url, &ident) {
        Ok(_) => outputln!("{} is present", ident),
        Err(_) => {
            outputln!("Uploading {} from {}", ident, pa.path.to_string_lossy());
            try!(upload(&mut pa, &url));
            outputln!("Complete");
        }
    }
    Ok(())
}

pub fn upload(mut pa: &mut PackageArchive, url: &str) -> BldrResult<()> {
    match depot::client::put_package(url, &mut pa) {
        Ok(()) => (),
        Err(BldrError{err: ErrorKind::HTTP(StatusCode::Conflict), ..}) => {
            outputln!("Package already exists on remote; skipping.");
        }
        Err(BldrError{err: ErrorKind::HTTP(StatusCode::UnprocessableEntity), ..}) => {
            return Err(bldr_error!(ErrorKind::PackageArchiveMalformed(format!("{}", pa.path.to_string_lossy()))));
        }
        Err(e @ BldrError{err: ErrorKind::HTTP(_), ..}) => {
            outputln!("Unexpected response from remote");
            return Err(e);
        }
        Err(e) => {
            outputln!("The package might exist on the remote - we fast abort, so.. :)");
            return Err(e);
        }
    }
    Ok(())
}
