// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

/// Collect all the configuration data that is exposed to users, and render it.

use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use ansi_term::Colour::Purple;
use rustc_serialize::Encodable;
use toml;
use handlebars::{Handlebars, JsonRender};

use common::gossip_file::GOSSIP_TOML;
use census::{Census, CensusList};
use config::Config;
use error::{Error, Result};
use hcore::package::PackageInstall;
use hcore::crypto;
use package::Package;
use util;
use util::convert;
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
    // Keeps a list of the configuration files we have renders, and only re-writes them if they
    // have changed.
    config_hash: HashMap<String, String>,
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
    pub fn new(config: &Config,
               package: &Package,
               cl: &CensusList,
               bindings: Vec<String>)
               -> Result<ServiceConfig> {
        let cfg = try!(Cfg::new(package));
        let bind = try!(Bind::new(bindings, &cl));
        Ok(ServiceConfig {
            pkg: Pkg::new(&package.pkg_install),
            hab: Hab::new(),
            sys: Sys::new(&config),
            cfg: cfg,
            svc: Svc::new(cl),
            bind: bind,
            config_hash: HashMap::new(),
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
    pub fn svc(&mut self, cl: &CensusList) {
        self.svc = Svc::new(cl);
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

        debug!("Registering configuration templates");
        let mut handlebars = Handlebars::new();
        // By default, handlebars escapes HTML. We don't want that.
        handlebars.register_escape_fn(never_escape_fn);

        // Register all the templates; this makes them available as partials!
        // I suspect this will be useful, but I think we'll want to make this
        // more explicit... in a minute, we render all the config files anyway.
        let config_files = try!(pkg.config_files());
        for config in config_files.iter() {
            let path = pi.installed_path().join("config").join(config);
            debug!("Config template {} at {:?}", config, &path);
            try!(handlebars.register_template_file(config, &path));
        }

        let final_data = convert::toml_to_json(final_toml);
        let mut should_restart = false;
        for config in config_files {
            debug!("Rendering template {}", &config);
            let template_data = try!(handlebars.render(&config, &final_data));
            let file_hash = try!(crypto::hash::hash_string(&template_data));
            let filename = pi.svc_config_path().join(&config).to_string_lossy().into_owned();
            if self.config_hash.contains_key(&filename) {
                if file_hash == *self.config_hash.get(&filename).unwrap() {
                    debug!("Configuration {} has not changed; not restarting", filename);
                    continue;
                } else {
                    debug!("Configuration {} has changed; restarting", filename);
                    outputln!("Updated {}", Purple.bold().paint(config));
                    self.config_hash.insert(filename.clone(), file_hash);
                    let mut config_file = try!(File::create(&filename));
                    try!(config_file.write_all(&template_data.into_bytes()));
                    should_restart = true;
                }
            } else {
                debug!("Configuration {} does not exist; restarting", filename);
                outputln!("Updated {}", Purple.bold().paint(config));
                self.config_hash.insert(filename.clone(), file_hash);
                let mut config_file = try!(File::create(&filename));
                try!(config_file.write_all(&template_data.into_bytes()));
                should_restart = true
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
    fn new(cl: &CensusList) -> Svc {
        let mut top = service_entry(cl.local_census());
        let mut all: Vec<toml::Value> = Vec::new();
        let mut named = toml::Table::new();
        for (_sg, c) in cl.iter() {
            all.push(toml::Value::Table(service_entry(c)));
            let mut group = if named.contains_key(&c.service) {
                named.get(&c.service).unwrap().as_table().unwrap().clone()
            } else {
                toml::Table::new()
            };
            group.insert(c.group.clone(), toml::Value::Table(service_entry(c)));
            named.insert(c.service.clone(), toml::Value::Table(group));
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
    let service = toml::Value::String(census.service.clone());
    let group = toml::Value::String(census.group.clone());
    let ident = toml::Value::String(census.service_group());
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
fn toml_merge(left: &toml::Table, right: &toml::Table) -> toml::Table {
    let mut final_map = toml::Table::new();
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
        let mut left = toml::Table::new();
        if let Some(toml::Value::Table(ref right)) = self.default {
            left = toml_merge(&left, right);
        }
        if let Some(toml::Value::Table(ref right)) = self.user {
            left = toml_merge(&left, right);
        }
        if let Some(toml::Value::Table(ref right)) = self.gossip {
            left = toml_merge(&left, right);
        }
        if let Some(toml::Value::Table(ref right)) = self.environment {
            left = toml_merge(&left, right);
        }
        toml::Value::Table(left)
    }

    fn load_default(&mut self, pkg: &Package) -> Result<()> {
        // Default
        let mut file = match File::open(pkg.path().join("default.toml")) {
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
    fn new(config: &Config) -> Sys {
        let ip = match util::sys::ip() {
            Ok(ip) => ip,
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
            gossip_ip: config.gossip_listen_ip().to_string(),
            gossip_port: config.gossip_listen_port(),
            sidecar_ip: config.sidecar_listen_ip().to_string(),
            sidecar_port: config.sidecar_listen_port(),
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

    use census::{CensusEntry, Census, CensusList};
    use config::Config;
    use gossip::member::MemberId;
    use hcore::package::{PackageIdent, PackageInstall};
    use package::Package;
    use service_config::ServiceConfig;
    use VERSION;

    fn gen_pkg() -> Package {
        let pkg_install = PackageInstall::new_from_parts(
            PackageIdent::from_str("neurosis/sovereign/2000/20160222201258").unwrap(),
            PathBuf::from("/"),
            PathBuf::from("/fakeo"),
            PathBuf::from("/fakeo/here"));
        Package {
            origin: String::from("neurosis"),
            name: String::from("sovereign"),
            version: String::from("2000"),
            release: String::from("20160222201258"),
            deps: Vec::new(),
            tdeps: Vec::new(),
            pkg_install: pkg_install,
        }
    }

    fn gen_census_list() -> CensusList {
        let ce = CensusEntry::new("redis", "default", MemberId::new_v4());
        let c = Census::new(ce);
        CensusList::new(c)
    }

    #[test]
    fn to_toml_hab() {
        let pkg = gen_pkg();
        let cl = gen_census_list();
        let sc = ServiceConfig::new(&Config::default(), &pkg, &cl, Vec::new()).unwrap();
        let toml = sc.to_toml().unwrap();
        let version = toml.lookup("hab.version").unwrap().as_str().unwrap();
        assert_eq!(version, VERSION);
    }

    #[test]
    fn to_toml_pkg() {
        let pkg = gen_pkg();
        let cl = gen_census_list();
        let sc = ServiceConfig::new(&Config::default(), &pkg, &cl, Vec::new()).unwrap();
        let toml = sc.to_toml().unwrap();
        let name = toml.lookup("pkg.name").unwrap().as_str().unwrap();
        assert_eq!(name, "sovereign");
    }

    #[test]
    fn to_toml_sys() {
        let pkg = gen_pkg();
        let cl = gen_census_list();
        let sc = ServiceConfig::new(&Config::default(), &pkg, &cl, Vec::new()).unwrap();
        let toml = sc.to_toml().unwrap();
        let ip = toml.lookup("sys.ip").unwrap().as_str().unwrap();
        let re = Regex::new(r"\d+\.\d+\.\d+\.\d+").unwrap();
        assert!(re.is_match(&ip));
    }

    mod sys {
        use config::Config;
        use service_config::Sys;
        use regex::Regex;

        #[test]
        fn ip() {
            let s = Sys::new(&Config::default());
            let re = Regex::new(r"\d+\.\d+\.\d+\.\d+").unwrap();
            assert!(re.is_match(&s.ip));
        }

        #[test]
        fn hostname() {
            let s = Sys::new(&Config::default());
            let re = Regex::new(r"\w+").unwrap();
            assert!(re.is_match(&s.hostname));
        }

        #[test]
        fn to_toml() {
            let s = Sys::new(&Config::default());
            let toml = s.to_toml().unwrap();
            let ip = toml.lookup("ip").unwrap().as_str().unwrap();
            let re = Regex::new(r"\d+\.\d+\.\d+\.\d+").unwrap();
            assert!(re.is_match(&ip));
        }
    }

    mod hab {
        use service_config::Hab;
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
