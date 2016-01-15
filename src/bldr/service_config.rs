// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! Stores the configuration data for the service, and manages binding that data to any
//! configuration files we need to render.
//!
//! Components:
//!
//! * ServiceConfigItem: A single component of the configuration data (defaults, from the
//! environment, etc.)
//! * ServiceConfig: A container holding all the ServiceConfigItems.

use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hasher;

use toml;
use mustache;
use fnv::FnvHasher;
use ansi_term::Colour::Purple;

use util;
use util::convert;
use error::{BldrResult, ErrorKind};
use package::Package;

static LOGKEY: &'static str = "SC";

/// A single component of the configuration (such as defaults)
#[derive(Debug, Clone, Eq, PartialEq)]
struct ServiceConfigItem {
    /// The current toml data
    pub toml_string: String,
    /// Whether this item has been updated, but not written
    pub updated: bool,
}

impl ServiceConfigItem {
    /// Create a new ServiceConfigItem, with a backing string
    pub fn new(toml_string: String) -> ServiceConfigItem {
        ServiceConfigItem {
            toml_string: toml_string,
            updated: true,
        }
    }

    /// Set the current string, and set our udpated flag to true.
    pub fn set(&mut self, toml_string: String) {
        if self.toml_string != toml_string {
            self.updated = true;
            self.toml_string = toml_string;
        }
    }

    /// Set the updated flag to false; we have been written!
    pub fn written(&mut self) {
        self.updated = false;
    }

    /// Retrieve a reference to the current config string
    pub fn get(&self) -> &str {
        &self.toml_string
    }
}

/// The global ServiceConfig. Contains an entry for each type of configuration we track, and a hash
/// of the latest FNV value of any configuration files we render.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ServiceConfig {
    default: ServiceConfigItem,
    user: ServiceConfigItem,
    environment: ServiceConfigItem,
    sys: ServiceConfigItem,
    bldr: ServiceConfigItem,
    census: ServiceConfigItem,
    watch: ServiceConfigItem,
    config_fnv: HashMap<String, u64>,
}

impl ServiceConfig {
    /// Create a new ServiceConfig, based on the package we are running.
    ///
    /// # Failures
    ///
    /// * If we cannot generate the system configuration (IP address, hostname, etc.)
    pub fn new(pkg: &Package) -> BldrResult<ServiceConfig> {
        let sys = try!(util::sys::to_toml());
        let env = match env::var(&format!("BLDR_{}", pkg.name)) {
            Ok(val) => val,
            Err(e) => {
                debug!("Looking up environment variable BLDR_{} failed: {:?}",
                       pkg.name,
                       e);
                String::new()
            }
        };
        Ok(ServiceConfig {
            default: ServiceConfigItem::new(read_default_toml_file(pkg).unwrap_or(String::new())),
            user: ServiceConfigItem::new(String::new()),
            environment: ServiceConfigItem::new(env),
            sys: ServiceConfigItem::new(sys),
            bldr: ServiceConfigItem::new(bldr_data(pkg)),
            census: ServiceConfigItem::new(String::new()),
            watch: ServiceConfigItem::new(String::new()),
            config_fnv: HashMap::new(),
        })
    }

    /// Set the default configuration string
    pub fn default(&mut self, toml_string: String) {
        self.default.set(toml_string);
    }

    /// Set the user configuration string
    pub fn user(&mut self, toml_string: String) {
        self.user.set(toml_string);
    }

    /// Set the environment configuration string
    pub fn environment(&mut self, toml_string: String) {
        self.environment.set(toml_string);
    }

    /// Set the sys configuration string
    pub fn sys(&mut self, toml_string: String) {
        self.sys.set(toml_string);
    }

    /// Set the bldr configuration string
    pub fn bldr(&mut self, toml_string: String) {
        self.bldr.set(toml_string);
    }

    /// Set the census configuration string
    pub fn census(&mut self, toml_string: String) {
        self.census.set(toml_string);
    }

    /// Set the watch configuration string
    pub fn watch(&mut self, toml_string: String) {
        self.watch.set(toml_string);
    }

    /// Returns true if any of the backing configuration strings have been updated
    pub fn is_updated(&mut self) -> bool {
        let order = [&self.default,
                     &self.user,
                     &self.census,
                     &self.watch,
                     &self.sys,
                     &self.bldr,
                     &self.environment];
        // If nothing has been updated, nothing needs to be written either - just return!
        order.into_iter().any(|&cfg| cfg.updated)
    }

    /// Compiles our final toml data, based on walking and merging all the various configuration
    /// types. Merges in this order
    ///
    /// * default
    /// * user
    /// * census
    /// * watch
    /// * sys
    /// * bldr
    /// * environment
    ///
    /// # Failures
    ///
    /// * If we cannot parse some toml
    /// * If there is no configuration at all
    pub fn compile_toml(&self) -> BldrResult<BTreeMap<String, toml::Value>> {
        let order = [&self.default,
                     &self.user,
                     &self.census,
                     &self.watch,
                     &self.sys,
                     &self.bldr,
                     &self.environment];

        let mut base_toml: Option<BTreeMap<String, toml::Value>> = None;

        for cfg in order.into_iter() {
            let mut toml_parser = toml::Parser::new(cfg.get());
            let toml_value = try!(toml_parser.parse()
                                .ok_or(bldr_error!(ErrorKind::TomlParser(toml_parser.errors))));
            if let Some(base) = base_toml {
                base_toml = Some(toml_merge(base, toml_value));
            } else {
                base_toml = Some(toml_value);
            }
        }

        let final_toml = match base_toml {
            Some(toml) => toml,
            None => return Err(bldr_error!(ErrorKind::NoConfiguration)),
        };
        Ok(final_toml)
    }

    /// Write the configuration to disk.
    ///
    /// Returns true if the service should restart - only happens if the final, rendered
    /// configuration files have changed.
    ///
    /// Does nothing if nothing needs to be written (if no data has changed).
    ///
    /// * Compiles all the toml
    /// * Writes to config.toml
    /// * Creates the mustache data structure
    /// * Renders each configuration file template
    /// * Clears the updated bit on every ServiceConfigEntry.
    ///
    /// # Failures
    ///
    /// * We cannot compile the toml
    /// * We cannot write config.toml
    /// * We cannot get a list of configuration files from the package
    /// * We cannot write the configuration files
    pub fn write(&mut self, pkg: &Package) -> BldrResult<bool> {
        // If nothing is updated, do not write, and do not restart
        if !self.is_updated() {
            return Ok(false);
        }
        let final_toml = try!(self.compile_toml());
        // RAII will close the file when this scope ends
        {
            let mut last_toml = try!(File::create(pkg.srvc_join_path("config.toml")));
            try!(write!(&mut last_toml, "{}", toml::encode_str(&final_toml)));
        }

        let final_data = convert::toml_table_to_mustache(final_toml);

        let mut should_restart = false;

        let config_files = try!(pkg.config_files());
        for config in config_files {
            let template = try!(mustache::compile_path(pkg.join_path(&format!("config/{}",
                                                                              config))));
            let mut config_vec = Vec::new();
            let filename = pkg.srvc_join_path(&format!("config/{}", config));
            template.render_data(&mut config_vec, &final_data);
            let mut hasher = FnvHasher::default();
            hasher.write(&mut config_vec);
            let new_file_fnv = hasher.finish();
            if self.config_fnv.contains_key(&filename) {
                let last_config_fnv = self.config_fnv.get(&filename).unwrap().clone();
                if new_file_fnv == last_config_fnv {
                    debug!("Configuration {} has not changed; not restarting", filename);
                    continue;
                } else {
                    debug!("Configuration {} has changed; restarting", filename);
                    outputln!("Updated {}", Purple.bold().paint(config));
                    self.config_fnv.insert(filename.clone(), new_file_fnv);
                    let mut config_file = try!(File::create(&filename));
                    try!(config_file.write_all(&config_vec));
                    should_restart = true;
                }
            } else {
                debug!("Configuration {} does not exist; restarting", filename);
                outputln!("Updated {}", Purple.bold().paint(config));
                self.config_fnv.insert(filename.clone(), new_file_fnv);
                let mut config_file = try!(File::create(&filename));
                try!(config_file.write_all(&config_vec));
                should_restart = true
            }
        }

        self.default.written();
        self.user.written();
        self.census.written();
        self.watch.written();
        self.sys.written();
        self.bldr.written();
        self.environment.written();

        if pkg.supervisor_running() {
            Ok(should_restart)
        } else {
            // If the supervisor isn't running yet, we don't have to worry about
            // restarting it, obviously
            Ok(false)
        }
    }
}

// Shallow merges two toml tables.
fn toml_merge(left: BTreeMap<String, toml::Value>,
              right: BTreeMap<String, toml::Value>)
              -> BTreeMap<String, toml::Value> {
    let mut final_map = BTreeMap::new();
    for (left_key, left_value) in left.iter() {
        match right.get(left_key) {
            Some(right_value) => {
                final_map.insert(left_key.clone(), right_value.clone());
            }
            None => {
                final_map.insert(left_key.clone(), left_value.clone());
            }
        }
    }
    for (right_key, right_value) in right.iter() {
        if !final_map.contains_key(right_key) {
            final_map.insert(right_key.clone(), right_value.clone());
        }
    }
    final_map
}

// Reads the default.toml file in, and returns the string.
//
// # Failures
//
// * If we cannot read the file
fn read_default_toml_file(pkg: &Package) -> BldrResult<String> {
    let mut file = try!(File::open(pkg.join_path("default.toml")));
    let mut config = String::new();
    try!(file.read_to_string(&mut config));
    Ok(config)
}

// Generates the toml for the bldr data from a package.
fn bldr_data(pkg: &Package) -> String {
    let mut toml_string = String::from("[bldr]\n");
    toml_string.push_str(&format!("derivation = \"{}\"", pkg.derivation));
    toml_string.push_str(&format!("name = \"{}\"", pkg.name));
    toml_string.push_str(&format!("version = \"{}\"", pkg.version));
    toml_string.push_str(&format!("release = \"{}\"", pkg.release));
    let expose_string = String::new();
    toml_string.push_str(&format!("expose = [{}]",
                                  pkg.exposes()
                                     .iter()
                                     .fold(expose_string, |acc, p| format!("{}{},", acc, p))));
    toml_string
}
