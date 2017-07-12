// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

//! Encapsulate running the `hab-sup` executable for tests.

use std::env;
use std::process::{Child, Command, Stdio};
use std::path::{Path, PathBuf};
use std::string::ToString;

use super::test_butterfly;

extern crate rand;
use self::rand::distributions::{IndependentSample, Range};

pub struct TestSup {
    pub hab_root: PathBuf,
    pub origin_name: String,
    pub package_name: String,
    pub service_group: String,
    pub http_port: u16,
    pub butterfly_port: u16,
    pub butterfly_client: test_butterfly::Client,
    pub cmd: Command,
    pub process: Option<Child>,
}

/// Return a random unprivileged TCP port number.
fn random_port() -> u16 {
    let between = Range::new(1024, ::std::u16::MAX);
    let mut rng = rand::thread_rng();
    between.ind_sample(&mut rng)
}

/// Find an executable relative to the current integration testing
/// executable.
///
/// Thus if the current executable is
///
///    /home/me/habitat/target/debug/deps/compilation-ccaf2f45c24e3840
///
/// and we look for `hab-sup`, we'll find it at
///
///    /home/me/habitat/target/debug/hab-sup
///
fn find_exe<B>(binary_name: B) -> PathBuf
    where B: AsRef<Path>
{
    let exe_root = env::current_exe()
        .unwrap()
        .parent()  // deps
        .unwrap()
        .parent()  // debug
        .unwrap()
        .to_path_buf();
    let bin = exe_root.join(binary_name.as_ref());
    assert!(
        bin.exists(),
        format!("Expected to find a {:?} executable at {:?}", binary_name.as_ref(), bin)
    );
    bin
}

impl TestSup {
    /// Create a new `TestSup` that will listen on randomly-selected
    /// ports for both gossip and HTTP requests so tests run in
    /// parallel don't step on each other.
    ///
    /// See also `new`.
    pub fn new_with_random_ports<R, O, P, S>(
        fs_root: R,
        origin: O,
        pkg_name: P,
        service_group: S,
    ) -> TestSup
    where
        R: AsRef<Path>,
        O: ToString,
        P: ToString,
        S: ToString,
    {
        let http_port = random_port();
        let butterfly_port = http_port + 1;
        TestSup::new(
            fs_root,
            origin,
            pkg_name,
            service_group,
            http_port,
            butterfly_port,
        )
    }

    /// Bundle up a Habitat supervisor process along with an
    /// associated Butterfly client for injecting new configuration
    /// values. The supervisor executable is the one that has been
    /// compiled for the current `cargo test` invocation.
    ///
    /// The supervisor is configured to run a single package for a
    /// test. This package is assumed to have already been installed
    /// relative to `fs_root` (i.e., the `FS_ROOT` environment
    /// variable, which in our tests will be a randomly-named
    /// temporary directory that this supervisor will view as `/`.).
    ///
    /// A Butterfly client is also created for interacting with this
    /// supervisor and package. It is properly configured according to
    /// the value provided for `butterfly_port`. To use it, see the
    /// `apply_config` function.
    ///
    /// (No HTTP interaction with the supervisor is currently called
    /// for, so we don't have a HTTP client.)
    pub fn new<R, O, P, S>(
        fs_root: R,
        origin: O,
        pkg_name: P,
        service_group: S,
        http_port: u16,
        butterfly_port: u16,
    ) -> TestSup
    where
        R: AsRef<Path>,
        O: ToString,
        P: ToString,
        S: ToString,
    {
        let sup_exe = find_exe("hab-sup");
        let launcher_exe = find_exe("hab-launch");

        let mut cmd = Command::new(&launcher_exe);
        let listen_host = "0.0.0.0";

        let origin = origin.to_string();
        let pkg_name = pkg_name.to_string();
        let service_group = service_group.to_string();

        cmd.env("FS_ROOT", fs_root.as_ref().to_string_lossy().as_ref())
            .env("HAB_SUP_BINARY", &sup_exe)
            .env("HAB_DEPOT_URL", "http://hab.sup.test/v1/depot")
            .arg("start")
            .arg("--listen-gossip")
            .arg(format!("{}:{}", listen_host, butterfly_port))
            .arg("--listen-http")
            .arg(format!("{}:{}", listen_host, http_port))
            .arg(format!("{}/{}", origin, pkg_name))
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let bc = test_butterfly::Client::new(&pkg_name, &service_group, butterfly_port);

        TestSup {
            hab_root: fs_root.as_ref().to_path_buf(),
            origin_name: origin,
            package_name: pkg_name,
            service_group: service_group.to_string(),
            http_port: http_port,
            butterfly_port: butterfly_port,
            butterfly_client: bc,
            cmd: cmd,
            process: None,
        }
    }

    /// Spawn a process actually running the supervisor.
    pub fn start(&mut self) {
        let child = self.cmd.spawn().expect("Couldn't start the supervisor!");
        self.process = Some(child);
    }

    /// The equivalent of performing `hab apply` with the given
    /// configuration.
    pub fn apply_config<T>(&mut self, toml_config: T)
    where
        T: ToString,
    {
        self.butterfly_client.apply(toml_config.to_string())
    }
}

// We kill the supervisor so you don't have to!
impl Drop for TestSup {
    fn drop(&mut self) {
        self.process
            .take()
            .expect("No process to kill!")
            .kill()
            .expect("Tried to kill supervisor!");
    }
}
