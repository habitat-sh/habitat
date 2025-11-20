//! Encapsulate the filesystem root that the Supervisor will see in
//! our tests (corresponds to the `FS_ROOT` environment variable). At
//! creation, it will generate a new, randomly-named temp directory on
//! the (real) filesystem, which is deleted when the `HabRoot`
//! instance is dropped.

use crate::hcore::{fs::PKG_PATH,
                   package::{PackageIdent,
                             metadata::MetaFile}};

use std::{path::{Path,
                 PathBuf},
          string::ToString};
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
    pub fn pkg_dir_path(&self, origin: &str, pkg_name: &str) -> PathBuf {
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
    pub fn pkg_ident(&self, origin: &str, pkg_name: &str) -> PackageIdent {
        PackageIdent::new(origin, pkg_name, Some("1.0.0"), Some("20170721000000"))
    }

    /// Returns the path to the package type metafile for a given package.
    // This is only used on non-x86 platforms
    #[allow(dead_code)]
    pub fn pkg_type_path(&self, origin: &str, pkg_name: &str) -> PathBuf {
        self.pkg_dir_path(origin, pkg_name)
            .join(MetaFile::PackageType.to_string())
    }

    /// Returns the path to the target metafile for a given package.
    // This is only used on non-x86 platforms
    #[allow(dead_code)]
    pub fn target_path(&self, origin: &str, pkg_name: &str) -> PathBuf {
        self.pkg_dir_path(origin, pkg_name)
            .join(MetaFile::Target.to_string())
    }

    /// Returns the path to the service user metafile for a given package.
    pub fn svc_user_path(&self, origin: &str, pkg_name: &str) -> PathBuf {
        self.pkg_dir_path(origin, pkg_name)
            .join(MetaFile::SvcUser.to_string())
    }

    /// Returns the path to the service group metafile for a given package.
    pub fn svc_group_path(&self, origin: &str, pkg_name: &str) -> PathBuf {
        self.pkg_dir_path(origin, pkg_name)
            .join(MetaFile::SvcGroup.to_string())
    }

    /// The path to which a spec file should be written for a given
    /// package name.
    pub fn spec_dir_path(&self, service_group: &str) -> PathBuf {
        self.0
            .as_ref()
            .to_path_buf()
            .join("hab")
            .join("sup")
            .join(service_group)
            .join("specs")
    }

    /// The path to which a spec file should be written for a given
    /// package name.
    pub fn spec_path(&self, pkg_name: &str, service_group: &str) -> PathBuf {
        self.spec_dir_path(service_group)
            .join(format!("{}.spec", pkg_name))
    }

    /// Path to the service directory for a package
    pub fn svc_dir_path<P>(&self, pkg_name: P) -> PathBuf
        where P: AsRef<Path>
    {
        self.0
            .as_ref()
            .to_path_buf()
            .join("hab")
            .join("svc")
            .join(pkg_name.as_ref())
    }
}

impl AsRef<Path> for HabRoot {
    fn as_ref(&self) -> &Path { self.0.path() }
}
