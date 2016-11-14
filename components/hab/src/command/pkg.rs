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

    use common::ui::{Status, UI};
    use hcore::package::{PackageIdent, PackageInstall};
    use hcore::os::filesystem;

    use error::{Error, Result};
    use exec::find_command_in_pkg;

    pub fn start(ui: &mut UI,
                 ident: &PackageIdent,
                 binary: &str,
                 dest_path: &Path,
                 fs_root_path: &Path)
                 -> Result<()> {
        let dst_path = fs_root_path.join(try!(dest_path.strip_prefix("/")));
        let dst = dst_path.join(&binary);
        try!(ui.begin(format!("Symlinking {} from {} into {}",
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
            try!(ui.status(Status::Creating,
                           format!("parent directory {}", dst_path.display())));
            try!(fs::create_dir_all(&dst_path))
        }
        match fs::read_link(&dst) {
            Ok(path) => {
                if path != src {
                    try!(fs::remove_file(&dst));
                    try!(filesystem::symlink(&src, &dst));
                }
            }
            Err(_) => try!(filesystem::symlink(&src, &dst)),
        }
        try!(ui.end(format!("Binary {} from {} symlinked to {}",
                            &binary,
                            &pkg_install.ident(),
                            &dst.display())));
        Ok(())
    }
}

pub mod build {
    use std::ffi::OsString;

    use common::ui::UI;

    use error::Result;
    use command::studio;

    pub fn start(ui: &mut UI,
                 plan_context: &str,
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
        studio::start(ui, args)
    }
}

pub mod exec {
    use std::env;
    use std::ffi::OsString;

    use hcore::os::process;
    use hcore::package::{PackageIdent, PackageInstall};
    use hcore::fs::find_command;

    use error::{Error, Result};

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
        Ok(try!(process::become_command(command, args)))
    }
}

pub mod export {
    use common::ui::UI;
    use hcore::package::PackageIdent;

    use error::Result;

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

    pub fn start(ui: &mut UI, ident: &PackageIdent, format: &ExportFormat) -> Result<()> {
        inner::start(ui, ident, format)
    }

    pub fn format_for(ui: &mut UI, value: &str) -> Result<ExportFormat> {
        inner::format_for(ui, value)
    }

    #[cfg(target_os = "linux")]
    mod inner {
        use std::ffi::OsString;
        use std::path::Path;
        use std::str::FromStr;

        use common::command::package::install;
        use common::ui::{Status, UI};
        use hcore::fs::{cache_artifact_path, FS_ROOT_PATH};
        use hcore::package::{PackageIdent, PackageInstall};
        use hcore::url::default_depot_url;

        use {PRODUCT, VERSION};
        use command::pkg::exec;
        use error::{Error, Result};
        use super::ExportFormat;

        pub fn format_for(_ui: &mut UI, value: &str) -> Result<ExportFormat> {
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

        pub fn start(ui: &mut UI, ident: &PackageIdent, format: &ExportFormat) -> Result<()> {
            let format_ident = format.pkg_ident();
            match PackageInstall::load(format.pkg_ident(), None) {
                Ok(_) => {}
                _ => {
                    try!(ui.status(Status::Missing, format!("package for {}", &format_ident)));
                    try!(install::start(ui,
                                        &default_depot_url(),
                                        &format_ident.to_string(),
                                        PRODUCT,
                                        VERSION,
                                        Path::new(FS_ROOT_PATH),
                                        &cache_artifact_path(None)));
                }
            }
            let pkg_arg = OsString::from(&ident.to_string());
            exec::start(&format_ident, &format.cmd(), vec![pkg_arg])
        }
    }

    #[cfg(not(target_os = "linux"))]
    mod inner {
        use error::{Error, Result};
        use common::ui::UI;
        use hcore::package::PackageIdent;
        use std::env;
        use super::ExportFormat;

        pub fn format_for(ui: &mut UI, value: &str) -> Result<ExportFormat> {
            try!(ui.warn(format!("∅ Exporting {} packages from this operating system is not yet \
                               supported. Try running this command again on a 64-bit Linux \
                               operating system.\n",
                              value)));
            try!(ui.br());
            let e = Error::UnsupportedExportFormat(value.to_string());
            Err(e)
        }

        pub fn start(ui: &mut UI, _ident: &PackageIdent, _format: &ExportFormat) -> Result<()> {
            let subcmd = env::args().nth(1).unwrap_or("<unknown>".to_string());
            let subsubcmd = env::args().nth(2).unwrap_or("<unknown>".to_string());
            try!(ui.warn("Exporting packages from this operating system is not yet supported. Try \
                       running this command again on a 64-bit Linux operating system."));
            try!(ui.br());
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

pub mod provides {
    use std::collections::HashSet;
    use std::path::Path;

    use walkdir::WalkDir;

    use error::{Error, Result};
    use hcore::fs::PKG_PATH;

    pub fn start(filename: &str,
                 fs_root_path: &Path,
                 full_releases: bool,
                 full_path: bool)
                 -> Result<()> {
        let mut found = HashSet::new();
        // count the # of directories in the path to the package dir
        // ex: /hab/pkg == 2
        let prefix_count = Path::new(PKG_PATH).components().count();
        // the location of installed packages
        let pkg_root = fs_root_path.join(PKG_PATH);

        let mut found_any = false;

        // recursively walk the directories in pkg_root looking for matches
        for entry in WalkDir::new(pkg_root).into_iter().filter_map(|e| e.ok()) {
            if let Some(f) = entry.path().file_name().and_then(|f| f.to_str()) {

                if filename == f {
                    found_any = true;
                    let mut comps = entry.path().components();

                    // skip prefix_count segments of the path
                    let _ = try!(comps.nth(prefix_count).ok_or(Error::FileNotFound(f.to_string())));

                    let segments = if full_releases {
                        // take all 4 segments of the path
                        // ex: core/busybox-static/1.24.2/20160708162350
                        comps.take(4)
                    } else {
                        // only take 2 segments of the path
                        // ex: core/busybox-static
                        comps.take(2)
                    };

                    let mapped_segs: Vec<String> =
                        segments.map(|c| c.as_os_str().to_string_lossy().into_owned()).collect();
                    let pkg_name = mapped_segs.join("/");

                    // if we show the full path, then don't bother stuffing
                    // the result into the found HashSet, as we want to
                    // print out each path we find.
                    if full_path {
                        println!("{}: {}", &pkg_name, &entry.path().to_string_lossy());
                    } else {
                        found.insert(pkg_name);
                    }
                }
            }
        }
        // if we're not using full_path, then using a set will filter out
        // duplicates. This shows the filtered set of matches
        for entry in &found {
            println!("{}", entry);
        }
        if found_any {
            Ok(())
        } else {
            Err(Error::ProvidesError(filename.to_string()))
        }
    }
}

pub mod search {
    use error::Result;
    use depot_client::Client;
    use {PRODUCT, VERSION};

    pub fn start(st: &str, url: &str) -> Result<()> {
        let depot_client = try!(Client::new(url, PRODUCT, VERSION, None));
        let (packages, more) = try!(depot_client.search_package(st.to_string()));
        match packages.len() {
            0 => println!("No packages found that match '{}'", st),
            _ => {
                for p in &packages {
                    if let (&Some(ref version), &Some(ref release)) = (&p.version, &p.release) {
                        println!("{}/{}/{}/{}", p.origin, p.name, version, release);
                    } else {
                        println!("{}/{}", p.origin, p.name);
                    }
                }
                if more {
                    println!("Search returned too many items, only showing the first {}",
                             packages.len());
                }
            }
        }
        Ok(())
    }
}

pub mod sign {
    use std::path::Path;

    use common::ui::{Status, UI};
    use hcore::crypto::{artifact, SigKeyPair};

    use error::Result;

    pub fn start(ui: &mut UI, origin: &SigKeyPair, src: &Path, dst: &Path) -> Result<()> {
        try!(ui.begin(format!("Signing {}", src.display())));
        try!(ui.status(Status::Signing,
                       format!("{} with {} to create {}",
                               src.display(),
                               &origin.name_with_rev(),
                               dst.display())));
        try!(artifact::sign(src, dst, origin));
        try!(ui.end(format!("Signed artifact {}.", dst.display())));
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

    use common::ui::{Status, UI};
    use common::command::package::install::{RETRIES, RETRY_WAIT};
    use depot_client::{self, Client};
    use hcore::crypto::artifact::get_artifact_header;
    use hcore::crypto::keys::parse_name_with_rev;
    use hcore::package::{PackageArchive, PackageIdent};
    use hyper::status::StatusCode;

    use {PRODUCT, VERSION};
    use error::{Error, Result};

    use retry::retry;

    /// Upload a package from the cache to a Depot. The latest version/release of the package
    /// will be uploaded if not specified.
    ///
    /// # Failures
    ///
    /// * Fails if it cannot find a package
    /// * Fails if the package doesn't have a `.hart` file in the cache
    /// * Fails if it cannot upload the file
    pub fn start<P: AsRef<Path>>(ui: &mut UI,
                                 url: &str,
                                 token: &str,
                                 archive_path: &P,
                                 key_path: &P)
                                 -> Result<()> {
        let mut archive = PackageArchive::new(PathBuf::from(archive_path.as_ref()));

        let hart_header = try!(get_artifact_header(&archive_path.as_ref()));

        let key_buf = key_path.as_ref().to_path_buf();
        let public_keyfile_name = format!("{}.pub", &hart_header.key_name);
        let public_keyfile = key_buf.join(&public_keyfile_name);

        try!(ui.status(Status::Signed,
                       format!("artifact with {}", &public_keyfile_name)));

        let (name, rev) = try!(parse_name_with_rev(&hart_header.key_name));
        let depot_client = try!(Client::new(url, PRODUCT, VERSION, None));

        try!(ui.begin(format!("Uploading public origin key {}", &public_keyfile_name)));

        match depot_client.put_origin_key(&name, &rev, &public_keyfile, token, ui.progress()) {
            Ok(()) => {
                try!(ui.status(Status::Uploaded,
                               format!("public origin key {}", &public_keyfile_name)));
            }
            Err(depot_client::Error::APIError(StatusCode::Conflict, _)) => {
                try!(ui.status(Status::Using,
                               format!("existing public origin key {}", &public_keyfile_name)));
            }
            Err(err) => return Err(Error::from(err)),
        };

        try!(ui.begin(format!("Uploading {}", archive_path.as_ref().display())));
        let tdeps = try!(archive.tdeps());
        for dep in tdeps.into_iter() {
            match depot_client.show_package(dep.clone()) {
                Ok(_) => try!(ui.status(Status::Using, format!("existing {}", &dep))),
                Err(depot_client::Error::APIError(StatusCode::NotFound, _)) => {
                    let candidate_path = match archive_path.as_ref().parent() {
                        Some(p) => PathBuf::from(p),
                        None => unreachable!(),
                    };
                    if retry(RETRIES,
                             RETRY_WAIT,
                             || attempt_upload_dep(ui, &depot_client, token, &dep, &candidate_path),
                             |res| res.is_ok())
                        .is_err() {
                        return Err(Error::from(depot_client::Error::UploadFailed(format!("We tried \
                                                                                          {} times \
                                                                                          but \
                                                                                          could \
                                                                                          not \
                                                                                          upload \
                                                                                          {}. \
                                                                                          Giving \
                                                                                          up.",
                                                                                         RETRIES,
                                                                                         &dep))));
                    }
                }
                Err(e) => return Err(Error::from(e)),
            }
        }
        let ident = try!(archive.ident());
        match depot_client.show_package(ident.clone()) {
            Ok(_) => {
                try!(ui.status(Status::Using, format!("existing {}", &ident)));
                Ok(())
            }
            Err(depot_client::Error::APIError(StatusCode::NotFound, _)) => {
                if retry(RETRIES,
                         RETRY_WAIT,
                         || upload_into_depot(ui, &depot_client, token, &ident, &mut archive),
                         |res| res.is_ok())
                    .is_err() {
                    return Err(Error::from(depot_client::Error::UploadFailed(format!("We tried \
                                                                                      {} times \
                                                                                      but could \
                                                                                      not upload \
                                                                                      {}. Giving \
                                                                                      up.",
                                                                                     RETRIES,
                                                                                     &ident))));
                }
                try!(ui.end(format!("Upload of {} complete.", &ident)));
                Ok(())
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    fn upload_into_depot(ui: &mut UI,
                         depot_client: &Client,
                         token: &str,
                         ident: &PackageIdent,
                         mut archive: &mut PackageArchive)
                         -> Result<()> {
        try!(ui.status(Status::Uploading, archive.path.display()));
        match depot_client.put_package(&mut archive, token, ui.progress()) {
            Ok(_) => (),
            Err(depot_client::Error::APIError(StatusCode::Conflict, _)) => {
                println!("Package already exists on remote; skipping.");
            }
            Err(depot_client::Error::APIError(StatusCode::UnprocessableEntity, _)) => {
                return Err(Error::PackageArchiveMalformed(format!("{}", archive.path.display())));
            }
            Err(e) => return Err(Error::from(e)),
        };
        try!(ui.status(Status::Uploaded, ident));
        Ok(())
    }

    fn attempt_upload_dep(ui: &mut UI,
                          depot_client: &Client,
                          token: &str,
                          ident: &PackageIdent,
                          archives_dir: &PathBuf)
                          -> Result<()> {
        let candidate_path = archives_dir.join(ident.archive_name().unwrap());
        if candidate_path.is_file() {
            let mut archive = PackageArchive::new(candidate_path);
            upload_into_depot(ui, &depot_client, token, &ident, &mut archive)
        } else {
            try!(ui.status(Status::Missing,
                           format!("artifact for {} was not found in {}",
                                   ident.archive_name().unwrap(),
                                   archives_dir.display())));
            Err(Error::FileNotFound(archives_dir.to_string_lossy().into_owned()))
        }
    }
}

pub mod verify {
    use std::path::Path;

    use common::ui::{Status, UI};
    use hcore::crypto::artifact;

    use error::Result;

    pub fn start(ui: &mut UI, src: &Path, cache: &Path) -> Result<()> {
        try!(ui.begin(format!("Verifying artifact {}", &src.display())));
        let (name_with_rev, hash) = try!(artifact::verify(src, cache));
        try!(ui.status(Status::Verified,
                       format!("checksum {} signed with {}", &hash, &name_with_rev)));
        try!(ui.end(format!("Verified artifact {}.", &src.display())));
        Ok(())
    }
}

pub mod header {
    use std::path::Path;

    use common::ui::UI;
    use hcore::crypto::artifact;
    use std::io::{self, Write};

    use error::Result;

    pub fn start(ui: &mut UI, src: &Path) -> Result<()> {
        try!(ui.begin(format!("Reading package header for {}", &src.display())));
        try!(ui.para(""));
        if let Ok(header) = artifact::get_artifact_header(src) {
            try!(io::stdout().write(format!("Package        : {}\n", &src.display()).as_bytes()));
            try!(io::stdout()
                .write(format!("Format Version : {}\n", header.format_version).as_bytes()));
            try!(io::stdout().write(format!("Key Name       : {}\n", header.key_name).as_bytes()));
            try!(io::stdout().write(format!("Hash Type      : {}\n", header.hash_type).as_bytes()));
            try!(io::stdout()
                .write(format!("Raw Signature  : {}\n", header.signature_raw).as_bytes()));
        } else {
            try!(ui.warn("Failed to read package header."));
        }
        Ok(())
    }
}
