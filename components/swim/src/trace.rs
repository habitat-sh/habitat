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

//! This module handles the writing of swim trace files, which can later be post-processed to see
//! whats happening in a network.

use time;

use std::default::Default;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::io::Write;

use server::Server;
use message::swim::Swim;

/// The trace struct handles writing trace files to a directory path.
#[derive(Debug)]
pub struct Trace {
    pub directory: PathBuf,
    pub file: Option<fs::File>,
    pub on: bool,
}

impl Default for Trace {
    fn default() -> Trace {
        Trace {
            directory: PathBuf::from("/tmp/habitat-swim-trace"),
            file: None,
            on: false,
        }
    }
}

impl Trace {
    /// Initialize the trace object; only happens once.
    pub fn init(&mut self, server: &Server) {
        if self.file.is_none() {
            let now = time::now_utc();
            let filename = format!("{}-{}.swimtrace", server.name(), now.rfc3339());
            match fs::File::create(self.directory.join(&filename)) {
                Ok(f) => self.file = Some(f),
                Err(e) => {
                    panic!("Trace requested, but cannot create file {:?}: {}",
                           self.directory.join(&filename),
                           e)
                }
            }
        }
    }

    /// Returns true if the TRACE_SWIM environment variable exists.
    pub fn on(&self) -> bool {
        match env::var("TRACE_SWIM") {
            Ok(_val) => true,
            Err(_e) => self.on,
        }
    }

    /// Write a line to the trace file.
    pub fn write(&mut self,
                 module_path: &str,
                 line: &u32,
                 server_name: &str,
                 server_id: &str,
                 listening: &str,
                 thread_name: &str,
                 msg_type: &str,
                 to_addr: &str,
                 payload: Option<&Swim>) {
        let now = time::now_utc();
        let time_string = format!("{}-{}-{}-{}-{}-{}-{}",
                                  1900 + now.tm_year,
                                  now.tm_mon + 1,
                                  now.tm_mday,
                                  now.tm_hour,
                                  now.tm_min,
                                  now.tm_sec,
                                  now.tm_nsec);
        let dump = format!("{:#?}", self);
        match self.file.as_mut() {
            Some(mut file) => {
                match write!(file,
                             "{}!*!{}!*!{}!*!{}!*!{}!*!{}!*!{}!*!{}!*!{}!*!{:?}\n",
                             time_string,
                             module_path,
                             line,
                             server_name,
                             server_id,
                             listening,
                             thread_name,
                             msg_type,
                             to_addr,
                             payload) {
                    Ok(_) => {}
                    Err(e) => panic!("Trace requested, but failed to write {:?}", e),
                }
            }
            None => {
                panic!("Trace requested, but init was never called; use the trace! macro \
                        instead: {:#?}",
                       dump);
            }
        }
    }

    // Outbound
    // datetime!*!server.name!*!member.id!*!socket!*!outbound!*!MsgType!*!ToAddr!*!Payload
    //
    // Inbound
    // datetime!*!server.name!*!member.id!*!socket!*!inbound!*!MsgType!*!FromAddr!*!Payload
}

macro_rules! trace_swim {
    ($server:expr, $msg_type:expr, $to_addr:expr, $payload:expr) => {
        {
            let mut trace = $server.trace.write().expect("Trace write lock is poisoned");
            if trace.on() {
                trace.init($server);
                let member_id = {
                    let member = $server.member.read().expect("Member lock is read poisoned");
                    String::from(member.get_id())
                };
                let local_addr = $server.socket.local_addr().expect("Socket has no listen address; should be impossible");
                let thread_name = String::from(thread::current().name().expect("Tried to trace an un-named thread; use thread-builder"));
                trace.write(
                    &module_path!(),
                    &line!(),
                    &$server.name(),
                    &member_id,
                    &format!("{}", local_addr),
                    &thread_name,
                    $msg_type,
                    $to_addr,
                    $payload
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod trace {
        use trace::Trace;
        use std::path::Path;

        #[test]
        fn default() {
            let trace = Trace::default();
            assert_eq!(trace.directory, Path::new("/tmp/habitat-swim-trace"));
        }
    }
}
