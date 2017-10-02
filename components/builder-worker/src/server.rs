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

use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::Arc;

use hab_net;
use hab_net::socket::DEFAULT_CONTEXT;
use protocol::{self, message};
use zmq;

use config::Config;
use error::{Error, Result};
use feat;
use heartbeat::{HeartbeatCli, HeartbeatMgr};
use log_forwarder::LogForwarder;
use runner::{RunnerCli, RunnerMgr};

enum State {
    Ready,
    Busy,
}

impl Default for State {
    fn default() -> State {
        State::Ready
    }
}

pub struct Server {
    config: Arc<Config>,
    /// Dealer Socket connected to JobSrv
    fe_sock: zmq::Socket,
    hb_cli: HeartbeatCli,
    runner_cli: RunnerCli,
    state: State,
    msg: zmq::Message,
    net_ident: Arc<String>,
}

impl Server {
    pub fn new(config: Config) -> Result<Self> {
        let net_ident = hab_net::socket::srv_ident();
        let fe_sock = (**DEFAULT_CONTEXT).as_mut().socket(zmq::DEALER)?;
        let hb_cli = HeartbeatCli::new(net_ident.clone());
        let runner_cli = RunnerCli::new();
        fe_sock.set_identity(net_ident.as_bytes())?;
        Ok(Server {
            config: Arc::new(config),
            fe_sock: fe_sock,
            hb_cli: hb_cli,
            runner_cli: runner_cli,
            state: State::default(),
            msg: zmq::Message::new()?,
            net_ident: Arc::new(net_ident),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        if self.config.auth_token.is_empty() {
            error!(
                "ERROR: No 'auth_token' config value specified which prevents the \
                   worker from download fetching signing keys."
            );
            return Err(Error::NoAuthTokenError);
        }
        self.enable_features_from_config();

        HeartbeatMgr::start(&self.config, (&*self.net_ident).clone())?;
        RunnerMgr::start(self.config.clone(), self.net_ident.clone())?;
        LogForwarder::start(&self.config)?;
        self.hb_cli.connect()?;
        self.runner_cli.connect()?;
        for (_, queue, _) in self.config.jobsrv_addrs() {
            println!("Connecting to job queue, {}", queue);
            self.fe_sock.connect(&queue)?;
        }

        let mut fe_msg = false;
        let mut runner_msg = false;
        info!("builder-worker is ready to go.");
        loop {
            {
                let mut items = [
                    self.fe_sock.as_poll_item(1),
                    self.runner_cli.as_poll_item(1),
                ];
                zmq::poll(&mut items, -1)?;
                if items[0].get_revents() & zmq::POLLIN > 0 {
                    fe_msg = true;
                }
                if items[1].get_revents() & zmq::POLLIN > 0 {
                    runner_msg = true;
                }
            }
            if runner_msg {
                {
                    let reply = self.runner_cli.recv_complete()?;
                    self.fe_sock.send(reply, 0)?;
                }
                self.set_ready()?;
                runner_msg = false;
            }
            if fe_msg {
                self.fe_sock.recv(&mut self.msg, 0)?;
                self.fe_sock.recv(&mut self.msg, 0)?;
                match self.state {
                    State::Ready => {
                        self.runner_cli.send(&self.msg)?;
                        {
                            let reply = self.runner_cli.recv_ack()?;
                            self.fe_sock.send(reply, 0)?;
                        }
                        self.set_busy()?;
                    }
                    State::Busy => {
                        let mut reply = message::decode::<protocol::jobsrv::Job>(&self.msg)?;
                        reply.set_state(protocol::jobsrv::JobState::Rejected);
                        self.fe_sock.send(&message::encode(&reply)?, 0)?;
                    }
                }
                fe_msg = false;
            }
        }
    }

    fn set_busy(&mut self) -> Result<()> {
        self.hb_cli.set_busy()?;
        self.state = State::Busy;
        Ok(())
    }

    fn set_ready(&mut self) -> Result<()> {
        self.hb_cli.set_ready()?;
        self.state = State::Ready;
        Ok(())
    }

    fn enable_features_from_config(&self) {
        let features: HashMap<_, _> =
            HashMap::from_iter(vec![("LIST", feat::List), ("DOCKER", feat::Docker)]);
        let features_enabled = self.config.features_enabled.split(",").map(|f| {
            f.trim().to_uppercase()
        });
        for key in features_enabled {
            if features.contains_key(key.as_str()) {
                info!("Enabling feature: {}", key);
                feat::enable(features.get(key.as_str()).unwrap().clone());
            }
        }

        if feat::is_enabled(feat::List) {
            println!("Listing possible feature flags: {:?}", features.keys());
            println!("Enable features by populating 'features_enabled' in config");
        }
    }
}

pub fn run(config: Config) -> Result<()> {
    Server::new(config)?.run()
}
