//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hasher;

use toml;
use mustache;
use fnv::FnvHasher;
use ansi_term::Colour::{Purple, White};

use util;
use error::{BldrError, BldrResult};
use pkg::Package;

#[derive(Debug, Clone, Eq, PartialEq)]
struct ServiceConfigItem {
    pub toml_string: String,
    pub updated: bool
}

impl ServiceConfigItem {
    pub fn new(toml_string: String) -> ServiceConfigItem {
        ServiceConfigItem{
            toml_string: toml_string,
            updated: true
        }
    }

    pub fn set(&mut self, toml_string: String) {
        if self.toml_string != toml_string {
            self.updated = true;
            self.toml_string = toml_string;
        }
    }

    pub fn written(&mut self) {
        self.updated = false;
    }

    pub fn get(&self) -> &str {
        &self.toml_string
    }
}

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
    pub fn new(pkg: &Package) -> BldrResult<ServiceConfig> {
        let sys = try!(util::sys::to_toml());
        let env = match env::var(&format!("BLDR_{}", pkg.name)) {
            Ok(val) => val,
            Err(e) => {
                debug!("Looking up environment variable BLDR_{} failed: {:?}", pkg.name, e);
                String::new()
            }
        };
        Ok(ServiceConfig{
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

    pub fn default(&mut self, toml_string: String) {
        self.default.set(toml_string);
    }

    pub fn user(&mut self, toml_string: String) {
        self.user.set(toml_string);
    }

    pub fn environment(&mut self, toml_string: String) {
        self.environment.set(toml_string);
    }

    pub fn sys(&mut self, toml_string: String) {
        self.sys.set(toml_string);
    }

    pub fn bldr(&mut self, toml_string: String) {
        self.bldr.set(toml_string);
    }

    pub fn census(&mut self, toml_string: String) {
        self.census.set(toml_string);
    }

    pub fn watch(&mut self, toml_string: String) {
        self.watch.set(toml_string);
    }

    pub fn is_updated(&mut self) -> bool {
        let order = [&self.default, &self.user, &self.census, &self.watch, &self.sys, &self.bldr, &self.environment];
        // If nothing has been updated, nothing needs to be written either - just return!
        order.into_iter().any(|&cfg| cfg.updated)
    }

    pub fn compile_toml(&mut self) -> BldrResult<BTreeMap<String, toml::Value>> {
        let order = [&self.default, &self.user, &self.census, &self.watch, &self.sys, &self.bldr, &self.environment];

        let mut base_toml: Option<BTreeMap<String, toml::Value>> = None;

        for cfg in order.into_iter() {
            let mut toml_parser = toml::Parser::new(cfg.get());
            let toml_value = try!(toml_parser.parse().ok_or(BldrError::TomlParser(toml_parser.errors)));
            if let Some(base) = base_toml {
                base_toml = Some(toml_merge(base, toml_value));
            } else {
                base_toml = Some(toml_value);
            }
        }

        let final_toml = match base_toml {
            Some(toml) => toml,
            None => return Err(BldrError::NoConfiguration)
        };
        Ok(final_toml)
    }

    pub fn write(&mut self, pkg: &Package) -> BldrResult<bool> {
        // If nothing is updated, do not write, and do not restart
        if ! self.is_updated() {
            return Ok(false);
        }
        let final_toml = try!(self.compile_toml());
        // RAII will close the file when this scope ends
        {
            let mut last_toml = try!(File::create(pkg.srvc_join_path("config.toml")));
            try!(write!(&mut last_toml, "{}", toml::encode_str(&final_toml)));
        }

        let final_data = toml_table_to_mustache(final_toml);

        let mut should_restart = false;

        let config_files = try!(pkg.config_files());
        for config in config_files {
            let template = try!(mustache::compile_path(pkg.join_path(&format!("config/{}", config))));
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
                    println!("   {}({}): Updated {}", pkg.name, White.bold().paint("C"), Purple.bold().paint(&config));
                    self.config_fnv.insert(filename.clone(), new_file_fnv);
                    let mut config_file = try!(File::create(&filename));
                    try!(config_file.write_all(&config_vec));
                    should_restart = true;
                }
            } else {
                debug!("Configuration {} does not exist; restarting", filename);
                println!("   {}({}): Updated {}", pkg.name, White.bold().paint("C"), Purple.bold().paint(&config));
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

fn toml_merge(left: BTreeMap<String, toml::Value>, right: BTreeMap<String, toml::Value>) -> BTreeMap<String, toml::Value> {
    let mut final_map = BTreeMap::new();
    for (left_key, left_value) in left.iter() {
        match right.get(left_key) {
            Some(right_value) => { final_map.insert(left_key.clone(), right_value.clone()); },
            None => { final_map.insert(left_key.clone(), left_value.clone()); },
        }
    }
    for (right_key, right_value) in right.iter() {
        if ! final_map.contains_key(right_key) {
            final_map.insert(right_key.clone(), right_value.clone());
        }
    }
    final_map
}

fn toml_table_to_mustache(toml: BTreeMap<String, toml::Value>) -> mustache::Data {
    let mut hashmap = HashMap::new();
    for (key, value) in toml.iter() {
        hashmap.insert(format!("{}", key), toml_to_mustache(value.clone()));
    }
    mustache::Data::Map(hashmap)
}

fn toml_to_mustache(value: toml::Value) -> mustache::Data {
    match value {
        toml::Value::String(s) => mustache::Data::StrVal(format!("{}", s)),
        toml::Value::Integer(i) => mustache::Data::StrVal(format!("{}", i)),
        toml::Value::Float(i) => mustache::Data::StrVal(format!("{}", i)),
        toml::Value::Boolean(b) => mustache::Data::Bool(b),
        toml::Value::Datetime(s) => mustache::Data::StrVal(format!("{}", s)),
        toml::Value::Array(a) => toml_vec_to_mustache(a),
        toml::Value::Table(t) => toml_table_to_mustache(t),
    }
}

fn toml_vec_to_mustache(toml: Vec<toml::Value>) -> mustache::Data {
    let mut mvec = vec![];
    for x in toml.iter() {
        mvec.push(toml_to_mustache(x.clone()))
    }
    mustache::Data::VecVal(mvec)
}

fn read_default_toml_file(pkg: &Package) -> BldrResult<String> {
    let mut file = try!(File::open(pkg.join_path("default.toml")));
    let mut config = String::new();
    try!(file.read_to_string(&mut config));
    Ok(config)
}

fn bldr_data(pkg: &Package) -> String {
    let mut toml_string = String::from("[bldr]\n");
    toml_string.push_str(&format!("derivation = \"{}\"", pkg.derivation));
    toml_string.push_str(&format!("name = \"{}\"", pkg.name));
    toml_string.push_str(&format!("version = \"{}\"", pkg.version));
    toml_string.push_str(&format!("release = \"{}\"", pkg.release));
    let expose_string = String::new();
    toml_string.push_str(&format!("expose = [{}]", pkg.exposes().iter().fold(expose_string, |acc, p| format!("{}{},", acc, p))));
    toml_string
}

#[cfg(test)]
mod tests {
    use super::toml_table_to_mustache;
    use toml;
    use mustache;

    #[test]
    fn toml_data_is_rendered_to_mustache() {
        let toml = r#"
                daemonize = "no"
                slaveof = "127.0.0.1 6380"

                [winks]
                left = "yes"
                right = "no"
                wiggle = [ "snooze", "looze" ]
            "#;
        let toml_value = toml::Parser::new(toml).parse().unwrap();
        let template = mustache::compile_str("hello {{daemonize}} for {{slaveof}} {{winks.right}} {{winks.left}} {{# winks.wiggle}} {{.}} {{/winks.wiggle}}");
        let mut bytes = vec![];
        let data = toml_table_to_mustache(toml_value);
        template.render_data(&mut bytes, &data);
        assert_eq!(String::from_utf8(bytes).unwrap(), "hello no for 127.0.0.1 6380 no yes  snooze  looze ".to_string());
    }
}
