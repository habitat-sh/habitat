// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

use config::Config;
use error::Result;
use hab_net::server::ZMQ_CONTEXT;
use std::sync::{mpsc, Arc, RwLock};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use zmq;

/// In-memory zmq address for LogForwarder
pub const INPROC_ADDR: &'static str = "inproc://logger";

pub struct LogForwarder {
    /// The socket on which log data is received from workers.
    pub intake_sock: zmq::Socket,
    /// The socket from which log data is forwarded to the appropriate
    /// job server.
    pub output_sock: zmq::Socket,
    /// The configuration of the worker server; used to obtain job
    /// server connection information.
    config: Arc<RwLock<Config>>,
}

impl LogForwarder {
    pub fn new(config: Arc<RwLock<Config>>) -> Result<Self> {
        let intake_sock = (**ZMQ_CONTEXT).as_mut().socket(zmq::PULL)?;
        let output_sock = (**ZMQ_CONTEXT).as_mut().socket(zmq::DEALER)?;

        output_sock.set_sndhwm(5000)?;
        output_sock.set_linger(5000)?;
        output_sock.set_immediate(true)?;

        Ok(LogForwarder {
               intake_sock: intake_sock,
               output_sock: output_sock,
               config: config,
           })
    }

    pub fn start(config: Arc<RwLock<Config>>) -> Result<JoinHandle<()>> {
        let (tx, rx) = mpsc::sync_channel(0);
        let handle = thread::Builder::new()
            .name("log".to_string())
            .spawn(move || {
                       let mut log = Self::new(config).unwrap();
                       log.run(tx).unwrap();
                   })
            .unwrap();
        match rx.recv() {
            Ok(()) => Ok(handle),
            Err(e) => panic!("log thread startup error, err={}", e),
        }
    }

    pub fn run(&mut self, rz: mpsc::SyncSender<()>) -> Result<()> {
        {
            let cfg = self.config.read().unwrap();
            let addrs = cfg.jobsrv_addrs();
            if addrs.len() == 1 {
                let (_, _, ref log) = addrs[0];
                println!("Connecting to Job Server Log port, {}", log);
                self.output_sock.connect(&log)?;
            } else {
                panic!("Routing logs to more than one Job Server is not yet implemented");
            }
        }
        self.intake_sock.bind(INPROC_ADDR)?;

        // Signal back to the spawning process that we're good
        rz.send(()).unwrap();

        // This hacky sleep is recommended and required by zmq for
        // connections to establish
        thread::sleep(Duration::from_millis(100));

        // Basically just need to pass things through... proxy time!
        // If we ever have multiple JobServers these need to be sent
        // to, then we might need some additional logic.
        zmq::proxy(&mut self.intake_sock, &mut self.output_sock)?;
        Ok(())
    }
}
