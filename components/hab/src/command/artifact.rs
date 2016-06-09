// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

pub mod hash {
    use hcore::crypto::hash;

    use error::Result;

    pub fn start(src: &str) -> Result<()> {
        let h = try!(hash::hash_file(&src));
        println!("{}", h);
        Ok(())
    }
}

pub mod sign {
    use std::path::Path;

    use ansi_term::Colour::{Blue, Green, Yellow};
    use hcore::crypto::{artifact, SigKeyPair};

    use error::Result;

    pub fn start(origin: &SigKeyPair, src: &Path, dst: &Path) -> Result<()> {
        println!("{}",
                 Yellow.bold().paint(format!("» Signing {}", src.display())));
        println!("{} {} with {} to create {}",
                 Green.paint("☛ Signing"),
                 src.display(),
                 &origin.name_with_rev(),
                 dst.display());
        try!(artifact::sign(src, dst, origin));
        println!("{}",
                 Blue.paint(format!("★ Signed artifact {}.", dst.display())));
        Ok(())
    }
}

pub mod upload {
    //! Uploads a package to a [Depot](../depot).
    //!
    //! # Examples
    //!
    //! ```bash
    //! $ hab artifact upload /path/to/acme-redis-2.0.7-2112010203120101-x86_64-linux.hart \
    //!     -u http://localhost:9632
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

    use ansi_term::Colour::{Blue, Green, Red, Yellow};
    use common::command::ProgressBar;
    use hcore::package::{PackageArchive, PackageIdent};
    use depot_client::{self, Client};
    use hyper::status::StatusCode;

    use error::{Error, Result};

    /// Upload a package from the cache to a Depot. The latest version/release of the package
    /// will be uploaded if not specified.
    ///
    /// # Failures
    ///
    /// * Fails if it cannot find a package
    /// * Fails if the package doesn't have a `.hart` file in the cache
    /// * Fails if it cannot upload the file
    pub fn start<P: AsRef<Path>>(url: &str, token: &str, archive_path: &P) -> Result<()> {
        let mut archive = PackageArchive::new(PathBuf::from(archive_path.as_ref()));
        println!("{}",
                 Yellow.bold().paint(format!("» Uploading {}", archive_path.as_ref().display())));
        let depot_client = try!(Client::new(url, None));
        let tdeps = try!(archive.tdeps());
        for dep in tdeps.into_iter() {
            match depot_client.show_package(dep.clone()) {
                Ok(_) => println!("{} {}", Green.paint("→ Exists"), &dep),
                Err(depot_client::Error::RemotePackageNotFound(_)) => {
                    let candidate_path = match archive_path.as_ref().parent() {
                        Some(p) => PathBuf::from(p),
                        None => unreachable!(),
                    };
                    try!(attempt_upload_dep(&depot_client, token, &dep, &candidate_path));
                }
                Err(e) => return Err(Error::from(e)),
            }
        }
        let ident = try!(archive.ident());
        match depot_client.show_package(ident.clone()) {
            Ok(_) => println!("{} {}", Green.paint("→ Exists"), &ident),
            Err(_) => {
                try!(upload_into_depot(&depot_client, token, &ident, &mut archive));
            }
        }
        println!("{}",
                 Blue.paint(format!("★ Upload of {} complete.", &ident)));
        Ok(())
    }

    fn upload_into_depot(depot_client: &Client,
                         token: &str,
                         ident: &PackageIdent,
                         mut archive: &mut PackageArchive)
                         -> Result<()> {
        println!("{} {}",
                 Green.bold().paint("↑ Uploading"),
                 archive.path.display());
        let mut progress = ProgressBar::default();
        match depot_client.put_package(&mut archive, token, Some(&mut progress)) {
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
        };
        println!("{} {}", Green.bold().paint("✓ Uploaded"), ident);
        Ok(())
    }

    fn attempt_upload_dep(depot_client: &Client,
                          token: &str,
                          ident: &PackageIdent,
                          archives_dir: &PathBuf)
                          -> Result<()> {
        let candidate_path = archives_dir.join(ident.archive_name().unwrap());

        if candidate_path.is_file() {
            let mut archive = PackageArchive::new(candidate_path);
            match upload_into_depot(&depot_client, token, &ident, &mut archive) {
                Ok(()) => Ok(()),
                Err(Error::DepotClient(depot_client::Error::HTTP(e))) => {
                    return Err(Error::DepotClient(depot_client::Error::HTTP(e)))
                }
                Err(Error::PackageArchiveMalformed(e)) => {
                    return Err(Error::PackageArchiveMalformed(e))
                }
                Err(e) => {
                    println!("Unknown error encountered: {:?}", e);
                    return Err(e);
                }
            }
        } else {
            println!("{} artifact for {} was not found in {}",
                     Red.bold().paint("✗ Missing"),
                     ident.archive_name().unwrap(),
                     archives_dir.display());
            return Err(Error::FileNotFound(archives_dir.to_string_lossy()
                .into_owned()));
        }
    }
}

pub mod verify {
    use std::path::Path;

    use ansi_term::Colour::{Blue, Green, Yellow};
    use hcore::crypto::artifact;

    use error::Result;

    pub fn start(src: &Path, cache: &Path) -> Result<()> {
        println!("{}",
                 Yellow.bold().paint(format!("» Verifying artifact {}", &src.display())));
        let (name_with_rev, hash) = try!(artifact::verify(src, cache));
        println!("{} checksum {} signed with {}",
                 Green.bold().paint("✓ Verifed"),
                 &hash,
                 &name_with_rev);
        println!("{}",
                 Blue.paint(format!("★ Verified artifact {}.", &src.display())));
        Ok(())
    }
}
