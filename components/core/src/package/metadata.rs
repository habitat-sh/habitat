// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use std::collections::HashMap;
use std::env;
use std::fmt;
use std::iter::{FromIterator, IntoIterator};
use std::path::PathBuf;
use std::str::FromStr;
use std::vec::IntoIter;

use error::{Error, Result};

#[cfg(not(windows))]
const ENV_PATH_SEPARATOR: char = ':';

#[cfg(windows)]
const ENV_PATH_SEPARATOR: char = ';';

pub fn parse_key_value(s: &str) -> Result<HashMap<String, String>> {
    Ok(HashMap::from_iter(
        s.lines()
            .map(|l| l.splitn(2, '=').collect::<Vec<_>>())
            .map(|kv| (kv[0].to_string(), kv[1].to_string())),
    ))
}

#[derive(Debug)]
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
            Some(exports) => exports.split(' ').map(|t| t.to_string()).collect(),
        };
        Ok(Bind {
            service: service,
            exports: exports,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
    pub separator: Option<char>,
}

#[derive(Debug)]
pub struct PkgEnv {
    inner: Vec<EnvVar>,
}

impl PkgEnv {
    pub fn new(values: HashMap<String, String>, separators: HashMap<String, String>) -> Self {
        Self {
            inner: values
                .into_iter()
                .map(|(key, value)| if let Some(sep) = separators.get(&key) {
                    EnvVar {
                        key: key,
                        value: value,
                        separator: sep.to_owned().pop(),
                    }
                } else {
                    EnvVar {
                        key: key,
                        value: value,
                        separator: None,
                    }
                })
                .collect(),
        }
    }

    pub fn from_paths(paths: Vec<PathBuf>) -> Self {
        let p = env::join_paths(&paths).expect("Failed to build path string");
        Self {
            inner: vec![
                EnvVar {
                    key: "PATH".to_string(),
                    value: p.into_string().expect(
                        "Failed to convert path to utf8 string"
                    ),
                    separator: Some(ENV_PATH_SEPARATOR),
                },
            ],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl IntoIterator for PkgEnv {
    type Item = EnvVar;
    type IntoIter = IntoIter<EnvVar>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum MetaFile {
    Binds,
    BindsOptional,
    CFlags,
    Config,
    Deps,
    TDeps,
    Environment,
    EnvironmentSep,
    Exports,
    Exposes,
    Ident,
    LdRunPath,
    LdFlags,
    Manifest,
    Path,
    SvcUser,
    SvcGroup,
    Target,
}

impl fmt::Display for MetaFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = match *self {
            MetaFile::Binds => "BINDS",
            MetaFile::BindsOptional => "BINDS_OPTIONAL",
            MetaFile::CFlags => "CFLAGS",
            MetaFile::Config => "default.toml",
            MetaFile::Deps => "DEPS",
            MetaFile::TDeps => "TDEPS",
            MetaFile::Environment => "ENVIRONMENT",
            MetaFile::EnvironmentSep => "ENVIRONMENT_SEP",
            MetaFile::Exports => "EXPORTS",
            MetaFile::Exposes => "EXPOSES",
            MetaFile::Ident => "IDENT",
            MetaFile::LdRunPath => "LD_RUN_PATH",
            MetaFile::LdFlags => "LDFLAGS",
            MetaFile::Manifest => "MANIFEST",
            MetaFile::Path => "PATH",
            MetaFile::SvcUser => "SVC_USER",
            MetaFile::SvcGroup => "SVC_GROUP",
            MetaFile::Target => "TARGET",
        };
        write!(f, "{}", id)
    }
}

#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    #[should_panic]
    fn malformed_file() {
        parse_key_value(&"PATH").unwrap();
    }

    #[test]
    fn can_parse_environment_file() {
        let mut m: HashMap<String, String> = HashMap::new();
        m.insert(
            "PATH".to_string(),
            "/hab/pkgs/python/setuptools/35.0.1/20170424072606/bin".to_string(),
        );
        m.insert(
            "PYTHONPATH".to_string(),
            "/hab/pkgs/python/setuptools/35.0.1/20170424072606/lib/python3.6/site-packages"
                .to_string(),
        );

        assert_eq!(parse_key_value(&ENVIRONMENT).unwrap(), m);
    }

    #[test]
    fn can_parse_environment_sep_file() {
        let mut m: HashMap<String, String> = HashMap::new();
        m.insert("PATH".to_string(), ":".to_string());
        m.insert("PYTHONPATH".to_string(), ":".to_string());

        assert_eq!(parse_key_value(&ENVIRONMENT_SEP).unwrap(), m);
    }

    #[test]
    fn can_parse_exports_file() {
        let mut m: HashMap<String, String> = HashMap::new();
        m.insert("status-port".to_string(), "status.port".to_string());
        m.insert("port".to_string(), "front-end.port".to_string());

        assert_eq!(parse_key_value(&EXPORTS).unwrap(), m);
    }

    #[test]
    fn build_pkg_env() {
        let mut result = PkgEnv::new(
            parse_key_value(&ENVIRONMENT).unwrap(),
            parse_key_value(&ENVIRONMENT_SEP).unwrap(),
        ).into_iter()
            .collect::<Vec<_>>();
        // Sort the result by key, so we have a guarantee of order
        result.sort_by_key(|v| v.key.to_owned());

        let expected = vec![
            EnvVar {
                key: "PATH".to_string(),
                value: "/hab/pkgs/python/setuptools/35.0.1/20170424072606/bin".to_string(),
                separator: Some(':'),
            },
            EnvVar {
                key: "PYTHONPATH".to_string(),
                value: "/hab/pkgs/python/setuptools/35.0.1/20170424072606/lib/python3.6/site-packages"
                    .to_string(),
                separator: Some(':'),
            },
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn build_pkg_env_is_empty() {
        let result = PkgEnv::new(HashMap::new(), HashMap::new());
        assert!(result.is_empty());
    }

    #[test]
    fn build_pkg_env_from_path() {
        let result = PkgEnv::from_paths(vec![PathBuf::from(PATH)])
            .into_iter()
            .collect::<Vec<_>>();

        let expected = vec![
            EnvVar {
                key: "PATH".to_string(),
                value: "/hab/pkgs/python/setuptools/35.0.1/20170424072606/bin".to_string(),
                separator: Some(ENV_PATH_SEPARATOR),
            },
        ];

        assert_eq!(result, expected);
    }
}
