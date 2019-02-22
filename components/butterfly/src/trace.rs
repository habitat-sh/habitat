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

//! This module handles the writing of swim trace files, which can later be post-processed to see
//! whats happening in a network.

use time;

use std::{env,
          fmt,
          fs,
          io::Write,
          path::PathBuf};

use crate::server::Server;

#[derive(Debug, Clone, Copy)]
pub enum TraceKind {
    MemberUpdate,
    ProbeBegin,
    ProbeAckReceived,
    ProbeComplete,
    ProbeConfirmed,
    ProbeSuspect,
    ProbeDeparted,
    ProbePingReq,
    RecvAck,
    RecvPing,
    RecvPingReq,
    RecvRumor,
    SendAck,
    SendForwardAck,
    SendPing,
    SendPingReq,
    SendRumor,
    TestEvent,
}

impl fmt::Display for TraceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TraceKind::MemberUpdate => write!(f, "MemberUpdate"),
            TraceKind::ProbeBegin => write!(f, "ProbeBegin"),
            TraceKind::ProbeAckReceived => write!(f, "ProbeAckReceived"),
            TraceKind::ProbeConfirmed => write!(f, "ProbeConfirmed"),
            TraceKind::ProbeComplete => write!(f, "ProbeComplete"),
            TraceKind::ProbeSuspect => write!(f, "ProbeSuspect"),
            TraceKind::ProbeDeparted => write!(f, "ProbeDeparted"),
            TraceKind::ProbePingReq => write!(f, "ProbePingReq"),
            TraceKind::RecvAck => write!(f, "RecvAck"),
            TraceKind::RecvPing => write!(f, "RecvPing"),
            TraceKind::RecvPingReq => write!(f, "RecvPingReq"),
            TraceKind::RecvRumor => write!(f, "RecvRumor"),
            TraceKind::SendAck => write!(f, "SendAck"),
            TraceKind::SendForwardAck => write!(f, "SendForwardAck"),
            TraceKind::SendPing => write!(f, "SendPing"),
            TraceKind::SendPingReq => write!(f, "SendPingReq"),
            TraceKind::SendRumor => write!(f, "SendRumor"),
            TraceKind::TestEvent => write!(f, "TestEvent"),
        }
    }
}

#[derive(Debug)]
pub struct TraceWrite<'a> {
    pub kind: TraceKind,
    pub time: String,
    pub module_path: &'a str,
    pub line: u32,
    pub thread_name: &'a str,
    pub server_name: Option<&'a str>,
    pub member_id: Option<&'a str>,
    pub to_member_id: Option<&'a str>,
    pub listening: Option<&'a str>,
    pub to_addr: Option<&'a str>,
    pub swim: Option<&'a str>,
    pub rumor: Option<&'a str>,
}

impl<'a> TraceWrite<'a> {
    pub fn new(
        kind: TraceKind,
        module_path: &'a str,
        line: u32,
        thread_name: &'a str,
    ) -> TraceWrite<'a> {
        let now = time::now_utc();
        let time_string = format!(
            "{}-{}-{}-{}-{}-{}-{}",
            1900 + now.tm_year,
            now.tm_mon + 1,
            now.tm_mday,
            now.tm_hour,
            now.tm_min,
            now.tm_sec,
            now.tm_nsec
        );
        TraceWrite {
            kind,
            time: time_string,
            module_path,
            line,
            thread_name,
            server_name: None,
            member_id: None,
            to_member_id: None,
            listening: None,
            to_addr: None,
            swim: None,
            rumor: None,
        }
    }
}

impl<'a> fmt::Display for TraceWrite<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.time)?;
        write!(f, "^{}", self.kind)?;
        write!(f, "^{}", self.thread_name)?;
        write!(f, "^{}", self.module_path)?;
        write!(f, "^{}", self.line)?;
        write!(f, "^{}", self.server_name.unwrap_or(""))?;
        write!(f, "^{}", self.member_id.unwrap_or(""))?;
        write!(f, "^{}", self.to_member_id.unwrap_or(""))?;
        write!(f, "^{}", self.listening.unwrap_or(""))?;
        write!(f, "^{}", self.to_addr.unwrap_or(""))?;
        write!(f, "^{}", self.swim.unwrap_or(""))?;
        write!(f, "^{}", self.rumor.unwrap_or(""))?;
        writeln!(f)
    }
}

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
            match fs::create_dir_all(&self.directory) {
                Ok(_) => match fs::File::create(self.directory.join(&filename)) {
                    Ok(f) => self.file = Some(f),
                    Err(e) => panic!(
                        "Trace requested, but cannot create file {:?}: {}",
                        self.directory.join(&filename),
                        e
                    ),
                },
                Err(e) => {
                    panic!(
                        "Trace requested, but cannot create directory {:?}: {}",
                        self.directory, e
                    );
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
    pub fn write(&mut self, trace_write: TraceWrite<'_>) {
        let dump = format!("{:#?}", self);
        match self.file.as_mut() {
            Some(file) => match write!(file, "{}", trace_write) {
                Ok(_) => {}
                Err(e) => panic!("Trace requested, but failed to write {:?}", e),
            },
            None => {
                panic!(
                    "Trace requested, but init was never called; use the trace! macro instead: \
                     {:#?}",
                    dump
                );
            }
        }
    }
}

#[macro_export]
macro_rules! trace_it {
    (TEST: $server:expr, $payload:expr) => {{
        let trace_on = $server.trace.read().expect("Trace lock is poisoned").on();
        if trace_on {
            use habitat_butterfly::trace::{TraceKind,
                                           TraceWrite};
            use std::thread;
            let mut trace = $server.trace.write().expect("Trace lock is poisoned");
            trace.init($server);
            let thread = thread::current();
            let thread_name = thread.name().unwrap_or("undefined");
            let member_id = $server.member_id();
            let server_name = $server.name();
            let payload = format!("{} {} {}", server_name, member_id, $payload);
            let mut tw =
                TraceWrite::new(TraceKind::TestEvent, module_path!(), line!(), thread_name);
            tw.server_name = Some(&server_name);
            tw.member_id = Some(member_id);
            tw.rumor = Some(&payload);
            trace.write(tw);
        }
    }};

    (TEST_NET: $net:expr, $payload:expr) => {{
        for x in $net.members.iter() {
            let trace_on = x.trace.read().expect("Trace lock is poisoned").on();
            if trace_on {
                use habitat_butterfly::trace::{TraceKind,
                                               TraceWrite};
                use std::thread;
                let mut trace = x.trace.write().expect("Trace lock is poisoned");
                trace.init(x);
                let thread = thread::current();
                let thread_name = thread.name().unwrap_or("undefined");
                let member_id = x.member_id();
                let server_name = x.name();
                let payload = format!("{} {} {}", server_name, member_id, $payload);

                let mut tw =
                    TraceWrite::new(TraceKind::TestEvent, module_path!(), line!(), thread_name);
                tw.server_name = Some(&server_name);
                tw.member_id = Some(member_id);
                tw.rumor = Some(&payload);
                trace.write(tw);
            }
        }
    }};

    (MEMBERSHIP: $server:expr, $msg_type:expr, $member_id:expr, $mem_incar:expr, $health:expr) => {{
        let trace_on = $server.trace.read().expect("Trace lock is poisoned").on();
        if trace_on {
            use crate::trace::TraceWrite;
            let mut trace = $server.trace.write().expect("Trace lock is poisoned");
            trace.init($server);
            let thread = thread::current();
            let thread_name = thread.name().unwrap_or("undefined");
            let member_id = $server.member_id();
            let server_name = $server.name();
            let rumor_text = format!("{}-{}-{}", $member_id, $mem_incar, $health);

            let mut tw = TraceWrite::new($msg_type, module_path!(), line!(), thread_name);
            tw.server_name = Some(&server_name);
            tw.member_id = Some(member_id);
            tw.rumor = Some(&rumor_text);
            trace.write(tw);
        }
    }};

    (PROBE: $server:expr, $msg_type:expr, $to_member_id:expr, $to_addr:expr) => {{
        let trace_on = $server.trace.read().expect("Trace lock is poisoned").on();
        if trace_on {
            use crate::trace::TraceWrite;
            let mut trace = $server.trace.write().expect("Trace lock is poisoned");
            trace.init($server);
            let thread = thread::current();
            let thread_name = thread.name().unwrap_or("undefined");
            let listening = format!("{}", $server.swim_addr());
            let to_addr = format!("{}", $to_addr);
            let member_id = $server.member_id();
            let server_name = $server.name();

            let mut tw = TraceWrite::new($msg_type, module_path!(), line!(), thread_name);
            tw.server_name = Some(&server_name);
            tw.member_id = Some(member_id);
            tw.to_member_id = Some($to_member_id);
            tw.listening = Some(&listening);
            tw.to_addr = Some(&to_addr);
            tw.swim = None;
            tw.rumor = None;
            trace.write(tw);
        }
    }};
    (SWIM: $server:expr, $msg_type:expr, $to_member_id:expr, $to_addr:expr, $payload:expr) => {{
        let trace_on = $server.trace.read().expect("Trace lock is poisoned").on();
        if trace_on {
            use crate::trace::TraceWrite;
            let mut trace = $server.trace.write().expect("Trace lock is poisoned");
            trace.init($server);
            let thread = thread::current();
            let thread_name = thread.name().unwrap_or("undefined");
            let listening = format!("{}", $server.swim_addr());
            let to_addr = format!("{}", $to_addr);
            let member_id = $server.member_id();
            let server_name = $server.name();
            let mut swim_str = String::new();
            for m_string in $payload
                .membership
                .iter()
                .map(|m| format!("{}-{}-{:?} ", m.member.id, m.member.incarnation, m.health))
            {
                swim_str.push_str(&format!("{} ", &m_string)[..]);
            }
            let mut tw = TraceWrite::new($msg_type, module_path!(), line!(), thread_name);
            tw.server_name = Some(&server_name);
            tw.member_id = Some(member_id);
            tw.to_member_id = Some($to_member_id);
            tw.listening = Some(&listening);
            tw.to_addr = Some(&to_addr);
            tw.swim = Some(&swim_str);
            tw.rumor = None;
            trace.write(tw);
        }
    }};
    (GOSSIP: $server:expr, $msg_type:expr, $to_member_id:expr, $payload:expr) => {{
        let trace_on = $server.trace.read().expect("Trace lock is poisoned").on();
        if trace_on {
            let mut trace = $server.trace.write().expect("Trace lock is poisoned");
            use crate::{rumor,
                        trace::TraceWrite};
            trace.init($server);
            let thread = thread::current();
            let thread_name = thread.name().unwrap_or("undefined");
            let listening = format!("{}", $server.gossip_addr());
            let member_id = $server.member_id();
            let server_name = $server.name();
            let rp = match $payload.kind {
                rumor::RumorKind::Membership(ref membership) => format!(
                    "{}-{}-{:?}",
                    membership.member.id, membership.member.incarnation, membership.health
                ),
                rumor::RumorKind::Service(ref service) => format!(
                    "{}-{}-{}",
                    service.member_id, service.service_group, service.incarnation
                ),
                rumor::RumorKind::ServiceConfig(ref service_config) => format!(
                    "{}-{}-{}",
                    service_config.service_group,
                    service_config.incarnation,
                    service_config.encrypted
                ),
                rumor::RumorKind::ServiceFile(ref service_file) => format!(
                    "{}-{}-{}-{}",
                    service_file.service_group,
                    service_file.incarnation,
                    service_file.encrypted,
                    service_file.filename
                ),
                rumor::RumorKind::Election(ref election) => format!(
                    "{}-{}-{}-{}-{:?}-{:?}",
                    election.member_id,
                    election.service_group,
                    election.term,
                    election.suitability,
                    election.status,
                    election.votes
                ),
                rumor::RumorKind::ElectionUpdate(ref election) => format!(
                    "{}-{}-{}-{}-{:?}-{:?}",
                    election.member_id,
                    election.service_group,
                    election.term,
                    election.suitability,
                    election.status,
                    election.votes
                ),
                rumor::RumorKind::Departure(ref departure) => format!("{}", departure.member_id),
            };

            let mut tw = TraceWrite::new($msg_type, module_path!(), line!(), thread_name);
            tw.server_name = Some(&server_name);
            tw.member_id = Some(member_id);
            tw.to_member_id = Some($to_member_id);
            tw.listening = Some(&listening);
            tw.swim = None;
            tw.rumor = Some(&rp);
            trace.write(tw);
        }
    }};
}

#[cfg(test)]
mod tests {
    mod trace {
        use crate::trace::Trace;
        use std::path::Path;

        #[test]
        fn default() {
            let trace = Trace::default();
            assert_eq!(trace.directory, Path::new("/tmp/habitat-swim-trace"));
        }
    }
}
