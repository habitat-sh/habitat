// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

//! Uploads a package to a [Depot](../depot).
//!
//! # Examples
//!
//! ```bash
//! $ hab artifact upload /path/to/chef-redis-2.0.7-2112010203120101.hab -u http://localhost:9632
//! ```
//!
//! Will upload a package to the Depot.
//!
//! # Notes
//!
//! This should be extended to cover uploading specific packages, and finding them by ways more
//! complex than just latest version.
//!

use std::path::{Path, PathBuf};

use hcore::package::{PackageArchive, PackageIdent};
use depot_client;
use hyper::status::StatusCode;

use error::{Error, Result};

/// Upload a package from the cache to a Depot. The latest version/release of the package
/// will be uploaded if not specified.
///
/// # Failures
///
/// * Fails if it cannot find a package
/// * Fails if the package doesn't have a `.hab` file in the cache
/// * Fails if it cannot upload the file
pub fn start<P: AsRef<Path>>(url: &str, archive_path: &P) -> Result<()> {
    let mut archive = PackageArchive::new(PathBuf::from(archive_path.as_ref()));
    println!("Checking that all dependencies are in the depot...");
    let tdeps = try!(archive.tdeps());
    for dep in tdeps.iter() {
        match depot_client::show_package(url, dep) {
            Ok(_) => println!("Package {} is present in the depot", dep),
            Err(depot_client::Error::RemotePackageNotFound(_)) => {
                let candidate_path = match archive_path.as_ref().parent() {
                    Some(p) => PathBuf::from(p),
                    None => unreachable!(),
                };
                try!(attempt_upload_dep(url, dep, &candidate_path));
            }
            Err(e) => return Err(Error::from(e)),
        }
    }
    let ident = try!(archive.ident());
    match depot_client::show_package(url, &ident) {
        Ok(_) => println!("{} is present", ident),
        Err(_) => {
            try!(upload_into_depot(&url, &ident, &mut archive));
            println!("Complete");
        }
    }
    Ok(())
}

fn upload_into_depot(url: &str,
                     ident: &PackageIdent,
                     mut archive: &mut PackageArchive)
                     -> Result<()> {
    println!("Uploading {} from {}", ident, archive.path.display());
    match depot_client::put_package(url, &mut archive) {
        Ok(()) => (),
        Err(depot_client::Error::HTTP(StatusCode::Conflict)) => {
            println!("Package already exists on remote; skipping.");
        }
        Err(depot_client::Error::HTTP(StatusCode::UnprocessableEntity)) => {
            return Err(Error::PackageArchiveMalformed(format!("{}", archive.path.display())));
        }
        Err(e @ depot_client::Error::HTTP(_)) => {
            println!("Unexpected response from remote");
            return Err(Error::from(e));
        }
        Err(e) => {
            println!("The package might exist on the remote - we fast abort, so.. :)");
            return Err(Error::from(e));
        }
    }
    Ok(())
}

fn attempt_upload_dep(url: &str, ident: &PackageIdent, archives_dir: &PathBuf) -> Result<()> {
    let candidate_path = archives_dir.join(ident.archive_name().unwrap());

    if candidate_path.is_file() {
        let mut archive = PackageArchive::new(candidate_path);
        match upload_into_depot(&url, &ident, &mut archive) {
            Ok(()) => Ok(()),
            Err(Error::DepotClient(depot_client::Error::HTTP(e))) => {
                return Err(Error::DepotClient(depot_client::Error::HTTP(e)))
            }
            Err(Error::PackageArchiveMalformed(e)) => return Err(Error::PackageArchiveMalformed(e)),
            Err(_) => {
                // This is because of a bug where the depot and client code seem to not
                // correctly deal with conflicts.
                println!("Trying the next package anyway...");
                Ok(())
            }
        }
    } else {
        println!("Cannot find an archive for {} at {}",
                 ident.archive_name().unwrap(),
                 archives_dir.display());
        return Err(Error::FileNotFound(archives_dir.to_string_lossy()
                                                   .into_owned()));
    }
}
