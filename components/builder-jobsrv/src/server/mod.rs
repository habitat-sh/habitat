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

pub mod handlers;

use std::ops::Deref;
use std::sync::{mpsc, Arc, RwLock};
use std::time::{Duration, Instant};
use std::thread::{self, JoinHandle};

use dbcache::InstaSet;
use linked_hash_map::LinkedHashMap;
use hab_net::{Application, Dispatcher, Supervisor};
use hab_net::config::ToAddrString;
use hab_net::server::{Envelope, NetIdent, RouteConn, Service, ZMQ_CONTEXT};
use protobuf::{parse_from_bytes, Message};
use protocol::net;
use protocol::jobsrv;
use zmq;

use config::Config;
use data_store::DataStore;
use error::{Error, Result};

const BE_LISTEN_ADDR: &'static str = "inproc://backend";
const WORKER_MGR_ADDR: &'static str = "inproc://work-manager";
const WORKER_TIMEOUT_MS: u64 = 33_000;

pub struct ServerState {
    pub work_manager: WorkManager,
    datastore: Option<DataStore>,
}

impl ServerState {
    pub fn datastore(&mut self) -> &mut DataStore {
        self.datastore.as_mut().unwrap()
    }
}

impl Default for ServerState {
    fn default() -> Self {
        let work_manager = WorkManager::default();
        ServerState {
            datastore: None,
            work_manager: work_manager,
        }
    }
}

pub struct WorkManager {
    socket: zmq::Socket,
}

impl WorkManager {
    pub fn connect(&mut self) -> Result<()> {
        try!(self.socket.connect(WORKER_MGR_ADDR));
        Ok(())
    }

    pub fn notify_work(&mut self) -> Result<()> {
        try!(self.socket.send(&[1], 0));
        Ok(())
    }
}

impl Default for WorkManager {
    fn default() -> WorkManager {
        let socket = (**ZMQ_CONTEXT).as_mut().socket(zmq::DEALER).unwrap();
        socket.set_sndhwm(1).unwrap();
        socket.set_linger(0).unwrap();
        socket.set_immediate(true).unwrap();
        WorkManager { socket: socket }
    }
}

pub struct Worker {
    config: Arc<RwLock<Config>>,
    state: ServerState,
}

impl Worker {
    fn try_connect_datastore(&mut self) {
        loop {
            let result = {
                let cfg = self.config.read().unwrap();
                DataStore::open(cfg.deref())
            };
            match result {
                Ok(datastore) => {
                    self.state.datastore = Some(datastore);
                    break;
                }
                Err(e) => {
                    error!("{}", e);
                    thread::sleep(Duration::from_millis(5000));
                }
            }
        }
    }
}

impl Dispatcher for Worker {
    type Config = Config;
    type Error = Error;
    type State = ServerState;

    fn message_queue() -> &'static str {
        BE_LISTEN_ADDR
    }

    fn dispatch(message: &mut Envelope,
                sock: &mut zmq::Socket,
                state: &mut ServerState)
                -> Result<()> {
        match message.message_id() {
            "JobCreate" => handlers::job_create(message, sock, state),
            "JobGet" => handlers::job_get(message, sock, state),
            _ => panic!("unexpected message: {:?}", message.message_id()),
        }
    }

    fn context(&mut self) -> &mut zmq::Context {
        (**ZMQ_CONTEXT).as_mut()
    }

    fn new(config: Arc<RwLock<Config>>) -> Self {
        let state = ServerState::default();
        Worker {
            config: config,
            state: state,
        }
    }

    fn init(&mut self) -> Result<()> {
        try!(self.state.work_manager.connect());
        self.try_connect_datastore();
        Ok(())
    }

    fn state(&mut self) -> &mut ServerState {
        &mut self.state
    }
}

pub struct Server {
    config: Arc<RwLock<Config>>,
    router: RouteConn,
    be_sock: zmq::Socket,
}

impl Server {
    pub fn new(config: Config) -> Result<Self> {
        let router = try!(RouteConn::new(Self::net_ident(), (**ZMQ_CONTEXT).as_mut()));
        let be = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::DEALER));
        Ok(Server {
            config: Arc::new(RwLock::new(config)),
            router: router,
            be_sock: be,
        })
    }

    pub fn reconfigure(&self, config: Config) -> Result<()> {
        {
            let mut cfg = self.config.write().unwrap();
            *cfg = config;
        }
        // * disconnect from removed routers
        // * notify remaining routers of any shard hosting changes
        // * connect to new shard servers
        Ok(())
    }
}

impl Application for Server {
    type Error = Error;

    fn run(&mut self) -> Result<()> {
        try!(self.be_sock.bind(BE_LISTEN_ADDR));
        let cfg1 = self.config.clone();
        let cfg2 = self.config.clone();
        let sup: Supervisor<Worker> = Supervisor::new(cfg1);
        let work_mgr = try!(WorkerManager::start(cfg2));
        {
            let cfg = self.config.read().unwrap();
            try!(sup.start(cfg.worker_threads));
        }
        try!(self.connect());
        try!(zmq::proxy(&mut self.router.socket, &mut self.be_sock));
        work_mgr.join().unwrap();
        Ok(())
    }
}

impl Service for Server {
    type Application = Self;
    type Config = Config;
    type Error = Error;

    fn protocol() -> net::Protocol {
        net::Protocol::JobSrv
    }

    fn config(&self) -> &Arc<RwLock<Self::Config>> {
        &self.config
    }

    fn conn(&self) -> &RouteConn {
        &self.router
    }

    fn conn_mut(&mut self) -> &mut RouteConn {
        &mut self.router
    }
}

impl NetIdent for Server {}

struct WorkerManager {
    config: Arc<RwLock<Config>>,
    datastore: DataStore,
    hb_sock: zmq::Socket,
    rq_sock: zmq::Socket,
    work_mgr_sock: zmq::Socket,
    msg: zmq::Message,
    workers: LinkedHashMap<String, Instant>,
}

impl WorkerManager {
    pub fn new(config: Arc<RwLock<Config>>) -> Result<Self> {
        let datastore = {
            let cfg = config.read().unwrap();
            try!(DataStore::open(cfg.deref()))
        };
        let hb_sock = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::SUB));
        let rq_sock = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::ROUTER));
        let work_mgr_sock = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::DEALER));
        try!(rq_sock.set_router_mandatory(true));
        try!(hb_sock.set_subscribe(&[]));
        try!(work_mgr_sock.set_rcvhwm(1));
        try!(work_mgr_sock.set_linger(0));
        try!(work_mgr_sock.set_immediate(true));
        let msg = try!(zmq::Message::new());
        Ok(WorkerManager {
            config: config,
            datastore: datastore,
            hb_sock: hb_sock,
            rq_sock: rq_sock,
            work_mgr_sock: work_mgr_sock,
            msg: msg,
            workers: LinkedHashMap::new(),
        })
    }

    pub fn start(config: Arc<RwLock<Config>>) -> Result<JoinHandle<()>> {
        let (tx, rx) = mpsc::sync_channel(1);
        let handle = thread::Builder::new()
            .name("worker-manager".to_string())
            .spawn(move || {
                let mut manager = Self::new(config).unwrap();
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

pub fn run(config: Config) -> Result<()> {
    try!(Server::new(config)).run()
}
