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

use bldr_core::logger::Logger;
use error::{Error, Result};
use hab_net::socket::DEFAULT_CONTEXT;
use protobuf::Message;
use protocol::jobsrv::{JobLogComplete, JobLogChunk};
use std::io::{BufRead, BufReader};
use std::process;
use super::workspace::Workspace;
use zmq;

const INPROC_ADDR: &'static str = "inproc://logger";
const EOL_MARKER: &'static str = "\n";

/// ZMQ protocol frame to indicate a log line is being sent
const LOG_LINE: &'static str = "L";
/// ZMQ protocol frame to indicate a log has finished
const LOG_COMPLETE: &'static str = "C";


pub struct LogPipe {
    job_id: u64,
    sock: zmq::Socket,
    logger: Logger,
    line_count: u64,
}

impl LogPipe {
    pub fn new(workspace: &Workspace) -> Self {
        let sock = (**DEFAULT_CONTEXT).as_mut().socket(zmq::PUSH).unwrap();
        sock.set_immediate(true).unwrap();
        sock.set_linger(5000).unwrap();
        sock.connect(INPROC_ADDR).unwrap();

        let ident = format!("log_pipe-{}.log", workspace.job.get_id());

        let mut logger = Logger::init(workspace.root(), &ident);
        logger.log_ident(&ident);

        LogPipe {
            job_id: workspace.job.get_id(),
            sock,
            logger,
            line_count: 0,
        }
    }

    /// Stream log output via ZMQ back to the Job Server for
    /// aggregation and streaming to downstream clients.
    ///
    /// Contents of STDOUT are streamed before any from STDERR (if
    /// any).
    pub fn pipe(&mut self, process: &mut process::Child) -> Result<()> {
        self.logger.log("About to log stdout");
        if let Some(ref mut stdout) = process.stdout {
            let reader = BufReader::new(stdout);
            self.stream_lines(reader)?;
        }
        self.logger.log("Finished logging stdout");
        Ok(())
    }

    pub fn pipe_stdout(&mut self, content: &[u8]) -> Result<()> {
        self.logger.log("About to log stdout");
        self.stream_lines(BufReader::new(content))?;
        self.logger.log("Finished logging stdout");
        Ok(())
    }

    /// Send the lines of the reader out over the ZMQ socket as
    /// `JobLogChunk` messages.
    ///
    /// `line_count` is the line number to start with when generating
    /// JobLogChunk messages. This allows us to send multiple output
    /// to the same job (i.e. standard output and standard error).
    fn stream_lines<B: BufRead>(&mut self, reader: B) -> Result<()> {
        for line in reader.lines() {
            self.line_count += 1;
            let mut l: String = line.unwrap();
            self.logger.log(format!("{}", l).as_ref());
            l = l + EOL_MARKER;

            let mut chunk = JobLogChunk::new();
            chunk.set_job_id(self.job_id);
            chunk.set_seq(self.line_count);
            chunk.set_content(l.clone());

            if let Err(e) = self.sock.send_str(LOG_LINE, zmq::SNDMORE) {
                self.logger.log(
                    format!("ZMQ error when sending LOG_LINE: {:?}", &e).as_ref(),
                );
                return Err(Error::Zmq(e));
            }
            if let Err(e) = self.sock.send(
                chunk.write_to_bytes().unwrap().as_slice(),
                0,
            )
            {
                self.logger.log(
                    format!("ZMQ error when sending JobLogChunk {:?} : {:?}", &chunk, &e)
                        .as_ref(),
                );
                return Err(Error::Zmq(e));
            }
        }

        Ok(())
    }

    fn complete(&mut self) {
        self.logger.log(
            "About to tell log_forwarder that the job is complete",
        );

        // Signal that the log is finished
        let mut complete = JobLogComplete::new();
        complete.set_job_id(self.job_id);
        debug!("completing log_forwarder, job_id={}", self.job_id);
        if let Err(e) = self.sock.send_str(LOG_COMPLETE, zmq::SNDMORE) {
            self.logger.log(
                format!("ZMQ error when sending LOG_COMPLETE: {:?}", &e).as_ref(),
            );
            warn!("ZMQ error when sending LOG_COMPLETE: {:?}", &e);
            return;
        }
        if let Err(e) = self.sock.send(
            complete.write_to_bytes().unwrap().as_slice(),
            0,
        )
        {
            self.logger.log(
                format!(
                    "ZMQ error when sending JobLogComplete {:?} : {:?}",
                    &complete,
                    &e
                ).as_ref(),
            );
            warn!(
                "ZMQ error when sending JobLogComplete {:?} : {:?}",
                &complete,
                &e
            );
            return;
        }
        self.logger.log(
            "Finished telling log_forwarder that the job is complete",
        );
    }
}

impl Drop for LogPipe {
    fn drop(&mut self) {
        self.complete();
    }
}
