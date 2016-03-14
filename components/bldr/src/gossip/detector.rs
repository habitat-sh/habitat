// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! The failure detector.
//!
//! This module tracks the status of all our outbound connections, and takes care of marking them
//! as suspect or confirmed.

use std::collections::HashMap;

use rustc_serialize::{Encodable, Encoder};
use time::{SteadyTime, Duration};

use gossip::member::MemberId;

/// Failure time in milliseconds
pub static REQUEST_FAILURE_TIME: i64 = 5000;

/// How long before we declare this member all the way gone
pub static REQUEST_CONFIRM_TIME: i64 = 10000;

/// The status of a request
#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq, Eq)]
pub enum Status {
    Running,
    AwaitingAck,
    PingReq,
    Failed,
}

/// The state of a request; a status and a timeout.
#[derive(Debug)]
pub struct RequestState {
    pub status: Status,
    pub timeout: SteadyTime,
}

impl RequestState {
    /// Create a new RequestState. Starts out in 'Running'.
    pub fn new() -> RequestState {
        RequestState {
            status: Status::Running,
            timeout: SteadyTime::now() + Duration::milliseconds(REQUEST_FAILURE_TIME),
        }
    }
}

// We have to hand-write the serialization due to our use of SteadyTime.
impl Encodable for RequestState {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        try!(s.emit_struct("RequestState", 2, |s| {
            try!(s.emit_struct_field("status", 0, |s| self.status.encode(s)));
            try!(s.emit_struct_field("timeout", 1, |s| (format!("{}", self.timeout)).encode(s)));
            Ok(())
        }));
        Ok(())
    }
}

/// The failure detector tracks open pending requests.
#[derive(Debug, RustcEncodable)]
pub struct Detector {
    pub open_requests: HashMap<MemberId, RequestState>,
}

impl Detector {
    /// Create a new failure detector.
    pub fn new() -> Detector {
        Detector { open_requests: HashMap::new() }
    }

    /// Returns true if a member has an open request
    pub fn exists(&self, member_id: &MemberId) -> bool {
        self.open_requests.contains_key(member_id)
    }

    /// Start a new request
    pub fn start(&mut self, member_id: MemberId) {
        self.open_requests.insert(member_id, RequestState::new());
    }

    /// Set a requests state to PingReq
    pub fn pingreq(&mut self, member_id: &MemberId) {
        if let Some(rs) = self.open_requests.get_mut(member_id) {
            rs.status = Status::PingReq;
            rs.timeout = SteadyTime::now() + Duration::milliseconds(REQUEST_FAILURE_TIME);
        }
    }

    /// Sets a requests state to AwaitingAck
    pub fn awaiting_ack(&mut self, member_id: &MemberId) {
        if let Some(rs) = self.open_requests.get_mut(member_id) {
            rs.status = Status::AwaitingAck;
            rs.timeout = SteadyTime::now() + Duration::milliseconds(REQUEST_FAILURE_TIME);
        }
    }

    /// Sets a requests state to Failed
    pub fn failed(&mut self, member_id: &MemberId) {
        if let Some(rs) = self.open_requests.get_mut(member_id) {
            rs.status = Status::Failed;
            rs.timeout = SteadyTime::now() + Duration::milliseconds(REQUEST_CONFIRM_TIME);
        }
    }

    /// On success, we remove the request from the detector
    pub fn success(&mut self, member_id: &MemberId) {
        self.open_requests.remove(member_id);
    }

    /// Checks the timeout of connections. Returns a tuple of (suspect, confirmed) members.
    pub fn expire(&mut self) -> (Vec<MemberId>, Vec<MemberId>, Vec<MemberId>) {
        debug!("Detector State: {:#?}", self);
        let mut pingreq_members = Vec::new();
        let mut failed_members = Vec::new();
        let mut confirmed_members = Vec::new();
        for (member_id, request_state) in self.open_requests.iter() {
            if SteadyTime::now() > request_state.timeout {
                match request_state.status {
                    Status::AwaitingAck => pingreq_members.push(member_id.clone()),
                    Status::PingReq => failed_members.push(member_id.clone()),
                    Status::Failed => confirmed_members.push(member_id.clone()),
                    _ => {}
                }
            }
        }
        for member_id in failed_members.iter() {
            self.failed(member_id);
        }
        for member_id in confirmed_members.iter() {
            self.open_requests.remove(member_id);
        }
        (pingreq_members, failed_members, confirmed_members)
    }
}

#[cfg(test)]
mod test {
    mod request_state {
        use gossip::detector::{Status, RequestState};

        #[test]
        fn new() {
            let rs = RequestState::new();
            // RequestState starts in Running
            assert_eq!(rs.status, Status::Running);
        }

    }

    mod detector {
        use gossip::member::MemberId;
        use gossip::detector::{Detector, Status};

        #[test]
        fn start() {
            let mut d = Detector::new();
            let id = MemberId::new_v4();
            d.start(id);
            assert!(d.open_requests.contains_key(&id));
        }

        #[test]
        fn pingreq() {
            let mut d = Detector::new();
            let id = MemberId::new_v4();
            d.start(id);
            d.pingreq(&id);
            assert!(d.open_requests.contains_key(&id));
            assert_eq!(d.open_requests.get(&id).unwrap().status, Status::PingReq);
        }
    }
}
