// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::sync::{mpsc, Arc, RwLock};
use std::time::Duration;
use std::thread::{self, JoinHandle};

use hab_net::server::NetIdent;
use protobuf::{parse_from_bytes, Message};
use protocol;
use zmq;

use config::Config;
use error::Result;

const HEARTBEAT_MS: i64 = 30_000;
const HB_INPROC_ADDR: &'static str = "inproc://heartbeat";
const HB_CMD_PULSE: &'static str = "R";
const HB_CMD_PAUSE: &'static str = "P";
const RUNNER_INPROC_ADDR: &'static str = "inproc://runner";
const WORK_ACK: &'static str = "A";
const WORK_COMPLETE: &'static str = "C";


#[cfg(target_os = "linux")]
fn worker_os() -> protocol::jobsrv::Os {
    protocol::jobsrv::Os::Linux
}

#[cfg(target_os = "windows")]
fn worker_os() -> protocol::jobsrv::Os {
    protocol::jobsrv::Os::Windows
}

#[cfg(target_os = "macos")]
fn worker_os() -> protocol::jobsrv::Os {
    protocol::jobsrv::Os::Darwin
}

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
    config: Arc<RwLock<Config>>,
    ctx: Arc<RwLock<zmq::Context>>,
    fe_sock: zmq::Socket,
    hb_conn: zmq::Socket,
    runner_sock: zmq::Socket,
    state: State,
    msg: zmq::Message,
}

impl Server {
    pub fn new(config: Config) -> Result<Self> {
        let mut ctx = zmq::Context::new();
        let fe_sock = try!(ctx.socket(zmq::DEALER));
        let hb_conn = try!(ctx.socket(zmq::REQ));
        let runner_sock = try!(ctx.socket(zmq::DEALER));
        try!(fe_sock.set_identity(Self::net_ident().as_bytes()));
        Ok(Server {
            config: Arc::new(RwLock::new(config)),
            ctx: Arc::new(RwLock::new(ctx)),
            fe_sock: fe_sock,
            hb_conn: hb_conn,
            runner_sock: runner_sock,
            state: State::default(),
            msg: try!(zmq::Message::new()),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let cfg1 = self.config.clone();
        let cfg2 = self.config.clone();
        let ctx1 = self.ctx.clone();
        let ctx2 = self.ctx.clone();
        let heartbeat = try!(Heartbeat::start(cfg1, ctx1));
        let runner = try!(Runner::start(cfg2, ctx2));
        try!(self.hb_conn.connect(HB_INPROC_ADDR));
        try!(self.runner_sock.connect(RUNNER_INPROC_ADDR));

        {
            let cfg = self.config.read().unwrap();
            for (_, queue) in cfg.jobsrv_addrs() {
                println!("Connecting to job queue, {}", queue);
                try!(self.fe_sock.connect(&queue));
            }
        }

        let mut fe_sock = false;
        let mut runner_sock = false;
        let mut reply = protocol::jobsrv::Job::new();
        loop {
            {
                let mut items = [self.fe_sock.as_poll_item(1), self.runner_sock.as_poll_item(1)];
                try!(zmq::poll(&mut items, -1));
                if items[0].get_revents() & zmq::POLLIN > 0 {
                    fe_sock = true;
                }
                if items[1].get_revents() & zmq::POLLIN > 0 {
                    runner_sock = true;
                }
            }
            if runner_sock {
                try!(self.runner_sock.recv(&mut self.msg, 0));
                if Some(WORK_COMPLETE) != self.msg.as_str() {
                    unreachable!("run:1, received unexpected response from runner");
                }

                try!(self.runner_sock.recv(&mut self.msg, 0));
                try!(self.fe_sock.send(&*self.msg, 0));
                try!(self.set_ready());
                runner_sock = false;
            }
            if fe_sock {
                try!(self.fe_sock.recv(&mut self.msg, 0));
                try!(self.fe_sock.recv(&mut self.msg, 0));
                match self.state {
                    State::Ready => {
                        self.runner_sock.send(&*self.msg, 0).unwrap();
                        self.runner_sock.recv(&mut self.msg, 0).unwrap();
                        if Some(WORK_ACK) != self.msg.as_str() {
                            unreachable!("run:2, received unexpected response from runner");
                        }

                        self.runner_sock.recv(&mut self.msg, 0).unwrap();
                        let job_id: u64 = self.msg.as_str().unwrap().parse().unwrap();
                        reply.set_id(job_id);
                        reply.set_state(protocol::jobsrv::JobState::Processing);

                        try!(self.set_busy());
                        try!(self.fe_sock.send(&try!(reply.write_to_bytes()), 0));
                    }
                    State::Busy => {
                        reply = parse_from_bytes(&self.msg).unwrap();
                        reply.set_state(protocol::jobsrv::JobState::Rejected);
                        let bytes = try!(reply.write_to_bytes());
                        try!(self.fe_sock.send(&bytes, 0));
                    }
                }
                fe_sock = false;
            }
        }
        heartbeat.join().unwrap();
        runner.join().unwrap();
        Ok(())
    }

    fn set_busy(&mut self) -> Result<()> {
        try!(self.hb_conn.send_str(PulseState::Pause.as_ref(), 0));
        try!(self.hb_conn.recv(&mut self.msg, 0));
        self.state = State::Busy;
        Ok(())
    }

    fn set_ready(&mut self) -> Result<()> {
        try!(self.hb_conn.send_str(PulseState::Pulse.as_ref(), 0));
        try!(self.hb_conn.recv(&mut self.msg, 0));
        self.state = State::Ready;
        Ok(())
    }
}

impl NetIdent for Server {}

impl Drop for Server {
    fn drop(&mut self) {
        self.fe_sock.close().unwrap();
    }
}

#[derive(PartialEq)]
enum PulseState {
    Pause,
    Pulse,
}

impl AsRef<str> for PulseState {
    fn as_ref(&self) -> &str {
        match *self {
            PulseState::Pause => HB_CMD_PAUSE,
            PulseState::Pulse => HB_CMD_PULSE,
        }
    }
}

impl Default for PulseState {
    fn default() -> PulseState {
        PulseState::Pulse
    }
}

struct Heartbeat {
    state: PulseState,
    config: Arc<RwLock<Config>>,
    ctx: Arc<RwLock<zmq::Context>>,
    pub_sock: zmq::Socket,
    be_sock: zmq::Socket,
    reg: protocol::jobsrv::Heartbeat,
}

impl Heartbeat {
    fn new(config: Arc<RwLock<Config>>, ctx: Arc<RwLock<zmq::Context>>) -> Result<Self> {
        let (pub_sock, be_sock) = {
            let mut ctx = ctx.write().unwrap();
            let pub_sock = try!(ctx.socket(zmq::PUB));
            let be_sock = try!(ctx.socket(zmq::REP));
            (pub_sock, be_sock)
        };
        try!(pub_sock.set_immediate(true));
        try!(pub_sock.set_sndhwm(1));
        try!(pub_sock.set_linger(0));
        let mut reg = protocol::jobsrv::Heartbeat::new();
        reg.set_endpoint(Server::net_ident());
        reg.set_os(worker_os());
        reg.set_state(protocol::jobsrv::WorkerState::Ready);
        Ok(Heartbeat {
            state: PulseState::default(),
            config: config,
            ctx: ctx,
            pub_sock: pub_sock,
            be_sock: be_sock,
            reg: reg,
        })
    }

    pub fn start(config: Arc<RwLock<Config>>,
                 ctx: Arc<RwLock<zmq::Context>>)
                 -> Result<JoinHandle<()>> {
        let (tx, rx) = mpsc::sync_channel(0);
        let handle = thread::Builder::new()
            .name("heartbeat".to_string())
            .spawn(move || {
                let mut heartbeat = Self::new(config, ctx).unwrap();
                heartbeat.run(tx).unwrap();
            })
            .unwrap();
        match rx.recv() {
            Ok(()) => Ok(handle),
            Err(e) => panic!("heartbeat thread startup error, err={}", e),
        }
    }

    fn run(&mut self, rz: mpsc::SyncSender<()>) -> Result<()> {
        {
            let cfg = self.config.read().unwrap();
            for (hb, _) in cfg.jobsrv_addrs() {
                println!("Connecting to heartbeat, {}", hb);
                try!(self.pub_sock.connect(&hb));
            }
        }
        try!(self.be_sock.bind(HB_INPROC_ADDR));
        rz.send(()).unwrap();
        // Needed for connections to establish. Wow zmq. Good thing we need to use a splay anyway.
        thread::sleep(Duration::from_millis(100));
        let mut be_sock = false;
        let mut msg = try!(zmq::Message::new());
        loop {
            if self.state == PulseState::Pulse {
                try!(self.pulse());
            }
            {
                let mut items = [self.be_sock.as_poll_item(1)];
                // Poll until timeout or message is received. Checking for the zmq::POLLIN flag on
                // a poll item's revents will let you know if you have received a message or not
                // on that socket.
                try!(zmq::poll(&mut items, HEARTBEAT_MS));
                if items[0].get_revents() & zmq::POLLIN > 0 {
                    be_sock = true;
                }
            }
            if be_sock {
                try!(self.be_sock.recv(&mut msg, 0));
                match msg.as_str() {
                    Some(HB_CMD_PAUSE) => {
                        self.pause();
                        try!(self.be_sock.send(&[], 0));
                    }
                    Some(HB_CMD_PULSE) => {
                        self.resume();
                        try!(self.be_sock.send(&[], 0));
                    }
                    _ => (),
                }
                be_sock = false;
            }
        }
        Ok(())
    }

    fn pause(&mut self) {
        debug!("heartbeat paused");
        self.reg.set_state(protocol::jobsrv::WorkerState::Busy);
        self.state = PulseState::Pause;
    }

    fn resume(&mut self) {
        debug!("heartbeat resumed");
        self.reg.set_state(protocol::jobsrv::WorkerState::Ready);
        self.state = PulseState::Pulse;
    }

    fn pulse(&mut self) -> Result<()> {
        debug!("heartbeat pulsed");
        try!(self.pub_sock.send(&self.reg.write_to_bytes().unwrap(), 0));
        Ok(())
    }
}

pub struct Runner {
    #[allow(dead_code)]
    config: Arc<RwLock<Config>>,
    #[allow(dead_code)]
    ctx: Arc<RwLock<zmq::Context>>,
    sock: zmq::Socket,
}

impl Runner {
    fn new(config: Arc<RwLock<Config>>, ctx: Arc<RwLock<zmq::Context>>) -> Result<Self> {
        let sock = {
            let mut ctx = ctx.write().unwrap();
            try!(ctx.socket(zmq::DEALER))
        };
        Ok(Runner {
            config: config,
            ctx: ctx,
            sock: sock,
        })
    }

    pub fn start(config: Arc<RwLock<Config>>,
                 ctx: Arc<RwLock<zmq::Context>>)
                 -> Result<JoinHandle<()>> {
        let (tx, rx) = mpsc::sync_channel(0);
        let handle = thread::Builder::new()
            .name("runner".to_string())
            .spawn(move || {
                let mut runner = Self::new(config, ctx).unwrap();
                runner.run(tx).unwrap();
            })
            .unwrap();
        match rx.recv() {
            Ok(()) => Ok(handle),
            Err(e) => panic!("runner thread startup error, err={}", e),
        }
    }

    fn run(&mut self, rz: mpsc::SyncSender<()>) -> Result<()> {
        try!(self.sock.bind(RUNNER_INPROC_ADDR));
        rz.send(()).unwrap();
        let mut msg = try!(zmq::Message::new());
        loop {
            try!(self.sock.recv(&mut msg, 0));
            let mut job: protocol::jobsrv::Job = parse_from_bytes(&msg).unwrap();
            debug!("processing job={:?}", job);
            try!(self.sock.send_str(WORK_ACK, zmq::SNDMORE));
            try!(self.sock.send_str(&job.get_id().to_string(), 0));
            self.execute_job(&mut job);
            try!(self.sock.send_str(WORK_COMPLETE, zmq::SNDMORE));
            try!(self.sock.send(&job.write_to_bytes().unwrap(), 0));
        }
        Ok(())
    }

    fn execute_job(&mut self, job: &mut protocol::jobsrv::Job) {
        thread::sleep(Duration::from_millis(5_000));
        // set Failed on failure
        debug!("job complete, {:?}", job);
        job.set_state(protocol::jobsrv::JobState::Complete);
    }
}

pub fn run(config: Config) -> Result<()> {
    try!(Server::new(config)).run()
}
