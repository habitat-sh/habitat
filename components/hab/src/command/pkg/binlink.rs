use std::{collections::HashMap,
          env,
          fs::{self,
               File},
          io::{BufRead,
               BufReader},
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
const COMMENT_MARKER: &str = "REM";
#[cfg(unix)]
const COMMENT_MARKER: &str = "#";
#[cfg(windows)]
const PATH_SEPARATOR: char = ';';
#[cfg(unix)]
const PATH_SEPARATOR: char = ':';
#[cfg(windows)]
const SET_OR_EXPORT: &str = "SET";
#[cfg(unix)]
const SET_OR_EXPORT: &str = "export";

const DEFAULT_INTERPRETER: &str = "/bin/sh";

pub fn start(ui: &mut UI,
             ident: &PackageIdent,
             binary: &str,
             dest_path: &Path,
             fs_root_path: &Path,
             force: bool)
             -> Result<()> {
    let dst_path = fs_root_path.join(dest_path.strip_prefix("/")?);
    ui.begin(format!("Binlinking {} from {} into {}",
                     binary,
                     ident,
                     dst_path.display()))?;
    let pkg_install = PackageInstall::load(ident, Some(fs_root_path))?;
    let mut src = match hfs::find_command_in_pkg(binary, &pkg_install, fs_root_path)? {
        Some(c) => c,
        None => {
            return Err(Error::CommandNotFoundInPkg((pkg_install.ident()
                                                               .to_string(),
                                                    binary.to_string())));
        }
    };
    if cfg!(target_os = "windows") {
        src = fs_root_path.join(src.strip_prefix("/")?);
    }
    if !dst_path.is_dir() {
        ui.status(Status::Creating,
                  format!("parent directory {}", dst_path.display()))?;
        fs::create_dir_all(&dst_path)?
    }

    let binlink = Binlink::new(&src, &dst_path, &PathBuf::from(DEFAULT_INTERPRETER))?;
    let ui_binlinked = format!("Binlinked {} from {} to {}",
                               binary,
                               pkg_install.ident(),
                               binlink.link.display(),);
    match Binlink::from_file(&binlink.link) {
        Ok(link) => {
            if force {
                fs::remove_file(link.link)?;
                binlink.link(pkg_install.environment_for_command()?)?;
                ui.end(ui_binlinked)?;
            } else {
                ui.warn(format!("Skipping binlink because {} already exists at {}. Use --force \
                                 to overwrite",
                                binary,
                                link.link.display(),))?;
            }
        }
        Err(_) => {
            binlink.link(pkg_install.environment_for_command()?)?;
            ui.end(ui_binlinked)?;
        }
    }

    if cfg!(target_os = "windows") && !is_dest_on_path(&dst_path) {
        ui.warn(format!("Binlink destination '{}' is not on the PATH. Consider setting it \
                         manually or running 'hab setup' to add it to the machine PATH.",
                        dst_path.display(),))?;
    }

    Ok(())
}

pub fn binlink_all_in_pkg(ui: &mut UI,
                          pkg_ident: &PackageIdent,
                          dest_path: &Path,
                          fs_root_path: &Path,
                          force: bool)
                          -> Result<()> {
    let pkg_path = PackageInstall::load(pkg_ident, Some(fs_root_path))?;
    for bin_path in pkg_path.paths()? {
        for bin in fs::read_dir(fs_root_path.join(bin_path.strip_prefix("/")?))? {
            let bin_file = bin?;
            // Skip any directory entries as we're looking for files and symlinks
            if bin_file.file_type()?.is_dir() {
                continue;
            }
            if cfg!(target_os = "windows") {
                match bin_file.path().extension() {
                    Some(executable_extensions) => {
                        match env::var_os("PATHEXT") {
                            Some(val) => {
                                let any_matches = env::split_paths(&val.to_string_lossy()
                                                                       .to_uppercase()).any(|e| {
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
                        }
                    }
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
            self::start(ui, pkg_ident, &bin_name, dest_path, fs_root_path, force)?;
        }
    }
    Ok(())
}

fn is_dest_on_path(dest_dir: &Path) -> bool {
    if let Some(val) = env::var_os("PATH") {
        env::split_paths(&val).any(|p| p == dest_dir)
    } else {
        false
    }
}

#[allow(dead_code)]
struct Binlink {
    link:                PathBuf,
    target:              PathBuf,
    default_interpreter: PathBuf,
}

impl Binlink {
    pub fn new(target: &Path, link: &Path, default_interpreter: &Path) -> Result<Self> {
        Ok(Self { link:                Self::binstub_path(&target, link)?,
                  target:              target.to_path_buf(),
                  default_interpreter: default_interpreter.to_path_buf(), })
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        // its possible the link could already exist as a symlink either
        // because a previous version of habitat did the binlinking or
        // the symlink created outside of habitat
        fs::read_link(path).map(|target| Self::with_default_interpreter(target, path.into()))
                           .or_else(|_| Self::try_from_script(path))
    }

    fn try_from_script(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        for line in BufReader::new(file).lines() {
            let ln = line?;
            if ln.to_uppercase().starts_with(COMMENT_MARKER) {
                let (_, rest) = ln.split_at(COMMENT_MARKER.len());
                if let Some(target) = Self::get_target_from_toml(rest) {
                    return Ok(Self::with_default_interpreter(target.into(), path.into()));
                }
            }
        }
        Err(Error::CannotParseBinlinkTarget(path.to_path_buf()))
    }

    fn with_default_interpreter(target: PathBuf, link: PathBuf) -> Self {
        Self { link,
               target,
               default_interpreter: PathBuf::from(DEFAULT_INTERPRETER) }
    }

    fn get_target_from_toml(toml: &str) -> Option<String> {
        toml.parse()
            .ok()
            .as_ref()
            .and_then(toml::value::Value::as_table)
            // Prior to 0.84.0, we used 'source' so we fallback
            // to 'source' for links created with older versions
            .and_then(|toml_table| toml_table.get("target").or_else(|| toml_table.get("source")))
            .and_then(toml::value::Value::as_str)
            .map(String::from)
    }

    pub fn link(&self, env: HashMap<String, String>) -> Result<()> {
        #[cfg(windows)]
        {
            fs::write(&self.link, self.stub_template(env)?.as_bytes())?;
            Ok(())
        }

        #[cfg(unix)]
        {
            use std::{io::Write,
                      os::unix::fs::OpenOptionsExt};
            fs::OpenOptions::new().create(true)
                                  .write(true)
                                  .truncate(true)
                                  .mode(0o775)
                                  .open(&self.link)?
                                  .write_all(self.stub_template(env)?.as_bytes())?;
            Ok(())
        }
    }

    fn binstub_path(target: &Path, link: &Path) -> Result<PathBuf> {
        #[cfg(windows)]
        {
            let bin_name = match target.file_stem() {
                Some(name) => name,
                None => return Err(Error::CannotParseBinlinkTarget(target.to_path_buf())),
            };
            let mut path = link.join(bin_name);
            path.set_extension("bat");
            Ok(path)
        }

        #[cfg(unix)]
        match target.file_name() {
            Some(name) => Ok(link.join(name)),
            None => Err(Error::CannotParseBinlinkTarget(target.to_path_buf())),
        }
    }

    fn stub_template(&self, env: HashMap<String, String>) -> Result<String> {
        let mut exports = String::new();
        for (key, mut value) in env.into_iter() {
            if key == "PATH" {
                value.push(PATH_SEPARATOR);
                value.push_str(&Self::interpolated_var("PATH"));
            }
            exports.push_str(&format!("{} {}={}\n", SET_OR_EXPORT, key, value));
        }

        #[cfg(windows)]
        {
            Ok(format!(include_str!("../../../static/template_binstub.\
                                     bat"),
                       target = self.target.display(),
                       env = exports))
        }

        #[cfg(unix)]
        {
            // We need to prevent the binstub from using itself as an interpreter.
            // On linux we use `/bin/sh` as the binstub interpreter and if this package
            // includes a `sh` binary, we risk a circularly linked interpreter. We can
            // detect if the link path is the same as the interpreter and if so, use the
            // target binary as the interpreter.
            let interpreter = {
                if self.default_interpreter.canonicalize()? == self.link {
                    &self.target
                } else {
                    &self.default_interpreter
                }
            };

            Ok(format!(include_str!("../../../static/template_binstub.sh"),
                       target = self.target.display(),
                       env = exports,
                       interpreter = interpreter.display()))
        }
    }

    fn interpolated_var(name: &str) -> String {
        #[cfg(windows)]
        {
            format!("%{}%", name)
        }
        #[cfg(unix)]
        {
            format!(r#""${}""#, name)
        }
    }
}

#[cfg(test)]
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
            rootfs_src_dir = rootfs.path()
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
        let curr_path = format!("{}{}",
                                super::PATH_SEPARATOR,
                                Binlink::interpolated_var("PATH"));

        start(&mut ui,
              &ident,
              "magicate.exe",
              &dst_path,
              rootfs.path(),
              force).unwrap();
        assert!(fs::read_to_string(rootfs_bin_dir.join(magicate_link)).unwrap().contains(&format!("PATH={}{}", rootfs_src_dir.to_string_lossy(), curr_path)));
        assert_eq!(rootfs_src_dir.join("magicate.exe"),
                   Binlink::from_file(&rootfs_bin_dir.join(magicate_link)).unwrap()
                                                                          .target);

        start(&mut ui,
              &ident,
              "hypnoanalyze.exe",
              &dst_path,
              rootfs.path(),
              force).unwrap();
        assert!(fs::read_to_string(rootfs_bin_dir.join(hypnoanalyze_link)).unwrap().contains(&format!("PATH={}{}", rootfs_src_dir.to_string_lossy(), curr_path)));
        assert_eq!(rootfs_src_dir.join("hypnoanalyze.exe"),
                   Binlink::from_file(&rootfs_bin_dir.join(hypnoanalyze_link)).unwrap()
                                                                              .target);
    }

    #[test]
    #[cfg(unix)]
    fn link_points_to_interpreter() {
        use std::path::PathBuf;

        let rootfs = TempDir::new().unwrap();
        let rootfs_bin_dir = rootfs.path().join("bin");
        let rootfs_bin_sh = rootfs_bin_dir.join("sh");
        fs::create_dir_all(&rootfs_bin_dir).unwrap();

        let link = Binlink::new(&PathBuf::from("/src/binary/sh"),
                                &rootfs_bin_dir,
                                &rootfs_bin_sh).unwrap();
        link.link(HashMap::new()).unwrap();

        assert!(fs::read_to_string(rootfs_bin_sh).unwrap()
                                                 .contains("#!/src/binary/sh"));
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
            rootfs_src_dir = rootfs.path()
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

        assert_eq!(rootfs_src_dir.join("bin/magicate.exe"),
                   Binlink::from_file(&rootfs_bin_dir.join(magicate_link)).unwrap()
                                                                          .target);
        assert_eq!(rootfs_src_dir.join("bin/hypnoanalyze.exe"),
                   Binlink::from_file(&rootfs_bin_dir.join(hypnoanalyze_link)).unwrap()
                                                                              .target);
        assert_eq!(rootfs_src_dir.join("sbin/securitize.exe"),
                   Binlink::from_file(&rootfs_bin_dir.join(securitize_link)).unwrap()
                                                                            .target);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn binlink_all_in_pkg_symlinks_only_executables() {
        let rootfs = TempDir::new().unwrap();
        let mut tools = HashMap::new();
        tools.insert("bin", vec!["magicate.exe", "hypnoanalyze.dll"]);
        let ident = fake_bin_pkg_install("acme/securetools", tools, rootfs.path());
        let dst_path = Path::new("/opt/bin");

        let rootfs_src_dir =
            rootfs.path()
                  .join(hcore::fs::pkg_install_path(&ident, None::<&Path>).strip_prefix("/")
                                                                          .unwrap());
        let rootfs_bin_dir = rootfs.path().join("opt/bin");
        let force = true;

        let (mut ui, _stdout, _stderr) = ui();
        binlink_all_in_pkg(&mut ui, &ident, &dst_path, rootfs.path(), force).unwrap();

        assert_eq!(rootfs_src_dir.join("bin/magicate.exe"),
                   Binlink::from_file(&rootfs_bin_dir.join("magicate.bat")).unwrap()
                                                                           .target);
        assert!(Binlink::from_file(&rootfs_bin_dir.join("hypnoanalyze.bat")).is_err());
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
            rootfs_src_dir = rootfs.path()
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

        assert_eq!(rootfs_src_dir.join("bin/magicate.exe"),
                   Binlink::from_file(&rootfs_bin_dir.join(magicate_link)).unwrap()
                                                                          .target);
        assert_eq!(rootfs_src_dir.join("bin/moar/bonus-round.exe"),
                   Binlink::from_file(&rootfs_bin_dir.join(bonus_round_link)).unwrap()
                                                                             .target);
    }

    fn ui() -> (UI, OutputBuffer, OutputBuffer) {
        let stdout_buf = OutputBuffer::new();
        let stderr_buf = OutputBuffer::new();

        let ui = UI::with_streams(Box::new(io::empty()),
                                  || Box::new(stdout_buf.clone()),
                                  || Box::new(stderr_buf.clone()),
                                  ColorChoice::Never,
                                  false);

        (ui, stdout_buf, stderr_buf)
    }

    fn fake_bin_pkg_install<P>(ident: &str,
                               binaries: HashMap<&str, Vec<&str>>,
                               rootfs: P)
                               -> PackageIdent
        where P: AsRef<Path>
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
        write_file(prefix.join("TARGET"), &PackageTarget::active_target());
        let mut paths = Vec::new();
        for (path, bins) in binaries {
            let abspath = hcore::fs::pkg_install_path(&ident, None::<&Path>).join(path);
            paths.push(abspath.to_string_lossy().into_owned());
            for bin in bins {
                write_file(prefix.join(path).join(bin), "");
            }
        }
        write_file(prefix.join("PATH"),
                   env::join_paths(paths).unwrap().to_str().unwrap());
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
            OutputBuffer { cursor: Arc::new(RwLock::new(Cursor::new(Vec::new()))), }
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
