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

use std::path::PathBuf;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::thread::{self, JoinHandle};

use bldr_core;
use bldr_core::job::Job;
use hab_net::{ErrCode, NetError};
use hab_net::conn::RouteClient;
use hab_net::socket::DEFAULT_CONTEXT;
use linked_hash_map::LinkedHashMap;
use protobuf::{parse_from_bytes, Message, RepeatedField};
use protocol::jobsrv;
use protocol::originsrv::{OriginIntegrationRequest, OriginIntegrationResponse,
                          OriginProjectIntegrationRequest, OriginProjectIntegrationResponse};
use zmq;

use config::Config;
use data_store::DataStore;
use error::Result;

use super::scheduler::ScheduleClient;

const WORKER_MGR_ADDR: &'static str = "inproc://work-manager";
const WORKER_TIMEOUT_MS: u64 = 33_000; // 33 sec
const DEFAULT_POLL_TIMEOUT_MS: u64 = 60_000; // 60 secs
const JOB_TIMEOUT_CONVERT_MS: u64 = 60_000; // Conversion from mins to milli-seconds

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
        WorkerMgrClient { socket: socket }
    }
}

#[derive(Debug)]
pub struct Worker {
    pub ident: String,
    pub state: jobsrv::WorkerState,
    pub expiry: Instant,
    pub job_id: Option<u64>,
    pub job_expiry: Option<Instant>,
    pub quarantined: bool,
}

impl Worker {
    pub fn new(ident: &str) -> Self {
        Worker {
            ident: ident.to_string(),
            state: jobsrv::WorkerState::Ready,
            expiry: Instant::now() + Duration::from_millis(WORKER_TIMEOUT_MS),
            job_id: None,
            job_expiry: None,
            quarantined: false,
        }
    }

    pub fn ready(&mut self) {
        self.state = jobsrv::WorkerState::Ready;
        self.expiry = Instant::now() + Duration::from_millis(WORKER_TIMEOUT_MS);
        self.job_id = None;
        self.job_expiry = None;
        self.quarantined = false;
    }

    pub fn busy(&mut self, job_id: u64, job_timeout: u64) {
        self.state = jobsrv::WorkerState::Busy;
        self.expiry = Instant::now() + Duration::from_millis(WORKER_TIMEOUT_MS);

        if self.job_id.is_none() {
            self.job_id = Some(job_id);
            self.job_expiry = Some(
                Instant::now() +
                    Duration::from_millis(job_timeout * JOB_TIMEOUT_CONVERT_MS),
            );
        } else {
            assert!(self.job_id.unwrap() == job_id);
        }

        self.quarantined = false;
    }

    pub fn refresh(&mut self) {
        self.expiry = Instant::now() + Duration::from_millis(WORKER_TIMEOUT_MS);
    }

    pub fn quarantine(&mut self) {
        self.expiry = Instant::now() + Duration::from_millis(WORKER_TIMEOUT_MS);
        self.quarantined = true;
    }

    pub fn is_quarantined(&self) -> bool {
        self.quarantined
    }

    pub fn is_expired(&self) -> bool {
        self.expiry < Instant::now()
    }

    pub fn is_job_expired(&self) -> bool {
        if self.job_expiry.is_some() {
            self.job_expiry.unwrap() < Instant::now()
        } else {
            false
        }
    }
}

pub struct WorkerMgr {
    datastore: DataStore,
    key_dir: PathBuf,
    route_conn: RouteClient,
    hb_sock: zmq::Socket,
    rq_sock: zmq::Socket,
    work_mgr_sock: zmq::Socket,
    msg: zmq::Message,
    workers: LinkedHashMap<String, Worker>,
    worker_command: String,
    worker_heartbeat: String,
    schedule_cli: ScheduleClient,
    job_timeout: u64,
}

impl WorkerMgr {
    pub fn new(cfg: &Config, datastore: DataStore, route_conn: RouteClient) -> Result<Self> {
        let hb_sock = (**DEFAULT_CONTEXT).as_mut().socket(zmq::SUB)?;
        let rq_sock = (**DEFAULT_CONTEXT).as_mut().socket(zmq::ROUTER)?;
        let work_mgr_sock = (**DEFAULT_CONTEXT).as_mut().socket(zmq::DEALER)?;
        rq_sock.set_router_mandatory(true)?;
        hb_sock.set_subscribe(&[])?;

        let mut schedule_cli = ScheduleClient::default();
        schedule_cli.connect()?;

        Ok(WorkerMgr {
            datastore: datastore,
            key_dir: cfg.key_dir.clone(),
            route_conn: route_conn,
            hb_sock: hb_sock,
            rq_sock: rq_sock,
            work_mgr_sock: work_mgr_sock,
            msg: zmq::Message::new()?,
            workers: LinkedHashMap::new(),
            worker_command: cfg.net.worker_command_addr(),
            worker_heartbeat: cfg.net.worker_heartbeat_addr(),
            schedule_cli: schedule_cli,
            job_timeout: cfg.job_timeout,
        })
    }

    pub fn start(cfg: &Config, datastore: DataStore, conn: RouteClient) -> Result<JoinHandle<()>> {
        let mut manager = Self::new(cfg, datastore, conn)?;
        let (tx, rx) = mpsc::sync_channel(1);
        let handle = thread::Builder::new()
            .name("worker-manager".to_string())
            .spawn(move || { manager.run(tx).unwrap(); })
            .unwrap();
        match rx.recv() {
            Ok(()) => Ok(handle),
            Err(e) => panic!("worker-manager thread startup error, err={}", e),
        }
    }

    fn run(&mut self, rz: mpsc::SyncSender<()>) -> Result<()> {
        self.work_mgr_sock.bind(WORKER_MGR_ADDR)?;
        println!("Listening for commands on {}", self.worker_command);
        self.rq_sock.bind(&self.worker_command)?;
        println!("Listening for heartbeats on {}", self.worker_heartbeat);
        self.hb_sock.bind(&self.worker_heartbeat)?;
        let mut hb_sock = false;
        let mut rq_sock = false;
        let mut work_mgr_sock = false;
        let mut process_work = false;
        let mut last_processed = Instant::now();

        rz.send(()).unwrap();

        // Load busy worker state
        self.load_workers()?;

        info!("builder-jobsrv is ready to go.");

        loop {
            {
                let mut items = [
                    self.hb_sock.as_poll_item(1),
                    self.rq_sock.as_poll_item(1),
                    self.work_mgr_sock.as_poll_item(1),
                ];

                if let Err(err) = zmq::poll(&mut items, DEFAULT_POLL_TIMEOUT_MS as i64) {
                    warn!("Worker-manager unable to complete ZMQ poll: err {:?}", err);
                };
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
                if let Err(err) = self.process_heartbeat() {
                    warn!("Worker-manager unable to process heartbeat: err {:?}", err);
                };
                hb_sock = false;
            }
            if let Err(err) = self.expire_workers() {
                warn!("Worker-manager unable to expire workers: err {:?}", err);
            }
            if rq_sock {
                if let Err(err) = self.process_job_status() {
                    warn!("Worker-manager unable to process job status: err {:?}", err);
                }
                rq_sock = false;
            }
            if work_mgr_sock {
                process_work = true;
                work_mgr_sock = false;

                if let Err(err) = self.work_mgr_sock.recv(&mut self.msg, 0) {
                    warn!(
                        "Worker-manager unable to complete socket receive: err {:?}",
                        err
                    );
                }
            }

            // Handle potential work in pending_jobs queue
            let now = Instant::now();
            if process_work ||
                (&now > &(last_processed + Duration::from_millis(DEFAULT_POLL_TIMEOUT_MS)))
            {
                if let Err(err) = self.process_work() {
                    warn!("Worker-manager unable to process work: err {:?}", err);
                }
                last_processed = now;
            }
        }
    }

    fn load_workers(&mut self) -> Result<()> {
        let workers = self.datastore.get_busy_workers()?;

        for worker in workers {
            let mut bw = Worker::new(worker.get_ident());
            bw.busy(worker.get_job_id(), self.job_timeout);
            if worker.get_quarantined() {
                bw.quarantine();
            }

            self.workers.insert(worker.get_ident().to_owned(), bw);
        }

        Ok(())
    }

    fn save_worker(&mut self, worker: &Worker) -> Result<()> {
        let mut bw = jobsrv::BusyWorker::new();
        bw.set_ident(worker.ident.clone());
        bw.set_job_id(worker.job_id.unwrap()); // unwrap Ok
        bw.set_quarantined(worker.quarantined);

        self.datastore.upsert_busy_worker(&bw)
    }

    fn delete_worker(&mut self, worker: &Worker) -> Result<()> {
        let mut bw = jobsrv::BusyWorker::new();
        bw.set_ident(worker.ident.clone());
        bw.set_job_id(worker.job_id.unwrap()); // unwrap Ok

        self.datastore.delete_busy_worker(&bw)
    }

    fn process_work(&mut self) -> Result<()> {
        loop {
            // Exit if we don't have any ready workers
            let worker_ident = match self.workers.iter().find(|t| {
                t.1.state == jobsrv::WorkerState::Ready
            }) {
                Some(t) => t.0.clone(),
                None => return Ok(()),
            };

            // Take one job from the pending list
            let mut jobs = self.datastore.pending_jobs(1)?;
            // 0 means there are no pending jobs, so we can exit
            if jobs.len() == 0 {
                break;
            }

            // This unwrap is fine, because we just checked our length
            let mut job = Job::new(jobs.pop().unwrap());

            self.add_integrations_to_job(&mut job);
            self.add_project_integrations_to_job(&mut job);

            match self.dispatch_job(&job, &worker_ident) {
                Ok(()) => {
                    let mut worker = self.workers.remove(&worker_ident).unwrap(); // unwrap Ok
                    worker.busy(job.get_id(), self.job_timeout);
                    self.save_worker(&worker)?;
                    self.workers.insert(worker_ident, worker);
                }
                Err(err) => {
                    warn!(
                        "Failed to dispatch job to worker {}, err={:?}",
                        worker_ident,
                        err
                    );
                    job.set_state(jobsrv::JobState::Pending);
                    self.datastore.update_job(&job)?;
                    return Ok(()); // Exit instead of re-trying immediately
                }
            }
        }
        Ok(())
    }

    fn dispatch_job(&mut self, job: &Job, worker_ident: &str) -> Result<()> {
        debug!("Dispatching job to worker {:?}: {:?}", worker_ident, job);

        self.rq_sock.send_str(&worker_ident, zmq::SNDMORE)?;
        self.rq_sock.send(&[], zmq::SNDMORE)?;
        self.rq_sock.send(&job.write_to_bytes().unwrap(), 0)?;

        Ok(())
    }

    fn add_integrations_to_job(&mut self, job: &mut Job) {
        let mut integrations = RepeatedField::new();
        let mut integration_request = OriginIntegrationRequest::new();
        let origin = job.get_project().get_origin_name().to_string();
        integration_request.set_origin(origin);

        match self.route_conn.route::<OriginIntegrationRequest, OriginIntegrationResponse>(
            &integration_request,
        ) {
            Ok(oir) => {
                for i in oir.get_integrations() {
                    let mut oi = i.clone();
                    let plaintext = match bldr_core::integrations::decrypt(
                        &self.key_dir,
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

    fn add_project_integrations_to_job(&mut self, job: &mut Job) {
        let mut integrations = RepeatedField::new();
        let mut req = OriginProjectIntegrationRequest::new();
        let origin = job.get_project().get_origin_name().to_string();
        let name = job.get_project().get_package_name().to_string();
        req.set_origin(origin);
        req.set_name(name);

        match self.route_conn
            .route::<OriginProjectIntegrationRequest, OriginProjectIntegrationResponse>(&req) {
            Ok(opir) => {
                for opi in opir.get_integrations() {
                    integrations.push(opi.clone());
                }
                job.set_project_integrations(integrations);
            }
            Err(e) => {
                debug!("Error fetching project integrations. e = {:?}", e);
            }
        }
    }

    fn expire_workers(&mut self) -> Result<()> {
        loop {
            if let Some(worker) = self.workers.front() {
                if !worker.1.is_expired() {
                    break;
                }
            } else {
                break;
            }

            let worker = self.workers.pop_front().unwrap().1;
            debug!("Expiring worker due to missed heartbeat: {:?}", worker);

            if worker.state == jobsrv::WorkerState::Busy {
                self.requeue_job(worker.job_id.unwrap())?; // unwrap Ok
                self.delete_worker(&worker)?;
            }
        }

        Ok(())
    }

    fn requeue_job(&mut self, job_id: u64) -> Result<()> {
        let mut req = jobsrv::JobGet::new();
        req.set_id(job_id);

        match self.datastore.get_job(&req)? {
            Some(mut job) => {
                debug!("Requeing job {:?}", job_id);
                job.set_state(jobsrv::JobState::Pending);
                self.datastore.update_job(&job)?;
            }
            None => {
                warn!(
                    "Unable to requeue job {:?} (not found)",
                    job_id,
                );
            }
        }

        Ok(())
    }

    fn fail_job(&mut self, job_id: u64) -> Result<()> {
        let mut req = jobsrv::JobGet::new();
        req.set_id(job_id);

        match self.datastore.get_job(&req)? {
            Some(mut job) => {
                job.set_state(jobsrv::JobState::Failed);
                let err = NetError::new(ErrCode::TIMEOUT, "js:wrk-fail:1");
                job.set_error(err.take_err());
                debug!(
                    "Job {:?} timed out after {} minutes",
                    job_id,
                    self.job_timeout
                );
                self.datastore.update_job(&job)?;
            }
            None => {
                warn!(
                    "Unable to fail job {:?} (not found)",
                    job_id,
                );
            }
        };

        Ok(())
    }

    fn is_job_complete(&mut self, job_id: u64) -> Result<bool> {
        let mut req = jobsrv::JobGet::new();
        req.set_id(job_id);

        let ret = match self.datastore.get_job(&req)? {
            Some(job) => {
                match job.get_state() {
                    jobsrv::JobState::Pending |
                    jobsrv::JobState::Processing |
                    jobsrv::JobState::Dispatched => false,
                    jobsrv::JobState::Complete |
                    jobsrv::JobState::Failed |
                    jobsrv::JobState::Rejected => true,
                }
            }
            None => {
                warn!(
                    "Unable to check job completeness {:?} (not found)",
                    job_id,
                );
                false
            }
        };

        Ok(ret)
    }

    fn process_heartbeat(&mut self) -> Result<()> {
        self.hb_sock.recv(&mut self.msg, 0)?;
        let heartbeat: jobsrv::Heartbeat = parse_from_bytes(&self.msg)?;
        debug!("Got heartbeat: {:?}", heartbeat);

        let worker_ident = heartbeat.get_endpoint().to_string();

        let mut worker = match self.workers.remove(&worker_ident) {
            Some(worker) => worker,
            None => {
                if heartbeat.get_state() == jobsrv::WorkerState::Ready {
                    Worker::new(&worker_ident)
                } else {
                    warn!(
                        "Unexpacted Busy heartbeat from unknown worker {}",
                        worker_ident
                    );
                    return Ok(()); // Something went wrong, don't process this HB
                }
            }
        };

        match (worker.state, heartbeat.get_state()) {
            (jobsrv::WorkerState::Ready, jobsrv::WorkerState::Busy) => {
                warn!(
                    "Unexpected Busy heartbeat from known worker {}",
                    worker_ident
                );
                return Ok(()); // Something went wrong, don't process this HB
            }
            (jobsrv::WorkerState::Busy, jobsrv::WorkerState::Busy) => {
                // Check to see if job has expired and quarantine this worker if so.
                // It's probably hung in a build or in some other semi-bad state
                // TODO (SA): Send cancel message to the worker to attempt to
                // properly cancel the job and do a better recovery
                let job_id = worker.job_id.unwrap(); // unwrap Ok
                if worker.is_job_expired() {
                    if !worker.is_quarantined() {
                        self.fail_job(job_id)?;
                    };
                    worker.quarantine();
                } else {
                    worker.refresh();
                }
            }
            (jobsrv::WorkerState::Busy, jobsrv::WorkerState::Ready) => {
                if !self.is_job_complete(worker.job_id.unwrap())? {
                    // Handle potential race condition where a Ready heartbeat
                    // is received right *after* the job has been dispatched
                    warn!(
                        "Unexpected Ready heartbeat from incomplete job: {}",
                        worker.job_id.unwrap()
                    );
                    worker.refresh();
                } else {
                    self.delete_worker(&worker)?;
                    worker.ready();
                }
            }
            _ => worker.ready(),
        };

        assert!(!worker.is_expired());
        self.workers.insert(worker_ident, worker);
        Ok(())
    }

    fn process_job_status(&mut self) -> Result<()> {
        self.rq_sock.recv(&mut self.msg, 0)?;
        self.rq_sock.recv(&mut self.msg, 0)?;

        let job = Job::new(parse_from_bytes::<jobsrv::Job>(&self.msg)?);
        debug!("Got job status: {:?}", job);
        self.datastore.update_job(&job)?;
        self.schedule_cli.notify()?;

        Ok(())
    }
}
