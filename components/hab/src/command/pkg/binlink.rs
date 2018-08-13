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

use std::fs;
use std::path::Path;

use common::ui::{Status, UIWriter, UI};
use hcore::fs as hfs;
use hcore::os::filesystem;
use hcore::package::{PackageIdent, PackageInstall};

use error::{Error, Result};

pub fn start(
    ui: &mut UI,
    ident: &PackageIdent,
    binary: &str,
    dest_path: &Path,
    fs_root_path: &Path,
    force: bool,
) -> Result<()> {
    let dst_path = fs_root_path.join(dest_path.strip_prefix("/")?);
    let dst = dst_path.join(&binary);
    ui.begin(format!(
        "Binlinking {} from {} into {}",
        &binary,
        &ident,
        dst_path.display()
    ))?;
    let pkg_install = PackageInstall::load(&ident, Some(fs_root_path))?;
    let src = match hfs::find_command_in_pkg(binary, &pkg_install, fs_root_path)? {
        Some(c) => c,
        None => {
            return Err(Error::CommandNotFoundInPkg((
                pkg_install.ident().to_string(),
                binary.to_string(),
            )))
        }
    };
    if !dst_path.is_dir() {
        ui.status(
            Status::Creating,
            format!("parent directory {}", dst_path.display()),
        )?;
        fs::create_dir_all(&dst_path)?
    }
    let ui_binlinked = format!(
        "Binlinked {} from {} to {}",
        &binary,
        &pkg_install.ident(),
        &dst.display(),
    );
    match fs::read_link(&dst) {
        Ok(path) => {
            if force && path != src {
                fs::remove_file(&dst)?;
                filesystem::symlink(&src, &dst)?;
                ui.end(&ui_binlinked)?;
            } else if path != src {
                ui.warn(format!(
                    "Skipping binlink because {} already exists at {}. Use --force to overwrite",
                    &binary,
                    &dst.display(),
                ))?;
            } else {
                ui.end(&ui_binlinked)?;
            }
        }
        Err(_) => {
            filesystem::symlink(&src, &dst)?;
            ui.end(&ui_binlinked)?;
        }
    }
    Ok(())
}

pub fn binlink_all_in_pkg<F, D>(
    ui: &mut UI,
    pkg_ident: &PackageIdent,
    dest_path: D,
    fs_root_path: F,
    force: bool,
) -> Result<()>
where
    D: AsRef<Path>,
    F: AsRef<Path>,
{
    let fs_root_path = fs_root_path.as_ref();

    let pkg_path = PackageInstall::load(&pkg_ident, Some(fs_root_path))?;
    for bin_path in pkg_path.paths()? {
        for bin in fs::read_dir(fs_root_path.join(bin_path.strip_prefix("/")?))? {
            let bin_file = bin?;
            // Skip any directory entries as we're looking for files and symlinks
            if bin_file.file_type()?.is_dir() {
                continue;
            }
            let bin_name = match bin_file.file_name().to_str() {
                Some(bn) => bn.to_owned(),
                None => {
                    ui.warn("Invalid binary name found. Skipping binlink")?;
                    continue;
                }
            };
            self::start(
                ui,
                &pkg_ident,
                &bin_name,
                dest_path.as_ref(),
                &fs_root_path,
                force,
            )?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::fs::{self, File};
    use std::io::{self, Cursor, Write};
    use std::path::Path;
    use std::str::{self, FromStr};
    use std::sync::{Arc, RwLock};

    use common::ui::{Coloring, UI};
    use hcore;
    use hcore::package::{PackageIdent, PackageTarget};
    use tempdir::TempDir;

    use super::{binlink_all_in_pkg, start};

    #[cfg(target_os = "linux")]
    #[test]
    fn start_symlinks_binaries() {
        let rootfs = TempDir::new("rootfs").unwrap();
        let mut tools = HashMap::new();
        tools.insert("bin", vec!["magicate", "hypnoanalyze"]);
        let ident = fake_bin_pkg_install("acme/cooltools", tools, rootfs.path());
        let dst_path = Path::new("/opt/bin");

        let rootfs_src_dir = hcore::fs::pkg_install_path(&ident, None::<&Path>).join("bin");
        let rootfs_bin_dir = rootfs.path().join("opt/bin");
        let force = true;

        let (mut ui, _stdout, _stderr) = ui();
        start(&mut ui, &ident, "magicate", &dst_path, rootfs.path(), force).unwrap();
        assert_eq!(
            rootfs_src_dir.join("magicate"),
            rootfs_bin_dir.join("magicate").read_link().unwrap()
        );

        start(
            &mut ui,
            &ident,
            "hypnoanalyze",
            &dst_path,
            rootfs.path(),
            force,
        ).unwrap();
        assert_eq!(
            rootfs_src_dir.join("hypnoanalyze"),
            rootfs_bin_dir.join("hypnoanalyze").read_link().unwrap()
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn binlink_all_in_pkg_symlinks_all_binaries() {
        let rootfs = TempDir::new("rootfs").unwrap();
        let mut tools = HashMap::new();
        tools.insert("bin", vec!["magicate", "hypnoanalyze"]);
        tools.insert("sbin", vec!["securitize", "conceal"]);
        let ident = fake_bin_pkg_install("acme/securetools", tools, rootfs.path());
        let dst_path = Path::new("/opt/bin");

        let rootfs_src_dir = hcore::fs::pkg_install_path(&ident, None::<&Path>);
        let rootfs_bin_dir = rootfs.path().join("opt/bin");
        let force = true;

        let (mut ui, _stdout, _stderr) = ui();
        binlink_all_in_pkg(&mut ui, &ident, &dst_path, rootfs.path(), force).unwrap();

        assert_eq!(
            rootfs_src_dir.join("bin/magicate"),
            rootfs_bin_dir.join("magicate").read_link().unwrap()
        );
        assert_eq!(
            rootfs_src_dir.join("bin/hypnoanalyze"),
            rootfs_bin_dir.join("hypnoanalyze").read_link().unwrap()
        );
        assert_eq!(
            rootfs_src_dir.join("sbin/securitize"),
            rootfs_bin_dir.join("securitize").read_link().unwrap()
        );
    }
    #[cfg(target_os = "linux")]
    #[test]
    fn binlink_all_in_pkg_skips_invalid_sub_dirs() {
        let rootfs = TempDir::new("rootfs").unwrap();
        let mut tools = HashMap::new();
        tools.insert("bin", vec!["magicate"]);
        tools.insert("bin/moar", vec!["bonus-round"]);
        let ident = fake_bin_pkg_install("acme/securetools", tools, rootfs.path());
        let dst_path = Path::new("/opt/bin");

        let rootfs_src_dir = hcore::fs::pkg_install_path(&ident, None::<&Path>);
        let rootfs_bin_dir = rootfs.path().join("opt/bin");
        let force = true;

        // Create an empty subdirectory that is not strictly a directory containing package
        // binaries
        fs::create_dir_all(
            hcore::fs::pkg_install_path(&ident, Some(rootfs.path())).join("bin/__junk__"),
        ).unwrap();

        let (mut ui, _stdout, _stderr) = ui();
        binlink_all_in_pkg(&mut ui, &ident, &dst_path, rootfs.path(), force).unwrap();

        assert_eq!(
            rootfs_src_dir.join("bin/magicate"),
            rootfs_bin_dir.join("magicate").read_link().unwrap()
        );
        assert_eq!(
            rootfs_src_dir.join("bin/moar/bonus-round"),
            rootfs_bin_dir.join("bonus-round").read_link().unwrap()
        );
    }

    fn ui() -> (UI, OutputBuffer, OutputBuffer) {
        let stdout_buf = OutputBuffer::new();
        let stderr_buf = OutputBuffer::new();

        let ui = UI::with_streams(
            Box::new(io::empty()),
            || Box::new(stdout_buf.clone()),
            || Box::new(stderr_buf.clone()),
            Coloring::Never,
            false,
        );

        (ui, stdout_buf, stderr_buf)
    }

    fn fake_bin_pkg_install<P>(
        ident: &str,
        binaries: HashMap<&str, Vec<&str>>,
        rootfs: P,
    ) -> PackageIdent
    where
        P: AsRef<Path>,
    {
        let mut ident = PackageIdent::from_str(ident).unwrap();
        if let None = ident.version {
            ident.version = Some("1.2.3".into());
        }
        if let None = ident.release {
            ident.release = Some("21120102121200".into());
        }
        let prefix = hcore::fs::pkg_install_path(&ident, Some(rootfs));
        write_file(prefix.join("IDENT"), &ident.to_string());
        write_file(prefix.join("TARGET"), PackageTarget::active_target());
        let mut paths = Vec::new();
        for (path, bins) in binaries {
            let abspath = hcore::fs::pkg_install_path(&ident, None::<&Path>).join(path);
            paths.push(abspath.to_string_lossy().into_owned());
            for bin in bins {
                write_file(prefix.join(path).join(bin), "");
            }
        }
        write_file(prefix.join("PATH"), &paths.join(":"));
        ident
    }

    fn write_file<P: AsRef<Path>>(file: P, content: &str) {
        fs::create_dir_all(
            file.as_ref()
                .parent()
                .expect("Parent directory doesn't exist"),
        ).expect("Failed to create parent directory");
        let mut f = File::create(file).expect("File is not created");
        f.write_all(content.as_bytes())
            .expect("Bytes not written to file");
    }

    #[derive(Clone)]
    pub struct OutputBuffer {
        pub cursor: Arc<RwLock<Cursor<Vec<u8>>>>,
    }

    impl OutputBuffer {
        fn new() -> Self {
            OutputBuffer {
                cursor: Arc::new(RwLock::new(Cursor::new(Vec::new()))),
            }
        }
    }

    impl Write for OutputBuffer {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.cursor
                .write()
                .expect("Cursor lock is poisoned")
                .write(buf)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.cursor
                .write()
                .expect("Cursor lock is poisoned")
                .flush()
        }
    }
}
