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
use std::time::{Duration, Instant};
use std::thread::{self, JoinHandle};

use linked_hash_map::LinkedHashMap;
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
    datastore: DataStore,
    hb_sock: zmq::Socket,
    rq_sock: zmq::Socket,
    work_mgr_sock: zmq::Socket,
    msg: zmq::Message,
    workers: LinkedHashMap<String, Instant>,
}

impl WorkerMgr {
    pub fn new(config: Arc<RwLock<Config>>, datastore: DataStore) -> Result<Self> {
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

    pub fn start(cfg: Arc<RwLock<Config>>, ds: DataStore) -> Result<JoinHandle<()>> {
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
            let worker_command = cfg.net.worker_command_addr();
            let worker_heartbeat = cfg.net.worker_heartbeat_addr();
            println!("Listening for commands on {}", worker_command);
            self.rq_sock.bind(&worker_command)?;
            println!("Listening for heartbeats on {}", worker_heartbeat);
            self.hb_sock.bind(&worker_heartbeat)?;
        }
        let mut hb_sock = false;
        let mut rq_sock = false;
        let mut work_mgr_sock = false;
        let mut process_work = false;
        rz.send(()).unwrap();

        // Reset any Dispatched jobs to Pending
        self.datastore.reset_jobs()?;

        loop {
            {
                let timeout = self.poll_timeout();
                let mut items = [
                    self.hb_sock.as_poll_item(1),
                    self.rq_sock.as_poll_item(1),
                    self.work_mgr_sock.as_poll_item(1),
                ];
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
                process_work = try!(self.process_heartbeat());
                hb_sock = false;
            }
            self.expire_workers();
            if rq_sock {
                try!(self.process_job_status());
                rq_sock = false;
            }
            if work_mgr_sock {
                process_work = true;
                work_mgr_sock = false;
                try!(self.work_mgr_sock.recv(&mut self.msg, 0));
            }

            // Handle potential work in pending_jobs queue
            if process_work {
                try!(self.process_work());
            }
        }
    }

    fn poll_timeout(&self) -> i64 {
        if let Some((_, expiry)) = self.workers.front() {
            let timeout = *expiry - Instant::now();
            (timeout.as_secs() as i64 * 1000) + (timeout.subsec_nanos() as i64 / 1000 / 1000)
        } else {
            -1
        }
    }

    fn process_work(&mut self) -> Result<()> {
        loop {
            // Take one job from the pending list
            let mut jobs = self.datastore.pending_jobs(1)?;
            // 0 means there are no pending jobs, so we can exit
            if jobs.len() == 0 {
                debug!("process_work, no pending jobs");
                break;
            }
            // This unwrap is fine, because we just checked our length
            let mut job = jobs.pop().unwrap();

            match self.workers.pop_front() {
                Some((worker, _)) => {
                    debug!("sending work, worker={:?}, job={:?}", worker, job);
                    if self.rq_sock.send_str(&worker, zmq::SNDMORE).is_err() {
                        debug!("failed to send, worker went away, worker={:?}", worker);
                        job.set_state(jobsrv::JobState::Pending);
                        self.datastore.update_job(&job)?;
                        continue;
                    }
                    if self.rq_sock.send(&[], zmq::SNDMORE).is_err() {
                        debug!("failed to send, worker went away, worker={:?}", worker);
                        job.set_state(jobsrv::JobState::Pending);
                        self.datastore.update_job(&job)?;
                        continue;
                    }
                    if self.rq_sock
                        .send(&job.write_to_bytes().unwrap(), 0)
                        .is_err()
                    {
                        debug!("failed to send, worker went away, worker={:?}", worker);
                        job.set_state(jobsrv::JobState::Pending);
                        self.datastore.update_job(&job)?;
                        continue;
                    }
                }
                None => {
                    debug!("no workers available - bailing for now");
                    job.set_state(jobsrv::JobState::Pending);
                    self.datastore.update_job(&job)?;
                    return Ok(());
                }
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

    fn process_heartbeat(&mut self) -> Result<bool> {
        try!(self.hb_sock.recv(&mut self.msg, 0));
        let heartbeat: jobsrv::Heartbeat = try!(parse_from_bytes(&self.msg));
        debug!("heartbeat={:?}", heartbeat);
        let result = match heartbeat.get_state() {
            jobsrv::WorkerState::Ready => {
                let now = Instant::now();
                let expiry = now + Duration::from_millis(WORKER_TIMEOUT_MS);
                self.workers.insert(
                    heartbeat.get_endpoint().to_string(),
                    expiry,
                );
                true
            }
            jobsrv::WorkerState::Busy => {
                self.workers.remove(heartbeat.get_endpoint());
                false
            }
        };
        Ok(result)
    }

    fn process_job_status(&mut self) -> Result<()> {
        // Pop message delimiter
        try!(self.rq_sock.recv(&mut self.msg, 0));
        // Pop message body
        try!(self.rq_sock.recv(&mut self.msg, 0));
        let job: jobsrv::Job = try!(parse_from_bytes(&self.msg));
        debug!("job_status={:?}", job);
        try!(self.datastore.update_job(&job));

        Ok(())
    }
}
