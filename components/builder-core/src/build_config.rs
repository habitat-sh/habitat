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
use std::path::PathBuf;
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

#[derive(Debug, Deserialize, Serialize)]
pub struct BuildCfg(HashMap<String, ProjectCfg>);

impl BuildCfg {
    pub fn from_slice(bytes: &[u8]) -> Result<Self> {
        let inner = toml::from_slice::<HashMap<String, ProjectCfg>>(bytes)
            .map_err(|e| Error::DecryptError(e.to_string()))?;
        Ok(BuildCfg(inner))
    }

    /// List of all registered projects for this `BuildCfg`.
    pub fn projects(&self) -> Vec<&ProjectCfg> {
        self.0.values().collect()
    }

    /// Returns true if the given branch & file path combination should result in a new build
    /// being automatically triggered by a GitHub Push notification.
    pub fn triggered_by<T>(&self, branch: &str, paths: &[T]) -> Vec<&ProjectCfg>
    where
        T: AsRef<str>,
    {
        self.0
            .values()
            .filter(|p| p.triggered_by(branch, paths))
            .collect()
    }
}

impl ConfigFile for BuildCfg {
    type Error = Error;
}

impl Default for BuildCfg {
    fn default() -> Self {
        let mut cfg = HashMap::default();
        cfg.insert("default".into(), ProjectCfg::default());
        BuildCfg(cfg)
    }
}

impl Deref for BuildCfg {
    type Target = HashMap<String, ProjectCfg>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Pattern {
    inner: glob::Pattern,
    options: glob::MatchOptions,
}

impl Pattern {
    fn default_options() -> glob::MatchOptions {
        glob::MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        }
    }

    pub fn matches<T>(&self, value: T) -> bool
    where
        T: AsRef<str>,
    {
        self.inner.matches_with(value.as_ref(), &self.options)
    }
}

impl<'de> Deserialize<'de> for Pattern {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Pattern::from_str(&s).map_err(de::Error::custom)
    }
}

impl fmt::Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl FromStr for Pattern {
    type Err = glob::PatternError;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        let inner: glob::Pattern = FromStr::from_str(value)?;
        Ok(Pattern {
            inner: inner,
            options: Pattern::default_options(),
        })
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

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectCfg {
    /// Branches which trigger an automatic rebuild on push notification from a GitHub push
    /// notification (default: ["master"]).
    #[serde(default = "ProjectCfg::default_branches")]
    pub branches: Vec<String>,
    /// Additional Release Channel to promote built packages into.
    #[serde(default)]
    pub channels: Vec<String>,
    /// Unix style file globs which are matched against changed files from a GitHub push
    /// notification to determine if an automatic rebuild should occur.
    #[serde(default)]
    pub paths: Vec<Pattern>,
    /// Relative filepath to the project's Habitat Plan (default: "habitat").
    #[serde(default = "ProjectCfg::default_plan_path")]
    plan_path: PathBuf,
}

impl ProjectCfg {
    fn default_branches() -> Vec<String> {
        vec!["master".to_string()]
    }

    fn default_plan_path() -> PathBuf {
        PathBuf::from("habitat")
    }

    fn default_plan_pattern() -> Pattern {
        Pattern::from_str("habitat/*").unwrap()
    }

    pub fn plan_file(&self) -> PathBuf {
        self.plan_path.join("plan.sh")
    }

    /// Returns true if the given branch & file path combination should result in a new build
    /// being automatically triggered by a GitHub Push notification
    fn triggered_by<T>(&self, branch: &str, paths: &[T]) -> bool
    where
        T: AsRef<str>,
    {
        if !self.branches.iter().any(|b| b == branch) {
            return false;
        }
        let plan_pattern = Pattern::from_str(&self.plan_path.join("*").to_string_lossy())
            .unwrap_or(Self::default_plan_pattern());
        paths.iter().any(|p| {
            plan_pattern.matches(p.as_ref()) || self.paths.iter().any(|i| i.matches(p.as_ref()))
        })
    }
}

impl Default for ProjectCfg {
    fn default() -> Self {
        ProjectCfg {
            branches: ProjectCfg::default_branches(),
            channels: vec![],
            paths: vec![],
            plan_path: ProjectCfg::default_plan_path(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const CONFIG: &'static str = r#"
    [hab-sup]
    plan_path = "components/hab-sup"
    branches = [
      "master",
      "dev",
    ]
    channels = [
      "stable"
    ]
    paths = [
      "components/net/*"
    ]

    [builder-api]
    plan_path = "components/builder-api/habitat"
    paths = [
      "components/net/*"
    ]

    [default]
    "#;

    #[test]
    fn triggered_by() {
        let cfg = BuildCfg::from_slice(CONFIG.as_bytes()).unwrap();
        let hab_sup = cfg.get("hab-sup").unwrap();
        let bldr_api = cfg.get("builder-api").unwrap();
        let default = cfg.get("default").unwrap();

        assert!(hab_sup.triggered_by(
            "master",
            &["components/hab-sup/Cargo.toml"],
        ));
        assert!(hab_sup.triggered_by(
            "master",
            &["components/hAb-Sup/Cargo.toml"],
        ));
        assert!(hab_sup.triggered_by(
            "dev",
            &["components/hab-sup/Cargo.toml"],
        ));
        assert_eq!(hab_sup.triggered_by("master", &["components"]), false);

        assert!(bldr_api.triggered_by(
            "master",
            &["components/builder-api/habitat/plan.sh"],
        ));
        assert!(bldr_api.triggered_by(
            "master",
            &["components/net/Cargo.toml"],
        ));

        assert!(default.triggered_by("master", &["habitat/plan.sh"]));
        assert!(default.triggered_by("master", &["habitat/hooks/init"]));
        assert_eq!(default.triggered_by("dev", &["habitat/plan.sh"]), false);
        assert_eq!(default.triggered_by("master", &["components"]), false);
    }
}
