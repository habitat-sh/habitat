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
    Sh,
    Bash,
}

impl Default for Command {
    fn default() -> Command { Command::Install }
}

#[derive(Default)]
pub struct Config {
    command: Command,
    package: Option<String>,
    url: Option<String>,
    wait: bool,
    topology: Option<String>,
}

impl Config {
    pub fn new(command: Command) -> Config {
        Config{ command: command, ..Default::default() }
    }

    pub fn set_command(&mut self, command: Command) -> &mut Config {
        self.command = command;
        self
    }

    pub fn command(&self) -> Command {
        self.command.clone()
    }

    pub fn set_package(&mut self, package: &str) -> &mut Config {
        self.package = Some(String::from(package));
        self
    }

    pub fn package(&self) -> Option<&str> {
        match self.package {
            Some(ref value) => Some(value),
            None => None
        }
    }

    pub fn set_url(&mut self, url: &str) -> &mut Config {
        self.url = Some(String::from(url));
        self
    }

    pub fn url(&self) -> Option<&str> {
        match self.url {
            Some(ref value) => Some(value),
            None => None
        }
    }

    pub fn set_wait(&mut self, wait: bool) -> &mut Config {
        self.wait = wait;
        self
    }

    pub fn wait(&self) -> bool {
        self.wait
    }

    pub fn set_topology(&mut self, topology: &str) -> &mut Config {
        self.topology = Some(String::from(topology));
        self
    }

    pub fn topology(&self) -> Option<&str> {
        match self.topology {
            Some(ref value) => Some(value),
            None => None
        }
    }

}

#[cfg(test)]
mod tests {
    use super::{Config, Command};

    #[test]
    fn new() {
        let c = Config::new(Command::Start);
        assert_eq!(c.command(), Command::Start);
    }

    #[test]
    fn command() {
        let mut c = Config::new(Command::Start);
        c.set_command(Command::Bash);
        assert_eq!(c.command(), Command::Bash);
    }

    #[test]
    fn package() {
        let mut c = Config::new(Command::Start);
        c.set_package("foolio");
        assert_eq!(c.package(), Some("foolio"));
    }

    #[test]
    fn url() {
        let mut c = Config::new(Command::Start);
        c.set_url("http://foolio.com");
        assert_eq!(c.url(), Some("http://foolio.com"));
    }

    #[test]
    fn wait() {
        let mut c = Config::new(Command::Start);
        c.set_wait(true);
        assert_eq!(c.wait(), true);
    }

    #[test]
    fn topology() {
        let mut c = Config::new(Command::Start);
        c.set_topology("leader");
        assert_eq!(c.topology(), Some("leader"));
    }
}
