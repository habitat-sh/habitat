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

use error::{BldrResult, BldrError};
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use regex::Regex;
use std::fs::{self, DirEntry};
use std::path::PathBuf;
use std::io;
use util;

use std::io::prelude::*;
use std::fs::File;
use std::collections::{HashMap, BTreeMap};
use mustache;
use toml;
use ansi_term::Colour::{Purple, White};
use std::env;
use std::process::Command;
use discovery;

#[derive(Debug, Clone, Eq)]
pub struct Package {
    pub derivation: String,
    pub name: String,
    pub version: String,
    pub release: String
}

pub enum Signal {
    Status,
    Up,
    Down,
    Once,
    Pause,
    Cont,
    Hup,
    Alarm,
    Interrupt,
    Quit,
    One,
    Two,
    Term,
    Kill,
    Exit,
    Start,
    Stop,
    Reload,
    Restart,
    Shutdown,
    ForceStop,
    ForceReload,
    ForceRestart,
    ForceShutdown,
    TryRestart
}

impl Package {
    pub fn signal(&self, signal: Signal) -> BldrResult<()> {
        let busybox_pkg = try!(latest("busybox"));
        let signal_arg = match signal {
            Signal::Status => "status",
            Signal::Up => "up",
            Signal::Down => "down",
            Signal::Once => "once",
            Signal::Pause => "pause",
            Signal::Cont => "cont",
            Signal::Hup => "hup",
            Signal::Alarm => "alarm",
            Signal::Interrupt => "interrupt",
            Signal::Quit => "quit",
            Signal::One => "1",
            Signal::Two => "2",
            Signal::Term => "term",
            Signal::Kill => "kill",
            Signal::Exit => "exit",
            Signal::Start => "start",
            Signal::Stop => "stop",
            Signal::Reload => "reload",
            Signal::Restart => "restart",
            Signal::Shutdown => "shutdown",
            Signal::ForceStop => "force-stop",
            Signal::ForceReload => "force-reload",
            Signal::ForceRestart => "force-restart",
            Signal::ForceShutdown => "force-shutdown",
            Signal::TryRestart => "try-restart",
        };
        let output = try!(
            Command::new(busybox_pkg.join_path("bin/sv"))
            .arg(signal_arg)
            .arg(&format!("/opt/bldr/srvc/{}", self.name))
            .output()
            );
        match output.status.success() {
            true => return Ok(()),
            false => {
                match signal {
                    Signal::ForceShutdown => return Ok(()),
                    _ => {},
                }
                println!("   {}: Failed to send signal to the process supervisor", self.name);
                let outstr = try!(String::from_utf8(output.stdout));
                let errstr = try!(String::from_utf8(output.stderr));
                println!("   {}(O): {}", self.name, outstr);
                println!("   {}(E): {}", self.name, errstr);
                println!("   {}: Code {:?}", self.name, output.status.code());
                return Err(BldrError::SupervisorSignalFailed)
            }
        }
    }

    /// The path to the package on disk.
    pub fn path(&self) -> String {
        format!("/opt/bldr/pkgs/{}/{}/{}/{}", self.derivation, self.name, self.version, self.release)
    }

    /// Join a string to the path on disk.
    pub fn join_path(&self, join: &str) -> String {
        format!("/opt/bldr/pkgs/{}/{}/{}/{}/{}", self.derivation, self.name, self.version, self.release, join)
    }

    /// The on disk srvc path for this package.
    pub fn srvc_path(&self) -> String {
        format!("/opt/bldr/srvc/{}", self.name)
    }

    /// Join a string to the on disk srvc path for this package.
    pub fn srvc_join_path(&self, join: &str) -> String {
        format!("/opt/bldr/srvc/{}/{}", self.name, join)
    }

    /// Create the service path for this package.
    pub fn create_srvc_path(&self) -> BldrResult<()> {
        println!("   {}: Creating srvc paths", &self.name);
        try!(fs::create_dir_all(self.srvc_join_path("config")));
        try!(fs::create_dir_all(self.srvc_join_path("data")));
        try!(util::perm::set_owner(&self.srvc_join_path("data"), "bldr:bldr"));
        try!(util::perm::set_permissions(&self.srvc_join_path("data"), "0700"));
        Ok(())
    }

    /// Copy the "run" file to the srvc path.
    pub fn copy_run(&self) -> BldrResult<()> {
        println!("   {}: Copying the run file", &self.name);
        try!(fs::copy(self.join_path("run"), self.srvc_join_path("run")));
        try!(util::perm::set_permissions(&self.srvc_join_path("run"), "0755"));
        Ok(())
    }

    /// Return an iterator of the configuration file names to render.
    ///
    /// This does not return the full path, for convenience with the path
    /// helpers above.
    pub fn config_files(&self) -> BldrResult<Vec<String>> {
        let mut files: Vec<String> = Vec::new();
        for config_r in try!(fs::read_dir(self.join_path("config"))) {
            let config = try!(extract_direntry(config_r));
            match config.path().file_name() {
                Some(filename) => {
                    files.push(filename.to_string_lossy().into_owned().to_string());
                },
                None => unreachable!()
            }
        }
        Ok(files)
    }

    /// Configure the actual configuration files, from various data sources.
    ///
    /// First we use default.toml, which is the mandatory specification for everything
    /// we can configure. Then we layer in any service discovery frameworks (etcd, consul,
    /// zookeeper, chef). Then we layer in environment configuration.
    pub fn config_data(&self, wait: bool) -> BldrResult<()> {
        let pkg_print = if wait {
            format!("{}({})", self.name, White.bold().paint("C"))
        } else {
            self.name.clone()
        };
        println!("   {}: Loading data from default.toml", pkg_print);
        let mut default_toml_file = try!(File::open(self.join_path("default.toml")));
        let mut toml_data = String::new();
        try!(default_toml_file.read_to_string(&mut toml_data));
        let mut toml_parser = toml::Parser::new(&toml_data);
        let default_toml_value = try!(toml_parser.parse().ok_or(BldrError::TomlParser(toml_parser.errors)));

        let discovery_toml = match discovery::etcd::get_config(&self.name, wait) {
            Some(discovery_toml_value) => {
                toml_merge(default_toml_value, discovery_toml_value)
            },
            None => if wait {
                println!("   {}: No data returned from discovery service", pkg_print);
                return Ok(())
            } else {
                default_toml_value
            }
        };

        println!("   {}: Overlaying environment configuration", pkg_print);
        let env_toml = try!(self.env_to_toml());
        let final_data = match env_toml {
            Some(env_toml_value) => toml_table_to_mustache(toml_merge(discovery_toml, env_toml_value)),
            None => toml_table_to_mustache(discovery_toml)
        };

        println!("   {}: Writing out configuration files", pkg_print);
        let config_files = try!(self.config_files());
        for config in config_files {
            let template = try!(mustache::compile_path(self.join_path(&format!("config/{}", config))));
            println!("   {}: Rendering {}", pkg_print, Purple.bold().paint(&config));
            let mut config_file = try!(File::create(self.srvc_join_path(&format!("config/{}", config))));
            template.render_data(&mut config_file, &final_data);
        }
        println!("   {}: Configured", pkg_print);
        if wait {
            println!("   {}: Restarting on configuration change", pkg_print);
            try!(self.signal(Signal::Restart));
        }
        Ok(())
    }

    fn env_to_toml(&self) -> BldrResult<Option<BTreeMap<String, toml::Value>>> {
        let pkg = &self.name;
        let toml_data = match env::var(&format!("BLDR_{}", pkg)) {
            Ok(val) => val,
            Err(e) => {
                debug!("Looking up environment variable BLDR_{} failed: {:?}", pkg, e);
                return Ok(None)
            }
        };
        let mut toml_parser = toml::Parser::new(&toml_data);
        let toml_value = try!(toml_parser.parse().ok_or(BldrError::TomlParser(toml_parser.errors)));
        Ok(Some(toml_value))
    }
}

/// A completely shallow merge of two Toml tables. For v0 of Bldr, if you set any nested key,
/// you must set *all* the keys in that nesting, or your out of luck. Someday, this will need
/// to become a legitimate deep merge.
///
/// We use toml as the middle language because its implementation in rust lends itself to easy
/// cloning of even the deep data.
fn toml_merge(left: BTreeMap<String, toml::Value>, right: BTreeMap<String, toml::Value>) -> BTreeMap<String, toml::Value> {
    let mut final_map = BTreeMap::new();
    for (left_key, left_value) in left.iter() {
        match right.get(left_key) {
            Some(right_value) => { final_map.insert(left_key.clone(), right_value.clone()); },
            None => { final_map.insert(left_key.clone(), left_value.clone()); },
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

pub fn latest(pkg: &str) -> BldrResult<Package> {
    let pl = try!(package_list());
    let latest: Option<Package> = pl.iter().filter(|&p| p.name == pkg)
        .fold(None, |winner, b| {
            match winner {
                Some(a) => {
                    match a.partial_cmp(&b) {
                        Some(Ordering::Greater) => Some(a),
                        Some(Ordering::Equal) => Some(a),
                        Some(Ordering::Less) => Some(b.clone()),
                        None => Some(a)
                    }
                }
                None => Some(b.clone())
            }
        });
    latest.ok_or(BldrError::PackageNotFound)
}

fn extract_direntry(direntry: Result<DirEntry, io::Error>) -> BldrResult<DirEntry> {
    match direntry {
        Ok(x) => Ok(x),
        Err(e) => Err(From::from(e))
    }
}

fn extract_filename(direntry: PathBuf) -> BldrResult<String> {
    let value = try!(direntry.file_name().ok_or(BldrError::FileNameError));
    Ok(value.to_string_lossy().into_owned().to_string())
}

/// So, hey rust. You're cool and all. But you haven't stabilized
/// is_dir(), which means if we want to stay on stable rust, we can't
/// actually check to see if the shit we are dealing with is a directory.
///
/// The result is pretty simple - we are going to throw a failure if you
/// put anything in any of the interstitial directories in /opt/bldr/pkgs
/// that isn't a directory. Fun times.
pub fn package_list() -> BldrResult<Vec<Package>> {
    let mut package_list: Vec<Package> = Vec::new();
    for derivation_r in try!(fs::read_dir("/opt/bldr/pkgs")) {
        let derivation = try!(extract_direntry(derivation_r));
        for name_r in try!(fs::read_dir(derivation.path())) {
            let name = try!(extract_direntry(name_r));
            for version_r in try!(fs::read_dir(name.path())) {
                let version = try!(extract_direntry(version_r));
                for release_r in try!(fs::read_dir(version.path())) {
                    let release = try!(extract_direntry(release_r));
                    let d = try!(extract_filename(derivation.path()));
                    let n = try!(extract_filename(name.path()));
                    let v = try!(extract_filename(version.path()));
                    let r = try!(extract_filename(release.path()));
                    let package = Package{
                        derivation: d,
                        name: n,
                        version: v,
                        release: r
                    };
                    package_list.push(package);
                }
            }
        }
    }
    Ok(package_list)
}

/// Sorts two packages according to their version.
///
/// We are a bit more strict than your average package management solution on versioning.
/// What we support is the "some number of digits or dots" (the version number),
/// followed by an optional "-" and any alphanumeric string (the extension). When determining sort order, we:
///
/// * Separate the version numbers from the extensions
/// * Split the version numbers into an array of digits on any '.' characters. Digits are convered
///   into <u64>.
/// * Compare the version numbers by iterating over them. If 'a' is greater or lesser than 'b', we
///   return that as the result. If it is equal, we move to the next digit and repeat. If one of
///   the version numbers is exhausted before the other, it gains 0's for the missing slot.
/// * If the version numbers are equal, but either A or B has an extension (but not both) than the
///   version without the extension is greater. (1.0.0 is greater than 1.0.0-alpha6)
/// * If both have an extension, it is compared lexicographically, with the result as the final
///   ordering.
///
/// Returns a BldrError if we fail to match for any reason.
pub fn version_sort(a_version: &str, b_version: &str) -> BldrResult<Ordering> {
    let (a_parts, a_extension) = try!(split_version(a_version));
    let (b_parts, b_extension) = try!(split_version(b_version));
    let mut a_iter = a_parts.iter();
    let mut b_iter = b_parts.iter();
    loop {
        let mut a_exhausted = false;
        let mut b_exhausted = false;
        let a_num = match a_iter.next() {
            Some(i) => try!(i.parse::<u64>()),
            None => { a_exhausted = true; 0u64 }
        };
        let b_num = match b_iter.next() {
            Some(i) => try!(i.parse::<u64>()),
            None => { b_exhausted = true; 0u64 }
        };
        if a_exhausted && b_exhausted {
            break;
        }
        match a_num.cmp(&b_num) {
            Ordering::Greater => {
                return Ok(Ordering::Greater)
            },
            Ordering::Equal => {
                continue
            },
            Ordering::Less => {
                return Ok(Ordering::Less)
            }
        }
    }

    // If you have equal digits, and one has an extension, it is
    // the plain digits who win.
    // 1.0.0-alpha1 vs 1.0.0
    if a_extension.is_some() && b_extension.is_none() {
        return Ok(Ordering::Less);
    } else if a_extension.is_none() && b_extension.is_some() {
        return Ok(Ordering::Greater);
    } else if a_extension.is_none() && b_extension.is_none() {
        return Ok(Ordering::Equal);
    } else {
        let a = match a_extension {
            Some(a) => a,
            None => String::new(),
        };
        let b = match b_extension {
            Some(b) => b,
            None => String::new(),
        };
        return Ok(a.cmp(&b))
    }
}

fn split_version(version: &str) -> BldrResult<(Vec<&str>, Option<String>)> {
    let re = try!(Regex::new(r"([\d\.]+)(-.+)?"));
    let caps = match re.captures(version) {
        Some(caps) => caps,
        None => return Err(BldrError::BadVersion)
    };
    let version_number = caps.at(1).unwrap();
    let extension = match caps.at(2) {
        Some(e) => {
            let mut estr: String = e.to_string();
            estr.remove(0);
            Some(estr)
        },
        None => None
    };
    let version_parts: Vec<&str> = version_number.split('.').collect();
    Ok((version_parts, extension))
}

impl PartialEq for Package {
    fn eq(&self, other: &Package) -> bool {
        if self.derivation != other.derivation {
            return false
        } else if self.name != other.name {
            return false
        } else if self.version != other.version {
            return false
        } else if self.release != other.release {
            return false
        } else {
            return true
        }
    }
}

impl PartialOrd for Package {
    /// Packages can be compared according to the following:
    ///
    /// * Derivation is ignored in the comparison - my redis and
    ///   your redis compare the same.
    /// * If the names are not equal, they cannot be compared.
    /// * If the versions are greater/lesser, return that as
    ///   the ordering.
    /// * If the versions are equal, return the greater/lesser
    ///   for the release.
    fn partial_cmp(&self, other: &Package) -> Option<Ordering> {
        if self.name != other.name {
            return None
        }
        let ord = match version_sort(&self.version, &other.version) {
            Ok(ord) => ord,
            Err(e) => {
                error!("This was a very bad version number: {:?}", e);
                return None
            }
        };
        match ord {
            Ordering::Greater => return Some(Ordering::Greater),
            Ordering::Less => return Some(Ordering::Less),
            Ordering::Equal => {
                return Some(self.release.cmp(&other.release))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Package, split_version, version_sort, toml_table_to_mustache};
    use std::cmp::Ordering;
    use std::cmp::PartialOrd;
    use toml;
    use mustache;

    #[test]
    fn package_partial_eq() {
        let a = Package{
            derivation: "bldr".to_string(),
            name: "bldr".to_string(),
            version: "1.0.0".to_string(),
            release: "20150521131555".to_string()
        };
        let b = Package{
            derivation: "bldr".to_string(),
            name: "bldr".to_string(),
            version: "1.0.0".to_string(),
            release: "20150521131555".to_string()
        };
        assert_eq!(a, b);
    }

    #[test]
    fn package_partial_ord() {
        let a = Package{
            derivation: "bldr".to_string(),
            name: "bldr".to_string(),
            version: "1.0.1".to_string(),
            release: "20150521131555".to_string()
        };
        let b = Package{
            derivation: "bldr".to_string(),
            name: "bldr".to_string(),
            version: "1.0.0".to_string(),
            release: "20150521131555".to_string()
        };
        match a.partial_cmp(&b) {
            Some(ord) => assert_eq!(ord, Ordering::Greater),
            None => panic!("Ordering should be greater")
        }
    }

    #[test]
    fn package_partial_ord_bad_name() {
        let a = Package{
            derivation: "bldr".to_string(),
            name: "snoopy".to_string(),
            version: "1.0.1".to_string(),
            release: "20150521131555".to_string()
        };
        let b = Package{
            derivation: "bldr".to_string(),
            name: "bldr".to_string(),
            version: "1.0.0".to_string(),
            release: "20150521131555".to_string()
        };
        match a.partial_cmp(&b) {
            Some(_) => panic!("We tried to return an order"),
            None => assert!(true)
        }
    }

    #[test]
    fn package_partial_ord_different_derivation() {
        let a = Package{
            derivation: "adam".to_string(),
            name: "bldr".to_string(),
            version: "1.0.0".to_string(),
            release: "20150521131555".to_string()
        };
        let b = Package{
            derivation: "bldr".to_string(),
            name: "bldr".to_string(),
            version: "1.0.0".to_string(),
            release: "20150521131555".to_string()
        };
        match a.partial_cmp(&b) {
            Some(ord) => assert_eq!(ord, Ordering::Equal),
            None => panic!("We failed to return an order")
        }
    }

    #[test]
    fn package_partial_ord_release() {
        let a = Package{
            derivation: "adam".to_string(),
            name: "bldr".to_string(),
            version: "1.0.0".to_string(),
            release: "20150521131556".to_string()
        };
        let b = Package{
            derivation: "bldr".to_string(),
            name: "bldr".to_string(),
            version: "1.0.0".to_string(),
            release: "20150521131555".to_string()
        };
        match a.partial_cmp(&b) {
            Some(ord) => assert_eq!(ord, Ordering::Greater),
            None => panic!("We failed to return an order")
        }
    }

    #[test]
    fn split_version_returns_both_parts() {
        let svr = split_version("1.2.3-beta16");
        match svr {
            Ok((version_parts, Some(extension))) => {
                assert_eq!(vec!["1", "2", "3"], version_parts);
                assert_eq!("beta16", extension);
            },
            Ok((_, None)) => panic!("Has an extension"),
            Err(e) => panic!("{:?}", e)
        }
    }

    #[test]
    fn version_sort_simple() {
        match version_sort("1.0.0", "2.0.0") {
            Ok(compare) => assert_eq!(compare, Ordering::Less),
            Err(e) => panic!("{:?}", e)
        }
        match version_sort("2.0.1", "2.0.0") {
            Ok(compare) => assert_eq!(compare, Ordering::Greater),
            Err(e) => panic!("{:?}", e)
        }
        match version_sort("2.1.1", "2.1.1") {
            Ok(compare) => assert_eq!(compare, Ordering::Equal),
            Err(e) => panic!("{:?}", e)
        }
        match version_sort("20150521131347", "20150521131346") {
            Ok(compare) => assert_eq!(compare, Ordering::Greater),
            Err(e) => panic!("{:?}", e)
        }
    }

    #[test]
    fn version_sort_complex() {
        match version_sort("1.0.0-alpha2", "1.0.0-alpha1") {
            Ok(compare) => assert_eq!(compare, Ordering::Greater),
            Err(e) => panic!("{:?}", e)
        }
        match version_sort("1.0.0-alpha1", "1.0.0-alpha2") {
            Ok(compare) => assert_eq!(compare, Ordering::Less),
            Err(e) => panic!("{:?}", e)
        }
        match version_sort("1.0.0-beta1", "1.0.0-alpha1000") {
            Ok(compare) => assert_eq!(compare, Ordering::Greater),
            Err(e) => panic!("{:?}", e)
        }
        match version_sort("2.1.1", "2.1.1-alpha2") {
            Ok(compare) => assert_eq!(compare, Ordering::Greater),
            Err(e) => panic!("{:?}", e)
        }
        match version_sort("2.1.1-alpha2", "2.1.1") {
            Ok(compare) => assert_eq!(compare, Ordering::Less),
            Err(e) => panic!("{:?}", e)
        }
    }

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
