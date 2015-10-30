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
use std::process::Command;
use std::fs::{self, DirEntry};
use std::path::PathBuf;
use std::io;
use std::io::prelude::*;
use std::fs::File;

use regex::Regex;

use util;
use health_check::{self, CheckResult};

#[derive(Debug, Clone, Eq)]
pub struct Package {
    pub derivation: String,
    pub name: String,
    pub version: String,
    pub release: String,
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
    pub fn signal(&self, signal: Signal) -> BldrResult<String> {
        let runit_pkg = try!(latest("runit", None));
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
            Command::new(runit_pkg.join_path("bin/sv"))
            .arg(signal_arg)
            .arg(&format!("/opt/bldr/srvc/{}", self.name))
            .output()
            );
        match output.status.success() {
            true => {
                let stdout = try!(String::from_utf8(output.stdout));
                return Ok(stdout)
            },
            false => {
                match signal {
                    Signal::ForceShutdown => {
                        let outstr = try!(String::from_utf8(output.stdout));
                        return Ok(outstr)
                    },
                    _ => {},
                }
                debug!("Failed to send signal to the process supervisor for {}", self.name);
                let outstr = try!(String::from_utf8(output.stdout));
                let errstr = try!(String::from_utf8(output.stderr));
                debug!("Supervisor (O): {}", outstr);
                debug!("Supervisor (E): {}", errstr);
                debug!("Supervisor Code {:?}", output.status.code());
                return Err(BldrError::SupervisorSignalFailed)
            }
        }
    }

    pub fn exposes(&self) -> Vec<String> {
        match fs::metadata(self.join_path("EXPOSES")) {
            Ok(_) => {
                let mut exposed_file = File::open(self.join_path("EXPOSES")).unwrap();
                let mut exposed_string = String::new();
                exposed_file.read_to_string(&mut exposed_string).unwrap();
                let v: Vec<String> = exposed_string.split(' ').map(|x| String::from(x.trim_right_matches('\n'))).collect();
                return v
            },
            Err(_) => {
                let v: Vec<String> = Vec::new();
                return v
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
        debug!("Creating srvc paths");
        try!(fs::create_dir_all(self.srvc_join_path("config")));
        try!(fs::create_dir_all(self.srvc_join_path("toml")));
        try!(fs::create_dir_all(self.srvc_join_path("data")));
        try!(fs::create_dir_all(self.srvc_join_path("var")));
        try!(util::perm::set_permissions(&self.srvc_join_path("toml"), "0700"));
        try!(util::perm::set_owner(&self.srvc_join_path("data"), "bldr:bldr"));
        try!(util::perm::set_permissions(&self.srvc_join_path("data"), "0700"));
        try!(util::perm::set_owner(&self.srvc_join_path("var"), "bldr:bldr"));
        try!(util::perm::set_permissions(&self.srvc_join_path("var"), "0700"));
        Ok(())
    }

    /// Copy the "run" file to the srvc path.
    pub fn copy_run(&self) -> BldrResult<()> {
        debug!("Copying the run file");
        try!(fs::copy(self.join_path("run"), self.srvc_join_path("run")));
        try!(util::perm::set_permissions(&self.srvc_join_path("run"), "0755"));
        Ok(())
    }

    pub fn topology_leader() -> BldrResult<()> {
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

    pub fn supervisor_running(&self) -> bool {
        let res = self.signal(Signal::Status);
        match res {
            Ok(_) => return true,
            Err(e) => {
                debug!("Supervisor not running?: {:?}", e);
                return false
            },
        }
    }

    pub fn health_check(&self) -> BldrResult<CheckResult> {
        match fs::metadata(&self.srvc_join_path("health")) {
            Ok(_) => {
                let result = try!(health_check::run(&self.srvc_join_path("health")));
                Ok(result)
            },
            Err(_) => {
                let status_output = try!(self.signal(Signal::Status));
                let last_config = try!(self.last_config());
                Ok(health_check::CheckResult{status: health_check::Status::Ok, output: format!("{}\n{}", status_output, last_config)})
            }
        }
    }

    pub fn cache_file(&self) -> PathBuf {
        PathBuf::from(format!("/opt/bldr/cache/pkgs/{}-{}-{}-{}.bldr", self.derivation, self.name, self.version, self.release))
    }

    pub fn last_config(&self) -> BldrResult<String> {
        let mut file = try!(File::open(self.srvc_join_path("config.toml")));
        let mut result = String::new();
        try!(file.read_to_string(&mut result));
        Ok(result)
    }
}

pub fn new(deriv: &str, name: &str, version: &str, release: &str) -> Package {
    Package{
        derivation: String::from(deriv),
        name: String::from(name),
        version: String::from(version),
        release: String::from(release),

    }
}

pub fn latest(pkg: &str, opt_path: Option<&str>) -> BldrResult<Package> {
    let path = if opt_path.is_some() { opt_path.unwrap() } else { "/opt/bldr/pkgs" };
    let pl = try!(package_list(path));
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
pub fn package_list(path: &str) -> BldrResult<Vec<Package>> {
    let mut package_list: Vec<Package> = Vec::new();
    for derivation_r in try!(fs::read_dir(path)) {
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
                        release: r,
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
    use super::{Package, split_version, version_sort};
    use std::cmp::Ordering;
    use std::cmp::PartialOrd;

    #[test]
    fn package_partial_eq() {
        let a = Package{
            derivation: "bldr".to_string(),
            name: "bldr".to_string(),
            version: "1.0.0".to_string(),
            release: "20150521131555".to_string(),
        };
        let b = Package{
            derivation: "bldr".to_string(),
            name: "bldr".to_string(),
            version: "1.0.0".to_string(),
            release: "20150521131555".to_string(),
        };
        assert_eq!(a, b);
    }

    #[test]
    fn package_partial_ord() {
        let a = Package{
            derivation: "bldr".to_string(),
            name: "bldr".to_string(),
            version: "1.0.1".to_string(),
            release: "20150521131555".to_string(),
        };
        let b = Package{
            derivation: "bldr".to_string(),
            name: "bldr".to_string(),
            version: "1.0.0".to_string(),
            release: "20150521131555".to_string(),
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
            release: "20150521131555".to_string(),
        };
        let b = Package{
            derivation: "bldr".to_string(),
            name: "bldr".to_string(),
            version: "1.0.0".to_string(),
            release: "20150521131555".to_string(),
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
            release: "20150521131555".to_string(),
        };
        let b = Package{
            derivation: "bldr".to_string(),
            name: "bldr".to_string(),
            version: "1.0.0".to_string(),
            release: "20150521131555".to_string(),
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
            release: "20150521131556".to_string(),
        };
        let b = Package{
            derivation: "bldr".to_string(),
            name: "bldr".to_string(),
            version: "1.0.0".to_string(),
            release: "20150521131555".to_string(),
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
}
