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

use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::ops::{Deref, DerefMut};
use std::path::Path;

use ansi_term::Colour::Purple;
use butterfly::rumor::service::SysInfo;
use hcore::fs::FS_ROOT_PATH;
use hcore::package::PackageInstall;
use hcore::crypto;
use toml;

use manager::census::{Census, CensusList};
use config::gconfig;
use error::{Error, Result};
use package::Package;
use templating::Template;
use util::{self, convert};
use util::users as hab_users;
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
    // Set to 'true' if we have data that needs to be sent to a configuration file
    #[serde(skip_deserializing)]
    pub needs_write: bool,
}

impl ServiceConfig {
    /// Takes a new package and a new census list, and returns a ServiceConfig. This function can
    /// fail, and indeed, we want it to - it causes the program to crash if we can not render the
    /// first pass of the configuration file.
    pub fn new(service_group: &str,
               package: &Package,
               cl: &CensusList,
               bindings: &[String])
               -> Result<ServiceConfig> {
        let cfg = try!(Cfg::new(package));
        let bind = try!(Bind::new(bindings, &cl));
        Ok(ServiceConfig {
            pkg: Pkg::new(&package.pkg_install),
            hab: Hab::new(),
            sys: Sys::new(),
            cfg: cfg,
            svc: Svc::new(service_group, cl),
            bind: bind,
            needs_write: true,
        })
    }

    /// Render this struct as toml.
    pub fn to_toml(&self) -> Result<toml::Value> {
        let mut top = toml::Table::new();

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

    pub fn to_exported(&self) -> Result<toml::Table> {
        self.cfg.to_exported(&self.pkg.exports)
    }

    /// Replace the `pkg` data.
    pub fn pkg(&mut self, pkg_install: &PackageInstall) {
        self.pkg = Pkg::new(pkg_install);
        self.needs_write = true
    }

    /// Replace the `svc` data.
    pub fn svc(&mut self, service_group: &str, cl: &CensusList) {
        self.svc = Svc::new(service_group, cl);
        self.needs_write = true
    }

    /// Replace the `svc` data.
    pub fn bind(&mut self, bindings: &[String], cl: &CensusList) {
        // This is only safe because we will fail the first time if the bindings are badly
        // formatted - so we know we can't fail here.
        self.bind = Bind::new(bindings, cl).unwrap();
        self.needs_write = true
    }

    /// Replace the `cfg` data.
    pub fn cfg(&mut self, package: &Package) {
        match Cfg::new(package) {
            Ok(cfg) => {
                self.cfg = cfg;
                self.needs_write = true;
            }
            Err(e) => outputln!("Failed to write new cfg tree: {}", e),
        }
    }

    /// Write the configuration to `config.toml`, and render the templated configuration files.
    pub fn write(&mut self, pkg: &Package) -> Result<bool> {
        let pi = &pkg.pkg_install;
        let final_toml = try!(self.to_toml());
        {
            let mut last_toml = try!(File::create(pi.svc_path().join("config.toml")));
            try!(write!(&mut last_toml, "{}", toml::encode_str(&final_toml)));
        }
        let mut template = Template::new();

        // Register all the templates; this makes them available as partials!
        // I suspect this will be useful, but I think we'll want to make this
        // more explicit... in a minute, we render all the config files anyway.
        let config_files = try!(pkg.config_files());
        for config in config_files.iter() {
            let path = pkg.config_from().join("config").join(config);
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
            let filename = pi.svc_config_path().join(&config).to_string_lossy().into_owned();
            let file_hash = match crypto::hash::hash_file(&filename) {
                Ok(file_hash) => file_hash,
                Err(e) => {
                    debug!("Cannot read the file in order to hash it: {}", e);
                    String::new()
                }
            };

            if file_hash.is_empty() {
                debug!("Configuration {} does not exist; restarting", filename);
                outputln!("Updated {} {}", Purple.bold().paint(config), template_hash);
                let mut config_file = try!(File::create(&filename));
                try!(config_file.write_all(&template_data.into_bytes()));
                should_restart = true
            } else {
                if file_hash == template_hash {
                    debug!("Configuration {} {} has not changed; not restarting.",
                           filename,
                           file_hash);
                    continue;
                } else {
                    debug!("Configuration {} has changed; restarting", filename);
                    outputln!("Updated {} {}", Purple.bold().paint(config), template_hash);
                    let mut config_file = try!(File::create(&filename));
                    try!(config_file.write_all(&template_data.into_bytes()));
                    should_restart = true;
                }
            }
        }
        self.needs_write = false;
        Ok(should_restart)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Bind(toml::Table);

impl Bind {
    fn new(binding_cfg: &[String], cl: &CensusList) -> Result<Bind> {
        let mut top = toml::Table::new();
        let bindings = try!(Bind::split_bindings(binding_cfg));
        for (bind, service_group) in bindings {
            match cl.get(&service_group) {
                Some(census) => {
                    top.insert(format!("has_{}", bind), toml::Value::Boolean(true));
                    top.insert(bind, toml::Value::Table(service_entry(census)));
                }
                None => {
                    top.insert(format!("has_{}", bind), toml::Value::Boolean(false));
                }
            }
        }
        Ok(Bind(top))
    }

    fn split_bindings(bindings: &[String]) -> Result<Vec<(String, String)>> {
        let mut bresult = Vec::new();
        for bind in bindings.into_iter() {
            let values: Vec<&str> = bind.splitn(2, ':').collect();
            if values.len() != 2 {
                return Err(sup_error!(Error::InvalidBinding(bind.clone())));
            } else {
                bresult.push((values[0].to_string(), values[1].to_string()));
            }
        }
        Ok(bresult)
    }

    fn to_toml(&self) -> toml::Value {
        toml::Value::Table(self.0.clone())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Svc(toml::Table);

impl Svc {
    fn new(service_group: &str, cl: &CensusList) -> Svc {
        let mut top = match cl.get(service_group) {
            Some(ce) => service_entry(ce),
            None => toml::Table::default(),
        };
        let mut all: Vec<toml::Value> = Vec::new();
        let mut named = toml::Table::new();
        for (_sg, c) in cl.iter() {
            all.push(toml::Value::Table(service_entry(c)));
            let mut group = if named.contains_key(c.get_service()) {
                named.get(c.get_service()).unwrap().as_table().unwrap().clone()
            } else {
                toml::Table::new()
            };
            group.insert(String::from(c.get_group()),
                         toml::Value::Table(service_entry(c)));
            named.insert(String::from(c.get_service()), toml::Value::Table(group));
        }
        top.insert("all".to_string(), toml::Value::Array(all));
        top.insert("named".to_string(), toml::Value::Table(named));
        Svc(top)
    }

    fn to_toml(&self) -> toml::Value {
        toml::Value::Table(self.0.clone())
    }
}

fn service_entry(census: &Census) -> toml::Table {
    let service = toml::Value::String(String::from(census.get_service()));
    let group = toml::Value::String(String::from(census.get_group()));
    let ident = toml::Value::String(census.get_service_group());
    let leader = census.get_leader().map(|ce| toml::encode(ce));
    let mut members: Vec<toml::Value> = Vec::new();
    let mut member_id = toml::Table::new();
    for (sg, ce) in census.iter() {
        members.push(toml::encode(ce));
        member_id.insert(format!("{}", sg), toml::encode(ce));
    }
    let mut result = toml::Table::new();
    result.insert("service".to_string(), service);
    result.insert("group".to_string(), group);
    result.insert("ident".to_string(), ident);
    if let Some(me) = census.me() {
        let toml_me = toml::encode(me);
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
    fn new(pkg: &Package) -> Result<Cfg> {
        let mut cfg = Cfg {
            default: None,
            user: None,
            gossip: None,
            environment: None,
        };
        try!(cfg.load_default(pkg));
        try!(cfg.load_user(pkg));
        try!(cfg.load_gossip(pkg));
        try!(cfg.load_environment(pkg));
        Ok(cfg)
    }

    pub fn to_toml(&self) -> Result<toml::Value> {
        let mut output_toml = toml::Table::new();
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

    fn to_exported(&self, exports: &HashMap<String, String>) -> Result<toml::Table> {
        let mut map = toml::Table::default();
        let cfg = try!(self.to_toml());
        for (key, path) in exports.iter() {
            // JW TODO: the TOML library only provides us with a
            // function to retrieve a value with a path which returns a
            // reference. We actually want the value for ourselves.
            // Let's improve this later to avoid allocating twice.
            if let Some(val) = cfg.lookup(&path) {
                map.insert(key.clone(), val.clone());
            }
        }
        Ok(map)
    }

    fn load_default(&mut self, pkg: &Package) -> Result<()> {
        let mut file = match File::open(pkg.config_from().join("default.toml")) {
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
                let mut toml_parser = toml::Parser::new(&config);
                let toml = try!(toml_parser.parse()
                    .ok_or(sup_error!(Error::TomlParser(toml_parser.errors))));
                self.default = Some(toml::Value::Table(toml));
            }
            Err(e) => {
                outputln!("Failed to read default.toml: {}", e);
                self.default = None;
            }
        }
        Ok(())
    }

    fn load_user(&mut self, pkg: &Package) -> Result<()> {
        let mut file = match File::open(pkg.svc_path().join("user.toml")) {
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
                let mut toml_parser = toml::Parser::new(&config);
                let toml = try!(toml_parser.parse()
                    .ok_or(sup_error!(Error::TomlParser(toml_parser.errors))));
                self.user = Some(toml::Value::Table(toml));
            }
            Err(e) => {
                outputln!("Failed to load user.toml: {}", e);
                self.user = None;
            }
        }
        Ok(())
    }

    fn load_gossip(&mut self, pkg: &Package) -> Result<()> {
        let mut file = match File::open(pkg.svc_path().join("gossip.toml")) {
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
                let mut toml_parser = toml::Parser::new(&config);
                let toml = try!(toml_parser.parse()
                    .ok_or(sup_error!(Error::TomlParser(toml_parser.errors))));
                self.gossip = Some(toml::Value::Table(toml));
            }
            Err(e) => {
                outputln!("Failed to load gossip.toml: {}", e);
                self.gossip = None;
            }
        }
        Ok(())
    }

    fn load_environment(&mut self, pkg: &Package) -> Result<()> {
        let var_name = format!("{}_{}", ENV_VAR_PREFIX, pkg.name)
            .to_ascii_uppercase()
            .replace("-", "_");
        match env::var(&var_name) {
            Ok(config) => {
                let mut toml_parser = toml::Parser::new(&config);
                let toml = try!(toml_parser.parse()
                    .ok_or(sup_error!(Error::TomlParser(toml_parser.errors))));
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
    pub deps: Vec<Pkg>,
    pub exposes: Vec<String>,
    pub exports: HashMap<String, String>,
    pub path: String,
    pub svc_path: String,
    pub svc_config_path: String,
    pub svc_data_path: String,
    pub svc_files_path: String,
    pub svc_static_path: String,
    pub svc_var_path: String,
    pub svc_user: Option<String>,
    pub svc_group: Option<String>,
    pub svc_user_default: String,
    pub svc_group_default: String,
}

impl Pkg {
    fn new(pkg_install: &PackageInstall) -> Pkg {
        let ident = pkg_install.ident();
        let pkg_deps = match pkg_install.tdeps() {
            Ok(deps) => deps,
            Err(_) => {
                outputln!("Failed to load transitive deps for {} - it will be missing from the \
                           configuration",
                          &ident);
                Vec::new()
            }
        };
        let mut deps = Vec::new();
        for d in pkg_deps.iter() {
            if let Ok(p) = PackageInstall::load(d, Some(Path::new(&*FS_ROOT_PATH))) {
                deps.push(Pkg::new(&p));
            } else {
                outputln!("Failed to load {} - it will be missing from the configuration",
                          &d)
            }
        }
        let exposes = match pkg_install.exposes() {
            Ok(exposes) => exposes,
            Err(_) => {
                outputln!("Failed to load exposes metadata for {} - \
                          it will be missing from the configuration",
                          &ident);
                Vec::new()
            }
        };
        let exports = match pkg_install.exports() {
            Ok(exports) => exports,
            Err(_) => {
                outputln!("Failed to load exposes metadata for {} - \
                          it will be missing from the configuration",
                          &ident);
                HashMap::<String, String>::default()
            }
        };
        let version = match ident.version.as_ref() {
            Some(v) => v.clone(),
            None => "".to_string(),
        };
        let release = match ident.release.as_ref() {
            Some(r) => r.clone(),
            None => "".to_string(),
        };

        let (default_svc_user, default_svc_group) =
            match hab_users::get_user_and_group(&pkg_install) {
                Ok((svc_user, svc_group)) => (svc_user, svc_group),
                Err(_e) => {
                    // TODO
                    panic!("Can't get default service and user");
                }
            };

        let svc_user = pkg_install.svc_user().unwrap_or(None);
        let svc_group = pkg_install.svc_group().unwrap_or(None);

        Pkg {
            origin: ident.origin.clone(),
            name: ident.name.clone(),
            version: version,
            release: release,
            ident: ident.to_string(),
            deps: deps,
            exposes: exposes,
            exports: exports,
            path: pkg_install.installed_path().to_string_lossy().into_owned(),
            svc_path: pkg_install.svc_path().to_string_lossy().into_owned(),
            svc_config_path: pkg_install.svc_config_path().to_string_lossy().into_owned(),
            svc_data_path: pkg_install.svc_data_path().to_string_lossy().into_owned(),
            svc_files_path: pkg_install.svc_files_path().to_string_lossy().into_owned(),
            svc_static_path: pkg_install.svc_static_path().to_string_lossy().into_owned(),
            svc_var_path: pkg_install.svc_var_path().to_string_lossy().into_owned(),
            svc_user: svc_user,
            svc_group: svc_group,
            svc_user_default: default_svc_user,
            svc_group_default: default_svc_group,
        }
    }

    fn to_toml(&self) -> Result<toml::Value> {
        let v = toml::encode(&self);
        Ok(v)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sys(SysInfo);

impl Sys {
    fn new() -> Sys {
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
            http_gateway_ip: gconfig().service_config_http_listen.ip().to_string(),
            http_gateway_port: gconfig().service_config_http_listen.port(),
        })
    }

    fn to_toml(&self) -> Result<toml::Value> {
        let v = toml::encode(&self);
        Ok(v)
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
        let v = toml::encode(&self);
        Ok(v)
    }
}


// Recursively merges the `other` TOML table into `me`
pub fn toml_merge(me: &mut toml::Table, other: &toml::Table) -> Result<()> {
    toml_merge_recurse(me, other, 0)
}

fn toml_merge_recurse(me: &mut toml::Table, other: &toml::Table, depth: u16) -> Result<()> {
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

fn is_toml_value_a_table(key: &str, table: &toml::Table) -> bool {
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

    use serde;
    use serde_json;
    use util::{self, convert};

    use super::*;
    use config::{gcache, Config};
    use error::Error;
    use manager::census::{CensusEntry, CensusList};
    use package::Package;
    use VERSION;

    fn gen_pkg() -> Package {
        let pkg_install = PackageInstall::new_from_parts(
            PackageIdent::from_str("neurosis/redis/2000/20160222201258").unwrap(),
            PathBuf::from("/"),
            PathBuf::from("/fakeo"),
            PathBuf::from("/fakeo/here"));
        let pkg_dep = PackageIdent::from_str("neurosis/jq-static/1000/20160222201259").unwrap();
        Package {
            origin: String::from("neurosis"),
            name: String::from("redis"),
            version: String::from("2000"),
            release: String::from("20160222201258"),
            deps: vec![pkg_dep],
            tdeps: Vec::new(),
            pkg_install: pkg_install,
        }
    }

    fn gen_census_list() -> CensusList {
        let mut ce = CensusEntry::default();
        let member_id = "0000000000000000000";
        ce.set_member_id(format!("{}", member_id));
        ce.set_service(String::from("redis"));
        ce.set_group(String::from("default"));
        let mut cl = CensusList::new();
        cl.insert(format!("{}", member_id), ce);
        cl
    }

    fn toml_from_string(content: &str) -> toml::Table {
        toml::Parser::new(content)
            .parse()
            .expect(&format!("Content should parse as TOML: {}", content))
    }

    #[test]
    fn to_toml_hab() {
        gcache(Config::default());
        let pkg = gen_pkg();
        let cl = gen_census_list();
        let sc = ServiceConfig::new("redis.default", &pkg, &cl, &Vec::new()).unwrap();
        let toml = sc.to_toml().unwrap();
        let version = toml.lookup("hab.version").unwrap().as_str().unwrap();
        assert_eq!(version, VERSION);
    }

    #[test]
    fn to_toml_pkg() {
        gcache(Config::default());
        let pkg = gen_pkg();
        let cl = gen_census_list();
        let sc = ServiceConfig::new("redis.default", &pkg, &cl, &Vec::new()).unwrap();
        let toml = sc.to_toml().unwrap();
        let name = toml.lookup("pkg.name").unwrap().as_str().unwrap();
        assert_eq!(name, "redis");
    }

    #[test]
    fn to_toml_sys() {
        gcache(Config::default());
        let pkg = gen_pkg();
        let cl = gen_census_list();
        let sc = ServiceConfig::new("redis.default", &pkg, &cl, &Vec::new()).unwrap();
        let toml = sc.to_toml().unwrap();
        let ip = toml.lookup("sys.ip").unwrap().as_str().unwrap();
        let re = Regex::new(r"\d+\.\d+\.\d+\.\d+").unwrap();
        assert!(re.is_match(&ip));
    }

    #[test]
    fn deserialize_service_config_from_json() {
        gcache(Config::default());
        let pkg = gen_pkg();
        let cl = gen_census_list();
        let sc = ServiceConfig::new("redis.default", &pkg, &cl, &Vec::new()).unwrap();
        let toml = sc.to_toml().unwrap();
        let data = convert::toml_to_json(toml);
        let _ = serde_json::from_value::<ServiceConfig>(data).unwrap();
    }

    #[test]
    fn merge_with_empty_me_table() {
        let mut me = toml_from_string("");
        let other = toml_from_string(r#"
            fruit = "apple"
            veggie = "carrot"
            "#);
        let expected = other.clone();
        toml_merge(&mut me, &other).unwrap();

        assert_eq!(me, expected);
    }

    #[test]
    fn merge_with_empty_other_table() {
        let mut me = toml_from_string(r#"
            fruit = "apple"
            veggie = "carrot"
            "#);
        let other = toml_from_string("");
        let expected = me.clone();
        toml_merge(&mut me, &other).unwrap();

        assert_eq!(me, expected);
    }

    #[test]
    fn merge_with_shallow_tables() {
        let mut me = toml_from_string(r#"
            fruit = "apple"
            veggie = "carrot"
            awesomeness = 10
            "#);
        let other = toml_from_string(r#"
            fruit = "orange"
            awesomeness = 99
            "#);
        let expected = toml_from_string(r#"
            fruit = "orange"
            veggie = "carrot"
            awesomeness = 99
            "#);
        toml_merge(&mut me, &other).unwrap();

        assert_eq!(me, expected);
    }

    #[test]
    fn merge_with_differing_value_types() {
        let mut me = toml_from_string(r#"
            fruit = "apple"
            veggie = "carrot"
            awesome_things = ["carrots", "kitties", "unicorns"]
            heat = 42
            "#);
        let other = toml_from_string(r#"
            heat = "hothothot"
            awesome_things = "habitat"
            "#);
        let expected = toml_from_string(r#"
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
        let mut me = toml_from_string(r#"
            frubnub = "foobar"

            [server]
            some-details = "initial"
            port = 1000
            "#);
        let other = toml_from_string(r#"
            [server]
            port = 5000
            more-details = "yep"
            "#);
        let expected = toml_from_string(r#"
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
        let mut me = toml_from_string(r#"
            [a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z.aa.ab.ac.ad]
            stew = "carrot"
            [a.b.c.d.e.f.foxtrot]
            fancy = "fork"
            "#);
        let other = toml_from_string(r#"
            [a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z.aa.ab.ac.ad]
            stew = "beef"
            [a.b.c.d.e.f.foxtrot]
            fancy = "feast"
            funny = "farm"
            "#);
        let expected = toml_from_string(r#"
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
        let mut me = toml_from_string(r#"
            [a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z.aa.ab.ac.ad.ae.af]
            stew = "carrot"
            "#);
        let other = toml_from_string(r#"
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
        use config::{gcache, Config};
        use super::super::Sys;
        use regex::Regex;

        #[test]
        fn ip() {
            gcache(Config::default());
            let s = Sys::new();
            let re = Regex::new(r"\d+\.\d+\.\d+\.\d+").unwrap();
            assert!(re.is_match(&s.ip));
        }

        #[test]
        fn hostname() {
            gcache(Config::default());
            let s = Sys::new();
            let re = Regex::new(r"\w+").unwrap();
            assert!(re.is_match(&s.hostname));
        }

        #[test]
        fn to_toml() {
            gcache(Config::default());
            let s = Sys::new();
            let toml = s.to_toml().unwrap();
            let ip = toml.lookup("ip").unwrap().as_str().unwrap();
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
            let version = version_toml.lookup("version").unwrap().as_str().unwrap();
            assert_eq!(version, VERSION);
        }
    }
}
