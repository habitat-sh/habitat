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

use crate::ui::{UIWriter,
                UI};
use habitat_api_client as api_client;
use habitat_core as hcore;
use lazy_static::lazy_static;
use std::{collections::HashMap,
          env,
          iter::FromIterator};

extern crate json;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg_attr(test, macro_use)]
extern crate serde_json;
#[cfg(windows)]
extern crate winapi;

pub use self::error::{Error,
                      Result};

pub mod cli;
pub mod command;
pub mod error;
pub mod locked_env_var;
pub mod output;
pub mod package_graph;
pub mod templating;
pub mod types;
pub mod ui;
pub mod util;

lazy_static::lazy_static! {
    pub static ref PROGRAM_NAME: String = {
        match env::current_exe() {
            Ok(path) => path.file_stem().and_then(|p| p.to_str()).unwrap().to_string(),
            Err(e) => {
                error!("Error getting path of current_exe: {}", e);
                String::from("hab-?")
            }
        }
    };
}

// TODO (CM): It would be nice to come up with a way to more
// programmatically manage these flags. It's a bit of a pain to define
// the flag, and then define the environment variables
// separately. Nothing statically guarantees that you've specified an
// variable for a flag.

// TODO (CM): It'd be great to have a built-in way to document them,
// too.

// TODO (CM): Part of that documentation might be *when* a flag was
// added. In general, long-lived flags are a code-smell.

// TODO (CM): It may also be useful to break out features by area of
// concern. We can have any number of bitflags-generated structs.

bitflags::bitflags! {
    /// All the feature flags that are recogized by Habitat.
    ///
    /// In general, feature flags are enabled by setting the corresponding
    /// environment variable.
    ///
    /// Your binary should call `FeatureFlag::from_env` to get a set
    /// of flags to use.
    ///
    /// To add a new feature flag, you will need to add the bit mask
    /// constant here, as well as a mapping from the feature to the
    /// environment variable to which it corresponds in the `ENV_VARS`
    /// map below.
    pub struct FeatureFlag: u32 {
        const LIST            = 0b0000_0000_0001;
        const TEST_EXIT       = 0b0000_0000_0010;
        const TEST_BOOT_FAIL  = 0b0000_0000_0100;
        const REDACT_HTTP     = 0b0000_0000_1000;
        const IGNORE_SIGNALS  = 0b0000_0001_0000;
        const INSTALL_HOOK    = 0b0000_0010_0000;
        const OFFLINE_INSTALL = 0b0000_0100_0000;
        const IGNORE_LOCAL    = 0b0000_1000_0000;
        const EVENT_STREAM    = 0b0001_0000_0000;
    }
}

lazy_static! {
    static ref ENV_VARS: HashMap<FeatureFlag, &'static str> = {
        let mapping = vec![(FeatureFlag::LIST, "HAB_FEAT_LIST"),
                           (FeatureFlag::TEST_EXIT, "HAB_FEAT_TEST_EXIT"),
                           (FeatureFlag::TEST_BOOT_FAIL, "HAB_FEAT_BOOT_FAIL"),
                           (FeatureFlag::REDACT_HTTP, "HAB_FEAT_REDACT_HTTP"),
                           (FeatureFlag::IGNORE_SIGNALS, "HAB_FEAT_IGNORE_SIGNALS"),
                           (FeatureFlag::INSTALL_HOOK, "HAB_FEAT_INSTALL_HOOK"),
                           (FeatureFlag::OFFLINE_INSTALL, "HAB_FEAT_OFFLINE_INSTALL"),
                           (FeatureFlag::IGNORE_LOCAL, "HAB_FEAT_IGNORE_LOCAL"),
                           (FeatureFlag::EVENT_STREAM, "HAB_FEAT_EVENT_STREAM")];
        HashMap::from_iter(mapping)
    };
}

impl FeatureFlag {
    /// If the environment variable for a flag is set to _anything_ but
    /// the empty string, it is activated.
    pub fn from_env(ui: &mut UI) -> Self {
        let mut flags = FeatureFlag::empty();

        for (feature, env_var) in ENV_VARS.iter() {
            if let Some(val) = env::var_os(env_var) {
                if !val.is_empty() {
                    flags.insert(*feature);
                    ui.warn(&format!("Enabling feature: {:?}", feature))
                      .unwrap();
                }
            }
        }

        // TODO (CM): Once the other TODOs above are done (especially the
        // documentation bits), it would be nice to extract this logic
        // into an actual discoverable CLI subcommand; it's a little weird
        // that you have to know how to enable a feature flag before you
        // can even find out that there *are* feature flags to enable.
        //
        // There's no reason why "list feature flags" should itself be a
        // feature-flag.
        if flags.contains(FeatureFlag::LIST) {
            ui.warn("Listing feature flags environment variables:")
              .unwrap();
            for (feature, env_var) in ENV_VARS.iter() {
                ui.warn(&format!("  * {:?}: {}={:?}",
                                 feature,
                                 env_var,
                                 env::var_os(env_var).unwrap_or_default()))
                  .unwrap();
            }
        }

        flags
    }
}
