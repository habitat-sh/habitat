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

//! Configuration for bldr.
//!
//! This module is populated from the CLI options in `main.rs`, and then passed through to the
//! [command](../command) modules. Check out the `config_from_args(..)` function there for more
//! details.
//!
//! See the [Config](struct.Config.html) struct for the specific options available.

use std::net;

use topology::Topology;
use repo;

#[derive(Debug, Clone, PartialEq, Eq)]
/// An enum with the various CLI commands. Used to keep track of what command was called.
pub enum Command {
    Install,
    Config,
    Start,
    Key,
    KeyUpload,
    Shell,
    Repo,
    Upload,
    Configuration,
}

// We provide a default command primarily so the Config struct can have sane defaults.
impl Default for Command {
    fn default() -> Command {
        Command::Install
    }
}

/// Holds our configuration options.
#[derive(Default)]
pub struct Config {
    command: Command,
    package: String,
    url: Option<String>,
    topology: Topology,
    group: String,
    path: String,
    deriv: String,
    version: Option<String>,
    release: Option<String>,
    watch: Vec<String>,
    key: String,
    listen_addr: repo::ListenAddr,
    port: repo::ListenPort,
    gossip_listen: String,
}

impl Config {
    /// Create a default `Config`
    pub fn new() -> Config {
        Config::default()
    }

    /// Set the `Command` we used
    pub fn set_command(&mut self, command: Command) -> &mut Config {
        self.command = command;
        self
    }

    /// Return the command we used
    pub fn command(&self) -> Command {
        self.command.clone()
    }

    /// Set the key
    pub fn set_key(&mut self, key: String) -> &mut Config {
        self.key = key;
        self
    }

    /// Return the key
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Set the package name
    pub fn set_package(&mut self, package: String) -> &mut Config {
        self.package = package;
        self
    }

    /// Return the package name
    pub fn package(&self) -> &str {
        &self.package
    }

    /// Set the derivation
    pub fn set_deriv(&mut self, deriv: String) -> &mut Config {
        self.deriv = deriv;
        self
    }

    /// Return the derivation
    pub fn deriv(&self) -> &str {
        &self.deriv
    }

    /// Set the version
    pub fn set_version(&mut self, version: String) -> &mut Config {
        self.version = Some(version);
        self
    }

    /// Return the version
    pub fn version(&self) -> &Option<String> {
        &self.version
    }

    /// Set the release
    pub fn set_release(&mut self, release: String) -> &mut Config {
        self.release = Some(release);
        self
    }

    /// Return the release
    pub fn release(&self) -> &Option<String> {
        &self.release
    }

    /// Set the path
    pub fn set_path(&mut self, path: String) -> &mut Config {
        self.path = path;
        self
    }

    /// Return the path
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Set the group
    pub fn set_group(&mut self, group: String) -> &mut Config {
        self.group = group;
        self
    }

    /// Return the group
    pub fn group(&self) -> &str {
        &self.group
    }

    /// Set the watch
    pub fn set_watch(&mut self, watch: Vec<String>) -> &mut Config {
        self.watch = watch;
        self
    }

    /// Return the watch
    pub fn watch(&self) -> &[String] {
        &self.watch
    }

    /// Set the url
    pub fn set_url(&mut self, url: String) -> &mut Config {
        self.url = Some(url);
        self
    }

    /// Return the url
    pub fn url(&self) -> &Option<String> {
        &self.url
    }

    /// Set the topology
    pub fn set_topology(&mut self, topology: Topology) -> &mut Config {
        self.topology = topology;
        self
    }

    /// Return the topology
    pub fn topology(&self) -> &Topology {
        &self.topology
    }

    pub fn set_port(&mut self, port: u16) -> &mut Config {
        self.port = repo::ListenPort(port);
        self
    }

    pub fn repo_addr(&self) -> net::SocketAddrV4 {
        net::SocketAddrV4::new(self.listen_addr.0.clone(), self.port.0.clone())
    }

    pub fn gossip_listen(&self) -> &str {
        &self.gossip_listen
    }

    pub fn set_gossip_listen(&mut self, gl: String) -> &mut Config {
        self.gossip_listen = gl;
        self
    }

    pub fn package_id(&self) -> String {
        if self.version.is_some() && self.release.is_some() {
            format!("{}/{}/{}/{}",
                    &self.deriv,
                    &self.package,
                    self.version.as_ref().unwrap(),
                    self.release.as_ref().unwrap())
        } else if self.version.is_some() {
            format!("{}/{}/{}",
                    self.deriv,
                    self.package,
                    self.version.as_ref().unwrap())
        } else {
            format!("{}/{}", self.deriv, self.package)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, Command};
    use topology::Topology;

    #[test]
    fn new() {
        let c = Config::new();
        assert_eq!(*c.topology(), Topology::Standalone);
    }

    #[test]
    fn command() {
        let mut c = Config::new();
        c.set_command(Command::Install);
        assert_eq!(c.command(), Command::Install);
    }

    #[test]
    fn key() {
        let mut c = Config::new();
        c.set_key(String::from("foolio"));
        assert_eq!(c.key(), "foolio");
    }

    #[test]
    fn package() {
        let mut c = Config::new();
        c.set_package(String::from("foolio"));
        assert_eq!(c.package(), "foolio");
    }

    #[test]
    fn path() {
        let mut c = Config::new();
        c.set_path(String::from("foolio"));
        assert_eq!(c.path(), "foolio");
    }

    #[test]
    fn url() {
        let mut c = Config::new();
        c.set_url(String::from("http://foolio.com"));
        assert_eq!(c.url().as_ref().unwrap(), "http://foolio.com");
    }

    #[test]
    fn topology() {
        let mut c = Config::new();
        c.set_topology(Topology::Leader);
        assert_eq!(*c.topology(), Topology::Leader);
    }
}
