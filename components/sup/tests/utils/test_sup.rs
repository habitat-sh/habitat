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

use std::{collections::HashSet,
          env,
          net::TcpListener,
          path::{Path,
                 PathBuf},
          process::{Child,
                    Command,
                    Stdio},
          string::ToString,
          sync::Mutex,
          thread,
          time::Duration};

use crate::hcore::url::BLDR_URL_ENVVAR;
use rand::{self,
           distributions::{Distribution,
                           Uniform}};

use super::test_butterfly;

lazy_static! {
    /// Keep track of all TCP ports currently being used by TestSup
    /// instances. Allows us to run tests in parallel without fear of
    /// port conflicts between them.
    static ref CLAIMED_PORTS: Mutex<HashSet<u16>> = {
        Mutex::new(HashSet::new())
    };
}

pub struct TestSup {
    pub hab_root:         PathBuf,
    pub origin_name:      String,
    pub package_name:     String,
    pub service_group:    String,
    pub http_port:        u16,
    pub butterfly_port:   u16,
    pub control_port:     u16,
    pub butterfly_client: test_butterfly::Client,
    pub cmd:              Command,
    pub process:          Option<Child>,
}

/// Return a free TCP port number. We test to see that the system has
/// not already bound the port, while also tracking which ports are
/// being used by other test supervisors that may be running alongside
/// this one.
///
/// Once you receive a port number from this function, you can be
/// reasonably sure that you're the only one that will be using
/// it. There could be a race condition if the machine the tests are
/// running on just happens to claim the same port number for
/// something between the time we check and the time the TestSup
/// claims it. If that happens to you, you should probably buy lottery
/// tickets, though.
///
/// This function will recursively call itself with a decremented
/// value for `tries` if it happens to pick a port that's already in
/// use. Once all tries are used up, it panics! Yay!
fn unclaimed_port(tries: u16) -> u16 {
    if tries == 0 {
        panic!("Couldn't find an unclaimed port for the test Supervisor!")
    }
    let p = random_port();
    match TcpListener::bind(format!("127.0.0.1:{}", p)) {
        Ok(_listener) => {
            // The system hasn't bound it. Now we make sure none of
            // our other tests have bound it.
            let mut ports = CLAIMED_PORTS.lock().unwrap();
            if ports.contains(&p) {
                // Oops, another test is using it, try again
                thread::sleep(Duration::from_millis(500));
                unclaimed_port(tries - 1)
            } else {
                // Nobody was using it. Return the port; the TcpListener
                // that is currently bound to the port will be dropped,
                // thus freeing the port for our use.
                ports.insert(p);
                p
            }
        }
        Err(_) => {
            // port already in use, try again
            unclaimed_port(tries - 1)
        }
    }
}

/// Return a random unprivileged, unregistered TCP port number.
fn random_port() -> u16 {
    // IANA port registrations go to 49151
    let between = Uniform::new_inclusive(49152, ::std::u16::MAX);
    let mut rng = rand::thread_rng();
    between.sample(&mut rng)
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
fn find_exe<B>(binary_name: B) -> PathBuf
    where B: AsRef<Path>
{
    let exe_root = env::current_exe()
        .unwrap()
        .parent() // deps
        .unwrap()
        .parent() // debug
        .unwrap()
        .to_path_buf();
    let bin = exe_root.join(binary_name.as_ref());
    assert!(bin.exists(),
            format!("Expected to find a {:?} executable at {:?}",
                    binary_name.as_ref(),
                    bin));
    bin
}

/// Return whether or not the tests are being run with the `--nocapture` flag meaning we want to
/// see more output.
fn nocapture_set() -> bool {
    if env::args().any(|arg| arg == "--nocapture") {
        true
    } else {
        match env::var("RUST_TEST_NOCAPTURE") {
            Ok(val) => &val != "0",
            Err(_) => false,
        }
    }
}

impl TestSup {
    /// Create a new `TestSup` that will listen on randomly-selected
    /// ports for both gossip and HTTP requests so tests run in
    /// parallel don't step on each other.
    ///
    /// See also `new`.
    pub fn new_with_random_ports<R>(fs_root: R,
                                    origin: &str,
                                    pkg_name: &str,
                                    service_group: &str)
                                    -> TestSup
        where R: AsRef<Path>
    {
        // We'll give 10 tries to find a free port number
        let http_port = unclaimed_port(10);
        let butterfly_port = unclaimed_port(10);
        let control_port = unclaimed_port(10);

        TestSup::new(fs_root,
                     origin,
                     pkg_name,
                     service_group,
                     http_port,
                     butterfly_port,
                     control_port)
    }

    /// Bundle up a Habitat Supervisor process along with an
    /// associated Butterfly client for injecting new configuration
    /// values. The Supervisor executable is the one that has been
    /// compiled for the current `cargo test` invocation.
    ///
    /// The Supervisor is configured to run a single package for a
    /// test. This package is assumed to have already been installed
    /// relative to `fs_root` (i.e., the `FS_ROOT` environment
    /// variable, which in our tests will be a randomly-named
    /// temporary directory that this Supervisor will view as `/`.).
    ///
    /// A Butterfly client is also created for interacting with this
    /// Supervisor and package. It is properly configured according to
    /// the value provided for `butterfly_port`. To use it, see the
    /// `apply_config` function.
    ///
    /// (No HTTP interaction with the Supervisor is currently called
    /// for, so we don't have a HTTP client.)
    pub fn new<R>(fs_root: R,
                  origin: &str,
                  pkg_name: &str,
                  service_group: &str,
                  http_port: u16,
                  butterfly_port: u16,
                  control_port: u16)
                  -> TestSup
        where R: AsRef<Path>
    {
        let sup_exe = find_exe("hab-sup");
        let launcher_exe = find_exe("hab-launch");

        let mut cmd = Command::new(&launcher_exe);
        let listen_host = "0.0.0.0";

        let origin = origin.to_string();
        let pkg_name = pkg_name.to_string();
        let service_group = service_group.to_string();

        cmd.env(
            "TESTING_FS_ROOT",
            fs_root.as_ref().to_string_lossy().as_ref(),
        )
        .env("HAB_SUP_BINARY", &sup_exe)
        .env(BLDR_URL_ENVVAR, "http://hab.sup.test")
        .arg("run")
        .arg("--listen-gossip")
        .arg(format!("{}:{}", listen_host, butterfly_port))
        .arg("--listen-http")
        .arg(format!("{}:{}", listen_host, http_port))
        .arg("--listen-ctl")
        .arg(format!("{}:{}", listen_host, control_port))
        // Note: we will have already dropped off the spec files
        // needed to run our test service, so we don't supply a
        // package identifier here
        .stdin(Stdio::null());
        if !nocapture_set() {
            cmd.stdout(Stdio::null());
            cmd.stderr(Stdio::null());
        }

        let bc = test_butterfly::Client::new(&pkg_name, &service_group, butterfly_port);

        TestSup { hab_root: fs_root.as_ref().to_path_buf(),
                  origin_name: origin,
                  package_name: pkg_name,
                  service_group: service_group.to_string(),
                  http_port,
                  butterfly_port,
                  control_port,
                  butterfly_client: bc,
                  cmd,
                  process: None }
    }

    /// Spawn a process actually running the Supervisor.
    pub fn start(&mut self) {
        let child = self.cmd.spawn().expect("Couldn't start the Supervisor!");
        self.process = Some(child);
    }

    /// Stop the Supervisor.
    pub fn stop(&mut self) {
        let mut ports = CLAIMED_PORTS.lock().unwrap();
        ports.remove(&self.http_port);
        ports.remove(&self.butterfly_port);
        ports.remove(&self.control_port);
        self.process
            .take()
            .expect("No process to kill!")
            .kill()
            .expect("Tried to kill Supervisor!");
    }

    /// The equivalent of performing `hab apply` with the given
    /// configuration.
    pub fn apply_config(&mut self, toml_config: &str) { self.butterfly_client.apply(toml_config) }
}

// We kill the Supervisor so you don't have to! We also free up the
// ports used by this Supervisor so other tests can use them.
impl Drop for TestSup {
    fn drop(&mut self) { self.stop(); }
}
