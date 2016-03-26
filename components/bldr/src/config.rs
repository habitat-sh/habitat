// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! Configuration for bldr.
//!
//! This module is populated from the CLI options in `main.rs`, and then passed through to the
//! [command](../command) modules. Check out the `config_from_args(..)` function there for more
//! details.
//!
//! See the [Config](struct.Config.html) struct for the specific options available.

use std::str::FromStr;

use core::package::PackageIdent;

use error::{BldrError, ErrorKind};
use gossip::server::GOSSIP_DEFAULT_PORT;
use topology::Topology;

static LOGKEY: &'static str = "CFG";

#[derive(Debug, Clone, PartialEq, Eq)]
/// An enum with the various CLI commands. Used to keep track of what command was called.
pub enum Command {
    Install,
    Config,
    Start,
    ImportKey,
    InjectConfigFile,
    ExportKey,
    UploadDepotKey,
    DownloadDepotKey,
    GenerateUserKey,
    GenerateServiceKey,
    ListKeys,
    Encrypt,
    Decrypt,
    Shell,
    Upload,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateStrategy {
    None,
    AtOnce,
}

impl UpdateStrategy {
    pub fn from_str(strategy: &str) -> Self {
        match strategy {
            "none" => UpdateStrategy::None,
            "at-once" => UpdateStrategy::AtOnce,
            s => panic!("Invalid update strategy {}", s),
        }
    }
}
impl Default for UpdateStrategy {
    fn default() -> UpdateStrategy {
        UpdateStrategy::None
    }
}

impl FromStr for Command {
    type Err = BldrError;
    fn from_str(s: &str) -> Result<Command, BldrError> {
        match s {
            "bash" => Ok(Command::Shell),
            "config" => Ok(Command::Config),
            "decrypt" => Ok(Command::Decrypt),
            "download-depot-key" => Ok(Command::DownloadDepotKey),
            "encrypt" => Ok(Command::Encrypt),
            "export-key" => Ok(Command::ExportKey),
            "generate-service-key" => Ok(Command::GenerateServiceKey),
            "generate-user-key" => Ok(Command::GenerateUserKey),
            "import-key" => Ok(Command::ImportKey),
            "inject-config-file" => Ok(Command::InjectConfigFile),
            "install" => Ok(Command::Install),
            "list-keys" => Ok(Command::ListKeys),
            "sh" => Ok(Command::Shell),
            "start" => Ok(Command::Start),
            "upload-depot-key" => Ok(Command::UploadDepotKey),
            "upload" => Ok(Command::Upload),
            _ => Err(bldr_error!(ErrorKind::CommandNotImplemented)),
        }
    }
}

// We provide a default command primarily so the Config struct can have sane defaults.
impl Default for Command {
    fn default() -> Command {
        Command::Install
    }
}

/// Holds our configuration options.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Config {
    command: Command,
    package: PackageIdent,
    url: Option<String>,
    topology: Topology,
    group: String,
    path: String,
    archive: String,
    watch: Vec<String>,
    key: String,
    password: Option<String>,
    email: Option<String>,
    expire_days: Option<u16>,
    gossip_listen: String,
    userkey: Option<String>,
    servicekey: Option<String>,
    infile: Option<String>,
    outfile: Option<String>,
    gossip_peer: Vec<String>,
    gossip_permanent: bool,
    update_strategy: UpdateStrategy,
    service_group: String,
    file_path: String,
    version_number: u64,
}

impl Config {
    /// Create a default `Config`
    pub fn new() -> Config {
        Config::default()
    }

    /// Set the archive
    pub fn set_archive(&mut self, archive: String) -> &mut Config {
        self.archive = archive;
        self
    }

    /// Return the archive
    pub fn archive(&self) -> &str {
        &self.archive
    }

    pub fn set_update_strategy(&mut self, strat: UpdateStrategy) -> &mut Config {
        self.update_strategy = strat;
        self
    }

    /// Return the command we used
    pub fn update_strategy(&self) -> UpdateStrategy {
        self.update_strategy.clone()
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

    /// Set the password
    pub fn set_password(&mut self, password: String) -> &mut Config {
        self.password = Some(password);
        self
    }

    /// Return the password
    pub fn password(&self) -> &Option<String> {
        &self.password
    }

    /// Set the email address
    pub fn set_email(&mut self, email: String) -> &mut Config {
        self.email = Some(email);
        self
    }

    /// Return the email address
    pub fn email(&self) -> &Option<String> {
        &self.email
    }

    /// Set the user key
    pub fn set_user_key(&mut self, userkey: String) -> &mut Config {
        self.userkey = Some(userkey);
        self
    }

    /// Return the user key
    pub fn user_key(&self) -> &Option<String> {
        &self.userkey
    }

    /// Set the service key
    pub fn set_service_key(&mut self, set_servicekey: String) -> &mut Config {
        self.servicekey = Some(set_servicekey);
        self
    }

    /// Return the service key
    pub fn service_key(&self) -> &Option<String> {
        &self.servicekey
    }

    /// Set the input file to encrypt/decrypt
    pub fn set_infile(&mut self, infile: String) -> &mut Config {
        self.infile = Some(infile);
        self
    }

    /// Return the input file to encrypt/decrypt
    pub fn infile(&self) -> &Option<String> {
        &self.infile
    }

    /// Set the input file to encrypt/decrypt
    pub fn set_outfile(&mut self, outfile: String) -> &mut Config {
        self.outfile = Some(outfile);
        self
    }

    /// Return the input file to encrypt/decrypt
    pub fn outfile(&self) -> &Option<String> {
        &self.outfile
    }

    /// Set the key expire days
    pub fn set_expire_days(&mut self, expire_days: u16) -> &mut Config {
        self.expire_days = Some(expire_days);
        self
    }

    pub fn expire_days(&self) -> &Option<u16> {
        &self.expire_days
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

    pub fn gossip_listen(&self) -> &str {
        &self.gossip_listen
    }

    pub fn set_gossip_listen(&mut self, gl: String) -> &mut Config {
        self.gossip_listen = gl;
        self
    }

    pub fn gossip_permanent(&self) -> bool {
        self.gossip_permanent
    }

    pub fn set_gossip_permanent(&mut self, p: bool) -> &mut Config {
        self.gossip_permanent = p;
        self
    }

    pub fn gossip_peer(&self) -> &[String] {
        &self.gossip_peer
    }

    /// Set the service group
    pub fn set_service_group(&mut self, sg: String) -> &mut Config {
        self.service_group = sg;
        self
    }

    /// Return the service group
    pub fn service_group(&self) -> &str {
        &self.service_group
    }

    /// Set the file path
    pub fn set_file_path(&mut self, fp: String) -> &mut Config {
        self.file_path = fp;
        self
    }

    /// Return the file path
    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    /// Set the version number
    pub fn set_version_number(&mut self, vn: u64) -> &mut Config {
        self.version_number = vn;
        self
    }

    /// Return the version number
    pub fn version_number(&self) -> &u64 {
        &self.version_number
    }

    pub fn set_gossip_peer(&mut self, mut gp: Vec<String>) -> &mut Config {
        for p in gp.iter_mut() {
            if p.find(':').is_none() {
                p.push_str(&format!(":{}", GOSSIP_DEFAULT_PORT));
            }
        }
        self.gossip_peer = gp;
        self
    }

    pub fn set_package(&mut self, ident: PackageIdent) -> &mut Config {
        self.package = ident;
        self
    }

    pub fn package(&self) -> &PackageIdent {
        &self.package
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
