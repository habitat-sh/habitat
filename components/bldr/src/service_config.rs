// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

/// Collect all the configuration data that is exposed to users, and render it.

use rustc_serialize::Encodable;

use toml;
use VERSION;
use error::{BldrResult, ErrorKind};
use package::Package;
use util;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::ascii::AsciiExt;
use std::collections::HashMap;
use util::convert;
use openssl::crypto::hash as openssl_hash;
use census::{Census, CensusList};
use mustache;
use ansi_term::Colour::Purple;

static LOGKEY: &'static str = "SC";

/// The top level struct for all our configuration - this corresponds to the top level namespaces
/// available in `config.toml`.
#[derive(Debug, RustcEncodable)]
pub struct ServiceConfig {
    bldr: Bldr,
    pkg: Pkg,
    sys: Sys,
    cfg: Cfg,
    svc: Svc,
    // Keeps a list of the configuration files we have renders, and only re-writes them if they
    // have changed.
    config_hash: HashMap<String, Vec<u8>>,
    // Set to 'true' if we have data that needs to be sent to a configuration file
    pub needs_write: bool,
}

impl ServiceConfig {
    /// Takes a new package and a new census list, and returns a ServiceConfig. This function can
    /// fail, and indeed, we want it to - it causes the program to crash if we can not render the
    /// first pass of the configuration file.
    pub fn new(package: &Package, cl: &CensusList) -> BldrResult<ServiceConfig> {
        let cfg = try!(Cfg::new(package));
        Ok(ServiceConfig {
            pkg: Pkg::new(package),
            bldr: Bldr::new(),
            sys: Sys::new(),
            cfg: cfg,
            svc: Svc::new(cl),
            config_hash: HashMap::new(),
            needs_write: true,
        })
    }

    /// Render this struct as toml.
    pub fn to_toml(&self) -> BldrResult<toml::Value> {
        let mut top = toml::Table::new();

        let bldr = try!(self.bldr.to_toml());
        top.insert(String::from("bldr"), bldr);

        let pkg = try!(self.pkg.to_toml());
        top.insert(String::from("pkg"), pkg);

        let sys = try!(self.sys.to_toml());
        top.insert(String::from("sys"), sys);

        let cfg = self.cfg.to_toml();
        top.insert(String::from("cfg"), cfg);

        let svc = self.svc.to_toml();
        top.insert(String::from("svc"), svc);

        Ok(toml::Value::Table(top))
    }

    /// Replace the `pkg` data.
    pub fn pkg(&mut self, package: &Package) {
        self.pkg = Pkg::new(package);
        self.needs_write = true
    }

    /// Replace the `svc` data.
    pub fn svc(&mut self, cl: &CensusList) {
        self.svc = Svc::new(cl);
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
    pub fn write(&mut self, pkg: &Package) -> BldrResult<bool> {
        let final_toml = try!(self.to_toml());
        {
            let mut last_toml = try!(File::create(pkg.svc_join_path("config.toml")));
            try!(write!(&mut last_toml, "{}", toml::encode_str(&final_toml)));
        }

        let final_data = convert::toml_to_mustache(final_toml);

        let mut should_restart = false;
        let config_files = try!(pkg.config_files());
        for config in config_files {
            let template = try!(mustache::compile_path(pkg.join_path(&format!("config/{}",
                                                                              config))));
            let mut config_vec = Vec::new();
            let filename = pkg.svc_join_path(&format!("config/{}", config));
            template.render_data(&mut config_vec, &final_data);
            let file_hash = openssl_hash::hash(openssl_hash::Type::SHA256, &config_vec);
            if self.config_hash.contains_key(&filename) {
                if file_hash == *self.config_hash.get(&filename).unwrap() {
                    debug!("Configuration {} has not changed; not restarting", filename);
                    continue;
                } else {
                    debug!("Configuration {} has changed; restarting", filename);
                    outputln!("Updated {}", Purple.bold().paint(config));
                    self.config_hash.insert(filename.clone(), file_hash);
                    let mut config_file = try!(File::create(&filename));
                    try!(config_file.write_all(&config_vec));
                    should_restart = true;
                }
            } else {
                debug!("Configuration {} does not exist; restarting", filename);
                outputln!("Updated {}", Purple.bold().paint(config));
                self.config_hash.insert(filename.clone(), file_hash);
                let mut config_file = try!(File::create(&filename));
                try!(config_file.write_all(&config_vec));
                should_restart = true
            }
        }
        if pkg.supervisor_running() {
            self.needs_write = false;
            Ok(should_restart)
        } else {
            // If the supervisor isn't running yet, we don't have to worry about
            // restarting it, obviously
            self.needs_write = false;
            Ok(false)
        }
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
        for (sg, c) in cl.iter() {
            all.push(toml::Value::Table(service_entry(c)));
            named.insert(sg.clone(), toml::Value::Table(service_entry(c)));
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
    fn new(pkg: &Package) -> BldrResult<Cfg> {
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

    fn load_default(&mut self, pkg: &Package) -> BldrResult<()> {
        // Default
        let mut file = match File::open(pkg.join_path("default.toml")) {
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
                                .ok_or(bldr_error!(ErrorKind::TomlParser(toml_parser.errors))));
                self.default = Some(toml::Value::Table(toml));
            }
            Err(e) => {
                outputln!("Failed to read default.toml: {}", e);
                self.default = None;
            }
        }
        Ok(())
    }

    fn load_user(&mut self, pkg: &Package) -> BldrResult<()> {
        let mut file = match File::open(pkg.svc_join_path("user.toml")) {
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
                                .ok_or(bldr_error!(ErrorKind::TomlParser(toml_parser.errors))));
                self.user = Some(toml::Value::Table(toml));
            }
            Err(e) => {
                outputln!("Failed to load user.toml: {}", e);
                self.user = None;
            }
        }
        Ok(())
    }

    fn load_gossip(&mut self, pkg: &Package) -> BldrResult<()> {
        let mut file = match File::open(pkg.svc_join_path("gossip.toml")) {
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
                                .ok_or(bldr_error!(ErrorKind::TomlParser(toml_parser.errors))));
                self.gossip = Some(toml::Value::Table(toml));
            }
            Err(e) => {
                outputln!("Failed to load gossip.toml: {}", e);
                self.gossip = None;
            }
        }
        Ok(())
    }

    fn load_environment(&mut self, pkg: &Package) -> BldrResult<()> {
        let var_name = format!("BLDR_{}", pkg.name).to_ascii_uppercase();
        match env::var(&var_name) {
            Ok(config) => {
                let mut toml_parser = toml::Parser::new(&config);
                let toml = try!(toml_parser.parse()
                                .ok_or(bldr_error!(ErrorKind::TomlParser(toml_parser.errors))));
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
}

impl Pkg {
    fn new(package: &Package) -> Pkg {
        let mut deps = Vec::new();
        for d in package.deps.iter() {
            if let Ok(p) = Package::load(d, None) {
                deps.push(Pkg::new(&p));
            } else {
                outputln!("Failed to load {} - it will be missing from the configuration",
                          d)
            }
        }
        Pkg {
            origin: package.origin.clone(),
            name: package.name.clone(),
            version: package.version.clone(),
            release: package.release.clone(),
            ident: package.ident(),
            deps: deps,
            exposes: package.exposes(),
            path: package.path(),
            svc_path: package.svc_path(),
        }
    }

    fn to_toml(&self) -> BldrResult<toml::Value> {
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
}

impl Sys {
    fn new() -> Sys {
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
        }
    }

    fn to_toml(&self) -> BldrResult<toml::Value> {
        let mut e = toml::Encoder::new();
        try!(self.encode(&mut e));
        let v = toml::Value::Table(e.toml);
        Ok(v)
    }
}

#[derive(Debug, RustcEncodable)]
pub struct Bldr {
    pub version: &'static str,
}

impl Bldr {
    fn new() -> Bldr {
        Bldr { version: VERSION }
    }

    fn to_toml(&self) -> BldrResult<toml::Value> {
        let mut e = toml::Encoder::new();
        try!(self.encode(&mut e));
        let v = toml::Value::Table(e.toml);
        Ok(v)
    }
}

#[cfg(test)]
mod test {
    use regex::Regex;
    use service_config::ServiceConfig;
    use package::Package;
    use gossip::member::MemberId;
    use census::{CensusEntry, Census, CensusList};
    use VERSION;

    fn gen_pkg() -> Package {
        Package {
            origin: String::from("neurosis"),
            name: String::from("sovereign"),
            version: String::from("2000"),
            release: String::from("20160222201258"),
            deps: Vec::new(),
            tdeps: Vec::new(),
        }
    }

    fn gen_census_list() -> CensusList {
        let ce = CensusEntry::new("redis", "default", MemberId::new_v4());
        let c = Census::new(ce);
        CensusList::new(c)
    }

    #[test]
    fn to_toml_bldr() {
        let pkg = gen_pkg();
        let cl = gen_census_list();
        let sc = ServiceConfig::new(&pkg, &cl).unwrap();
        let toml = sc.to_toml().unwrap();
        let version = toml.lookup("bldr.version").unwrap().as_str().unwrap();
        assert_eq!(version, VERSION);
    }

    #[test]
    fn to_toml_pkg() {
        let pkg = gen_pkg();
        let cl = gen_census_list();
        let sc = ServiceConfig::new(&pkg, &cl).unwrap();
        let toml = sc.to_toml().unwrap();
        let name = toml.lookup("pkg.name").unwrap().as_str().unwrap();
        assert_eq!(name, "sovereign");
    }

    #[test]
    fn to_toml_sys() {
        let pkg = gen_pkg();
        let cl = gen_census_list();
        let sc = ServiceConfig::new(&pkg, &cl).unwrap();
        let toml = sc.to_toml().unwrap();
        let ip = toml.lookup("sys.ip").unwrap().as_str().unwrap();
        let re = Regex::new(r"\d+\.\d+\.\d+\.\d+").unwrap();
        assert!(re.is_match(&ip));
    }

    mod sys {
        use service_config::Sys;
        use regex::Regex;

        #[test]
        fn ip() {
            let s = Sys::new();
            let re = Regex::new(r"\d+\.\d+\.\d+\.\d+").unwrap();
            assert!(re.is_match(&s.ip));
        }

        #[test]
        fn hostname() {
            let s = Sys::new();
            let re = Regex::new(r"\w+").unwrap();
            assert!(re.is_match(&s.hostname));
        }

        #[test]
        fn to_toml() {
            let s = Sys::new();
            let toml = s.to_toml().unwrap();
            let ip = toml.lookup("ip").unwrap().as_str().unwrap();
            let re = Regex::new(r"\d+\.\d+\.\d+\.\d+").unwrap();
            assert!(re.is_match(&ip));
        }
    }

    mod bldr {
        use VERSION;
        use service_config::Bldr;

        #[test]
        fn version() {
            let b = Bldr::new();
            assert_eq!(b.version, VERSION);
        }

        #[test]
        fn to_toml() {
            let b = Bldr::new();
            let version_toml = b.to_toml().unwrap();
            let version = version_toml.lookup("version").unwrap().as_str().unwrap();
            assert_eq!(version, VERSION);
        }
    }
}
