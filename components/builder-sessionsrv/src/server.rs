// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::thread;

use protobuf::{parse_from_bytes, Message};
use protocol::net::{self, ErrCode};
use protocol::sessionsrv::{AuthToken, GitHubAuth};
use zmq;

use config::Config;
use data_store::DataStore;
use error::{Error, Result};
use oauth::github;

const BE_LISTEN_ADDR: &'static str = "inproc://backend";

pub struct Worker {
    config: Arc<Mutex<Config>>,
    sock: zmq::Socket,
    datastore: DataStore,
}

impl Worker {
    pub fn new(context: &mut zmq::Context, config: Arc<Mutex<Config>>) -> Result<Self> {
        let sock = context.socket(zmq::DEALER).unwrap();
        Ok(Worker {
            config: config,
            sock: sock,
            datastore: DataStore::new(),
        })
    }

    fn start(mut self, rz: mpsc::SyncSender<()>) -> Result<()> {
        loop {
            {
                let cfg = self.config.lock().unwrap();
                match self.datastore.open(&cfg) {
                    Ok(()) => break,
                    Err(e) => {
                        error!("{}", e);
                        thread::sleep(Duration::from_millis(5000));
                    }
                }
            }
        }
        try!(self.sock.connect(BE_LISTEN_ADDR));
        let mut msg = zmq::Message::new().unwrap();
        rz.send(()).unwrap();
        loop {
            // JW TODO: abstract this out to be more developer friendly
            // pop client ident
            let ident = try!(self.sock.recv_msg(0));
            // pop lq ident
            let ident2 = try!(self.sock.recv_msg(0));
            // pop request frame
            try!(self.sock.recv(&mut msg, 0));
            // pop message-id
            try!(self.sock.recv(&mut msg, 0));
            // send reply
            //  -> client ident
            //  -> lq ident
            //  -> empty rep frame
            //  -> actual message
            self.sock.send_msg(ident, zmq::SNDMORE).unwrap();
            self.sock.send_msg(ident2, zmq::SNDMORE).unwrap();
            self.sock.send(&[], zmq::SNDMORE).unwrap();
            match self.dispatch(&msg) {
                Ok(()) => continue,
                Err(e) => return Err(e),
            }
        }
    }

    fn dispatch(&mut self, msg: &zmq::Message) -> Result<()> {
        match msg.as_str() {
            Some("LOGIN") => {
                let request = try!(self.sock.recv_msg(0));
                let req: GitHubAuth = parse_from_bytes(&request).unwrap();
                match github::authenticate(&req.get_code()) {
                    Ok(token) => {
                        let mut reply: AuthToken = AuthToken::new();
                        reply.set_token(token);
                        self.sock.send_str("AuthToken", zmq::SNDMORE).unwrap();
                        self.sock.send(&reply.write_to_bytes().unwrap(), 0).unwrap();
                    }
                    Err(Error::Auth(e)) => {
                        debug!("login failure: {:?}", e);
                        let reply = net::err(ErrCode::REMOTE_REJECTED, e.error);
                        self.sock.send_str("NetError", zmq::SNDMORE).unwrap();
                        self.sock.send(&reply.write_to_bytes().unwrap(), 0).unwrap();
                    }
                    Err(e @ Error::JsonDecode(_)) => {
                        debug!("login failure: {:?}", e);
                        let reply = net::err(ErrCode::BAD_REMOTE_REPLY, "ss::auth:1");
                        self.sock.send_str("NetError", zmq::SNDMORE).unwrap();
                        self.sock.send(&reply.write_to_bytes().unwrap(), 0).unwrap();
                    }
                    Err(e) => {
                        error!("unhandled login failure: {:?}", e);
                        let reply = net::err(ErrCode::BUG, "ss::auth:0");
                        self.sock.send_str("NetError", zmq::SNDMORE).unwrap();
                        self.sock.send(&reply.write_to_bytes().unwrap(), 0).unwrap();
                    }
                }

            }
            _ => panic!("unexpected message: {:?}", msg.as_str()),
        }
        Ok(())
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.sock.close().unwrap();
    }
}

pub struct Server {
    config: Arc<Mutex<Config>>,
    ctx: Arc<Mutex<zmq::Context>>,
    fe_sock: zmq::Socket,
    be_sock: zmq::Socket,
}

impl Server {
    pub fn new(config: Config) -> Self {
        let mut ctx = zmq::Context::new();
        let fe = ctx.socket(zmq::ROUTER).unwrap();
        let be = ctx.socket(zmq::DEALER).unwrap();
        Server {
            config: Arc::new(Mutex::new(config)),
            ctx: Arc::new(Mutex::new(ctx)),
            fe_sock: fe,
            be_sock: be,
        }
    }

    pub fn reconfigure(&self, config: Config) -> Result<()> {
        {
            let mut cfg = self.config.lock().unwrap();
            *cfg = config;
        }
        // obtain lock and replace our config
        // notify datastore to refresh it's connection if it needs to
        // notify sockets to reconnect if changes
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        {
            let cfg = self.config.lock().unwrap();
            try!(self.fe_sock.bind(&cfg.fe_addr()));
            try!(self.be_sock.bind(BE_LISTEN_ADDR));
            println!("Listening on ({})", cfg.fe_addr());
        }

        let ctx = self.ctx.clone();
        let cfg = self.config.clone();
        let sup = Supervisor::new(ctx, cfg);
        try!(sup.start());
        try!(zmq::proxy(&mut self.fe_sock, &mut self.be_sock));
        Ok(())
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.fe_sock.close().unwrap();
        self.be_sock.close().unwrap();
    }
}

pub struct Supervisor {
    context: Arc<Mutex<zmq::Context>>,
    config: Arc<Mutex<Config>>,
    workers: Vec<mpsc::Receiver<()>>,
}

impl Supervisor {
    pub fn new(ctx: Arc<Mutex<zmq::Context>>, config: Arc<Mutex<Config>>) -> Self {
        Supervisor {
            context: ctx,
            config: config,
            workers: vec![],
        }
    }

    pub fn start(mut self) -> Result<()> {
        try!(self.init());
        debug!("Supervisor ready");
        self.run()
    }

    fn init(&mut self) -> Result<()> {
        let count = {
            self.config.lock().unwrap().worker_count
        };
        for _i in 0..count {
            let rx = try!(self.spawn_worker());
            self.workers.push(rx);
        }
        let mut success = 0;
        while success != count {
            match self.workers[success].recv() {
                Ok(()) => {
                    debug!("Worker {} ready", success);
                    success += 1;
                }
                Err(_) => debug!("Worker {} failed to start", success),
            }
        }
        Ok(())
    }

    fn run(mut self) -> Result<()> {
        thread::spawn(move || {
            loop {
                let count = {
                    self.config.lock().unwrap().worker_count
                };
                for i in 0..count {
                    match self.workers[i].try_recv() {
                        Err(mpsc::TryRecvError::Disconnected) => {
                            println!("Worker {} restarting...", i);
                            let rx = self.spawn_worker().unwrap();
                            match rx.recv() {
                                Ok(()) => self.workers[i] = rx,
                                Err(_) => {
                                    println!("Worker {} failed restart!", i);
                                    self.workers.remove(i);
                                }
                            }
                        }
                        Ok(msg) => println!("Worker {} sent unexpected msg: {:?}", i, msg),
                        Err(mpsc::TryRecvError::Empty) => continue,
                    }
                }
                thread::sleep(Duration::from_millis(500));
            }
        });
        Ok(())
    }

    fn spawn_worker(&self) -> Result<mpsc::Receiver<()>> {
        let cfg = self.config.clone();
        let (tx, rx) = mpsc::sync_channel(1);
        let worker = try!(Worker::new(&mut self.context.lock().unwrap(), cfg));
        thread::spawn(move || {
            worker.start(tx).unwrap();
        });
        Ok(rx)
    }
}

pub fn run(config: Config) -> Result<()> {
    Server::new(config).run()
}
