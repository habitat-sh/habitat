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

use std::sync::{mpsc, Arc, RwLock};
use std::thread::{self, JoinHandle};

use protobuf::parse_from_bytes;
use hab_net::server::ZMQ_CONTEXT;
use zmq;

use protocol::jobsrv;
use config::Config;
use data_store::DataStore;
use error::Result;

const SCHEDULER_ADDR: &'static str = "inproc://scheduler";

pub struct ScheduleClient {
    socket: zmq::Socket,
}

impl ScheduleClient {
    pub fn connect(&mut self) -> Result<()> {
        try!(self.socket.connect(SCHEDULER_ADDR));
        Ok(())
    }

    pub fn notify_work(&mut self) -> Result<()> {
        try!(self.socket.send(&[1], 0));
        Ok(())
    }
}

impl Default for ScheduleClient {
    fn default() -> ScheduleClient {
        let socket = (**ZMQ_CONTEXT).as_mut().socket(zmq::DEALER).unwrap();
        socket.set_sndhwm(1).unwrap();
        socket.set_linger(0).unwrap();
        socket.set_immediate(true).unwrap();
        ScheduleClient { socket: socket }
    }
}

pub struct ScheduleMgr {
    config: Arc<RwLock<Config>>,
    datastore: Arc<RwLock<DataStore>>,
    work_sock: zmq::Socket,
    status_sock: zmq::Socket,
    msg: zmq::Message,
}

impl ScheduleMgr {
    pub fn new(config: Arc<RwLock<Config>>, datastore: Arc<RwLock<DataStore>>) -> Result<Self> {
        let status_sock = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::SUB));
        let work_sock = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::DEALER));
        try!(status_sock.set_subscribe(&[]));
        try!(work_sock.set_rcvhwm(1));
        try!(work_sock.set_linger(0));
        try!(work_sock.set_immediate(true));
        let msg = try!(zmq::Message::new());
        Ok(ScheduleMgr {
            config: config,
            datastore: datastore,
            work_sock: work_sock,
            status_sock: status_sock,
            msg: msg,
        })
    }

    pub fn start(cfg: Arc<RwLock<Config>>, ds: Arc<RwLock<DataStore>>) -> Result<JoinHandle<()>> {
        let (tx, rx) = mpsc::sync_channel(1);
        let handle = thread::Builder::new()
            .name("scheduler".to_string())
            .spawn(move || {
                let mut schedule_mgr = Self::new(cfg, ds).unwrap();
                schedule_mgr.run(tx).unwrap();
            })
            .unwrap();
        match rx.recv() {
            Ok(()) => Ok(handle),
            Err(e) => panic!("scheduler thread startup error, err={}", e),
        }
    }

    fn run(&mut self, rz: mpsc::SyncSender<()>) -> Result<()> {
        try!(self.work_sock.bind(SCHEDULER_ADDR));
        {
            let cfg = self.config.read().unwrap();

            for addr in cfg.jobsrv_addrs() {
                println!("Connecting to job status publisher: {}", addr);
                try!(self.status_sock.connect(&addr));
            }
        }

        let mut status_sock = false;
        let mut work_sock = false;
        rz.send(()).unwrap();
        loop {
            {
                let mut items = [self.work_sock.as_poll_item(1), self.status_sock.as_poll_item(1)];
                try!(zmq::poll(&mut items, -1));

                if (items[0].get_revents() & zmq::POLLIN) > 0 {
                    work_sock = true;
                }
                if (items[1].get_revents() & zmq::POLLIN) > 0 {
                    status_sock = true;
                }
            }

            if work_sock {
                try!(self.process_work());
                work_sock = false;
            }

            if status_sock {
                try!(self.process_status());
                status_sock = false;
            }
        }
    }

    fn process_work(&mut self) -> Result<()> {
        println!("Process work called");
        try!(self.work_sock.recv(&mut self.msg, 0));

        // TBD: Scheduling work will happen here
        Ok(())
    }

    fn process_status(&mut self) -> Result<()> {
        try!(self.status_sock.recv(&mut self.msg, 0));
        let job: jobsrv::Job = try!(parse_from_bytes(&self.msg));
        println!("Received job status: {:?}", job);

        let mut ds = self.datastore.write().unwrap();
        match ds.find_group_for_job(&job) {
            Some(group) => ds.update_group_job(&group, &job).unwrap(),
            None => {
                println!("Did not find any group for job: {}", job.get_id());
            }
        }

        Ok(())
    }
}
