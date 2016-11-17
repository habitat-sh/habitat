// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use time::SteadyTime;

use error::Result;
use hcore::util::sys::ip;
use hsup::manager::signals;

use super::Config;
use super::task::{Task, ExecContext, ExecParams};

/// we check child processes at most once every MINIMUM_LOOP_TIME_MS
static MINIMUM_LOOP_TIME_MS: i64 = 200;
static LOGKEY: &'static str = "CTRL";
pub const FIRST_GOSSIP_PORT: u16 = 9000;
pub const FIRST_HTTP_PORT: u16 = 8000;

pub struct Controller {
    pub config: Config,
    pub exec_ctx: ExecContext,
    pub children: Option<Vec<Task>>,
}

impl Controller {
    pub fn new(config: Config, exec_ctx: ExecContext) -> Controller {
        signals::init();

        Controller {
            config: config,
            exec_ctx: exec_ctx,
            children: None,
        }
    }

    /// iterate through the config ServiceDefs and create `Task`
    /// instances. A Controller contains "all the tasks", so
    /// it calculate gossip_port + http_port #s accordingly.
    pub fn create_children(&mut self) -> Result<()> {
        let mut children = Vec::new();
        let mut next_gossip_port = FIRST_GOSSIP_PORT;
        let mut next_http_port = FIRST_HTTP_PORT;

        let default_ip = try!(ip()).to_string();
        let listen_ip = try!(Ipv4Addr::from_str(&default_ip));


        let mut initial_peer: Option<SocketAddrV4> = self.config.dir_sup_listen;

        for sd in &self.config.service_defs {
            let exec_ctx = self.exec_ctx.clone();
            let exec_params = ExecParams::new(SocketAddrV4::new(listen_ip, next_gossip_port),
                                              SocketAddrV4::new(listen_ip, next_http_port),
                                              initial_peer);

            // after the first iteration, each child will connect to the previous
            initial_peer = Some(exec_params.gossip_listen.clone());

            let dc = Task::new(exec_ctx, exec_params, sd.clone());
            children.push(dc);

            // this will have to be more intelligent if we
            // let users define gossip/http ports
            next_gossip_port += 1;
            next_http_port += 1;
        }
        self.children = Some(children);
        Ok(())
    }

    /// Process config to create children, then run in a loop forever.
    pub fn start(&mut self) -> Result<()> {
        try!(self.create_children());

        if let None = self.children {
            outputln!("No services defined");
            return Ok(());
        } else if self.children.as_ref().unwrap().len() == 0 {
            outputln!("No services defined");
            return Ok(());
        }

        loop {
            let start_time = SteadyTime::now();

            // do the main loop "stuff"
            if !try!(self.next_iteration()) {
                // we received a signal, break out of this loop
                break;
            }

            // Slow down our loop
            let elapsed_time = SteadyTime::now() - start_time;
            let elapsed_millis = elapsed_time.num_milliseconds();

            if elapsed_millis < MINIMUM_LOOP_TIME_MS {
                thread::sleep(Duration::from_millis((MINIMUM_LOOP_TIME_MS - elapsed_millis) as u64));
            }
        }
        Ok(())
    }

    /// This is called at each iteration in the self::start() loop.
    /// It's pulled out into it's own function so it can be tested.
    pub fn next_iteration(&mut self) -> Result<bool> {
        match signals::check_for_signal() {
            Some(signals::SignalEvent::Shutdown) => {
                let mut children = self.children.as_mut().unwrap();
                for child in children.iter_mut() {
                    if let Some(pid) = child.pid {
                        outputln!("Sending SIGTERM to child {} (pid {})",
                                  &child.service_def.to_string(),
                                  &pid);
                        if let Err(e) =
                               signals::send_signal(pid, signals::unix::Signal::SIGTERM as u32) {
                            outputln!("Error sending SIGTERM to {} (pid {}): {}",
                                      &child.service_def.to_string(),
                                      &pid,
                                      e);

                        }
                    }
                }
                outputln!("Shutting down");
                return Ok(false);
            }
            Some(signals::SignalEvent::Passthrough(signal_code)) => {
                let mut children = self.children.as_mut().unwrap();
                for child in children.iter_mut() {
                    if let Some(pid) = child.pid {
                        outputln!("Sending {:?} to child {} (pid {})",
                                  &signal_code,
                                  &child.service_def.to_string(),
                                  &pid);
                        if let Err(e) = signals::send_signal(pid, signal_code) {
                            outputln!("Error sending {:?} to {} (pid {}): {}",
                                      &signal_code,
                                      &child.service_def.to_string(),
                                      &pid,
                                      e);

                        }
                    }
                }
            }
            None => {}
        }
        // leaving this here as I unwrap a couple lines down
        // and I don't want to try and defeat the type system
        if let None = self.children {
            debug!("No children");
            return Ok(false);
        }
        let mut children = self.children.as_mut().unwrap();
        for child in children.iter_mut() {
            if let Err(e) = child.check_process() {
                outputln!("Failed to check child process {}: {}",
                          &child.service_def.to_string(),
                          e);
            }
            if child.is_down() {
                match child.start() {
                    // the Task prints out a better "Started" message than
                    // we could (including ports etc)
                    Ok(_) => debug!("Started {}", &child.service_def.to_string()),
                    Err(e) => {
                        outputln!("Failed to start {}: {}", &child.service_def.to_string(), e)
                    }
                };
            }
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use time;
    use toml;

    use hcore::config::ConfigFile;
    use hcore::util::sys::ip;
    use task::ExecContext;
    use config::Config;
    use super::*;

    fn get_test_config() -> Config {
        let service_toml = r#"
        [sys]
        gossip_ip="192.168.1.2"
        gossip_port=9876
        [cfg.services.core.redis.somegroup.someorg]
        start = "foo"
        [cfg.services.core.rngd.foo.someorg]
        start = "bar"
        [cfg.services.myorigin.xyz.foo.otherorg]
        [foo]
        "#;

        let root: toml::Value = service_toml.parse().unwrap();
        Config::from_toml(root).unwrap()
    }

    // call a closure in a loop until it returns true
    // or timeout after 30 seconds and return false
    pub fn wait_until_true<F, T>(c: &mut T, some_fn: F) -> bool
        where F: Fn(&mut T) -> bool
    {
        let wait_duration = time::Duration::seconds(30);
        let current_time = time::now_utc().to_timespec();
        let stop_time = current_time + wait_duration;
        while time::now_utc().to_timespec() < stop_time {
            if some_fn(c) {
                return true;
            }
        }
        false
    }

    /// parse some config, make sure the arguments are generated correctly
    /// and then start some child processes and see if they restart
    /// when killed. We don't start full hab-sup processes.
    ///
    /// NOTE: The controller uses an instance of SignalNotifier,
    /// which is a Wonder actor that catches OS signals for us.
    /// It can only be started once per process (in "this" test process),
    /// so we can't currently share it between tests. Also, as tests
    /// are run concurrently, it wouldn't be possible to use a shared
    /// instance of SignalNotifier anyways.
    #[test]
    fn test_controller_with_sup_parent() {
        let mut ec = ExecContext::default();
        ec.sup_path = PathBuf::from("/bin/false");
        ec.service_root = PathBuf::from("/tmp");

        let config = get_test_config();
        let mut controller = Controller::new(config, ec);
        controller.create_children().unwrap();
        assert_eq!(3, controller.children.as_ref().unwrap().len());

        let test_ip = ip().unwrap().to_string();
        {

            let child = &controller.children.as_ref().unwrap()[0];
            let args = child.get_cmd_args().unwrap();
            assert_eq!(args.as_slice(),
                       ["start",
                        "core/redis",
                        "foo",
                        "--listen-peer",
                        format!("{}:9000", test_ip).as_str(),
                        "--listen-http",
                        format!("{}:8000", test_ip).as_str(),
                        "--group",
                        "somegroup",
                        "--org",
                        "someorg",
                        "--peer",
                        "192.168.1.2:9876"]);
        }
        {
            let child = &controller.children.as_ref().unwrap()[1];
            let args = child.get_cmd_args().unwrap();
            assert_eq!(args.as_slice(),
                       ["start",
                        "core/rngd",
                        "bar",
                        "--listen-peer",
                        // did we increment the port?
                        format!("{}:9001", test_ip).as_str(),
                        "--listen-http",
                        // did we increment the port?
                        format!("{}:8001", test_ip).as_str(),
                        "--group",
                        "foo",
                        "--org",
                        "someorg",
                        "--peer",
                        // is the peer set to the previous port?
                        format!("{}:9000", test_ip).as_str()]);
        }

        {
            let child = &controller.children.as_ref().unwrap()[2];
            let args = child.get_cmd_args().unwrap();

            assert_eq!(args.as_slice(),
                       ["start",
                        "myorigin/xyz",
                        "--listen-peer",
                        // did we increment the port?
                        format!("{}:9002", test_ip).as_str(),
                        "--listen-http",
                        // did we increment the port?
                        format!("{}:8002", test_ip).as_str(),
                        "--group",
                        "foo",
                        "--org",
                        "otherorg",
                        "--peer",
                        // is the peer set to the previous port?
                        format!("{}:9001", test_ip).as_str()]);
        }

        controller.next_iteration().unwrap();

        assert_eq!(1, controller.children.as_ref().unwrap()[0].starts);
        // We gave the child process bad args, so it won't start.
        // Lets wait for 30 seconds to see if we register restarts for the
        // children
        // let killpid = &controller.children.as_ref().unwrap()[0].pid.unwrap();
        // signals::send_signal_to_pid(*killpid, signals::Signal::SIGKILL).unwrap();
        assert!(wait_until_true(&mut controller, |c| {
            c.next_iteration().unwrap();
            c.children.as_ref().unwrap()[0].starts > 1 &&
            c.children.as_ref().unwrap()[1].starts > 1 &&
            c.children.as_ref().unwrap()[2].starts > 1
        }));

    }
}
