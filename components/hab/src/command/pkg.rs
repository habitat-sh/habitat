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

// Looking for `pkg::install`? That's in the `common` crate. You're welcome :)

pub mod binlink {
    use std::fs;
    use std::path::Path;
    use std::os::unix;

    use ansi_term::Colour::{Blue, Green, Yellow};
    use hcore::package::{PackageIdent, PackageInstall};

    use error::{Error, Result};
    use exec::find_command_in_pkg;

    pub fn start(ident: &PackageIdent,
                 binary: &str,
                 dest_path: &Path,
                 fs_root_path: &Path)
                 -> Result<()> {
        let dst_path = fs_root_path.join(try!(dest_path.strip_prefix("/")));
        let dst = dst_path.join(&binary);
        println!("{}",
                 Yellow.bold().paint(format!("» Symlinking {} from {} into {}",
                                             &binary,
                                             &ident,
                                             dst_path.display())));
        let pkg_install = try!(PackageInstall::load(&ident, Some(fs_root_path)));
        let src = match try!(find_command_in_pkg(binary, &pkg_install, fs_root_path)) {
            Some(c) => c,
            None => {
                return Err(Error::CommandNotFoundInPkg((pkg_install.ident().to_string(),
                                                        binary.to_string())))
            }
        };
        if !dst_path.is_dir() {
            println!("{} parent directory {}",
                     Green.paint("Ω Creating"),
                     dst_path.display());
            try!(fs::create_dir_all(&dst_path))
        }
        match fs::read_link(&dst) {
            Ok(path) => {
                if path != src {
                    try!(fs::remove_file(&dst));
                    try!(unix::fs::symlink(&src, &dst));
                }
            }
            Err(_) => try!(unix::fs::symlink(&src, &dst)),
        }
        println!("{}",
                 Blue.paint(format!("★ Binary {} from {} symlinked to {}",
                                    &binary,
                                    &pkg_install.ident(),
                                    &dst.display())));
        Ok(())
    }
}

pub mod build {
    use std::ffi::OsString;

    use error::Result;
    use command::studio;

    pub fn start(plan_context: &str,
                 root: Option<&str>,
                 src: Option<&str>,
                 keys: Option<&str>,
                 reuse: bool)
                 -> Result<()> {
        let mut args: Vec<OsString> = Vec::new();
        if let Some(root) = root {
            args.push("-r".into());
            args.push(root.into());
        }
        if let Some(src) = src {
            args.push("-s".into());
            args.push(src.into());
        }
        if let Some(keys) = keys {
            args.push("-k".into());
            args.push(keys.into());
        }
        args.push("build".into());
        if cfg!(not(target_os = "linux")) || reuse {
            args.push("-R".into());
        }
        args.push(plan_context.into());
        studio::start(args)
    }
}

pub mod exec {
    use std::env;
    use std::ffi::OsString;

    use hcore::package::{PackageIdent, PackageInstall};
    use hcore::fs::find_command;

    use error::{Error, Result};
    use exec;

    pub fn start(ident: &PackageIdent, command: &str, args: Vec<OsString>) -> Result<()> {
        let pkg_install = try!(PackageInstall::load(&ident, None));
        let env_path = try!(pkg_install.runtime_path());
        info!("Setting: PATH='{}'", &env_path);
        env::set_var("PATH", env_path);
        let command = match find_command(command) {
            Some(path) => path,
            None => return Err(Error::ExecCommandNotFound(command.to_string())),
        };
        let mut display_args = command.to_string_lossy().into_owned();
        for arg in &args {
            display_args.push(' ');
            display_args.push_str(arg.to_string_lossy().as_ref());
        }
        info!("Running: {}", display_args);
        exec::exec_command(command, args)
    }
}

pub mod export {
    use error::Result;
    use hcore::package::PackageIdent;

    #[allow(dead_code)]
    pub struct ExportFormat {
        pkg_ident: PackageIdent,
        cmd: String,
    }

    #[allow(dead_code)]
    impl ExportFormat {
        pub fn pkg_ident(&self) -> &PackageIdent {
            &self.pkg_ident
        }

        pub fn cmd(&self) -> &str {
            &self.cmd
        }
    }

    pub fn start(ident: &PackageIdent, format: &ExportFormat) -> Result<()> {
        inner::start(ident, format)
    }

    pub fn format_for(value: &str) -> Result<ExportFormat> {
        inner::format_for(value)
    }

    #[cfg(target_os = "linux")]
    mod inner {
        use command::pkg::exec;
        use common::command::package::install;
        use error::{Error, Result};
        use hcore::crypto::default_cache_key_path;
        use hcore::fs::{cache_artifact_path, FS_ROOT_PATH};
        use hcore::package::{PackageIdent, PackageInstall};
        use hcore::url::default_depot_url;
        use std::ffi::OsString;
        use std::path::Path;
        use std::str::FromStr;
        use super::ExportFormat;

        pub fn format_for(value: &str) -> Result<ExportFormat> {
            match value {
                "docker" => {
                    let format = ExportFormat {
                        pkg_ident: try!(PackageIdent::from_str("core/hab-pkg-dockerize")),
                        cmd: "hab-pkg-dockerize".to_string(),
                    };
                    Ok(format)
                }
                "aci" => {
                    let format = ExportFormat {
                        pkg_ident: try!(PackageIdent::from_str("core/hab-pkg-aci")),
                        cmd: "hab-pkg-aci".to_string(),
                    };
                    Ok(format)
                }
                "mesos" => {
                    let format = ExportFormat {
                        pkg_ident: try!(PackageIdent::from_str("core/hab-pkg-mesosize")),
                        cmd: "hab-pkg-mesosize".to_string(),
                    };
                    Ok(format)
                }
                "tar" => {
                    let format = ExportFormat {
                        pkg_ident: try!(PackageIdent::from_str("core/hab-pkg-tarize")),
                        cmd: "hab-pkg-tarize".to_string(),
                    };
                    Ok(format)
                }
                _ => Err(Error::UnsupportedExportFormat(value.to_string())),
            }
        }

        pub fn start(ident: &PackageIdent, format: &ExportFormat) -> Result<()> {
            let format_ident = format.pkg_ident();
            match PackageInstall::load(format.pkg_ident(), None) {
                Ok(_) => {}
                _ => {
                    println!("{} is not installed", &format_ident.to_string());
                    println!("Searching for {} in remote {}",
                             &format_ident.to_string(),
                             &default_depot_url());
                    try!(install::from_url(&default_depot_url(),
                                           format_ident,
                                           Path::new(FS_ROOT_PATH),
                                           &cache_artifact_path(None),
                                           &default_cache_key_path(None)));
                }
            }
            let pkg_arg = OsString::from(&ident.to_string());
            exec::start(&format_ident, &format.cmd(), vec![pkg_arg])
        }
    }

    #[cfg(not(target_os = "linux"))]
    mod inner {
        use ansi_term::Colour::Yellow;
        use error::{Error, Result};
        use hcore::package::PackageIdent;
        use std::env;
        use super::ExportFormat;

        pub fn format_for(value: &str) -> Result<ExportFormat> {
            let msg = format!("∅ Exporting {} packages from this operating system is not yet \
                               supported. Try running this command again on a 64-bit Linux \
                               operating system.\n",
                              value);
            println!("{}", Yellow.bold().paint(msg));
            let e = Error::UnsupportedExportFormat(value.to_string());
            Err(e)
        }

        pub fn start(_ident: &PackageIdent, _format: &ExportFormat) -> Result<()> {
            let subcmd = env::args().nth(1).unwrap_or("<unknown>".to_string());
            let subsubcmd = env::args().nth(2).unwrap_or("<unknown>".to_string());
            let msg = format!("∅ Exporting packages from this operating system is not yet \
                               supported. Try running this command again on a 64-bit Linux \
                               operating system.\n");
            println!("{}", Yellow.bold().paint(msg));
            Err(Error::SubcommandNotSupported(format!("{} {}", subcmd, subsubcmd)))

        }
    }
}

pub mod hash {
    use hcore::crypto::hash;

    use error::Result;

    pub fn start(src: &str) -> Result<()> {
        let h = try!(hash::hash_file(&src));
        println!("{}", h);
        Ok(())
    }
}

pub mod path {
    use std::path::Path;

    use hcore::package::{PackageIdent, PackageInstall};

    use error::Result;

    pub fn start(ident: &PackageIdent, fs_root_path: &Path) -> Result<()> {
        let pkg_install = try!(PackageInstall::load(ident, Some(fs_root_path)));
        println!("{}", pkg_install.installed_path().display());
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
    //! $ hab pkg upload /path/to/acme-redis-2.0.7-2112010203120101-x86_64-linux.hart \
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
    use hcore::crypto::artifact::get_artifact_header;
    use hcore::crypto::keys::parse_name_with_rev;
    use hcore::package::{PackageArchive, PackageIdent};
    use depot_client::{self, Client};
    use hyper::status::StatusCode::{self, Forbidden, Unauthorized};

    use error::{Error, Result};

    /// Upload a package from the cache to a Depot. The latest version/release of the package
    /// will be uploaded if not specified.
    ///
    /// # Failures
    ///
    /// * Fails if it cannot find a package
    /// * Fails if the package doesn't have a `.hart` file in the cache
    /// * Fails if it cannot upload the file
    pub fn start<P: AsRef<Path>>(url: &str,
                                 token: &str,
                                 archive_path: &P,
                                 key_path: &P)
                                 -> Result<()> {
        let mut archive = PackageArchive::new(PathBuf::from(archive_path.as_ref()));

        let hart_header = try!(get_artifact_header(&archive_path.as_ref()));

        let key_buf = key_path.as_ref().to_path_buf();
        let public_keyfile_name = format!("{}.pub", &hart_header.key_name);
        let public_keyfile = key_buf.join(&public_keyfile_name);

        println!("{}",
                 Green.paint(format!("☛ Artifact signed with {}", &public_keyfile_name)));

        let (name, rev) = try!(parse_name_with_rev(&hart_header.key_name));
        let depot_client = try!(Client::new(url, None));

        println!("{}",
                 Yellow.bold().paint(format!("» Uploading origin key {}", &public_keyfile_name)));

        match depot_client.put_origin_key(&name, &rev, &public_keyfile, token, None) {
            Ok(()) => {
                println!("{} {}",
                         Green.bold().paint("✓ Uploaded origin key"),
                         &public_keyfile_name);
            }
            Err(e @ depot_client::Error::HTTP(Forbidden)) |
            Err(e @ depot_client::Error::HTTP(Unauthorized)) => {
                return Err(Error::from(e));
            }

            Err(e @ depot_client::Error::HTTP(_)) => {
                debug!("Error uploading public key {}", e);
                println!("{} {}",
                         Green.bold()
                             .paint("→ Public key revision already exists in the depot"),
                         &public_keyfile_name);
            }
            Err(e) => {
                return Err(Error::DepotClient(e));
            }
        };

        println!("{}",
                 Yellow.bold().paint(format!("» Uploading {}", archive_path.as_ref().display())));

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
