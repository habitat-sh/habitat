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

#[cfg(windows)]
use std::fs::File;
#[cfg(windows)]
use std::io::{BufRead,
              BufReader};
use std::{env,
          fs,
          path::{Path,
                 PathBuf}};

use crate::{common::ui::{Status,
                         UIWriter,
                         UI},
            hcore::{fs as hfs,
                    package::{PackageIdent,
                              PackageInstall}}};

use crate::error::{Error,
                   Result};

#[cfg(windows)]
const BAT_COMMENT_MARKER: &str = "REM";

pub fn start(
    ui: &mut UI,
    ident: &PackageIdent,
    binary: &str,
    dest_path: &Path,
    fs_root_path: &Path,
    force: bool,
) -> Result<()> {
    let dst_path = fs_root_path.join(dest_path.strip_prefix("/")?);
    ui.begin(format!(
        "Binlinking {} from {} into {}",
        &binary,
        &ident,
        dst_path.display()
    ))?;
    let pkg_install = PackageInstall::load(&ident, Some(fs_root_path))?;
    let mut src = match hfs::find_command_in_pkg(binary, &pkg_install, fs_root_path)? {
        Some(c) => c,
        None => {
            return Err(Error::CommandNotFoundInPkg((
                pkg_install.ident().to_string(),
                binary.to_string(),
            )));
        }
    };
    if cfg!(target_os = "windows") {
        src = fs_root_path.join(src.strip_prefix("/")?);
    }
    if !dst_path.is_dir() {
        ui.status(
            Status::Creating,
            format!("parent directory {}", dst_path.display()),
        )?;
        fs::create_dir_all(&dst_path)?
    }
    let binlink = Binlink::new(&src, &dst_path)?;
    let ui_binlinked = format!(
        "Binlinked {} from {} to {}",
        &binary,
        &pkg_install.ident(),
        &binlink.dest.display(),
    );
    match Binlink::from_file(&binlink.dest) {
        Ok(link) => {
            if force && link.src != src {
                fs::remove_file(&link.dest)?;
                binlink.link()?;
                ui.end(&ui_binlinked)?;
            } else if link.src != src {
                ui.warn(format!(
                    "Skipping binlink because {} already exists at {}. Use --force to overwrite",
                    &binary,
                    &link.dest.display(),
                ))?;
            } else {
                ui.end(&ui_binlinked)?;
            }
        }
        Err(_) => {
            binlink.link()?;
            ui.end(&ui_binlinked)?;
        }
    }

    if cfg!(target_os = "windows") && !is_dest_on_path(&dst_path) {
        ui.warn(format!(
            "Binlink destination '{}' is not on the PATH. Consider setting it manually or running \
             'hab setup' to add it to the machine PATH.",
            dst_path.display(),
        ))?;
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
            if cfg!(target_os = "windows") {
                match bin_file.path().extension() {
                    Some(executable_extensions) => match env::var_os("PATHEXT") {
                        Some(val) => {
                            let any_matches =
                                env::split_paths(&val.to_string_lossy().to_uppercase()).any(|e| {
                                    e.to_string_lossy()
                                        == format!(
                                            ".{}",
                                            executable_extensions.to_string_lossy().to_uppercase()
                                        )
                                });
                            if !any_matches {
                                continue;
                            }
                        }
                        None => continue,
                    },
                    None => continue,
                }
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

fn is_dest_on_path<T: AsRef<Path>>(dest_dir: T) -> bool {
    if let Some(val) = env::var_os("PATH") {
        env::split_paths(&val).any(|p| p == dest_dir.as_ref())
    } else {
        false
    }
}

struct Binlink {
    dest: PathBuf,
    src: PathBuf,
}

#[cfg(not(target_os = "windows"))]
impl Binlink {
    pub fn new<T: AsRef<Path>>(src: T, dest_dir: T) -> Result<Self> {
        let bin_name = match src.as_ref().file_name() {
            Some(name) => name,
            None => return Err(Error::CannotParseBinlinkSource(src.as_ref().to_path_buf())),
        };

        Ok(Self {
            dest: dest_dir.as_ref().join(bin_name),
            src: src.as_ref().to_path_buf(),
        })
    }

    pub fn from_file<T: AsRef<Path>>(dest: T) -> Result<Self> {
        Ok(Binlink {
            dest: dest.as_ref().to_path_buf(),
            src: fs::read_link(&dest)?,
        })
    }

    pub fn link(&self) -> Result<()> {
        use crate::hcore::os::filesystem;

        filesystem::symlink(&self.src, &self.dest)?;
        Ok(())
    }
}

#[cfg(target_os = "windows")]
impl Binlink {
    pub fn new<T: AsRef<Path>>(src: T, dest_dir: T) -> Result<Self> {
        let bin_name = match src.as_ref().file_stem() {
            Some(name) => name,
            None => return Err(Error::CannotParseBinlinkSource(src.as_ref().to_path_buf())),
        };
        let mut path = dest_dir.as_ref().join(bin_name);
        path.set_extension("bat");

        Ok(Binlink {
            dest: path,
            src: src.as_ref().to_path_buf(),
        })
    }

    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Self> {
        use toml::Value::Table;

        let file = File::open(&path)?;
        for line in BufReader::new(file).lines() {
            let ln = line?;
            if ln.to_uppercase().starts_with(BAT_COMMENT_MARKER) {
                let (_, rest) = ln.split_at(BAT_COMMENT_MARKER.len());
                if let Ok(Table(toml_exp)) = rest.parse() {
                    if let Some(src) = toml_exp.get("source") {
                        if let Some(val) = src.as_str() {
                            return Ok(Binlink {
                                dest: path.as_ref().to_path_buf(),
                                src: PathBuf::from(val),
                            });
                        }
                    }
                }
            }
        }
        Err(Error::CannotParseBinlinkSource(path.as_ref().to_path_buf()))
    }

    pub fn link(&self) -> Result<()> {
        let template = format!(
            "@echo off\nREM source='{0}'\n\"{0}\" %*",
            self.src.display()
        );
        Ok(fs::write(&self.dest, template)?)
    }
}

#[cfg(test)]
#[cfg(any(target_os = "linux", target_os = "windows"))]
mod test {
    use std::{collections::HashMap,
              env,
              fs::{self,
                   File},
              io::{self,
                   Cursor,
                   Write},
              path::Path,
              str::{self,
                    FromStr},
              sync::{Arc,
                     RwLock}};
    use termcolor::ColorChoice;

    use crate::{common::ui::UI,
                hcore::{self,
                        package::{PackageIdent,
                                  PackageTarget}}};
    use tempfile::TempDir;

    use super::{binlink_all_in_pkg,
                start,
                Binlink};

    #[test]
    fn start_symlinks_binaries() {
        let rootfs = TempDir::new().unwrap();
        let mut tools = HashMap::new();
        tools.insert("bin", vec!["magicate.exe", "hypnoanalyze.exe"]);
        let ident = fake_bin_pkg_install("acme/cooltools", tools, rootfs.path());
        let dst_path = Path::new("/opt/bin");

        let mut rootfs_src_dir = hcore::fs::pkg_install_path(&ident, None::<&Path>).join("bin");
        if cfg!(target_os = "windows") {
            rootfs_src_dir = rootfs
                .path()
                .join(rootfs_src_dir.strip_prefix("/").unwrap());
        }
        let rootfs_bin_dir = rootfs.path().join("opt/bin");
        let force = true;

        let (mut ui, _stdout, _stderr) = ui();

        #[cfg(target_os = "linux")]
        let magicate_link = "magicate.exe";
        #[cfg(target_os = "windows")]
        let magicate_link = "magicate.bat";
        #[cfg(target_os = "linux")]
        let hypnoanalyze_link = "hypnoanalyze.exe";
        #[cfg(target_os = "windows")]
        let hypnoanalyze_link = "hypnoanalyze.bat";

        start(
            &mut ui,
            &ident,
            "magicate.exe",
            &dst_path,
            rootfs.path(),
            force,
        )
        .unwrap();
        assert_eq!(
            rootfs_src_dir.join("magicate.exe"),
            Binlink::from_file(rootfs_bin_dir.join(magicate_link))
                .unwrap()
                .src
        );

        start(
            &mut ui,
            &ident,
            "hypnoanalyze.exe",
            &dst_path,
            rootfs.path(),
            force,
        )
        .unwrap();
        assert_eq!(
            rootfs_src_dir.join("hypnoanalyze.exe"),
            Binlink::from_file(rootfs_bin_dir.join(hypnoanalyze_link))
                .unwrap()
                .src
        );
    }

    #[test]
    fn binlink_all_in_pkg_symlinks_all_binaries() {
        let rootfs = TempDir::new().unwrap();
        let mut tools = HashMap::new();
        tools.insert("bin", vec!["magicate.exe", "hypnoanalyze.exe"]);
        tools.insert("sbin", vec!["securitize.exe", "conceal.exe"]);
        let ident = fake_bin_pkg_install("acme/securetools", tools, rootfs.path());
        let dst_path = Path::new("/opt/bin");

        let mut rootfs_src_dir = hcore::fs::pkg_install_path(&ident, None::<&Path>);
        if cfg!(target_os = "windows") {
            rootfs_src_dir = rootfs
                .path()
                .join(rootfs_src_dir.strip_prefix("/").unwrap());
        }
        let rootfs_bin_dir = rootfs.path().join("opt/bin");
        let force = true;

        #[cfg(target_os = "linux")]
        let magicate_link = "magicate.exe";
        #[cfg(target_os = "windows")]
        let magicate_link = "magicate.bat";
        #[cfg(target_os = "linux")]
        let hypnoanalyze_link = "hypnoanalyze.exe";
        #[cfg(target_os = "windows")]
        let hypnoanalyze_link = "hypnoanalyze.bat";
        #[cfg(target_os = "linux")]
        let securitize_link = "securitize.exe";
        #[cfg(target_os = "windows")]
        let securitize_link = "securitize.bat";

        let (mut ui, _stdout, _stderr) = ui();
        binlink_all_in_pkg(&mut ui, &ident, &dst_path, rootfs.path(), force).unwrap();

        assert_eq!(
            rootfs_src_dir.join("bin/magicate.exe"),
            Binlink::from_file(rootfs_bin_dir.join(magicate_link))
                .unwrap()
                .src
        );
        assert_eq!(
            rootfs_src_dir.join("bin/hypnoanalyze.exe"),
            Binlink::from_file(rootfs_bin_dir.join(hypnoanalyze_link))
                .unwrap()
                .src
        );
        assert_eq!(
            rootfs_src_dir.join("sbin/securitize.exe"),
            Binlink::from_file(rootfs_bin_dir.join(securitize_link))
                .unwrap()
                .src
        );
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn binlink_all_in_pkg_symlinks_only_executables() {
        let rootfs = TempDir::new().unwrap();
        let mut tools = HashMap::new();
        tools.insert("bin", vec!["magicate.exe", "hypnoanalyze.dll"]);
        let ident = fake_bin_pkg_install("acme/securetools", tools, rootfs.path());
        let dst_path = Path::new("/opt/bin");

        let rootfs_src_dir = rootfs.path().join(
            hcore::fs::pkg_install_path(&ident, None::<&Path>)
                .strip_prefix("/")
                .unwrap(),
        );
        let rootfs_bin_dir = rootfs.path().join("opt/bin");
        let force = true;

        let (mut ui, _stdout, _stderr) = ui();
        binlink_all_in_pkg(&mut ui, &ident, &dst_path, rootfs.path(), force).unwrap();

        assert_eq!(
            rootfs_src_dir.join("bin/magicate.exe"),
            Binlink::from_file(rootfs_bin_dir.join("magicate.bat"))
                .unwrap()
                .src
        );
        assert!(Binlink::from_file(rootfs_bin_dir.join("hypnoanalyze.bat")).is_err());
    }

    #[test]
    fn binlink_all_in_pkg_skips_invalid_sub_dirs() {
        let rootfs = TempDir::new().unwrap();
        let mut tools = HashMap::new();
        tools.insert("bin", vec!["magicate.exe"]);
        tools.insert("bin/moar", vec!["bonus-round.exe"]);
        let ident = fake_bin_pkg_install("acme/securetools", tools, rootfs.path());
        let dst_path = Path::new("/opt/bin");

        let mut rootfs_src_dir = hcore::fs::pkg_install_path(&ident, None::<&Path>);
        if cfg!(target_os = "windows") {
            rootfs_src_dir = rootfs
                .path()
                .join(rootfs_src_dir.strip_prefix("/").unwrap());
        }
        let rootfs_bin_dir = rootfs.path().join("opt/bin");
        let force = true;

        // Create an empty subdirectory that is not strictly a directory containing package
        // binaries
        fs::create_dir_all(
            hcore::fs::pkg_install_path(&ident, Some(rootfs.path())).join("bin/__junk__"),
        )
        .unwrap();

        #[cfg(target_os = "linux")]
        let magicate_link = "magicate.exe";
        #[cfg(target_os = "windows")]
        let magicate_link = "magicate.bat";
        #[cfg(target_os = "linux")]
        let bonus_round_link = "bonus-round.exe";
        #[cfg(target_os = "windows")]
        let bonus_round_link = "bonus-round.bat";

        let (mut ui, _stdout, _stderr) = ui();
        binlink_all_in_pkg(&mut ui, &ident, &dst_path, rootfs.path(), force).unwrap();

        assert_eq!(
            rootfs_src_dir.join("bin/magicate.exe"),
            Binlink::from_file(rootfs_bin_dir.join(magicate_link))
                .unwrap()
                .src
        );
        assert_eq!(
            rootfs_src_dir.join("bin/moar/bonus-round.exe"),
            Binlink::from_file(rootfs_bin_dir.join(bonus_round_link))
                .unwrap()
                .src
        );
    }

    fn ui() -> (UI, OutputBuffer, OutputBuffer) {
        let stdout_buf = OutputBuffer::new();
        let stderr_buf = OutputBuffer::new();

        let ui = UI::with_streams(
            Box::new(io::empty()),
            || Box::new(stdout_buf.clone()),
            || Box::new(stderr_buf.clone()),
            ColorChoice::Never,
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
        if ident.version.is_none() {
            ident.version = Some("1.2.3".into());
        }
        if ident.release.is_none() {
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
        write_file(
            prefix.join("PATH"),
            env::join_paths(paths).unwrap().to_str().unwrap(),
        );
        ident
    }

    fn write_file<P: AsRef<Path>>(file: P, content: &str) {
        fs::create_dir_all(
            file.as_ref()
                .parent()
                .expect("Parent directory doesn't exist"),
        )
        .expect("Failed to create parent directory");
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
