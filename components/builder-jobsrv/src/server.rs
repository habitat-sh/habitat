// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::ops::Deref;
use std::sync::{mpsc, Arc, RwLock};
use std::time::{Duration, Instant};
use std::thread::{self, JoinHandle};

use dbcache::{self, RecordTable};
use linked_hash_map::LinkedHashMap;
use hab_net::server::{Application, Envelope, NetIdent, RouteConn, Service, Supervisor,
                      Supervisable, ToAddrString};
use protobuf::{parse_from_bytes, Message};
use protocol::net::{self, ErrCode};
use protocol::jobsrv;
use zmq;

use config::Config;
use data_store::{self, DataStore};
use error::{Error, Result};

const BE_LISTEN_ADDR: &'static str = "inproc://backend";
const WORKER_MGR_ADDR: &'static str = "inproc://work-manager";
const WORKER_TIMEOUT_MS: u64 = 33_000;

pub struct Worker {
    config: Arc<RwLock<Config>>,
    sock: zmq::Socket,
    work_manager: zmq::Socket,
    datastore: Option<DataStore>,
}

impl Worker {
    fn datastore(&self) -> &DataStore {
        self.datastore.as_ref().unwrap()
    }

    fn dispatch(&mut self, req: &mut Envelope) -> Result<()> {
        match req.message_id() {
            "JobCreate" => {
                let mut job = data_store::Job::new();
                self.datastore().jobs.write(&mut job).unwrap();
                self.datastore().job_queue.enqueue(&job).unwrap();
                try!(self.notify_work_mgr());
                let reply: jobsrv::Job = job.into();
                try!(req.reply_complete(&mut self.sock, &reply));
            }
            "JobGet" => {
                let msg: jobsrv::JobGet = try!(req.parse_msg());
                match self.datastore().jobs.find(msg.get_id()) {
                    Ok(job) => {
                        let reply: jobsrv::Job = job.into();
                        try!(req.reply_complete(&mut self.sock, &reply));
                    }
                    Err(dbcache::Error::EntityNotFound) => {
                        let err = net::err(ErrCode::ENTITY_NOT_FOUND, "jb:job-get:1");
                        try!(req.reply_complete(&mut self.sock, &err));
                    }
                    Err(e) => {
                        error!("datastore error, err={:?}", e);
                        let err = net::err(ErrCode::INTERNAL, "jb:job-get:2");
                        try!(req.reply_complete(&mut self.sock, &err));
                    }
                }
            }
            _ => panic!("unexpected message: {:?}", req.message_id()),
        }
        Ok(())
    }

    fn notify_work_mgr(&mut self) -> Result<()> {
        try!(self.work_manager.send(&[1], 0));
        Ok(())
    }

    fn try_connect_datastore(&mut self) {
        loop {
            let result = {
                let cfg = self.config.read().unwrap();
                DataStore::open(cfg.deref())
            };
            match result {
                Ok(datastore) => {
                    self.datastore = Some(datastore);
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

impl Supervisable for Worker {
    type Config = Config;
    type Error = Error;

    fn new(context: &mut zmq::Context, config: Arc<RwLock<Config>>) -> Self {
        let sock = context.socket(zmq::DEALER).unwrap();
        let work_manager = context.socket(zmq::DEALER).unwrap();
        work_manager.set_sndhwm(1).unwrap();
        work_manager.set_linger(0).unwrap();
        work_manager.set_immediate(true).unwrap();
        Worker {
            config: config,
            sock: sock,
            work_manager: work_manager,
            datastore: None,
        }
    }

    fn init(&mut self) -> Result<()> {
        self.try_connect_datastore();
        self.work_manager.connect(WORKER_MGR_ADDR).unwrap();
        Ok(())
    }

    fn on_message(&mut self, req: &mut Envelope) -> Result<()> {
        self.dispatch(req)
    }

    fn socket(&mut self) -> &mut zmq::Socket {
        &mut self.sock
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.sock.close().unwrap();
    }
}

pub struct Server {
    config: Arc<RwLock<Config>>,
    #[allow(dead_code)]
    ctx: Arc<RwLock<zmq::Context>>,
    router: RouteConn,
    be_sock: zmq::Socket,
}

impl Server {
    pub fn new(config: Config) -> Result<Self> {
        let mut ctx = zmq::Context::new();
        let router = try!(RouteConn::new(Self::net_ident(), &mut ctx));
        let be = try!(ctx.socket(zmq::DEALER));
        Ok(Server {
            config: Arc::new(RwLock::new(config)),
            ctx: Arc::new(RwLock::new(ctx)),
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
        let ctx1 = self.ctx.clone();
        let ctx2 = self.ctx.clone();
        let sup: Supervisor<Worker> = Supervisor::new(ctx1, cfg1);
        let work_mgr = try!(WorkerManager::start(ctx2, cfg2));
        {
            let cfg = self.config.read().unwrap();
            try!(sup.start(BE_LISTEN_ADDR, cfg.worker_threads));
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
    #[allow(dead_code)]
    ctx: Arc<RwLock<zmq::Context>>,
    datastore: DataStore,
    hb_sock: zmq::Socket,
    rq_sock: zmq::Socket,
    work_mgr_sock: zmq::Socket,
    msg: zmq::Message,
    workers: LinkedHashMap<String, Instant>,
}

impl WorkerManager {
    pub fn new(ctx: Arc<RwLock<zmq::Context>>, config: Arc<RwLock<Config>>) -> Result<Self> {
        let datastore = {
            let cfg = config.read().unwrap();
            try!(DataStore::open(cfg.deref()))
        };
        let (hb_sock, rq_sock, work_mgr_sock) = {
            let mut ctx = ctx.write().unwrap();
            let hb_sock = try!(ctx.socket(zmq::SUB));
            let rq_sock = try!(ctx.socket(zmq::ROUTER));
            let work_mgr_sock = try!(ctx.socket(zmq::DEALER));
            (hb_sock, rq_sock, work_mgr_sock)
        };
        try!(rq_sock.set_router_mandatory(true));
        try!(hb_sock.set_subscribe(&[]));
        try!(work_mgr_sock.set_rcvhwm(1));
        try!(work_mgr_sock.set_linger(0));
        try!(work_mgr_sock.set_immediate(true));
        let msg = try!(zmq::Message::new());
        Ok(WorkerManager {
            config: config,
            ctx: ctx,
            datastore: datastore,
            hb_sock: hb_sock,
            rq_sock: rq_sock,
            work_mgr_sock: work_mgr_sock,
            msg: msg,
            workers: LinkedHashMap::new(),
        })
    }

    pub fn start(ctx: Arc<RwLock<zmq::Context>>,
                 config: Arc<RwLock<Config>>)
                 -> Result<JoinHandle<()>> {
        let (tx, rx) = mpsc::sync_channel(1);
        let handle = thread::Builder::new()
                         .name("worker-manager".to_string())
                         .spawn(move || {
                             let mut manager = Self::new(ctx, config).unwrap();
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
                    // JW TODO: Wait for response back to ensure we can dequeue this. If state returned
                    // is not processing then we move onto next worker and assume this worker is
                    // no longer valid. Put work back on queue.
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
        let req: jobsrv::Job = try!(parse_from_bytes(&self.msg));
        debug!("job_status={:?}", req);
        let job = data_store::Job::from(req);
        try!(self.datastore.jobs.update(&job));
        Ok(())
    }
}

pub fn run(config: Config) -> Result<()> {
    try!(Server::new(config)).run()
}
