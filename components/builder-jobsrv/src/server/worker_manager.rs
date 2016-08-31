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

use std::sync::{mpsc, Arc, RwLock};
use std::time::{Duration, Instant};
use std::thread::{self, JoinHandle};

use dbcache::InstaSet;
use linked_hash_map::LinkedHashMap;
use hab_net::config::ToAddrString;
use hab_net::server::ZMQ_CONTEXT;
use protobuf::{parse_from_bytes, Message};
use protocol::jobsrv;
use zmq;

use config::Config;
use data_store::DataStore;
use error::Result;

const WORKER_MGR_ADDR: &'static str = "inproc://work-manager";
const WORKER_TIMEOUT_MS: u64 = 33_000;

pub struct WorkerMgrClient {
    socket: zmq::Socket,
}

impl WorkerMgrClient {
    pub fn connect(&mut self) -> Result<()> {
        try!(self.socket.connect(WORKER_MGR_ADDR));
        Ok(())
    }

    pub fn notify_work(&mut self) -> Result<()> {
        try!(self.socket.send(&[1], 0));
        Ok(())
    }
}

impl Default for WorkerMgrClient {
    fn default() -> WorkerMgrClient {
        let socket = (**ZMQ_CONTEXT).as_mut().socket(zmq::DEALER).unwrap();
        socket.set_sndhwm(1).unwrap();
        socket.set_linger(0).unwrap();
        socket.set_immediate(true).unwrap();
        WorkerMgrClient { socket: socket }
    }
}

pub struct WorkerMgr {
    config: Arc<RwLock<Config>>,
    datastore: Arc<Box<DataStore>>,
    hb_sock: zmq::Socket,
    rq_sock: zmq::Socket,
    work_mgr_sock: zmq::Socket,
    msg: zmq::Message,
    workers: LinkedHashMap<String, Instant>,
}

impl WorkerMgr {
    pub fn new(config: Arc<RwLock<Config>>, datastore: Arc<Box<DataStore>>) -> Result<Self> {
        let hb_sock = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::SUB));
        let rq_sock = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::ROUTER));
        let work_mgr_sock = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::DEALER));
        try!(rq_sock.set_router_mandatory(true));
        try!(hb_sock.set_subscribe(&[]));
        try!(work_mgr_sock.set_rcvhwm(1));
        try!(work_mgr_sock.set_linger(0));
        try!(work_mgr_sock.set_immediate(true));
        let msg = try!(zmq::Message::new());
        Ok(WorkerMgr {
            config: config,
            datastore: datastore,
            hb_sock: hb_sock,
            rq_sock: rq_sock,
            work_mgr_sock: work_mgr_sock,
            msg: msg,
            workers: LinkedHashMap::new(),
        })
    }

    pub fn start(cfg: Arc<RwLock<Config>>, ds: Arc<Box<DataStore>>) -> Result<JoinHandle<()>> {
        let (tx, rx) = mpsc::sync_channel(1);
        let handle = thread::Builder::new()
            .name("worker-manager".to_string())
            .spawn(move || {
                let mut manager = Self::new(cfg, ds).unwrap();
                manager.run(tx).unwrap();
            })
            .unwrap();
        match rx.recv() {
            Ok(()) => Ok(handle),
            Err(e) => panic!("worker-manager thread startup error, err={}", e),
        }
    }

    fn run(&mut self, rz: mpsc::SyncSender<()>) -> Result<()> {
        try!(self.work_mgr_sock.bind(WORKER_MGR_ADDR));
        {
            let cfg = self.config.read().unwrap();
            println!("Listening for commands on {}",
                     cfg.worker_command_addr.to_addr_string());
            try!(self.rq_sock.bind(&cfg.worker_command_addr.to_addr_string()));
            println!("Listening for heartbeats on {}",
                     cfg.worker_heartbeat_addr.to_addr_string());
            try!(self.hb_sock.bind(&cfg.worker_heartbeat_addr.to_addr_string()));
        }
        let mut hb_sock = false;
        let mut rq_sock = false;
        let mut work_mgr_sock = false;
        rz.send(()).unwrap();
        loop {
            {
                let timeout = self.poll_timeout();
                let mut items = [self.hb_sock.as_poll_item(1),
                                 self.rq_sock.as_poll_item(1),
                                 self.work_mgr_sock.as_poll_item(1)];
                // Poll until timeout or message is received. Checking for the zmq::POLLIN flag on
                // a poll item's revents will let you know if you have received a message or not
                // on that socket.
                try!(zmq::poll(&mut items, timeout));
                if (items[0].get_revents() & zmq::POLLIN) > 0 {
                    hb_sock = true;
                }
                if (items[1].get_revents() & zmq::POLLIN) > 0 {
                    rq_sock = true;
                }
                if (items[2].get_revents() & zmq::POLLIN) > 0 {
                    work_mgr_sock = true;
                }
            }
            if hb_sock {
                try!(self.process_heartbeat());
                hb_sock = false;
            }
            self.expire_workers();
            if rq_sock {
                try!(self.process_job_status());
                rq_sock = false;
            }
            if work_mgr_sock {
                try!(self.distribute_work());
            }
        }
        Ok(())
    }

    fn poll_timeout(&self) -> i64 {
        if let Some((_, expiry)) = self.workers.front() {
            let timeout = *expiry - Instant::now();
            (timeout.as_secs() as i64 * 1000) + (timeout.subsec_nanos() as i64 / 1000 / 1000)
        } else {
            -1
        }
    }

    fn distribute_work(&mut self) -> Result<()> {
        loop {
            let job = match self.datastore.job_queue.peek() {
                Ok(Some(job)) => job,
                Ok(None) => break,
                Err(e) => return Err(e),
            };
            match self.workers.pop_front() {
                Some((worker, _)) => {
                    debug!("sending work, worker={:?}, job={:?}", worker, job);
                    if self.rq_sock.send_str(&worker, zmq::SNDMORE).is_err() {
                        debug!("failed to send, worker went away, worker={:?}", worker);
                        continue;
                    }
                    if self.rq_sock.send(&[], zmq::SNDMORE).is_err() {
                        debug!("failed to send, worker went away, worker={:?}", worker);
                        continue;
                    }
                    if self.rq_sock.send(&job.write_to_bytes().unwrap(), 0).is_err() {
                        debug!("failed to send, worker went away, worker={:?}", worker);
                        continue;
                    }
                    // JW TODO: Wait for response back to ensure we can dequeue this. If state
                    // returned is not processing then we move onto next worker and assume this
                    // worker is no longer valid. Put work back on queue.
                    try!(self.datastore.job_queue.dequeue());
                    // Consume the to-do work notification if the queue is empty.
                    if try!(self.datastore.job_queue.peek()).is_none() {
                        try!(self.work_mgr_sock.recv(&mut self.msg, 0));
                    }
                    break;
                }
                None => break,
            }
        }
        Ok(())
    }

    fn expire_workers(&mut self) {
        let now = Instant::now();
        loop {
            if let Some((_, expiry)) = self.workers.front() {
                if expiry >= &now {
                    break;
                }
            } else {
                break;
            }
            let worker = self.workers.pop_front();
            debug!("expiring worker due to inactivity, worker={:?}", worker);
        }
    }

    fn process_heartbeat(&mut self) -> Result<()> {
        try!(self.hb_sock.recv(&mut self.msg, 0));
        let heartbeat: jobsrv::Heartbeat = try!(parse_from_bytes(&self.msg));
        debug!("heartbeat={:?}", heartbeat);
        match heartbeat.get_state() {
            jobsrv::WorkerState::Ready => {
                let now = Instant::now();
                let expiry = now + Duration::from_millis(WORKER_TIMEOUT_MS);
                self.workers.insert(heartbeat.get_endpoint().to_string(), expiry);
            }
            jobsrv::WorkerState::Busy => {
                self.workers.remove(heartbeat.get_endpoint());
            }
        }
        Ok(())
    }

    fn process_job_status(&mut self) -> Result<()> {
        // Pop message delimiter
        try!(self.rq_sock.recv(&mut self.msg, 0));
        // Pop message body
        try!(self.rq_sock.recv(&mut self.msg, 0));
        let job: jobsrv::Job = try!(parse_from_bytes(&self.msg));
        debug!("job_status={:?}", job);
        try!(self.datastore.jobs.update(&job));
        Ok(())
    }
}
