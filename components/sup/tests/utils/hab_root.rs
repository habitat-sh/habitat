//! Encapsulate the filesystem root that the Supervisor will see in
//! our tests (corresponds to the `FS_ROOT` environment variable). At
//! creation, it will generate a new, randomly-named temp directory on
//! the (real) filesystem, which is deleted when the `HabRoot`
//! instance is dropped.
//!
//! Provides many functions for accessing important paths and files
//! within the directory, which can be used for setting up testing
//! packages as well as for validating the state of the system
//! (e.g. verifying that templated files are changed when new
//! configuration values are applied).

use std::{fs::File,
          io::Read,
          path::{Path,
                 PathBuf},
          string::ToString,
          time::SystemTime};

use crate::hcore::{fs::PKG_PATH,
                   package::{metadata::MetaFile,
                             PackageIdent}};
use tempfile::{Builder,
               TempDir};

#[derive(Debug)]
pub struct HabRoot(TempDir);

impl HabRoot {
    pub fn new(name: &str) -> HabRoot {
        let s = name.to_string();
        let t =
            Builder::new().prefix(&s)
                          .tempdir()
                          .unwrap_or_else(|_| panic!("Could not create temporary directory {}", s));
        HabRoot(t)
    }

    /// Directory to which "expanded package" files should be placed.
    ///
    /// We assign a hard-coded version and release, because
    /// they aren't important for the things we're currently testing
    pub fn pkg_path<S, T>(&self, origin: S, pkg_name: T) -> PathBuf
        where S: AsRef<Path>,
              T: AsRef<Path>
    {
        let ident = self.pkg_ident(origin, pkg_name);

        self.0
            .path()
            .join(PKG_PATH)
            .join(ident.origin)
            .join(ident.name)
            .join(ident.version.as_ref().unwrap())
            .join(ident.release.as_ref().unwrap())
    }

    /// Directory to which "expanded package" files should be placed.
    ///
    /// We assign a hard-coded version and release, because
    /// they aren't important for the things we're currently testing
    pub fn pkg_ident<S, T>(&self, origin: S, pkg_name: T) -> PackageIdent
        where S: AsRef<Path>,
              T: AsRef<Path>
    {
        PackageIdent::new(origin.as_ref().to_str().unwrap(),
                          pkg_name.as_ref().to_str().unwrap(),
                          Some("1.0.0"),
                          Some("20170721000000"))
    }

    /// Returns the path to the service user metafile for a given package.
    pub fn svc_user_path<S, T>(&self, origin: S, pkg_name: T) -> PathBuf
        where S: AsRef<Path>,
              T: AsRef<Path>
    {
        self.pkg_path(origin, pkg_name)
            .join(MetaFile::SvcUser.to_string())
    }

    /// Returns the path to the service group metafile for a given package.
    pub fn svc_group_path<S, T>(&self, origin: S, pkg_name: T) -> PathBuf
        where S: AsRef<Path>,
              T: AsRef<Path>
    {
        self.pkg_path(origin, pkg_name)
            .join(MetaFile::SvcGroup.to_string())
    }

    /// The path to which a spec file should be written for a given
    /// package name.
    pub fn spec_dir<S>(&self, service_group: S) -> PathBuf
        where S: AsRef<Path>
    {
        self.0
            .as_ref()
            .to_path_buf()
            .join("hab")
            .join("sup")
            .join(service_group.as_ref())
            .join("specs")
    }

    /// The path to which a spec file should be written for a given
    /// package name.
    pub fn spec_path<S>(&self, pkg_name: &str, service_group: S) -> PathBuf
        where S: AsRef<Path>
    {
        self.spec_dir(service_group)
            .join(format!("{}.spec", pkg_name.to_string()))
    }

    /// Return the contents of a hook
    pub fn compiled_hook_contents<P, H>(&self, pkg_name: P, hook: H) -> String
        where P: AsRef<Path>,
              H: AsRef<Path>
    {
        Self::file_content(self.hook_path(pkg_name).join(hook.as_ref()))
    }

    /// Return the contents of a config file
    pub fn compiled_config_contents<P, C>(&self, pkg_name: P, config_file_name: C) -> String
        where P: AsRef<Path>,
              C: AsRef<Path>
    {
        Self::file_content(self.config_path(pkg_name).join(config_file_name.as_ref()))
    }

    /// Read the PID file for a package and return the currently
    /// running process' PID.
    ///
    /// Use this to determine if a process was restarted.
    pub fn pid_of<P>(&self, pkg_name: P) -> u32
        where P: AsRef<Path>
    {
        Self::file_content(self.svc_path(pkg_name.as_ref()).join("PID")).parse::<u32>()
                                                                        .expect("Couldn't parse \
                                                                                 PID file content \
                                                                                 as u32!")
    }

    /// Read the INSTALL_HOOK_STATUS file for a package and return the status value
    ///
    /// Use this to determine if an install hook was run and determine its success
    pub fn install_status_of<S, T>(&self, origin: S, pkg_name: T) -> u32
        where S: AsRef<Path>,
              T: AsRef<Path>
    {
        let path = self.pkg_path(origin, pkg_name).join("INSTALL_HOOK_STATUS");
        Self::file_content(path).parse::<u32>()
                                .expect("Couldn't parse status file content as u32!")
    }

    /// Retrieve the last modification time of the INSTALL_HOOK_STATUS file for a package
    ///
    /// Use this to determine if an install hook was run a subsequent time
    pub fn install_status_created<S, T>(&self, origin: S, pkg_name: T) -> SystemTime
        where S: AsRef<Path>,
              T: AsRef<Path>
    {
        let path = self.pkg_path(origin, pkg_name).join("INSTALL_HOOK_STATUS");
        let f = File::open(&path).unwrap_or_else(|_| panic!("Couldn't open file {:?}", path));
        f.metadata().unwrap().modified().unwrap()
    }

    /// Path to the service directory for a package
    fn svc_path<P>(&self, pkg_name: P) -> PathBuf
        where P: AsRef<Path>
    {
        self.0
            .as_ref()
            .to_path_buf()
            .join("hab")
            .join("svc")
            .join(pkg_name.as_ref())
    }

    /// Path to a the hooks directory for a package
    fn hook_path<P>(&self, pkg_name: P) -> PathBuf
        where P: AsRef<Path>
    {
        self.svc_path(pkg_name).join("hooks")
    }

    /// Path to a the config directory for a package
    fn config_path<P>(&self, pkg_name: P) -> PathBuf
        where P: AsRef<Path>
    {
        self.svc_path(pkg_name).join("config")
    }

    fn file_content<P>(path: P) -> String
        where P: AsRef<Path>
    {
        let mut buffer = String::new();
        let p = path.as_ref();
        let mut f = File::open(&p).unwrap_or_else(|_| panic!("Couldn't open file {:?}", p));
        f.read_to_string(&mut buffer)
         .unwrap_or_else(|_| panic!("Couldn't read file {:?}", p));
        buffer
    }
}

impl AsRef<Path> for HabRoot {
    fn as_ref(&self) -> &Path { &self.0.path() }
}
