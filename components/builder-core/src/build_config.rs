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
use std::fmt;
use std::ops::Deref;
use std::result;
use std::str::FromStr;
use std::string::ToString;

use glob;
use hab_core::channel::UNSTABLE_CHANNEL;
use hab_core::config::ConfigFile;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use toml;

use error::{Error, Result};

/// Postprocessing config file name
pub const BLDR_CFG: &'static str = ".bldr.toml";
pub const DEFAULT_CHANNEL: &'static str = UNSTABLE_CHANNEL;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BuildCfg(HashMap<String, ProjectCfg>);

impl BuildCfg {
    pub fn from_slice(bytes: &[u8]) -> Result<Self> {
        let value = toml::from_slice::<HashMap<String, ProjectCfg>>(bytes)
            .map_err(|e| Error::DecryptError(e.to_string()))?;
        Ok(BuildCfg(value))
    }
}

impl ConfigFile for BuildCfg {
    type Error = Error;
}

impl Deref for BuildCfg {
    type Target = HashMap<String, ProjectCfg>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectCfg {
    /// Branches which trigger an automatic rebuild on push notification from a GitHub push
    /// notification.
    #[serde(default = "default_branches")]
    pub branches: Vec<String>,
    /// Additional Release Channel to promote built packages into.
    #[serde(default)]
    pub channels: Vec<String>,
    /// Unix style file globs which are matched against changed files from a GitHub push
    /// notification to determine if an automatic rebuild should occur.
    #[serde(default)]
    pub paths: Vec<Pattern>,
}

impl ProjectCfg {
    /// Returns true if the given branch & file path combination should result in a new build
    /// being automatically triggered by a GitHub Push notification
    pub fn triggered_by<T>(&self, branch: &str, paths: &[T]) -> bool
    where
        T: AsRef<str>,
    {
        if !self.branches.iter().any(|b| b == branch) {
            return false;
        }
        paths.iter().any(|p| {
            self.paths.iter().any(|i| i.matches(p.as_ref()))
        })
    }
}

impl Default for ProjectCfg {
    fn default() -> Self {
        ProjectCfg {
            branches: default_branches(),
            paths: vec![],
            channels: vec![],
        }
    }
}

pub struct Pattern {
    inner: glob::Pattern,
    options: glob::MatchOptions,
}

impl Pattern {
    pub fn matches<T>(&self, value: T) -> bool
    where
        T: AsRef<str>,
    {
        self.inner.matches_with(value.as_ref(), &self.options)
    }

    fn default_options() -> glob::MatchOptions {
        glob::MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        }
    }
}

impl<'de> Deserialize<'de> for Pattern {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let inner: glob::Pattern = FromStr::from_str(&s).map_err(de::Error::custom)?;
        Ok(Pattern {
            inner: inner,
            options: Pattern::default_options(),
        })
    }
}

impl fmt::Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Serialize for Pattern {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.inner.to_string())
    }
}

fn default_branches() -> Vec<String> {
    vec!["master".to_string()]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_contents() {
        let raw = r#"
        [hab-sup]
        branches = [
            "master",
            "dev",
        ]
        channels = [
          "stable"
        ]
        paths = [
          "components/hab-sup/*"
        ]

        [builder-api]
        paths = [
          "components/builder-api/*"
        ]
        "#;
        let cfg = BuildCfg::from_slice(raw.as_bytes()).unwrap();
        assert_eq!(cfg.len(), 2);
        assert_eq!(
            cfg.get("hab-sup").unwrap().triggered_by(
                "master",
                &[
                    "components/hab-sup/Cargo.toml",
                ],
            ),
            true
        );
        assert_eq!(
            cfg.get("hab-sup").unwrap().triggered_by(
                "master",
                &[
                    "components/hAb-Sup/Cargo.toml",
                ],
            ),
            true
        );
        assert_eq!(
            cfg.get("hab-sup").unwrap().triggered_by(
                "dev",
                &[
                    "components/hab-sup/Cargo.toml",
                ],
            ),
            true
        );
        assert_eq!(
            cfg.get("hab-sup").unwrap().triggered_by(
                "master",
                &["components"],
            ),
            false
        );
        assert_eq!(
            cfg.get("builder-api").unwrap().triggered_by(
                "master",
                &[
                    "components/builder-api/Cargo.toml",
                ],
            ),
            true
        );
        assert_eq!(cfg.get("hab-sup").unwrap().branches, &["master", "dev"]);
    }
}
