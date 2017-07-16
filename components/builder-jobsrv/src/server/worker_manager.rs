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
use std::sync::{mpsc, Arc, RwLock};
use std::time::{Duration, Instant};
use std::thread::{self, JoinHandle};

use bldr_core;
use hab_net::socket::DEFAULT_CONTEXT;
use hab_net::routing::Broker;
use linked_hash_map::LinkedHashMap;
use protobuf::{parse_from_bytes, Message, RepeatedField};

use protocol::jobsrv;
use protocol::originsrv::{OriginIntegrationRequest, OriginIntegrationResponse};
use protocol::net::{self, ErrCode};
use zmq;

use config::Config;
use data_store::DataStore;
use error::Result;

const WORKER_MGR_ADDR: &'static str = "inproc://work-manager";
const WORKER_TIMEOUT_MS: u64 = 33_000;
const JOB_TIMEOUT_MS: u64 = 5_400_000;
const DEFAULT_POLL_TIMEOUT_MS: i64 = 60_000;

pub struct WorkerMgrClient {
    socket: zmq::Socket,
}

impl WorkerMgrClient {
    pub fn connect(&mut self) -> Result<()> {
        self.socket.connect(WORKER_MGR_ADDR)?;
        Ok(())
    }

    pub fn notify_work(&mut self) -> Result<()> {
        self.socket.send(&[1], 0)?;
        Ok(())
    }
}

impl Default for WorkerMgrClient {
    fn default() -> WorkerMgrClient {
        let socket = (**DEFAULT_CONTEXT).as_mut().socket(zmq::DEALER).unwrap();
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
    ready_workers: LinkedHashMap<String, Instant>,
    busy_workers: LinkedHashMap<String, Instant>,
    jobs: LinkedHashMap<u64, Instant>,
    worker_map: HashMap<String, u64>,
}

impl WorkerMgr {
    pub fn new(config: Arc<RwLock<Config>>, datastore: DataStore) -> Result<Self> {
        let hb_sock = (**DEFAULT_CONTEXT).as_mut().socket(zmq::SUB)?;
        let rq_sock = (**DEFAULT_CONTEXT).as_mut().socket(zmq::ROUTER)?;
        let work_mgr_sock = (**DEFAULT_CONTEXT).as_mut().socket(zmq::DEALER)?;
        rq_sock.set_router_mandatory(true)?;
        hb_sock.set_subscribe(&[])?;
        work_mgr_sock.set_rcvhwm(1)?;
        work_mgr_sock.set_linger(0)?;
        work_mgr_sock.set_immediate(true)?;
        let msg = zmq::Message::new()?;
        Ok(WorkerMgr {
            config: config,
            datastore: datastore,
            hb_sock: hb_sock,
            rq_sock: rq_sock,
            work_mgr_sock: work_mgr_sock,
            msg: msg,
            ready_workers: LinkedHashMap::new(),
            busy_workers: LinkedHashMap::new(),
            jobs: LinkedHashMap::new(),
            worker_map: HashMap::new(),
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
        self.work_mgr_sock.bind(WORKER_MGR_ADDR)?;
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
                zmq::poll(&mut items, timeout)?;
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
                process_work = self.process_heartbeat()?;
                hb_sock = false;
            }
            self.expire_workers()?;
            self.expire_jobs()?;
            if rq_sock {
                self.process_job_status()?;
                rq_sock = false;
            }
            if work_mgr_sock {
                process_work = true;
                work_mgr_sock = false;
                self.work_mgr_sock.recv(&mut self.msg, 0)?;
            }

            // Handle potential work in pending_jobs queue
            if process_work {
                self.process_work()?;
            }
        }
    }

    fn poll_timeout(&self) -> i64 {
        let now = Instant::now();
        let timeout;

        if let Some((_, expiry)) = self.ready_workers.front() {
            // uh-oh. our expiration date is in the past. it's supposed to be in the
            // future. blindly subtracting now from this will panic the current
            // thread, since Instant's are supposed to monotonically increase.
            // let's just timeout immediately instead.
            if expiry < &now {
                return 0;
            } else {
                timeout = *expiry - now;
            }

            (timeout.as_secs() as i64 * 1000) + (timeout.subsec_nanos() as i64 / 1000 / 1000)
        } else {
            DEFAULT_POLL_TIMEOUT_MS
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
            self.add_integrations_to_job(&mut job);

            match self.ready_workers.pop_front() {
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
                    let expiry = Instant::now() + Duration::from_millis(JOB_TIMEOUT_MS);
                    self.jobs.insert(job.get_id(), expiry);
                    self.worker_map.insert(worker, job.get_id());
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

    fn add_integrations_to_job(&mut self, job: &mut jobsrv::Job) {
        let key_dir = {
            let cfg = self.config.read().unwrap();
            cfg.key_dir.clone()
        };
        let mut conn = Broker::connect().unwrap();
        let mut integrations = RepeatedField::new();
        let mut integration_request = OriginIntegrationRequest::new();
        let origin = job.get_project().get_origin_name().to_string();
        integration_request.set_origin(origin);

        match conn.route::<OriginIntegrationRequest, OriginIntegrationResponse>(
            &integration_request,
        ) {
            Ok(oir) => {
                for i in oir.get_integrations() {
                    let mut oi = i.clone();
                    let plaintext = match bldr_core::integrations::decrypt(
                        key_dir.to_str().unwrap(),
                        i.get_body(),
                    ) {
                        Ok(p) => p,
                        Err(e) => {
                            debug!("Error decrypting integration. e = {:?}", e);
                            continue;
                        }
                    };
                    oi.set_body(plaintext);
                    integrations.push(oi);
                }

                job.set_integrations(integrations);
            }
            Err(e) => {
                debug!("Error fetching integrations. e = {:?}", e);
            }
        }
    }

    fn expire_workers(&mut self) -> Result<()> {
        let now = Instant::now();
        loop {
            if let Some((_, expiry)) = self.ready_workers.front() {
                if expiry >= &now {
                    break;
                }
            } else {
                break;
            }

            let (worker, _) = self.ready_workers.pop_front().unwrap();
            debug!(
                "expiring ready worker due to missed heartbeat, worker={:?}",
                worker
            );
        }

        loop {
            if let Some((_, expiry)) = self.busy_workers.front() {
                if expiry >= &now {
                    break;
                }
            } else {
                break;
            }

            let (worker, _) = self.busy_workers.pop_front().unwrap();
            debug!(
                "expiring busy worker due to missed heartbeat, worker={:?}",
                worker
            );

            // TODO: (SA) we need to handle a case where both worker and job srv
            // restart during progress of a job - the job_id can get lost.
            if self.worker_map.contains_key(&worker) {
                let job_id = self.worker_map.remove(&worker).unwrap();

                if self.jobs.contains_key(&job_id) {
                    self.jobs.remove(&job_id).unwrap();
                }

                let mut req = jobsrv::JobGet::new();
                req.set_id(job_id);

                match self.datastore.get_job(&req)? {
                    Some(mut job) => {
                        debug!("updating job {:?}, setting state back to Pending", job_id);
                        job.set_state(jobsrv::JobState::Pending);
                        self.datastore.update_job(&job)?;
                    }
                    None => {
                        warn!(
                            "did not find job id {:?} for busy worker {:?}!",
                            job_id,
                            worker
                        );
                    }
                }
            }
        }

        Ok(())
    }

    fn expire_jobs(&mut self) -> Result<()> {
        let now = Instant::now();
        loop {
            if let Some((_, expiry)) = self.jobs.front() {
                if expiry >= &now {
                    break;
                }
            } else {
                break;
            }

            let (job_id, _) = self.jobs.pop_front().unwrap();
            debug!("expiring job due to timeout, id={:?}", job_id);

            let mut req = jobsrv::JobGet::new();
            req.set_id(job_id);

            match self.datastore.get_job(&req)? {
                Some(mut job) => {
                    job.set_state(jobsrv::JobState::Failed);
                    let err = net::err(ErrCode::TIMEOUT, "js:err:1");
                    job.set_error(err);
                    debug!("updating job {:?} setting state to Failed", job_id);
                    self.datastore.update_job(&job)?;
                }
                None => {}
            }
        }

        Ok(())
    }

    fn process_heartbeat(&mut self) -> Result<bool> {
        self.hb_sock.recv(&mut self.msg, 0)?;
        let heartbeat: jobsrv::Heartbeat = parse_from_bytes(&self.msg)?;
        debug!("heartbeat={:?}", heartbeat);

        let now = Instant::now();
        let expiry = now + Duration::from_millis(WORKER_TIMEOUT_MS);
        let endpoint = heartbeat.get_endpoint().to_string();

        let result = match heartbeat.get_state() {
            jobsrv::WorkerState::Ready => {
                self.busy_workers.remove(&endpoint);
                self.ready_workers.insert(endpoint, expiry);
                true
            }
            jobsrv::WorkerState::Busy => {
                self.ready_workers.remove(&endpoint);
                self.busy_workers.insert(endpoint, expiry);
                false
            }
        };
        Ok(result)
    }

    fn process_job_status(&mut self) -> Result<()> {
        // Pop message delimiter
        self.rq_sock.recv(&mut self.msg, 0)?;
        // Pop message body
        self.rq_sock.recv(&mut self.msg, 0)?;
        let job: jobsrv::Job = parse_from_bytes(&self.msg)?;
        debug!("job_status={:?}", job);
        self.datastore.update_job(&job)?;

        match job.get_state() {
            jobsrv::JobState::Complete |
            jobsrv::JobState::Rejected |
            jobsrv::JobState::Failed => {
                self.jobs.remove(&job.get_id());
            }
            _ => (),
        }

        Ok(())
    }
}
