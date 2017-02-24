// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

use std::path::{Path, PathBuf};

use hab_core;
use hab_core::package::archive::PackageArchive;
use hab_core::config::ConfigFile;

use super::workspace::Workspace;
use depot_client;
use error::Error;
use {PRODUCT, VERSION};

/// Postprocessing config file name
const CONFIG_FILE: &'static str = "builder.toml";

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Publish {
    /// Whether publish is enabled
    pub enabled: bool,
    /// URL to Depot API
    pub url: String,
    /// Channel to publish to
    pub channel: String,
}

impl Publish {
    pub fn run(&mut self, archive: &mut PackageArchive, auth_token: &str) -> bool {
        if !self.enabled {
            return true;
        }

        debug!("post process: publish (url: {}, channel: {})",
               self.url,
               self.channel);

        // Things to solve right now
        // * Where do we get the token for authentication?
        // * Should the workers ask for a lease from the JobSrv?
        let client = depot_client::Client::new(&self.url, PRODUCT, VERSION, None).unwrap();
        if let Some(err) = client.x_put_package(archive, auth_token).err() {
            error!("post processing error uploading package, ERR={:?}", err);
            return false;
        };

        if let Some(err) = client
               .promote_package(archive, &self.channel, auth_token)
               .err() {
            error!("post processing error promoting package, ERR={:?}", err);
            return false;
        };
        true
    }
}

impl Default for Publish {
    fn default() -> Self {
        Publish {
            enabled: hab_core::url::default_depot_publish()
                .parse::<bool>()
                .unwrap(),
            url: hab_core::url::default_depot_url(),
            channel: hab_core::url::default_depot_channel(),
        }
    }
}

impl ConfigFile for Publish {
    type Error = Error;
}

pub struct PostProcessor {
    config_path: PathBuf,
}

impl PostProcessor {
    pub fn new(workspace: &Workspace) -> Self {
        let parent_path = Path::new(workspace.job.get_project().get_plan_path())
            .parent()
            .unwrap();
        let file_path = workspace.src().join(parent_path.join(CONFIG_FILE));

        PostProcessor { config_path: file_path }
    }

    pub fn run(&mut self, archive: &mut PackageArchive, auth_token: &str) -> bool {
        let mut cfg = if !self.config_path.exists() {
            debug!("no post processing config - using defaults");
            Publish::default()
        } else {
            debug!("using post processing config from builder.toml");
            match Publish::from_file(&self.config_path) {
                Ok(value) => value,
                Err(e) => {
                    debug!("failed to parse config file! {:?}", e);
                    return false;
                }
            }
        };

        debug!("starting post processing");
        cfg.run(archive, auth_token)
    }
}

#[cfg(test)]
mod tests {
    use hab_core::config::ConfigFile;
    use super::*;

    #[test]
    fn test_publish_config_from_toml() {
        let toml = r#"
        [publish]
        enabled = false
        url = "https://willem.habitat.sh/v1/depot"
        channel = "unstable"
        "#;

        let cfg = Publish::from_raw(toml).unwrap();
        assert_eq!("https://willem.habitat.sh/v1/depot", cfg.url);
        assert_eq!(false, cfg.enabled);
        assert_eq!("unstable", cfg.channel);
    }
}
