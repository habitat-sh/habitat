use super::{Identifiable,
            PackageIdent,
            PackageTarget,
            metadata::{MetaFile,
                       PackageType}};
use crate::{crypto::{Blake2bHash,
                     artifact},
            error::{Error,
                    Result},
            package::ident::FullyQualifiedPackageIdent};
use regex::Regex;
use serde::Serialize;
use std::{collections::HashMap,
          convert::{TryFrom,
                    TryInto},
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
            MetaFile::SvcGroup,
            Regex::new(&format!(
                r"^/?hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$",
                MetaFile::SvcGroup
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
            MetaFile::PackageType,
            Regex::new(&format!(
                r"^/?hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$",
                MetaFile::PackageType
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

    /// Calculate and return the Blake2b hash of the package archive.
    // TODO (CM): Convert this to Blake2bHash once Builder can use it
    pub fn checksum(&self) -> Result<String> { Ok(Blake2bHash::from_file(&self.path)?.to_string()) }

    pub fn cflags(&self) -> Option<&str> { self.read_metadata(MetaFile::CFlags) }

    pub fn config(&self) -> Option<&str> { self.read_metadata(MetaFile::Config) }

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
    pub fn is_a_service(&self) -> bool { self.svc_user().is_some() }

    /// Returns a list of package identifiers representing the runtime package dependencies for
    /// this archive.
    ///
    /// # Failures
    ///
    /// * If the archive cannot be read
    /// * If the archive cannot be verified
    pub fn deps(&self) -> Result<Vec<PackageIdent>> { self.read_deps(MetaFile::Deps) }

    /// Returns a list of package identifiers representing the transitive runtime package
    /// dependencies for this archive.
    ///
    /// # Failures
    ///
    /// * If the archive cannot be read
    /// * If the archive cannot be verified
    pub fn tdeps(&self) -> Result<Vec<PackageIdent>> { self.read_deps(MetaFile::TDeps) }

    /// Returns a list of package identifiers representing the buildtime package dependencies for
    /// this archive.
    ///
    /// # Failures
    ///
    /// * If the archive cannot be read
    /// * If the archive cannot be verified
    pub fn build_deps(&self) -> Result<Vec<PackageIdent>> { self.read_deps(MetaFile::BuildDeps) }

    /// Returns a list of package identifiers representing the transitive buildtime package
    /// dependencies for this archive.
    ///
    /// # Failures
    ///
    /// * If the archive cannot be read
    /// * If the archive cannot be verified
    pub fn build_tdeps(&self) -> Result<Vec<PackageIdent>> { self.read_deps(MetaFile::BuildTDeps) }

    pub fn exposes(&self) -> Result<Vec<u16>> {
        if let Some(data) = self.read_metadata(MetaFile::Exposes) {
            data.split_whitespace()
                .map(|port| port.parse().map_err(Error::InvalidPort))
                .collect()
        } else {
            Ok(vec![])
        }
    }

    pub fn ident(&self) -> Result<PackageIdent> {
        if let Some(data) = self.read_metadata(MetaFile::Ident) {
            PackageIdent::from_str(data)
        } else {
            Err(Error::MetaFileNotFound(MetaFile::Ident))
        }
    }

    pub fn ld_run_path(&self) -> Option<&str> { self.read_metadata(MetaFile::LdRunPath) }

    pub fn ldflags(&self) -> Option<&str> { self.read_metadata(MetaFile::LdFlags) }

    pub fn svc_user(&self) -> Option<&str> { self.read_metadata(MetaFile::SvcUser) }

    pub fn svc_group(&self) -> Option<&str> { self.read_metadata(MetaFile::SvcGroup) }

    pub fn manifest(&self) -> Result<&str> {
        if let Some(data) = self.read_metadata(MetaFile::Manifest) {
            Ok(data)
        } else {
            Err(Error::MetaFileNotFound(MetaFile::Manifest))
        }
    }

    pub fn package_type(&self) -> Result<PackageType> {
        if let Some(data) = self.read_metadata(MetaFile::PackageType) {
            PackageType::from_str(data)
        } else {
            Ok(PackageType::Standard)
        }
    }

    pub fn path(&self) -> Option<&str> { self.read_metadata(MetaFile::Path) }

    pub fn target(&self) -> Result<PackageTarget> {
        if let Some(data) = self.read_metadata(MetaFile::Target) {
            PackageTarget::from_str(data)
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

    fn read_deps(&self, file: MetaFile) -> Result<Vec<PackageIdent>> {
        let mut deps = vec![];

        if let Some(body) = self.read_metadata(file) {
            for id in body.lines() {
                let package = PackageIdent::from_str(id)?;
                if !package.fully_qualified() {
                    return Err(Error::FullyQualifiedPackageIdentRequired(package.to_string()));
                }
                deps.push(package);
            }
        }

        Ok(deps)
    }

    fn read_metadata(&self, file: MetaFile) -> Option<&str> {
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

// Exposes the extended archive and header metadata. Types are stored as non habitat primitives
// with the intent being ease of deserialization into content such as conveniently formatted json
// at the client display layer.
#[derive(Serialize)]
pub struct PackageArchiveInfo {
    pub format_version: String,
    pub key_name:       String,
    pub hash_type:      String,
    // This should probably be called `encoded_signature`, or perhaps
    // just `signature`, but this is a public interface at the moment.
    pub signature_raw:  String,
    pub ident:          String,
    pub origin:         String,
    pub name:           String,
    pub version:        String,
    pub release:        String,
    pub checksum:       String,
    pub target:         String,
    pub is_a_service:   bool,
    pub deps:           Vec<String>,
    pub tdeps:          Vec<String>,
    pub build_deps:     Vec<String>,
    pub build_tdeps:    Vec<String>,
    pub exposes:        Vec<u16>,
    pub manifest:       String,
    pub config:         Option<String>,
    pub svc_user:       Option<String>,
    pub svc_group:      Option<String>,
    pub ld_run_path:    Option<String>,
    pub ldflags:        Option<String>,
    pub cflags:         Option<String>,
}

impl TryFrom<PackageArchive> for PackageArchiveInfo {
    type Error = Error;

    fn try_from(archive: PackageArchive) -> Result<Self> {
        let header = artifact::get_artifact_header(&archive.path)?;
        let ident: FullyQualifiedPackageIdent = archive.ident()?.try_into()?;
        Ok(PackageArchiveInfo { format_version: header.format().clone(),

                                // TODO (CM): NamedRevision!
                                key_name:      header.signer().to_string(),
                                hash_type:     header.hash_type().clone(),
                                signature_raw: header.encoded_signature(),
                                origin:        ident.origin().to_string(),
                                name:          ident.name().to_string(),
                                ident:         ident.to_string(),
                                version:       ident.version().to_string(),
                                release:       ident.release().to_string(),
                                checksum:      archive.checksum()?,
                                target:        archive.target()?.to_string(),
                                is_a_service:  archive.is_a_service(),
                                deps:          archive.deps()?
                                                      .iter()
                                                      .map(ToString::to_string)
                                                      .collect(),
                                build_deps:    archive.build_deps()?
                                                      .iter()
                                                      .map(ToString::to_string)
                                                      .collect(),
                                tdeps:         archive.tdeps()?
                                                      .iter()
                                                      .map(ToString::to_string)
                                                      .collect(),
                                build_tdeps:   archive.build_tdeps()?
                                                      .iter()
                                                      .map(ToString::to_string)
                                                      .collect(),
                                exposes:       archive.exposes()?,
                                manifest:      archive.manifest()?.to_string(),
                                svc_user:      archive.svc_user().map(ToString::to_string),
                                svc_group:     archive.svc_group().map(ToString::to_string),
                                config:        archive.config().map(ToString::to_string),
                                ld_run_path:   archive.ld_run_path().map(ToString::to_string),
                                ldflags:       archive.ldflags().map(ToString::to_string),
                                cflags:        archive.cflags().map(ToString::to_string), })
    }
}

impl PackageArchiveInfo {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<PackageArchiveInfo> {
        let archive = PackageArchive::new(path.as_ref())?;
        PackageArchiveInfo::try_from(archive)
    }
}

#[cfg(test)]
mod test {
    use super::{super::target,
                *};
    use std::path::PathBuf;

    #[test]
    fn reading_artifact_metadata() {
        let hart =
            PackageArchive::new(fixtures().join("happyhumans-possums-8.1.\
                                                 4-20160427165340-x86_64-linux.hart")).unwrap();
        let ident = hart.ident().unwrap();
        assert_eq!(ident.origin, "happyhumans");
        assert_eq!(ident.name, "possums");
        assert_eq!(ident.version, Some("8.1.4".to_string()));
        assert_eq!(ident.release, Some("20160427165340".to_string()));
    }

    #[test]
    fn reading_artifact_extended_metadata() {
        let hart =
            PackageArchive::new(fixtures().join("unhappyhumans-possums-8.1.\
                                                 4-20160427165340-x86_64-linux.hart")).unwrap();
        let info = PackageArchiveInfo::from_path(hart.path).unwrap();
        assert_eq!(info.format_version, "HART-1");
        assert_eq!(info.key_name, "happyhumans-20160424223347");
        assert_eq!(info.hash_type, "BLAKE2b");
        assert_eq!(info.signature_raw, "AgdmAKa9wr4ExnSWe5rg2VJh6cc2vOfyXCs3JOnsSm1XtmtQNhhON6fhgp0hW0xZkNcgXmC1lQ7w5WdZU0M4Bjg4MDVlNTU3NWFiOGMwMjllNmQyNTgyNjEzNzlmYmQwMmQ0YmIzZDkwZTIwNjg0N2Q0NTUzYTFiZjczOTVkNjU=");
        assert_eq!(info.target, "x86_64-linux");
        assert_eq!(info.deps.len(), 0);
        assert_eq!(info.tdeps.len(), 1024);
        assert_eq!(info.tdeps[0], "core/glibc/2.22/20160612063629");
    }

    #[test]
    fn serialize_packagearchiveinfo() {
        let hart =
            PackageArchive::new(fixtures().join("happyhumans-possums-8.1.\
                                                 4-20160427165340-x86_64-linux.hart")).unwrap();
        let info = PackageArchiveInfo::from_path(hart.path).unwrap();
        let expected = serde_json::json!({
        "format_version": "HART-1",
        "key_name": "happyhumans-20160424223347",
        "hash_type": "BLAKE2b",
        "signature_raw": "U0cp/+npru0ZxhK76zm+PDVSV/707siyrO1r7T6CZZ4ShSLrIxyx8jLSMr5wnLuGrVIV358smQPWOSTOmyfFCjBmMmM1ZjRkZTE0NWM3Zjc4NjAxY2FhZTljN2I4NzY3MDk4NDEzZDA1NzM5ZGU5MTNjMDEyOTIyYjdlZWQ3NjA=",
        "ident": "happyhumans/possums/8.1.4/20160427165340",
        "origin": "happyhumans",
        "name": "possums",
        "version": "8.1.4",
        "release": "20160427165340",
        "checksum": "74d368d3642721b5045929845f6a1146fad50c7ecab7ab547603965e45a29a82",
        "target": "x86_64-linux",
        "is_a_service": false,
        "deps": [],
        "tdeps": [],
        "build_deps": [],
        "build_tdeps": [],
        "exposes": [],
        "manifest": "happyhumans possums\n=========================\n\nMaintainer: The Habitat Maintainers <humans@habitat.sh>\nVersion: 8.1.4\nRelease: 20160427165340\nArchitecture: x86_64\nSystem: linux\nTarget: x86_64-linux\nLicense: apachev2 \nSource: [nosuchfile.tar.gz](nosuchfile.tar.gz)\nSHA: \nPath: /hab/pkgs/happyhumans/possums/8.1.4/20160427165340\nBuild Dependencies:  \nDependencies:  \nInterpreters:  \n\nPlan\n========\n\nBuild Flags\n-----------\n\nCFLAGS: \nLDFLAGS: \nLD_RUN_PATH: \n\n```bash\npkg_name=possums\npkg_origin=happyhumans\npkg_version=8.1.4\npkg_maintainer=\"The Habitat Maintainers <humans@habitat.sh>\"\npkg_license=('apachev2')\npkg_source=nosuchfile.tar.gz\npkg_deps=()\npkg_build_deps=()\n\ndo_build() {\n  cp -v $PLAN_CONTEXT/signme.dat signme.dat\n}\n\ndo_install() {\n  install -v -D signme.dat $pkg_prefix/share/signme.dat\n}\n\n# Turn the remaining default phases into no-ops\n\ndo_download() {\n  return 0\n}\n\ndo_verify() {\n  return 0\n}\n\ndo_unpack() {\n  return 0\n}\n\ndo_prepare() {\n  return 0\n}\n```\n\nFiles\n-----\n4cc8037f90192a8eecdb9b386a289d35be3c8cd7f92bd6b1d0e2d783dea592c6  /hab/pkgs/happyhumans/possums/8.1.4/20160427165340/IDENT\nd3b7abad38647ed804b5017c5b990acab7c85648b552a97043d4d86c70ce1f9d  /hab/pkgs/happyhumans/possums/8.1.4/20160427165340/TARGET\nb5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c  /hab/pkgs/happyhumans/possums/8.1.4/20160427165340/share/signme.dat",
        "config": null,
        "svc_user": null,
        "svc_group": null,
        "ld_run_path": null,
        "ldflags": null,
        "cflags": null
              });
        let actual = serde_json::to_value(info).unwrap();
        assert_eq!(actual, expected);
    }

    pub fn root() -> PathBuf { PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests") }

    pub fn fixtures() -> PathBuf { root().join("fixtures") }

    #[test]
    fn reading_artifact_deps() {
        let hart =
            PackageArchive::new(fixtures().join("happyhumans-possums-8.1.\
                                                 4-20160427165340-x86_64-linux.hart")).unwrap();
        let _ = hart.deps().unwrap();
        let _ = hart.tdeps().unwrap();
    }

    #[test]
    fn reading_artifact_large_tdeps() {
        let hart =
            PackageArchive::new(fixtures().join("unhappyhumans-possums-8.1.\
                                                 4-20160427165340-x86_64-linux.hart")).unwrap();
        let tdeps = hart.tdeps().unwrap();
        assert_eq!(1024, tdeps.len());
    }

    #[test]
    #[cfg(feature = "x86_64-linux")]
    fn reading_artifact_target() {
        let hart =
            PackageArchive::new(fixtures().join("unhappyhumans-possums-8.1.\
                                                 4-20160427165340-x86_64-linux.hart")).unwrap();
        let target = hart.target().unwrap();

        assert_eq!(target::X86_64_LINUX, target);
    }
}
