use config::Config;
use error::Result;
use hab_net::server::ZMQ_CONTEXT;
use std::sync::{mpsc, Arc, RwLock};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use zmq;

/// In-memory zmq address for LogForwarder
pub const INPROC_ADDR: &'static str = "inproc://logger";

pub struct LogForwarder {
    /// The socket on which log data is received from workers.
    pub intake_sock: zmq::Socket,
    /// The socket from which log data is forwarded to the appropriate
    /// job server.
    pub output_sock: zmq::Socket,
    /// The configuration of the worker server; used to obtain job
    /// server connection information.
    config: Arc<RwLock<Config>>,
}

impl LogForwarder {
    pub fn new(config: Arc<RwLock<Config>>) -> Result<Self> {
        let intake_sock = (**ZMQ_CONTEXT).as_mut().socket(zmq::PULL)?;
        let output_sock = (**ZMQ_CONTEXT).as_mut().socket(zmq::DEALER)?;

        output_sock.set_sndhwm(5000)?;
        output_sock.set_linger(5000)?;
        output_sock.set_immediate(true)?;

        Ok(LogForwarder {
               intake_sock: intake_sock,
               output_sock: output_sock,
               config: config,
           })
    }

    pub fn start(config: Arc<RwLock<Config>>) -> Result<JoinHandle<()>> {
        let (tx, rx) = mpsc::sync_channel(0);
        let handle = thread::Builder::new()
            .name("log".to_string())
            .spawn(move || {
                       let mut log = Self::new(config).unwrap();
                       log.run(tx).unwrap();
                   })
            .unwrap();
        match rx.recv() {
            Ok(()) => Ok(handle),
            Err(e) => panic!("log thread startup error, err={}", e),
        }
    }

    pub fn run(&mut self, rz: mpsc::SyncSender<()>) -> Result<()> {
        {
            let cfg = self.config.read().unwrap();
            let addrs = cfg.jobsrv_addrs();
            if addrs.len() == 1 {
                let (_, _, ref log) = addrs[0];
                println!("Connecting to Job Server Log port, {}", log);
                self.output_sock.connect(&log)?;
            } else {
                panic!("Routing logs to more than one Job Server is not yet implemented");
            }
        }
        self.intake_sock.bind(INPROC_ADDR)?;

        // Signal back to the spawning process that we're good
        rz.send(()).unwrap();

        // This hacky sleep is recommended and required by zmq for
        // connections to establish
        thread::sleep(Duration::from_millis(100));

        // Basically just need to pass things through... proxy time!
        // If we ever have multiple JobServers these need to be sent
        // to, then we might need some additional logic.
        zmq::proxy(&mut self.intake_sock, &mut self.output_sock)?;
        Ok(())
    }
}
