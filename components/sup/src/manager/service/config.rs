// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

use ansi_term::Colour::Purple;
use rustc_serialize::Encodable;
use toml;
use handlebars::Handlebars;

use common::gossip_file::GOSSIP_TOML;
use manager::census::{CensusEntry, Census, CensusList};
use config::gconfig;
use error::{Error, Result};
use hcore::package::PackageInstall;
use hcore::crypto;
use package::Package;
use util;
use util::convert;
use util::handlebars_helpers;
use util::users as hab_users;
use VERSION;

static LOGKEY: &'static str = "SC";
static ENV_VAR_PREFIX: &'static str = "HAB";

/// The top level struct for all our configuration - this corresponds to the top level namespaces
/// available in `config.toml`.
#[derive(Debug, RustcEncodable)]
pub struct ServiceConfig {
    hab: Hab,
    pkg: Pkg,
    sys: Sys,
    cfg: Cfg,
    svc: Svc,
    bind: Bind,
    // Set to 'true' if we have data that needs to be sent to a configuration file
    pub needs_write: bool,
}

pub fn never_escape_fn(data: &str) -> String {
    String::from(data)
}

impl ServiceConfig {
    /// Takes a new package and a new census list, and returns a ServiceConfig. This function can
    /// fail, and indeed, we want it to - it causes the program to crash if we can not render the
    /// first pass of the configuration file.
    pub fn new(service_group: &str,
               package: &Package,
               cl: &CensusList,
               bindings: Vec<String>)
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

        let cfg = self.cfg.to_toml();
        top.insert(String::from("cfg"), cfg);

        let svc = self.svc.to_toml();
        top.insert(String::from("svc"), svc);

        let bind = self.bind.to_toml();
        top.insert(String::from("bind"), bind);

        Ok(toml::Value::Table(top))
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
    pub fn bind(&mut self, bindings: Vec<String>, cl: &CensusList) {
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
        let mut handlebars = Handlebars::new();

        debug!("Registering handlebars helpers");
        handlebars.register_helper("json", Box::new(handlebars_helpers::json_helper));
        handlebars.register_helper("toml", Box::new(handlebars_helpers::toml_helper));

        debug!("Registering configuration templates");
        // By default, handlebars escapes HTML. We don't want that.
        handlebars.register_escape_fn(never_escape_fn);

        // Register all the templates; this makes them available as partials!
        // I suspect this will be useful, but I think we'll want to make this
        // more explicit... in a minute, we render all the config files anyway.
        let config_files = try!(pkg.config_files());
        for config in config_files.iter() {
            let path = pkg.config_from().join("config").join(config);
            debug!("Config template {} from {:?}", config, &path);
            if let Err(e) = handlebars.register_template_file(config, &path) {
                outputln!("Error parsing config template file {}: {}",
                          path.to_string_lossy(),
                          e);
                return Err(sup_error!(Error::HandlebarsTemplateFileError(e)));
            }
        }

        let final_data = convert::toml_to_json(final_toml);
        let mut should_restart = false;
        for config in config_files {
            debug!("Rendering template {}", &config);
            let template_data = try!(handlebars.render(&config, &final_data));
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

#[derive(Debug, RustcEncodable)]
struct Bind {
    toml: toml::Table,
}

impl Bind {
    fn new(binding_cfg: Vec<String>, cl: &CensusList) -> Result<Bind> {
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
        Ok(Bind { toml: top })
    }

    fn split_bindings(bindings: Vec<String>) -> Result<Vec<(String, String)>> {
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
        toml::Value::Table(self.toml.clone())
    }
}

#[derive(Debug, RustcEncodable)]
struct Svc {
    toml: toml::Table,
}

impl Svc {
    fn new(service_group: &str, cl: &CensusList) -> Svc {
        let mut top = service_entry(cl.get(service_group).unwrap());
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
        Svc { toml: top }
    }

    fn to_toml(&self) -> toml::Value {
        toml::Value::Table(self.toml.clone())
    }
}

fn service_entry(census: &Census) -> toml::Table {
    let service = toml::Value::String(String::from(census.get_service()));
    let group = toml::Value::String(String::from(census.get_group()));
    let ident = toml::Value::String(census.get_service_group());
    let me = toml::encode(census.me());
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
    result.insert("me".to_string(), me);
    if let Some(l) = leader {
        result.insert("leader".to_string(), l);
    }
    result.insert("members".to_string(), toml::Value::Array(members));
    result.insert("member_id".to_string(), toml::Value::Table(member_id));
    result
}

#[derive(Debug, RustcEncodable)]
struct Cfg {
    default: Option<toml::Value>,
    user: Option<toml::Value>,
    gossip: Option<toml::Value>,
    environment: Option<toml::Value>,
}

// Shallow merges two toml tables.
fn toml_merge(existing: &toml::Table, ovr: &toml::Table) -> toml::Table {
    let mut final_map = existing.clone();
    for (ovr_key, ovr_value) in ovr.iter() {

        // attempt to merge nested tables
        let existing_v = existing.get(ovr_key).and_then(|v| v.as_table());
        let ovr_v = ovr.get(ovr_key).and_then(|v| v.as_table());
        if let None = existing.get(ovr_key) {
            final_map.insert(ovr_key.clone(), ovr_value.clone());
        }
        match (existing_v, ovr_v) {
            (Some(existing_t), Some(ovr_t)) => {
                let new_table = toml_merge(existing_t, ovr_t);
                let table_as_value = toml::Value::Table(new_table);
                final_map.insert(ovr_key.clone(), table_as_value);
                continue;
            }
            (Some(_), _) => {
                // this occurs when the value type in ovr != type in existing
                warn!("TOML structure mismatch for key {}", &ovr_key);
                continue;
            }
            (_, Some(_)) => {
                // this occurs when the value type in ovr != type in existing
                warn!("TOML structure mismatch for key {}", &ovr_key);
            }
            (_, _) => (), // ovr and existing are not toml::Tables
        }

        // if we have an override value, always include it
        final_map.insert(ovr_key.clone(), ovr_value.clone());
    }

    final_map
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

    fn to_toml(&self) -> toml::Value {
        let mut output_toml = toml::Table::new();
        if let Some(toml::Value::Table(ref default_cfg)) = self.default {
            output_toml = toml_merge(&output_toml, default_cfg);
        }
        if let Some(toml::Value::Table(ref user_cfg)) = self.user {
            output_toml = toml_merge(&output_toml, user_cfg);
        }
        if let Some(toml::Value::Table(ref gossip_cfg)) = self.gossip {
            output_toml = toml_merge(&output_toml, gossip_cfg);
        }
        if let Some(toml::Value::Table(ref env_cfg)) = self.environment {
            output_toml = toml_merge(&output_toml, env_cfg);
        }
        toml::Value::Table(output_toml)
    }

    fn load_default(&mut self, pkg: &Package) -> Result<()> {
        // Default
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
        let mut file = match File::open(pkg.svc_path().join(GOSSIP_TOML)) {
            Ok(file) => file,
            Err(e) => {
                debug!("Failed to open {}: {}", GOSSIP_TOML, e);
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
                outputln!("Failed to load {}: {}", GOSSIP_TOML, e);
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

#[derive(Debug, RustcEncodable)]
pub struct Pkg {
    pub origin: String,
    pub name: String,
    pub version: String,
    pub release: String,
    pub ident: String,
    pub deps: Vec<Pkg>,
    pub exposes: Vec<String>,
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
        let pkg_deps = match pkg_install.deps() {
            Ok(deps) => deps,
            Err(_) => {
                outputln!("Failed to load deps for {} - it will be missing from the configuration",
                          &ident);
                Vec::new()
            }
        };
        let mut deps = Vec::new();
        for d in pkg_deps.iter() {
            if let Ok(p) = PackageInstall::load(d, None) {
                deps.push(Pkg::new(&p));
            } else {
                outputln!("Failed to load {} - it will be missing from the configuration",
                          &d)
            }
        }
        let exposes: Vec<String> = match pkg_install.exposes() {
            Ok(exposes) => exposes,
            Err(_) => {
                outputln!("Failed to load exposes metadata for {} - \
                          it will be missing from the configuration",
                          &ident);
                Vec::new()
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
        let mut e = toml::Encoder::new();
        try!(self.encode(&mut e));
        let v = toml::Value::Table(e.toml);
        Ok(v)
    }
}

#[derive(Debug, RustcEncodable)]
pub struct Sys {
    pub ip: String,
    pub hostname: String,
    pub gossip_ip: String,
    pub gossip_port: u16,
    pub sidecar_ip: String,
    pub sidecar_port: u16,
}

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
        Sys {
            ip: ip,
            hostname: hostname,
            gossip_ip: gconfig().gossip_listen_ip().to_string(),
            gossip_port: gconfig().gossip_listen_port(),
            sidecar_ip: gconfig().http_listen_ip().to_string(),
            sidecar_port: gconfig().http_listen_port(),
        }
    }

    fn to_toml(&self) -> Result<toml::Value> {
        let mut e = toml::Encoder::new();
        try!(self.encode(&mut e));
        let v = toml::Value::Table(e.toml);
        Ok(v)
    }
}

#[derive(Debug, RustcEncodable)]
pub struct Hab {
    pub version: &'static str,
}

impl Hab {
    fn new() -> Self {
        Hab { version: VERSION }
    }

    fn to_toml(&self) -> Result<toml::Value> {
        let mut e = toml::Encoder::new();
        try!(self.encode(&mut e));
        let v = toml::Value::Table(e.toml);
        Ok(v)
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use std::str::FromStr;

    use regex::Regex;
    use toml;
    use uuid::Uuid;

    use manager::census::{CensusEntry, CensusList};
    use config::{gcache, Config};
    use hcore::package::{PackageIdent, PackageInstall};
    use package::Package;
    use super::ServiceConfig;
    use VERSION;
    use super::toml_merge;

    fn gen_pkg() -> Package {
        let pkg_install = PackageInstall::new_from_parts(
            PackageIdent::from_str("neurosis/redis/2000/20160222201258").unwrap(),
            PathBuf::from("/"),
            PathBuf::from("/fakeo"),
            PathBuf::from("/fakeo/here"));
        Package {
            origin: String::from("neurosis"),
            name: String::from("redis"),
            version: String::from("2000"),
            release: String::from("20160222201258"),
            deps: Vec::new(),
            tdeps: Vec::new(),
            pkg_install: pkg_install,
        }
    }

    fn gen_census_list() -> CensusList {
        let mut ce = CensusEntry::default();
        let member_id = Uuid::new_v4();
        ce.set_member_id(format!("{}", member_id.simple()));
        ce.set_service(String::from("redis"));
        ce.set_group(String::from("default"));
        let mut cl = CensusList::new();
        cl.insert(format!("{}", member_id.simple()), ce);
        cl
    }

    #[test]
    fn to_toml_hab() {
        gcache(Config::new());
        let pkg = gen_pkg();
        let cl = gen_census_list();
        let sc = ServiceConfig::new("redis.default", &pkg, &cl, Vec::new()).unwrap();
        let toml = sc.to_toml().unwrap();
        let version = toml.lookup("hab.version").unwrap().as_str().unwrap();
        assert_eq!(version, VERSION);
    }

    #[test]
    fn to_toml_pkg() {
        gcache(Config::new());
        let pkg = gen_pkg();
        let cl = gen_census_list();
        let sc = ServiceConfig::new("redis.default", &pkg, &cl, Vec::new()).unwrap();
        let toml = sc.to_toml().unwrap();
        let name = toml.lookup("pkg.name").unwrap().as_str().unwrap();
        assert_eq!(name, "redis");
    }

    #[test]
    fn to_toml_sys() {
        gcache(Config::new());
        let pkg = gen_pkg();
        let cl = gen_census_list();
        let sc = ServiceConfig::new("redis.default", &pkg, &cl, Vec::new()).unwrap();
        let toml = sc.to_toml().unwrap();
        let ip = toml.lookup("sys.ip").unwrap().as_str().unwrap();
        let re = Regex::new(r"\d+\.\d+\.\d+\.\d+").unwrap();
        assert!(re.is_match(&ip));
    }

    #[test]
    fn merge_into_an_empty_start() {
        let override_config = "rando_key = \"rando_override\"
            [server]
            port=\"9090\"
            [server.nested]
            nest = true";
        let default_toml = toml::Table::new();
        let override_toml = toml::Parser::new(override_config).parse().unwrap();
        {
            let result = toml_merge(&default_toml, &override_toml);
            assert!(result.contains_key("rando_key"));
            assert_eq!("rando_override",
                       result.get("rando_key").unwrap().as_str().unwrap());
            assert!(result.contains_key("server"));
            let server = result.get("server").unwrap().as_table().unwrap();
            assert!(server.contains_key("port"));
            assert_eq!("9090", server.get("port").unwrap().as_str().unwrap());
            let nested = server.get("nested").unwrap().as_table().unwrap();
            assert!(nested.contains_key("nest"));
            assert_eq!(true, nested.get("nest").unwrap().as_bool().unwrap());
        }
    }
    #[test]
    fn merge_a_few_things() {
        let default_config = "rando_key = \"thisisdefault\"
            [server]
            port = \"8080\"
            shutdown-port = \"8005\"
            redirect-port = \"8443\"

            [server.nested]
            nest = false
            message = \"don't override me bro\"";

        let override_config = "rando_key = \"rando_override\"
            [server]
            port=\"9090\"
            [server.nested]
            nest = true";
        let default_toml = toml::Parser::new(default_config).parse().unwrap();
        let override_toml = toml::Parser::new(override_config).parse().unwrap();
        {
            let result = toml_merge(&default_toml, &override_toml);
            assert!(result.contains_key("rando_key"));
            assert_eq!("rando_override",
                       result.get("rando_key").unwrap().as_str().unwrap());
            assert!(result.contains_key("server"));
            let server = result.get("server").unwrap().as_table().unwrap();
            assert!(server.contains_key("port"));
            assert_eq!("9090", server.get("port").unwrap().as_str().unwrap());
            assert!(server.contains_key("shutdown-port"));
            assert_eq!("8005",
                       server.get("shutdown-port").unwrap().as_str().unwrap());
            assert!(server.contains_key("redirect-port"));
            assert_eq!("8443",
                       server.get("redirect-port").unwrap().as_str().unwrap());
            let nested = server.get("nested").unwrap().as_table().unwrap();
            assert!(nested.contains_key("nest"));
            assert_eq!(true, nested.get("nest").unwrap().as_bool().unwrap());
        }
    }

    #[test]
    fn merge_only_keys() {
        let default_config = "rando_key = \"thisisdefault\"
            port = \"8080\"
            shutdown-port = \"8005\"
            redirect-port = \"8443\"";

        let override_config = "port=\"9090\"
            rando_key = \"rando_override\"";
        let default_toml = toml::Parser::new(default_config).parse().unwrap();
        let override_toml = toml::Parser::new(override_config).parse().unwrap();
        {
            let result = toml_merge(&default_toml, &override_toml);
            assert!(result.contains_key("rando_key"));
            assert_eq!("rando_override",
                       result.get("rando_key").unwrap().as_str().unwrap());
            assert!(result.contains_key("port"));
            assert_eq!("9090", result.get("port").unwrap().as_str().unwrap());
            assert!(result.contains_key("shutdown-port"));
            assert_eq!("8005",
                       result.get("shutdown-port").unwrap().as_str().unwrap());
            assert!(result.contains_key("redirect-port"));
            assert_eq!("8443",
                       result.get("redirect-port").unwrap().as_str().unwrap());
        }
    }

    mod sys {
        use config::{gcache, Config};
        use super::super::Sys;
        use regex::Regex;

        #[test]
        fn ip() {
            gcache(Config::new());
            let s = Sys::new();
            let re = Regex::new(r"\d+\.\d+\.\d+\.\d+").unwrap();
            assert!(re.is_match(&s.ip));
        }

        #[test]
        fn hostname() {
            gcache(Config::new());
            let s = Sys::new();
            let re = Regex::new(r"\w+").unwrap();
            assert!(re.is_match(&s.hostname));
        }

        #[test]
        fn to_toml() {
            gcache(Config::new());
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
