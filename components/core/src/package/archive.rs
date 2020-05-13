use super::{metadata::{MetaFile,
                       PackageType},
            Identifiable,
            PackageIdent,
            PackageTarget};
use crate::{crypto::{artifact,
                     hash},
            error::{Error,
                    Result}};
use regex::Regex;
use std::{collections::HashMap,
          error,
          io::Read,
          path::{Path,
                 PathBuf},
          result,
          str::{self,
                FromStr},
          string::ToString};
use tar::Archive;
use xz2::read::XzDecoder;

lazy_static::lazy_static! {
    static ref METAFILE_REGXS: HashMap<MetaFile, Regex> = {
        let mut map = HashMap::new();
        map.insert(
            MetaFile::CFlags,
            Regex::new(&format!(
                r"^/?hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$",
                MetaFile::CFlags
            ))
            .unwrap(),
        );
        map.insert(
            MetaFile::Config,
            Regex::new(&format!(
                r"^/?hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$",
                MetaFile::Config
            ))
            .unwrap(),
        );
        map.insert(
            MetaFile::Deps,
            Regex::new(&format!(
                r"^/?hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$",
                MetaFile::Deps
            ))
            .unwrap(),
        );
        map.insert(
            MetaFile::TDeps,
            Regex::new(&format!(
                r"^/?hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$",
                MetaFile::TDeps
            ))
            .unwrap(),
        );
        map.insert(
            MetaFile::BuildDeps,
            Regex::new(&format!(
                r"^/?hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$",
                MetaFile::BuildDeps,
            ))
            .unwrap(),
        );
        map.insert(
            MetaFile::BuildTDeps,
            Regex::new(&format!(
                r"^/?hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$",
                MetaFile::BuildTDeps,
            ))
            .unwrap(),
        );
        map.insert(
            MetaFile::Exposes,
            Regex::new(&format!(
                r"^/?hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$",
                MetaFile::Exposes
            ))
            .unwrap(),
        );
        map.insert(
            MetaFile::Ident,
            Regex::new(&format!(
                r"^/?hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$",
                MetaFile::Ident
            ))
            .unwrap(),
        );
        map.insert(
            MetaFile::LdRunPath,
            Regex::new(&format!(
                r"^/?hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$",
                MetaFile::LdRunPath
            ))
            .unwrap(),
        );
        map.insert(
            MetaFile::LdFlags,
            Regex::new(&format!(
                r"^/?hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$",
                MetaFile::LdFlags
            ))
            .unwrap(),
        );
        map.insert(
            MetaFile::SvcUser,
            Regex::new(&format!(
                r"^/?hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$",
                MetaFile::SvcUser
            ))
            .unwrap(),
        );
        map.insert(
            MetaFile::Services,
            Regex::new(&format!(
                r"^/?hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$",
                MetaFile::Services
            ))
            .unwrap(),
        );
        map.insert(
            MetaFile::ResolvedServices,
            Regex::new(&format!(
                r"^/?hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$",
                MetaFile::ResolvedServices
            ))
            .unwrap(),
        );
        map.insert(
            MetaFile::Manifest,
            Regex::new(&format!(
                r"^/?hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$",
                MetaFile::Manifest
            ))
            .unwrap(),
        );
        map.insert(
            MetaFile::Path,
            Regex::new(&format!(
                r"^/?hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$",
                MetaFile::Path
            ))
            .unwrap(),
        );
        map.insert(
            MetaFile::Target,
            Regex::new(&format!(
                r"^/?hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$",
                MetaFile::Target
            ))
            .unwrap(),
        );
        map.insert(
            MetaFile::Type,
            Regex::new(&format!(
                r"^/?hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$",
                MetaFile::Type
            ))
            .unwrap(),
        );
        map
    };
}

type Metadata = HashMap<MetaFile, String>;

#[derive(Debug)]
pub struct PackageArchive {
    pub path: PathBuf,
    metadata: Metadata,
}

impl PackageArchive {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let metadata = Self::get_all_metadata(&path)?;
        Ok(PackageArchive { path, metadata })
    }

    /// Calculate and return the checksum of the package archive in base64 format.
    ///
    /// # Failures
    ///
    /// * If the archive cannot be read
    pub fn checksum(&self) -> Result<String> { hash::hash_file(&self.path) }

    pub fn cflags(&mut self) -> Option<&str> { self.read_metadata(MetaFile::CFlags) }

    pub fn config(&mut self) -> Option<&str> { self.read_metadata(MetaFile::Config) }

    // hab-plan-build.sh only generates SVC_USER and SVC_GROUP files if it thinks a package is
    // a service. It determines that by checking for the presence of a run hook file or a
    // pkg_svc_run value. Therefore, if we can detect the presence of a SVC_USER file, we can
    // consider this archive a service.
    //
    // The allow below is necessary because `is_*` functions expect a `&self`, not `&mut self`.
    // It would be good to refactor this struct to do the read_metadata in new and then
    // eliminate the `&mut self`s on all the accessor functions, but that's a more involved
    // change than we want to undertake now.
    //
    // See https://rust-lang.github.io/rust-clippy/master/index.html#wrong_self_convention
    #[allow(clippy::wrong_self_convention)]
    pub fn is_a_service(&mut self) -> bool {
        match self.svc_user() {
            Ok(_) => true,
            _ => false,
        }
    }

    /// Returns a list of package identifiers representing the runtime package dependencies for
    /// this archive.
    ///
    /// # Failures
    ///
    /// * If the archive cannot be read
    /// * If the archive cannot be verified
    pub fn deps(&mut self) -> Result<Vec<PackageIdent>> { self.read_deps(MetaFile::Deps) }

    /// Returns a list of package identifiers representing the transitive runtime package
    /// dependencies for this archive.
    ///
    /// # Failures
    ///
    /// * If the archive cannot be read
    /// * If the archive cannot be verified
    pub fn tdeps(&mut self) -> Result<Vec<PackageIdent>> { self.read_deps(MetaFile::TDeps) }

    /// Returns a list of package identifiers representing the buildtime package dependencies for
    /// this archive.
    ///
    /// # Failures
    ///
    /// * If the archive cannot be read
    /// * If the archive cannot be verified
    pub fn build_deps(&mut self) -> Result<Vec<PackageIdent>> {
        self.read_deps(MetaFile::BuildDeps)
    }

    /// Returns a list of package identifiers representing the transitive buildtime package
    /// dependencies for this archive.
    ///
    /// # Failures
    ///
    /// * If the archive cannot be read
    /// * If the archive cannot be verified
    pub fn build_tdeps(&mut self) -> Result<Vec<PackageIdent>> {
        self.read_deps(MetaFile::BuildTDeps)
    }

    pub fn exposes(&mut self) -> Vec<u16> {
        if let Some(data) = self.read_metadata(MetaFile::Exposes) {
            data.split_whitespace()
                .filter_map(|port| port.parse().ok())
                .collect()
        } else {
            vec![]
        }
    }

    pub fn ident(&mut self) -> Result<PackageIdent> {
        if let Some(data) = self.read_metadata(MetaFile::Ident) {
            PackageIdent::from_str(&data)
        } else {
            Err(Error::MetaFileNotFound(MetaFile::Ident))
        }
    }

    pub fn ld_run_path(&mut self) -> Option<&str> { self.read_metadata(MetaFile::LdRunPath) }

    pub fn ldflags(&mut self) -> Option<&str> { self.read_metadata(MetaFile::LdFlags) }

    pub fn svc_user(&mut self) -> Result<&str> {
        if let Some(data) = self.read_metadata(MetaFile::SvcUser) {
            Ok(data)
        } else {
            Err(Error::MetaFileNotFound(MetaFile::SvcUser))
        }
    }

    pub fn manifest(&mut self) -> Result<&str> {
        if let Some(data) = self.read_metadata(MetaFile::Manifest) {
            Ok(data)
        } else {
            Err(Error::MetaFileNotFound(MetaFile::Manifest))
        }
    }

    pub fn package_type(&mut self) -> Result<PackageType> {
        if let Some(data) = self.read_metadata(MetaFile::Type) {
            PackageType::from_str(&data)
        } else {
            Ok(PackageType::Standalone)
        }
    }

    pub fn path(&mut self) -> Option<&str> { self.read_metadata(MetaFile::Path) }

    pub fn pkg_services(&mut self) -> Result<Vec<PackageIdent>> {
        self.read_deps(MetaFile::Services)
    }

    pub fn resolved_services(&mut self) -> Result<Vec<PackageIdent>> {
        self.read_deps(MetaFile::ResolvedServices)
    }

    pub fn target(&mut self) -> Result<PackageTarget> {
        if let Some(data) = self.read_metadata(MetaFile::Target) {
            PackageTarget::from_str(&data)
        } else {
            Err(Error::MetaFileNotFound(MetaFile::Target))
        }
    }

    /// A plain string representation of the archive's file name.
    pub fn file_name(&self) -> String {
        self.path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned()
    }

    /// Given a package name and a path to a file as an `&str`, verify
    /// the files signature.
    ///
    /// # Failures
    ///
    /// * Fails if it cannot verify the signature for any reason
    pub fn verify<P: AsRef<Path>>(&self, cache_key_path: &P) -> Result<(String, String)> {
        artifact::verify(&self.path, cache_key_path)
    }

    /// Given a package name and a path to a file as an `&str`, unpack
    /// the package.
    ///
    /// # Failures
    ///
    /// * If the package cannot be unpacked
    pub fn unpack(&self, fs_root_path: Option<&Path>) -> Result<()> {
        let root = fs_root_path.unwrap_or_else(|| Path::new("/"));
        let hart_payload_tar_xz = artifact::get_archive_reader(&self.path)?;
        let decoder = XzDecoder::new(hart_payload_tar_xz);
        let mut tar = Archive::new(decoder);
        tar.set_preserve_permissions(true);
        tar.set_preserve_mtime(true);
        tar.unpack(root)?;
        Ok(())
    }

    fn read_deps(&mut self, file: MetaFile) -> Result<Vec<PackageIdent>> {
        let mut deps = vec![];

        // For now, all deps files but SERVICES need fully-qualified
        // package identifiers
        let must_be_fully_qualified = { file != MetaFile::Services };

        if let Some(body) = self.read_metadata(file) {
            for id in body.lines() {
                let package = PackageIdent::from_str(id)?;
                if !package.fully_qualified() && must_be_fully_qualified {
                    return Err(Error::FullyQualifiedPackageIdentRequired(package.to_string()));
                }
                deps.push(package);
            }
        }

        Ok(deps)
    }

    fn read_metadata(&mut self, file: MetaFile) -> Option<&str> {
        self.metadata.get(&file).map(String::as_str)
    }

    fn get_all_metadata(path: impl AsRef<Path>) -> Result<Metadata> {
        let mut metadata = Metadata::new();
        let hart_payload_tar_xz = artifact::get_archive_reader(path)?;
        let decoder = XzDecoder::new(hart_payload_tar_xz);
        let mut tar = Archive::new(decoder);

        // Check all entries in the tar archive for metafiles and add them to the `Metadata` store
        for entry in tar.entries()? {
            let mut entry = entry?;
            let maybe_match = METAFILE_REGXS.iter().find(|(_, regex)| {
                                                       let path_bytes = entry.path_bytes();
                                                       let path_str =
                                                           String::from_utf8_lossy(&path_bytes);
                                                       regex.is_match(&path_str)
                                                   });

            let meta_file = if let Some((meta_file, _)) = maybe_match {
                *meta_file
            } else {
                continue;
            };

            let mut contents = String::new();
            entry.read_to_string(&mut contents)
                 .map_err(|_| Error::MetaFileMalformed(meta_file))?;
            // Hey, before you go - we are trimming whitespace for you. This
            // is handy, because later on, you just want the string you want.
            metadata.insert(meta_file, String::from(contents.trim()));

            // Exit early if we found all the metafiles
            if metadata.len() == METAFILE_REGXS.len() {
                break;
            }
        }

        Ok(metadata)
    }
}

pub trait FromArchive: Sized {
    type Error: error::Error;

    fn from_archive(archive: &mut PackageArchive) -> result::Result<Self, Self::Error>;
}

#[cfg(test)]
mod test {
    use super::{super::target,
                *};
    use std::path::PathBuf;

    #[test]
    fn reading_artifact_metadata() {
        let mut hart =
            PackageArchive::new(fixtures().join("happyhumans-possums-8.1.\
                                                 4-20160427165340-x86_64-linux.hart")).unwrap();
        let ident = hart.ident().unwrap();
        assert_eq!(ident.origin, "happyhumans");
        assert_eq!(ident.name, "possums");
        assert_eq!(ident.version, Some("8.1.4".to_string()));
        assert_eq!(ident.release, Some("20160427165340".to_string()));
    }

    pub fn root() -> PathBuf { PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests") }

    pub fn fixtures() -> PathBuf { root().join("fixtures") }

    #[test]
    fn reading_artifact_deps() {
        let mut hart =
            PackageArchive::new(fixtures().join("happyhumans-possums-8.1.\
                                                 4-20160427165340-x86_64-linux.hart")).unwrap();
        let _ = hart.deps().unwrap();
        let _ = hart.tdeps().unwrap();
    }

    #[test]
    fn reading_artifact_large_tdeps() {
        let mut hart =
            PackageArchive::new(fixtures().join("unhappyhumans-possums-8.1.\
                                                 4-20160427165340-x86_64-linux.hart")).unwrap();
        let tdeps = hart.tdeps().unwrap();
        assert_eq!(1024, tdeps.len());
    }

    #[test]
    #[cfg(feature = "x86_64-linux")]
    fn reading_artifact_target() {
        let mut hart =
            PackageArchive::new(fixtures().join("unhappyhumans-possums-8.1.\
                                                 4-20160427165340-x86_64-linux.hart")).unwrap();
        let target = hart.target().unwrap();

        assert_eq!(target::X86_64_LINUX, target);
    }
}
