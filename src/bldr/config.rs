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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Install,
    Config,
    Start,
    Key,
    KeyUpload,
    Shell,
    Repo,
    Upload,
}

impl Default for Command {
    fn default() -> Command {
        Command::Install
    }
}

#[derive(Default)]
pub struct Config {
    command: Command,
    package: String,
    url: String,
    topology: String,
    group: String,
    path: String,
    deriv: String,
    version: String,
    release: String,
    watch: Vec<String>,
    key: String,
}

impl Config {
    pub fn new() -> Config {
        Config::default()
    }

    pub fn set_command(&mut self, command: Command) -> &mut Config {
        self.command = command;
        self
    }

    pub fn command(&self) -> Command {
        self.command.clone()
    }

    pub fn set_key(&mut self, key: String) -> &mut Config {
        self.key = key;
        self
    }

    pub fn key(&self) -> &str {
        &self.key
    }


    pub fn set_package(&mut self, package: String) -> &mut Config {
        self.package = package;
        self
    }

    pub fn package(&self) -> &str {
        &self.package
    }

    pub fn set_deriv(&mut self, deriv: String) -> &mut Config {
        self.deriv = deriv;
        self
    }

    pub fn deriv(&self) -> &str {
        &self.deriv
    }

    pub fn set_version(&mut self, version: String) -> &mut Config {
        self.version = version;
        self
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn set_release(&mut self, release: String) -> &mut Config {
        self.release = release;
        self
    }

    pub fn release(&self) -> &str {
        &self.release
    }

    pub fn set_path(&mut self, path: String) -> &mut Config {
        self.path = path;
        self
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn set_group(&mut self, group: String) -> &mut Config {
        self.group = group;
        self
    }

    pub fn group(&self) -> &str {
        &self.group
    }

    pub fn set_watch(&mut self, watch: Vec<String>) -> &mut Config {
        self.watch = watch;
        self
    }

    pub fn watch(&self) -> &[String] {
        &self.watch
    }

    pub fn set_url(&mut self, url: String) -> &mut Config {
        self.url = url;
        self
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn set_topology(&mut self, topology: String) -> &mut Config {
        self.topology = topology;
        self
    }

    pub fn topology(&self) -> &str {
        &self.topology
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, Command};

    #[test]
    fn new() {
        let c = Config::new();
        assert_eq!(c.topology(), String::new());
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
        assert_eq!(c.url(), "http://foolio.com");
    }

    #[test]
    fn topology() {
        let mut c = Config::new();
        c.set_topology(String::from("leader"));
        assert_eq!(c.topology(), "leader");
    }
}
