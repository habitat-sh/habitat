use crate::error::{Error,
                   Result};
use serde::{Deserialize,
            Serialize};
use std::{self,
          collections::BTreeMap,
          env,
          fmt,
          fs::File,
          io::Read,
          iter::IntoIterator,
          path::{Path,
                 PathBuf},
          str::FromStr,
          string::ToString,
          vec::IntoIter};

#[cfg(not(windows))]
const ENV_PATH_SEPARATOR: char = ':';

#[cfg(windows)]
const ENV_PATH_SEPARATOR: char = ';';

pub fn parse_key_value(s: &str) -> Result<BTreeMap<String, String>> {
    Ok(s.lines()
        .map(|l| l.splitn(2, '=').collect::<Vec<_>>())
        .map(|kv| (kv[0].to_string(), kv[1].to_string()))
        .collect())
}

#[derive(Clone, Debug, Serialize)]
pub struct Bind {
    pub service: String,
    pub exports: Vec<String>,
}

impl FromStr for Bind {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let mut parts = line.split('=');
        let service = match parts.next() {
            None => return Err(Error::MetaFileBadBind),
            Some(service) => service.to_string(),
        };
        let exports = match parts.next() {
            None => return Err(Error::MetaFileBadBind),
            Some(exports) => exports.split_whitespace().map(str::to_string).collect(),
        };
        Ok(Bind { service, exports })
    }
}

impl fmt::Display for Bind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted_exports = self.exports.join(" ");
        write!(f, "[{}]={}", self.service, formatted_exports)
    }
}

#[derive(Debug, PartialEq)]
pub struct EnvVar {
    pub key:       String,
    pub value:     String,
    pub separator: Option<char>,
}

#[derive(Debug)]
pub struct PkgEnv {
    inner: Vec<EnvVar>,
}

impl PkgEnv {
    pub fn new(values: BTreeMap<String, String>, separators: &BTreeMap<String, String>) -> Self {
        Self { inner: values.into_iter()
                            .map(|(key, value)| {
                                if let Some(sep) = separators.get(&key) {
                                    EnvVar { key,
                                             value,
                                             separator: sep.to_owned().pop() }
                                } else {
                                    EnvVar { key,
                                             value,
                                             separator: None }
                                }
                            })
                            .collect(), }
    }

    pub fn from_paths(paths: &[PathBuf]) -> Self {
        let p = env::join_paths(paths).expect("Failed to build path string");
        Self { inner: vec![EnvVar { key:       "PATH".to_string(),
                                    value:     p.into_string()
                                                .expect("Failed to convert path to utf8 string"),
                                    separator: Some(ENV_PATH_SEPARATOR), }], }
    }

    pub fn is_empty(&self) -> bool { self.inner.is_empty() }
}

impl IntoIterator for PkgEnv {
    type IntoIter = IntoIter<EnvVar>;
    type Item = EnvVar;

    fn into_iter(self) -> Self::IntoIter { self.inner.into_iter() }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum MetaFile {
    Binds,
    BindsOptional,
    BuildDeps,
    BuildTDeps,
    CFlags,
    Config,
    Deps,
    Environment,
    EnvironmentSep,
    Exports,
    Exposes,
    Ident,
    LdFlags,
    LdRunPath,
    Manifest,
    Path,
    RuntimeEnvironment,
    RuntimeEnvironmentPaths,
    RuntimePath,
    RuntimeHabLdLibraryPath,
    ShutdownSignal,
    ShutdownTimeout,
    SvcGroup,
    SvcUser,
    Target,
    TDeps,
    PackageType,
}

impl fmt::Display for MetaFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = match *self {
            MetaFile::Binds => "BINDS",
            MetaFile::BindsOptional => "BINDS_OPTIONAL",
            MetaFile::BuildDeps => "BUILD_DEPS",
            MetaFile::BuildTDeps => "BUILD_TDEPS",
            MetaFile::CFlags => "CFLAGS",
            MetaFile::Config => "default.toml",
            MetaFile::Deps => "DEPS",
            MetaFile::Environment => "ENVIRONMENT",
            MetaFile::EnvironmentSep => "ENVIRONMENT_SEP",
            MetaFile::Exports => "EXPORTS",
            MetaFile::Exposes => "EXPOSES",
            MetaFile::Ident => "IDENT",
            MetaFile::LdFlags => "LDFLAGS",
            MetaFile::LdRunPath => "LD_RUN_PATH",
            MetaFile::Manifest => "MANIFEST",
            MetaFile::Path => "PATH",
            MetaFile::RuntimeEnvironment => "RUNTIME_ENVIRONMENT",
            MetaFile::RuntimeEnvironmentPaths => "RUNTIME_ENVIRONMENT_PATHS",
            MetaFile::RuntimePath => "RUNTIME_PATH",
            MetaFile::RuntimeHabLdLibraryPath => "RUNTIME_HAB_LD_LIBRARY_PATH",
            MetaFile::ShutdownSignal => "SHUTDOWN_SIGNAL",
            MetaFile::ShutdownTimeout => "SHUTDOWN_TIMEOUT",
            MetaFile::SvcGroup => "SVC_GROUP",
            MetaFile::SvcUser => "SVC_USER",
            MetaFile::Target => "TARGET",
            MetaFile::TDeps => "TDEPS",
            MetaFile::PackageType => "PACKAGE_TYPE",
        };
        write!(f, "{}", id)
    }
}

/// Read a metadata file from within a package directory if it exists
///
/// Returns the contents of the file
pub fn read_metafile<P: AsRef<Path>>(installed_path: P, file: MetaFile) -> Result<String> {
    match existing_metafile(installed_path, file) {
        Some(filepath) => {
            match File::open(filepath) {
                Ok(mut f) => {
                    let mut data = String::new();
                    if f.read_to_string(&mut data).is_err() {
                        return Err(Error::MetaFileMalformed(file));
                    }
                    Ok(data.trim().to_string())
                }
                Err(e) => Err(Error::MetaFileIO(e)),
            }
        }
        None => Err(Error::MetaFileNotFound(file)),
    }
}

/// Returns the path to a specified MetaFile in an installed path if it exists.
///
/// Useful for fallback logic for dealing with older Habitat packages.
fn existing_metafile<P: AsRef<Path>>(installed_path: P, file: MetaFile) -> Option<PathBuf> {
    let filepath = installed_path.as_ref().join(file.to_string());
    match std::fs::metadata(&filepath) {
        Ok(_) => Some(filepath),
        Err(_) => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum PackageType {
    Standard,
    Native,
}

impl fmt::Display for PackageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = match *self {
            PackageType::Standard => "standard",
            PackageType::Native => "native",
        };
        write!(f, "{}", id)
    }
}

impl FromStr for PackageType {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        match value {
            "standard" => Ok(PackageType::Standard),
            "native" => Ok(PackageType::Native),
            _ => Err(Error::InvalidPackageType(value.to_string())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Write;
    use tempfile::Builder;

    static ENVIRONMENT: &str = r#"PATH=/hab/pkgs/python/setuptools/35.0.1/20170424072606/bin
PYTHONPATH=/hab/pkgs/python/setuptools/35.0.1/20170424072606/lib/python3.6/site-packages
"#;
    static ENVIRONMENT_SEP: &str = r#"PATH=:
PYTHONPATH=:
"#;
    static EXPORTS: &str = r#"status-port=status.port
port=front-end.port
"#;
    static PATH: &str = "/hab/pkgs/python/setuptools/35.0.1/20170424072606/bin";

    /// Write the given contents into the specified metadata file for
    /// the package.
    fn write_metafile(install_dir: &Path, metafile: MetaFile, content: &str) {
        let path = install_dir.join(metafile.to_string());
        let mut f = File::create(path).expect("Could not create metafile");
        f.write_all(content.as_bytes())
         .expect("Could not write metafile contents");
    }

    #[test]
    #[should_panic]
    fn malformed_file() { parse_key_value("PATH").unwrap(); }

    #[test]
    fn can_parse_environment_file() {
        let mut m: BTreeMap<String, String> = BTreeMap::new();
        m.insert("PATH".to_string(),
                 "/hab/pkgs/python/setuptools/35.0.1/20170424072606/bin".to_string());
        m.insert(
            "PYTHONPATH".to_string(),
            "/hab/pkgs/python/setuptools/35.0.1/20170424072606/lib/python3.6/site-packages"
                .to_string(),
        );

        assert_eq!(parse_key_value(ENVIRONMENT).unwrap(), m);
    }

    #[test]
    fn can_parse_environment_sep_file() {
        let mut m: BTreeMap<String, String> = BTreeMap::new();
        m.insert("PATH".to_string(), ":".to_string());
        m.insert("PYTHONPATH".to_string(), ":".to_string());

        assert_eq!(parse_key_value(ENVIRONMENT_SEP).unwrap(), m);
    }

    #[test]
    fn can_parse_exports_file() {
        let mut m: BTreeMap<String, String> = BTreeMap::new();
        m.insert("status-port".to_string(), "status.port".to_string());
        m.insert("port".to_string(), "front-end.port".to_string());

        assert_eq!(parse_key_value(EXPORTS).unwrap(), m);
    }

    #[test]
    fn build_pkg_env() {
        let mut result =
            PkgEnv::new(parse_key_value(ENVIRONMENT).unwrap(),
                        &parse_key_value(ENVIRONMENT_SEP).unwrap()).into_iter()
                                                                   .collect::<Vec<_>>();
        // Sort the result by key, so we have a guarantee of order
        result.sort_by_key(|v| v.key.to_owned());

        let expected =
            vec![EnvVar { key:       "PATH".to_string(),
                          value:
                              "/hab/pkgs/python/setuptools/35.0.1/20170424072606/bin".to_string(),
                          separator: Some(':'), },
                 EnvVar { key:       "PYTHONPATH".to_string(),
                          value:     "/hab/pkgs/python/setuptools/35.0.1/20170424072606/lib/\
                                      python3.6/site-packages"
                                                              .to_string(),
                          separator: Some(':'), },];

        assert_eq!(result, expected);
    }

    #[test]
    fn build_pkg_env_is_empty() {
        let result = PkgEnv::new(BTreeMap::new(), &BTreeMap::new());
        assert!(result.is_empty());
    }

    #[test]
    fn build_pkg_env_from_path() {
        let result = PkgEnv::from_paths(&[PathBuf::from(PATH)]).into_iter()
                                                               .collect::<Vec<_>>();

        let expected = vec![EnvVar { key:       "PATH".to_string(),
                                     value:     "/hab/pkgs/python/setuptools/35.0.1/\
                                                 20170424072606/bin"
                                                                    .to_string(),
                                     separator: Some(ENV_PATH_SEPARATOR), }];

        assert_eq!(result, expected);
    }

    #[test]
    fn can_read_metafile() {
        let pkg_root = Builder::new().prefix("pkg-root").tempdir().unwrap();
        let install_dir = pkg_root.path();

        let expected = "core/foo=db:core/database";
        write_metafile(install_dir, MetaFile::Binds, expected);

        let bind_map = read_metafile(install_dir, MetaFile::Binds).unwrap();

        assert_eq!(expected, bind_map);
    }

    #[test]
    fn reading_a_non_existing_metafile_is_an_error() {
        let pkg_root = Builder::new().prefix("pkg-root").tempdir().unwrap();
        let install_dir = pkg_root.path();
        let bind_map = read_metafile(install_dir, MetaFile::Binds);

        assert!(bind_map.is_err());
    }
}
