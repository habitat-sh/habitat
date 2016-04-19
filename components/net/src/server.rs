// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::marker::PhantomData;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use zmq;

pub struct Supervisor<T>
    where T: Supervisable
{
    context: Arc<Mutex<zmq::Context>>,
    config: Arc<Mutex<T::Config>>,
    workers: Vec<mpsc::Receiver<()>>,
    _marker: PhantomData<T>,
}

impl<T> Supervisor<T>
    where T: Supervisable + 'static
{
    pub fn new(ctx: Arc<Mutex<zmq::Context>>, config: Arc<Mutex<T::Config>>) -> Self {
        Supervisor {
            context: ctx,
            config: config,
            workers: vec![],
            _marker: PhantomData,
        }
    }

    pub fn start(mut self, addr: &str, worker_count: usize) -> super::Result<()> {
        try!(self.init(addr, worker_count));
        debug!("Supervisor ready");
        self.run(addr.to_string(), worker_count)
    }

    fn init(&mut self, addr: &str, worker_count: usize) -> super::Result<()> {
        for _i in 0..worker_count {
            let rx = try!(self.spawn_worker(addr.to_string()));
            self.workers.push(rx);
        }
        let mut success = 0;
        while success != worker_count {
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

    fn run(mut self, addr: String, worker_count: usize) -> super::Result<()> {
        thread::spawn(move || {
            loop {
                for i in 0..worker_count {
                    match self.workers[i].try_recv() {
                        Err(mpsc::TryRecvError::Disconnected) => {
                            println!("Worker {} restarting...", i);
                            let rx = self.spawn_worker(addr.clone()).unwrap();
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

    fn spawn_worker(&self, addr: String) -> super::Result<mpsc::Receiver<()>> {
        let cfg = self.config.clone();
        let (tx, rx) = mpsc::sync_channel(1);
        let worker = T::new(&mut self.context.lock().unwrap(), cfg);
        thread::spawn(move || worker.start(addr, tx));
        Ok(rx)
    }
}

pub trait Supervisable : Sized + Send {
    type Config : Send;
    type Error : Send + From<zmq::Error>;

    fn new(context: &mut zmq::Context, config: Arc<Mutex<Self::Config>>) -> Self;

    fn init(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_message(&mut self, msg: zmq::Message) -> Result<(), Self::Error>;

    fn socket(&mut self) -> &mut zmq::Socket;

    fn start(mut self, be_addr: String, rz: mpsc::SyncSender<()>) -> Result<(), Self::Error> {
        try!(self.init());
        try!(self.socket().connect(&be_addr));
        rz.send(()).unwrap();
        loop {
            let msg = try!(self.socket().recv_msg(0));
            match self.on_message(msg) {
                Ok(()) => continue,
                Err(e) => return Err(e),
            }
        }
    }
}
