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

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::str;
use std::sync::{mpsc, Arc};
use std::thread::{self, JoinHandle};

use hab_net::socket::DEFAULT_CONTEXT;
use protobuf::parse_from_bytes;
use protocol::jobsrv::{JobLogComplete, JobLogChunk};
use server::log_archiver::{self, LogArchiver};
use server::log_directory::LogDirectory;
use zmq;

use config::Config;
use data_store::DataStore;
use error::Result;

/// ZMQ protocol frame to indicate a log line is being sent
const LOG_LINE: &'static str = "L";
/// ZMQ protocol frame to indicate a log has finished
const LOG_COMPLETE: &'static str = "C";

/// Listens for log messages from builders and consolidates output for
/// both streaming to clients and long-term storage.
pub struct LogIngester {
    intake_sock: zmq::Socket,
    msg: zmq::Message,
    log_dir: Arc<LogDirectory>,
    log_ingestion_addr: String,
    data_store: DataStore,
    archiver: Box<LogArchiver>,
}

impl LogIngester {
    pub fn new(config: &Config, log_dir: Arc<LogDirectory>, data_store: DataStore) -> Result<Self> {
        let intake_sock = (**DEFAULT_CONTEXT).as_mut().socket(zmq::ROUTER)?;
        intake_sock.set_router_mandatory(true)?;
        Ok(LogIngester {
            intake_sock: intake_sock,
            msg: zmq::Message::new()?,
            log_dir: log_dir,
            log_ingestion_addr: config.net.log_ingestion_addr(),
            data_store: data_store,
            archiver: log_archiver::from_config(&config.archive)?,
        })
    }

    pub fn start(
        cfg: &Config,
        log_dir: Arc<LogDirectory>,
        data_store: DataStore,
    ) -> Result<JoinHandle<()>> {
        let mut ingester = Self::new(cfg, log_dir, data_store)?;
        let (tx, rx) = mpsc::sync_channel(1);
        let handle = thread::Builder::new()
            .name("log-ingester".to_string())
            .spawn(move || { ingester.run(tx).unwrap(); })
            .unwrap();
        match rx.recv() {
            Ok(()) => Ok(handle),
            Err(e) => panic!("log-ingester thread startup error, err={}", e),
        }
    }

    fn run(&mut self, rz: mpsc::SyncSender<()>) -> Result<()> {
        println!("Listening for log data on {}", self.log_ingestion_addr);
        self.intake_sock.bind(&self.log_ingestion_addr)?;
        rz.send(()).unwrap();
        loop {
            // Right now we've got 3 frames per message:
            // 1: peer identity (we're using a ROUTER socket)
            // 2: a single-character code indicating message type:
            //    L = a line of log output
            //    C = the log is complete
            // 3: a protobuf message
            self.intake_sock.recv(&mut self.msg, 0)?; // identity frame

            match str::from_utf8(self.intake_sock.recv_bytes(0).unwrap().as_slice()).unwrap() {
                LOG_LINE => {
                    self.intake_sock.recv(&mut self.msg, 0)?; // protobuf message frame
                    match parse_from_bytes::<JobLogChunk>(&self.msg) {
                        Ok(chunk) => {
                            let log_file = self.log_dir.log_file_path(chunk.get_job_id());

                            // TODO: Consider caching file handles for
                            // currently-processing logs.
                            let open = OpenOptions::new().create(true).append(true).open(
                                log_file
                                    .as_path(),
                            );

                            match open {
                                Ok(mut file) => {
                                    file.write(chunk.get_content().as_bytes())?;
                                    file.flush()?;
                                }
                                Err(e) => {
                                    warn!("Could not open {:?} for appending! {:?}", log_file, e);
                                }
                            }
                        }
                        Err(e) => {
                            warn!("ERROR parsing JobLogChunk: {:?}", e);
                        }
                    }
                }
                LOG_COMPLETE => {
                    self.intake_sock.recv(&mut self.msg, 0)?; // protobuf message frame
                    match parse_from_bytes::<JobLogComplete>(&self.msg) {
                        Ok(complete) => {
                            if let Err(e) = self.complete_log(&complete) {
                                // TODO: Investigate error and attempt
                                // to remediate as appropriate.
                                warn!("Error completing log: {}", e);
                            }
                        }
                        Err(e) => {
                            warn!("ERROR parsing JobLogComplete: {:?}", e);
                        }
                    }
                }
                other => {
                    warn!("UNRECOGNIZED LOG PROTOCOL CODE: {:?}", other);
                }
            }
        }
    }

    /// Factored out the above loop to take advantage of ?'s behavior
    /// in Result-returning functions to collapse deeply branching
    /// code.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following scenarios:
    ///
    /// * Failure to archive the log file
    /// * Failure to mark job as archived in database
    /// * Failure to remove local log file
    ///
    /// This is also the _order_ in which these errors would occur, so
    /// a local log file is only removed after the log is successfully
    /// archived and marked as such in the database.
    fn complete_log(&self, complete: &JobLogComplete) -> Result<()> {
        let id = complete.get_job_id();
        debug!("Log complete for job {:?}", id);
        let log_file = self.log_dir.log_file_path(id);

        self.archiver.archive(id, &log_file)?;
        debug!("Archived log for job {}", id);
        self.data_store.mark_as_archived(id)?;
        fs::remove_file(&log_file)?;
        debug!("Successfully deleted local log file {:?}", log_file);
        Ok(())
    }
}
