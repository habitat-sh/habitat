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

/// Collect all the configuration data that is exposed to users, and render it.

use std;
use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

use ansi_term::Colour::Purple;
use butterfly::rumor::service::SysInfo;
use hcore::crypto;
use hcore::package::{PackageIdent, PackageInstall};
use hcore::service::ServiceGroup;
use toml;

use config::GossipListenAddr;
use manager::census::{Census, CensusList};
use error::{Error, Result};
use fs;
use http_gateway;
use supervisor::RuntimeConfig;
use templating::Template;
use util::{self, convert};
use VERSION;

static LOGKEY: &'static str = "SC";
static ENV_VAR_PREFIX: &'static str = "HAB";
/// The maximum TOML table merge depth allowed before failing the operation. The value here is
/// somewhat arbitrary (stack size cannot be easily computed beforehand and different libc
/// implementations will impose different size constraints), however a parallel data structure that
/// is deeper than this value crosses into overly complex territory when describing configuration
/// for a single service.
static TOML_MAX_MERGE_DEPTH: u16 = 30;

/// The top level struct for all our configuration - this corresponds to the top level
/// namespaces available in `config.toml`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServiceConfig {
    pub hab: Hab,
    pub pkg: Pkg,
    pub sys: Sys,
    pub cfg: Cfg,
    pub svc: Svc,
    pub bind: Bind,
    #[serde(skip_serializing, skip_deserializing, default="default_for_pathbuf")]
    pub config_root: PathBuf,
    #[serde(skip_serializing, skip_deserializing)]
    pub incarnation: u64,
    // Set to 'true' if we have data that needs to be sent to a configuration file
    #[serde(skip_serializing, skip_deserializing)]
    pub needs_write: bool,
    #[serde(skip_serializing, skip_deserializing)]
    supported_bindings: Vec<(String, ServiceGroup)>,
}

fn default_for_pathbuf() -> PathBuf {
    PathBuf::new()
}

impl ServiceConfig {
    /// Takes a new package and a new census list, and returns a ServiceConfig. This function can
    /// fail, and indeed, we want it to - it causes the program to crash if we can not render the
    /// first pass of the configuration file.
    pub fn new(package: &PackageInstall,
               runtime_cfg: &RuntimeConfig,
               config_root: PathBuf,
               bindings: Vec<(String, ServiceGroup)>,
               gossip_listen: &GossipListenAddr,
               http_listen: &http_gateway::ListenAddr)
               -> Result<ServiceConfig> {
        Ok(ServiceConfig {
            pkg: Pkg::new(package, runtime_cfg)?,
            hab: Hab::new(),
            sys: Sys::new(gossip_listen, http_listen),
            cfg: Cfg::new(package, &config_root)?,
            svc: Svc::default(),
            bind: Bind::default(),
            incarnation: 0,
            needs_write: true,
            supported_bindings: bindings,
            config_root: config_root,
        })
    }

    /// Return an iterator of the configuration file names to render.
    ///
    /// This does not return the full path, for convenience with the path
    /// helpers above.
    fn config_files<T: AsRef<Path> + fmt::Debug>(config_path: T) -> Result<Vec<String>> {
        let mut files: Vec<String> = Vec::new();
        debug!("Loading configuration from {:?}", config_path);
        match std::fs::read_dir(config_path) {
            Ok(config_path) => {
                for config in config_path {
                    let config = try!(config);
                    match config.path().file_name() {
                        Some(filename) => {
                            debug!("Looking in {:?}", filename);
                            files.push(filename.to_string_lossy().into_owned().to_string());
                        }
                        None => unreachable!(),
                    }
                }
            }
            Err(e) => {
                debug!("No config directory in package: {}", e);
            }
        }
        Ok(files)
    }

    /// Render this struct as toml.
    pub fn to_toml(&self) -> Result<toml::Value> {
        let mut top = toml::value::Table::new();

        let hab = try!(self.hab.to_toml());
        top.insert(String::from("hab"), hab);

        let pkg = try!(self.pkg.to_toml());
        top.insert(String::from("pkg"), pkg);

        let sys = try!(self.sys.to_toml());
        top.insert(String::from("sys"), sys);

        let cfg = try!(self.cfg.to_toml());
        top.insert(String::from("cfg"), cfg);

        let svc = self.svc.to_toml();
        top.insert(String::from("svc"), svc);

        let bind = self.bind.to_toml();
        top.insert(String::from("bind"), bind);

        Ok(toml::Value::Table(top))
    }

    pub fn to_exported(&self) -> Result<toml::value::Table> {
        self.cfg.to_exported(&self.pkg.exports)
    }

    pub fn populate(&mut self, service_group: &ServiceGroup, census_list: &CensusList) {
        self.bind.populate(&self.supported_bindings, census_list);
        self.svc.populate(service_group, census_list);
    }

    pub fn reload_gossip(&mut self) -> Result<()> {
        self.cfg.load_gossip(&self.pkg.name)
    }

    /// Write the configuration to `config.toml`, and render the templated configuration files.
    pub fn write(&mut self) -> Result<bool> {
        let final_toml = try!(self.to_toml());
        {
            let mut last_toml = try!(File::create(self.pkg.svc_path.join("config.toml")));
            try!(last_toml.write_all(&try!(toml::to_vec(&final_toml))));
        }
        let mut template = Template::new();

        // Register all the templates; this makes them available as partials!
        // I suspect this will be useful, but I think we'll want to make this
        // more explicit... in a minute, we render all the config files anyway.
        let config_path = self.config_root.join("config");
        let config_files = try!(Self::config_files(&config_path));
        for config in config_files.iter() {
            let path = config_path.join(config);
            debug!("Config template {} from {:?}", config, &path);
            if let Err(e) = template.register_template_file(config, &path) {
                outputln!("Error parsing config template file {}: {}",
                          path.to_string_lossy(),
                          e);
                return Err(sup_error!(Error::TemplateFileError(e)));
            }
        }

        let final_data = convert::toml_to_json(final_toml);
        let mut should_restart = false;
        for config in config_files {
            debug!("Rendering template {}", &config);
            let template_data = try!(template.render(&config, &final_data));
            let template_hash = try!(crypto::hash::hash_string(&template_data));
            let cfg_dest = self.pkg.svc_config_path.join(&config).to_string_lossy().into_owned();
            let file_hash = match crypto::hash::hash_file(&cfg_dest) {
                Ok(file_hash) => file_hash,
                Err(e) => {
                    debug!("Cannot read the file in order to hash it: {}", e);
                    String::new()
                }
            };
            if file_hash.is_empty() {
                debug!("Configuration {} does not exist; restarting", cfg_dest);
                outputln!("Updated {} {}", Purple.bold().paint(config), template_hash);
                let mut config_file = try!(File::create(&cfg_dest));
                try!(config_file.write_all(&template_data.into_bytes()));
                should_restart = true
            } else {
                if file_hash == template_hash {
                    debug!("Configuration {} {} has not changed; not restarting.",
                           cfg_dest,
                           file_hash);
                    continue;
                } else {
                    debug!("Configuration {} has changed; restarting", cfg_dest);
                    outputln!("Updated {} {}", Purple.bold().paint(config), template_hash);
                    let mut config_file = try!(File::create(&cfg_dest));
                    try!(config_file.write_all(&template_data.into_bytes()));
                    should_restart = true;
                }
            }
        }
        self.needs_write = false;
        Ok(should_restart)
    }

    pub fn reload_package(&mut self,
                          package: &PackageInstall,
                          config_root: PathBuf,
                          runtime: &RuntimeConfig)
                          -> Result<()> {
        self.config_root = config_root;
        self.pkg = Pkg::new(package, runtime)?;
        self.cfg = Cfg::new(package, &self.config_root)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Bind(toml::value::Table);

impl Bind {
    fn populate(&mut self, bindings: &[(String, ServiceGroup)], census_list: &CensusList) {
        self.0.clear();
        for &(ref bind, ref service_group) in bindings.iter() {
            match census_list.get(service_group) {
                Some(census) => {
                    self.0.insert(format!("has_{}", bind), toml::Value::Boolean(true));
                    self.0.insert(bind.to_string(), toml::Value::Table(service_entry(census)));
                }
                None => {
                    self.0.insert(format!("has_{}", bind), toml::Value::Boolean(false));
                }
            }
        }
    }

    fn to_toml(&self) -> toml::Value {
        toml::Value::Table(self.0.clone())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Svc(toml::value::Table);

impl Svc {
    pub fn populate(&mut self, service_group: &ServiceGroup, census_list: &CensusList) {
        let mut top = service_entry(census_list.get(&*service_group)
            .expect("Service Group's census entry missing from list!"));
        let mut all: Vec<toml::Value> = Vec::new();
        let mut named = toml::value::Table::new();
        for (_sg, c) in census_list.iter() {
            all.push(toml::Value::Table(service_entry(c)));
            let mut group = if named.contains_key(c.get_service()) {
                named.get(c.get_service()).unwrap().as_table().unwrap().clone()
            } else {
                toml::value::Table::new()
            };
            group.insert(String::from(c.get_group()),
                         toml::Value::Table(service_entry(c)));
            named.insert(String::from(c.get_service()), toml::Value::Table(group));
        }
        top.insert("all".to_string(), toml::Value::Array(all));
        top.insert("named".to_string(), toml::Value::Table(named));
        self.0 = top;
    }

    fn to_toml(&self) -> toml::Value {
        toml::Value::Table(self.0.clone())
    }
}

// TODO FN: The newer toml crate API return a `Result` when converting (as it always should have)
// which begs the question: how should we handle conversion failures in this function? We currently
// don't return a `Result<toml::value::Table>`--maybe we should? Remember--`.expect()` is a panic
// by a nicer name ;)
fn service_entry(census: &Census) -> toml::value::Table {
    let service = toml::Value::String(String::from(census.get_service()));
    let group = toml::Value::String(String::from(census.get_group()));
    let ident = toml::Value::String(census.get_service_group());
    let leader = census.get_leader()
        .map(|ce| toml::Value::try_from(ce).expect("Can't convert into TOML Value"));
    let mut members: Vec<toml::Value> = Vec::new();
    let mut member_id = toml::value::Table::new();
    for (sg, ce) in census.iter() {
        members.push(toml::Value::try_from(ce).expect("Can't convert into TOML Value"));
        member_id.insert(format!("{}", sg),
                         toml::Value::try_from(ce).expect("Can't convert into TOML Value"));
    }
    let mut result = toml::value::Table::new();
    result.insert("service".to_string(), service);
    result.insert("group".to_string(), group);
    result.insert("ident".to_string(), ident);
    if let Some(me) = census.me() {
        let toml_me = toml::Value::try_from(me).expect("Can't convert into TOML Value");
        result.insert("me".to_string(), toml_me);
    }
    if let Some(l) = leader {
        result.insert("leader".to_string(), l);
    }
    result.insert("members".to_string(), toml::Value::Array(members));
    result.insert("member_id".to_string(), toml::Value::Table(member_id));
    result
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cfg {
    pub default: Option<toml::Value>,
    pub user: Option<toml::Value>,
    pub gossip: Option<toml::Value>,
    pub environment: Option<toml::Value>,
}

impl Cfg {
    fn new<T: AsRef<Path>>(package: &PackageInstall, config_root: T) -> Result<Cfg> {
        let mut cfg = Cfg {
            default: None,
            user: None,
            gossip: None,
            environment: None,
        };
        try!(cfg.load_default(&config_root));
        try!(cfg.load_user(&package.ident.name));
        try!(cfg.load_gossip(&package.ident.name));
        try!(cfg.load_environment(&package.ident.name));
        Ok(cfg)
    }

    pub fn to_toml(&self) -> Result<toml::Value> {
        let mut output_toml = toml::value::Table::new();
        if let Some(toml::Value::Table(ref default_cfg)) = self.default {
            try!(toml_merge(&mut output_toml, default_cfg));
        }
        if let Some(toml::Value::Table(ref env_cfg)) = self.environment {
            try!(toml_merge(&mut output_toml, env_cfg));
        }
        if let Some(toml::Value::Table(ref user_cfg)) = self.user {
            try!(toml_merge(&mut output_toml, user_cfg));
        }
        if let Some(toml::Value::Table(ref gossip_cfg)) = self.gossip {
            try!(toml_merge(&mut output_toml, gossip_cfg));
        }
        Ok(toml::Value::Table(output_toml))
    }

    fn to_exported(&self, exports: &HashMap<String, String>) -> Result<toml::value::Table> {
        let mut map = toml::value::Table::default();
        let cfg = try!(self.to_toml());
        for (key, path) in exports.iter() {
            // JW TODO: the TOML library only provides us with a
            // function to retrieve a value with a path which returns a
            // reference. We actually want the value for ourselves.
            // Let's improve this later to avoid allocating twice.
            if let Some(val) = cfg.get(&path) {
                map.insert(key.clone(), val.clone());
            }
        }
        Ok(map)
    }

    fn load_default<T: AsRef<Path>>(&mut self, config_root: T) -> Result<()> {
        let mut file = match File::open(config_root.as_ref().join("default.toml")) {
            Ok(file) => file,
            Err(e) => {
                debug!("Failed to open default.toml: {}", e);
                self.default = None;
                return Ok(());
            }
        };
        let mut config = String::new();
        match file.read_to_string(&mut config) {
            Ok(_) => {
                let toml = try!(toml::de::from_str(&config)
                    .map_err(|e| sup_error!(Error::TomlParser(e))));
                self.default = Some(toml::Value::Table(toml));
            }
            Err(e) => {
                outputln!("Failed to read default.toml: {}", e);
                self.default = None;
            }
        }
        Ok(())
    }

    fn load_user(&mut self, package: &str) -> Result<()> {
        let mut file = match File::open(fs::svc_path(package).join("user.toml")) {
            Ok(file) => file,
            Err(e) => {
                debug!("Failed to open user.toml: {}", e);
                self.user = None;
                return Ok(());
            }
        };
        let mut config = String::new();
        match file.read_to_string(&mut config) {
            Ok(_) => {
                let toml = try!(toml::de::from_str(&config)
                    .map_err(|e| sup_error!(Error::TomlParser(e))));
                self.user = Some(toml::Value::Table(toml));
            }
            Err(e) => {
                outputln!("Failed to load user.toml: {}", e);
                self.user = None;
            }
        }
        Ok(())
    }

    fn load_gossip(&mut self, package: &str) -> Result<()> {
        let mut file = match File::open(fs::svc_path(package).join("gossip.toml")) {
            Ok(file) => file,
            Err(e) => {
                debug!("Failed to open gossip.toml: {}", e);
                self.gossip = None;
                return Ok(());
            }
        };
        let mut config = String::new();
        match file.read_to_string(&mut config) {
            Ok(_) => {
                let toml = try!(toml::de::from_str(&config)
                    .map_err(|e| sup_error!(Error::TomlParser(e))));
                self.gossip = Some(toml::Value::Table(toml));
            }
            Err(e) => {
                outputln!("Failed to load gossip.toml: {}", e);
                self.gossip = None;
            }
        }
        Ok(())
    }

    fn load_environment(&mut self, package: &str) -> Result<()> {
        let var_name = format!("{}_{}", ENV_VAR_PREFIX, package)
            .to_ascii_uppercase()
            .replace("-", "_");
        match env::var(&var_name) {
            Ok(config) => {
                let toml = try!(toml::de::from_str(&config)
                    .map_err(|e| sup_error!(Error::TomlParser(e))));
                self.environment = Some(toml::Value::Table(toml));

            }
            Err(e) => {
                debug!("Looking up environment variable {} failed: {:?}",
                       var_name,
                       e);
                self.environment = None;
            }
        };
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pkg {
    pub origin: String,
    pub name: String,
    pub version: String,
    pub release: String,
    pub ident: String,
    pub deps: Vec<PackageIdent>,
    pub exposes: Vec<String>,
    pub exports: HashMap<String, String>,
    pub path: PathBuf,
    pub svc_path: PathBuf,
    pub svc_config_path: PathBuf,
    pub svc_data_path: PathBuf,
    pub svc_files_path: PathBuf,
    pub svc_static_path: PathBuf,
    pub svc_var_path: PathBuf,
    pub svc_user: String,
    pub svc_group: String,
}

impl Pkg {
    fn new(package: &PackageInstall, runtime: &RuntimeConfig) -> Result<Pkg> {
        let ident = package.ident().clone();
        Ok(Pkg {
            ident: ident.to_string(),
            origin: ident.origin,
            name: ident.name,
            version: ident.version.expect("Couldn't read package version"),
            release: ident.release.expect("Couldn't read package release"),
            deps: package.tdeps()?,
            exposes: package.exposes()?,
            exports: package.exports()?,
            path: package.installed_path.clone(),
            svc_path: fs::svc_path(&package.ident.name),
            svc_config_path: fs::svc_config_path(&package.ident.name),
            svc_data_path: fs::svc_data_path(&package.ident.name),
            svc_files_path: fs::svc_files_path(&package.ident.name),
            svc_static_path: fs::svc_static_path(&package.ident.name),
            svc_var_path: fs::svc_var_path(&package.ident.name),
            svc_user: runtime.svc_user.to_string(),
            svc_group: runtime.svc_group.to_string(),
        })
    }

    fn to_toml(&self) -> Result<toml::Value> {
        Ok(try!(toml::Value::try_from(&self)))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sys(SysInfo);

impl Sys {
    fn new(gossip_listen: &GossipListenAddr, http_listen: &http_gateway::ListenAddr) -> Sys {
        let ip = match util::sys::ip() {
            Ok(ip) => ip.to_string(),
            Err(e) => {
                outputln!("IP Address lookup failed; using fallback of 127.0.0.1 ({})",
                          e);
                String::from("127.0.0.1")
            }
        };
        let hostname = match util::sys::hostname() {
            Ok(ip) => ip,
            Err(e) => {
                outputln!("Hostname lookup failed; using fallback of localhost ({})",
                          e);
                String::from("localhost")
            }
        };
        Sys(SysInfo {
            ip: ip,
            hostname: hostname,
            gossip_ip: gossip_listen.ip().to_string(),
            gossip_port: gossip_listen.port().to_string(),
            http_gateway_ip: http_listen.ip().to_string(),
            http_gateway_port: http_listen.port().to_string(),
        })
    }

    fn to_toml(&self) -> Result<toml::Value> {
        Ok(try!(toml::Value::try_from(&self)))
    }
}

impl Deref for Sys {
    type Target = SysInfo;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Sys {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Hab {
    pub version: String,
}

impl Hab {
    fn new() -> Self {
        Hab { version: VERSION.to_string() }
    }

    fn to_toml(&self) -> Result<toml::Value> {
        Ok(try!(toml::Value::try_from(&self)))
    }
}


// Recursively merges the `other` TOML table into `me`
pub fn toml_merge(me: &mut toml::value::Table, other: &toml::value::Table) -> Result<()> {
    toml_merge_recurse(me, other, 0)
}

fn toml_merge_recurse(me: &mut toml::value::Table,
                      other: &toml::value::Table,
                      depth: u16)
                      -> Result<()> {
    if depth > TOML_MAX_MERGE_DEPTH {
        return Err(sup_error!(Error::TomlMergeError(format!("Max recursive merge depth of {} \
                                                             exceeded.",
                                                            TOML_MAX_MERGE_DEPTH))));
    }

    for (key, other_value) in other.iter() {
        if is_toml_value_a_table(key, me) && is_toml_value_a_table(key, other) {
            let mut me_at_key = match *(me.get_mut(key).expect("Key should exist in Table")) {
                toml::Value::Table(ref mut t) => t,
                _ => {
                    return Err(sup_error!(Error::TomlMergeError(format!("Value at key {} \
                                                                         should be a Table",
                                                                        &key))));
                }
            };
            try!(toml_merge_recurse(&mut me_at_key,
                                    other_value.as_table().expect("TOML Value should be a Table"),
                                    depth + 1));
        } else {
            me.insert(key.clone(), other_value.clone());
        }
    }
    Ok(())
}

fn is_toml_value_a_table(key: &str, table: &toml::value::Table) -> bool {
    match table.get(key) {
        None => return false,
        Some(value) => {
            match value.as_table() {
                Some(_) => return true,
                None => return false,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use std::str::FromStr;

    use hcore::package::{PackageIdent, PackageInstall};
    use regex::Regex;
    use toml;

    use super::*;
    use config::GossipListenAddr;
    use error::Error;
    use http_gateway::ListenAddr;
    use supervisor::RuntimeConfig;
    use VERSION;

    fn gen_pkg() -> PackageInstall {
        PackageInstall::new_from_parts(PackageIdent::from_str("neurosis/redis/2000/20160222201258")
                                           .unwrap(),
                                       PathBuf::from("/"),
                                       PathBuf::from("/fakeo"),
                                       PathBuf::from("/fakeo/here"))
    }

    fn toml_from_str(content: &str) -> toml::value::Table {
        toml::from_str(content).expect(&format!("Content should parse as TOML: {}", content))
    }

    #[test]
    fn to_toml_hab() {
        let pkg = gen_pkg();
        let sc = ServiceConfig::new(&pkg,
                                    &RuntimeConfig::new("hab".to_string(), "hab".to_string()),
                                    PathBuf::from("/hab/pkgs/neurosis/redis/2000/20160222201258"),
                                    Vec::new(),
                                    &GossipListenAddr::default(),
                                    &ListenAddr::default())
            .unwrap();
        let toml = sc.to_toml().unwrap();
        let version =
            toml.get("hab").unwrap().as_table().unwrap().get("version").unwrap().as_str().unwrap();
        assert_eq!(version, VERSION);
    }

    #[test]
    fn to_toml_pkg() {
        let pkg = gen_pkg();
        let sc = ServiceConfig::new(&pkg,
                                    &RuntimeConfig::new("hab".to_string(), "hab".to_string()),
                                    PathBuf::from("/hab/pkgs/neurosis/redis/2000/20160222201258"),
                                    Vec::new(),
                                    &GossipListenAddr::default(),
                                    &ListenAddr::default())
            .unwrap();
        let toml = sc.to_toml().unwrap();
        let name =
            toml.get("pkg").unwrap().as_table().unwrap().get("name").unwrap().as_str().unwrap();
        assert_eq!(name, "redis");
    }

    #[test]
    fn to_toml_sys() {
        let pkg = gen_pkg();
        let sc = ServiceConfig::new(&pkg,
                                    &RuntimeConfig::new("hab".to_string(), "hab".to_string()),
                                    PathBuf::from("/hab/pkgs/neurosis/redis/2000/20160222201258"),
                                    Vec::new(),
                                    &GossipListenAddr::default(),
                                    &ListenAddr::default())
            .unwrap();
        let toml = sc.to_toml().unwrap();
        let ip = toml.get("sys").unwrap().as_table().unwrap().get("ip").unwrap().as_str().unwrap();
        let re = Regex::new(r"\d+\.\d+\.\d+\.\d+").unwrap();
        assert!(re.is_match(&ip));
    }

    #[test]
    fn merge_with_empty_me_table() {
        let mut me = toml_from_str("");
        let other = toml_from_str(r#"
            fruit = "apple"
            veggie = "carrot"
            "#);
        let expected = other.clone();
        toml_merge(&mut me, &other).unwrap();

        assert_eq!(me, expected);
    }

    #[test]
    fn merge_with_empty_other_table() {
        let mut me = toml_from_str(r#"
            fruit = "apple"
            veggie = "carrot"
            "#);
        let other = toml_from_str("");
        let expected = me.clone();
        toml_merge(&mut me, &other).unwrap();

        assert_eq!(me, expected);
    }

    #[test]
    fn merge_with_shallow_tables() {
        let mut me = toml_from_str(r#"
            fruit = "apple"
            veggie = "carrot"
            awesomeness = 10
            "#);
        let other = toml_from_str(r#"
            fruit = "orange"
            awesomeness = 99
            "#);
        let expected = toml_from_str(r#"
            fruit = "orange"
            veggie = "carrot"
            awesomeness = 99
            "#);
        toml_merge(&mut me, &other).unwrap();

        assert_eq!(me, expected);
    }

    #[test]
    fn merge_with_differing_value_types() {
        let mut me = toml_from_str(r#"
            fruit = "apple"
            veggie = "carrot"
            awesome_things = ["carrots", "kitties", "unicorns"]
            heat = 42
            "#);
        let other = toml_from_str(r#"
            heat = "hothothot"
            awesome_things = "habitat"
            "#);
        let expected = toml_from_str(r#"
            heat = "hothothot"
            fruit = "apple"
            veggie = "carrot"
            awesome_things = "habitat"
            "#);
        toml_merge(&mut me, &other).unwrap();

        assert_eq!(me, expected);
    }

    #[test]
    fn merge_with_table_values() {
        let mut me = toml_from_str(r#"
            frubnub = "foobar"

            [server]
            some-details = "initial"
            port = 1000
            "#);
        let other = toml_from_str(r#"
            [server]
            port = 5000
            more-details = "yep"
            "#);
        let expected = toml_from_str(r#"
            frubnub = "foobar"

            [server]
            port = 5000
            some-details = "initial"
            more-details = "yep"
            "#);
        toml_merge(&mut me, &other).unwrap();

        assert_eq!(me, expected);
    }

    #[test]
    fn merge_with_deep_table_values() {
        let mut me = toml_from_str(r#"
            [a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z.aa.ab.ac.ad]
            stew = "carrot"
            [a.b.c.d.e.f.foxtrot]
            fancy = "fork"
            "#);
        let other = toml_from_str(r#"
            [a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z.aa.ab.ac.ad]
            stew = "beef"
            [a.b.c.d.e.f.foxtrot]
            fancy = "feast"
            funny = "farm"
            "#);
        let expected = toml_from_str(r#"
            [a.b.c.d.e.f.foxtrot]
            funny = "farm"
            fancy = "feast"
            [a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z.aa.ab.ac.ad]
            stew = "beef"
            "#);
        toml_merge(&mut me, &other).unwrap();

        assert_eq!(me, expected);
    }

    #[test]
    fn merge_with_dangerously_deep_table_values() {
        let mut me = toml_from_str(r#"
            [a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z.aa.ab.ac.ad.ae.af]
            stew = "carrot"
            "#);
        let other = toml_from_str(r#"
            [a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z.aa.ab.ac.ad.ae.af]
            stew = "beef"
            "#);

        match toml_merge(&mut me, &other) {
            Err(e) => {
                match e.err {
                    Error::TomlMergeError(_) => assert!(true),
                    _ => panic!("Should fail with Error::TomlMergeError"),
                }
            }
            Ok(_) => panic!("Should not complete successfully"),
        }
    }

    mod sys {
        use super::super::Sys;
        use config::GossipListenAddr;
        use http_gateway::ListenAddr;
        use regex::Regex;

        #[test]
        fn ip() {
            let s = Sys::new(&GossipListenAddr::default(), &ListenAddr::default());
            let re = Regex::new(r"\d+\.\d+\.\d+\.\d+").unwrap();
            assert!(re.is_match(&s.ip));
        }

        #[test]
        fn hostname() {
            let s = Sys::new(&GossipListenAddr::default(), &ListenAddr::default());
            let re = Regex::new(r"\w+").unwrap();
            assert!(re.is_match(&s.hostname));
        }

        #[test]
        fn to_toml() {
            let s = Sys::new(&GossipListenAddr::default(), &ListenAddr::default());
            let toml = s.to_toml().unwrap();
            let ip = toml.get("ip").unwrap().as_str().unwrap();
            let re = Regex::new(r"\d+\.\d+\.\d+\.\d+").unwrap();
            assert!(re.is_match(&ip));
        }
    }

    mod hab {
        use super::super::Hab;
        use VERSION;

        #[test]
        fn version() {
            let h = Hab::new();
            assert_eq!(h.version, VERSION);
        }

        #[test]
        fn to_toml() {
            let h = Hab::new();
            let version_toml = h.to_toml().unwrap();
            let version = version_toml.get("version").unwrap().as_str().unwrap();
            assert_eq!(version, VERSION);
        }
    }
}
