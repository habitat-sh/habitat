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

use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::result;
use std::time::{UNIX_EPOCH, SystemTime};
use std::fmt;

use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use serde_json;

/// Sample envelope JSON payload
/// {
///   "timestamp": "1479330000.13442404",
///   "version": 1,
///   "event": {
///     "name": "project-create",
///     "origin" : "myorigin"
///     "package" : "mypackage"
///     "account" : "133508078967455744"
///   }
/// }
pub const SCHEMA_VERSION: u32 = 1;

// Macros to help hooking in the event logger into an Iron chain,
// and calling into the chained event logger.
#[macro_export]
macro_rules! define_event_log {
    () => {
        pub struct EventLog;
        impl typemap::Key for EventLog {
            type Value = EventLogger;
        }
    }
}

#[macro_export]
macro_rules! log_event {
    ($req:ident, $evt:expr) => {{
        let el = ($req).get::<persistent::Read<EventLog>>().unwrap();
        el.record_event($evt)
    }};
}

#[derive(Debug, Clone)]
pub enum Event {
    ProjectCreate {
        origin: String,
        package: String,
        account: String,
    },
    PackageUpload {
        origin: String,
        package: String,
        version: String,
        release: String,
        account: String,
    },
    OriginKeyUpload {
        origin: String,
        version: String,
        account: String,
    },
    OriginSecretKeyUpload {
        origin: String,
        version: String,
        account: String,
    },
    OriginInvitationSend {
        origin: String,
        user: String,
        id: String,
        account: String,
    },
    OriginInvitationAccept { id: String, account: String },
    OriginInvitationIgnore { id: String, account: String },
    JobCreate { package: String, account: String },
    GithubAuthenticate { user: String, account: String },
}

impl fmt::Display for Event {
    // TODO fn: As of rustfmt 0.7.1 the following match block is not well understood. The tool puts
    // all match arms on the same line which blows over the 100-column max which then fails the
    // tool with a `"line exceeded maximum length"` error. This ignore should be removed when we
    // upgrade rustfmt and retry.
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Event::ProjectCreate { origin: _, package: _, account: _ } => "project-create",
            Event::PackageUpload { origin: _, package: _, version: _, release: _, account: _ } => {
                "package-upload"
            }
            Event::OriginKeyUpload { origin: _, version: _, account: _ } => "origin-key-upload",
            Event::OriginSecretKeyUpload { origin: _, version: _, account: _ } => {
                "origin-secret-key-upload"
            }
            Event::OriginInvitationSend { origin: _, user: _, id: _, account: _ } => {
                "origin-invitation-send"
            }
            Event::OriginInvitationAccept { id: _, account: _ } => "origin-invitation-accept",
            Event::OriginInvitationIgnore { id: _, account: _ } => "origin-invitation-ignore",
            Event::JobCreate { package: _, account: _ } => "job-create",
            Event::GithubAuthenticate { user: _, account: _ } => "github-authenticate",
        };

        write!(f, "{}", msg)
    }
}

impl Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let strukt = match *self {
            Event::ProjectCreate { origin: ref o, package: ref p, account: ref a } => {
                let mut strukt = try!(serializer.serialize_struct("event", 4));
                try!(strukt.serialize_field("name", &self.to_string()));
                try!(strukt.serialize_field("origin", o));
                try!(strukt.serialize_field("package", p));
                try!(strukt.serialize_field("account", a));
                strukt
            }
            Event::PackageUpload { origin: ref o,
                                   package: ref p,
                                   version: ref v,
                                   release: ref r,
                                   account: ref a } => {
                let mut strukt = try!(serializer.serialize_struct("event", 6));
                try!(strukt.serialize_field("name", &self.to_string()));
                try!(strukt.serialize_field("origin", o));
                try!(strukt.serialize_field("package", p));
                try!(strukt.serialize_field("version", v));
                try!(strukt.serialize_field("release", r));
                try!(strukt.serialize_field("account", a));
                strukt
            }
            Event::OriginInvitationSend { origin: ref o,
                                          user: ref u,
                                          id: ref i,
                                          account: ref a } => {
                let mut strukt = try!(serializer.serialize_struct("event", 5));
                try!(strukt.serialize_field("name", &self.to_string()));
                try!(strukt.serialize_field("origin", o));
                try!(strukt.serialize_field("user", u));
                try!(strukt.serialize_field("id", i));
                try!(strukt.serialize_field("account", a));
                strukt
            }
            Event::OriginInvitationAccept { id: ref i, account: ref a } => {
                let mut strukt = try!(serializer.serialize_struct("event", 3));
                try!(strukt.serialize_field("name", &self.to_string()));
                try!(strukt.serialize_field("id", i));
                try!(strukt.serialize_field("account", a));
                strukt
            }
            Event::OriginInvitationIgnore { id: ref i, account: ref a } => {
                let mut strukt = try!(serializer.serialize_struct("event", 3));
                try!(strukt.serialize_field("name", &self.to_string()));
                try!(strukt.serialize_field("id", i));
                try!(strukt.serialize_field("account", a));
                strukt
            }
            Event::JobCreate { package: ref p, account: ref a } => {
                let mut strukt = try!(serializer.serialize_struct("event", 3));
                try!(strukt.serialize_field("name", &self.to_string()));
                try!(strukt.serialize_field("package", p));
                try!(strukt.serialize_field("account", a));
                strukt
            }
            Event::GithubAuthenticate { user: ref u, account: ref a } => {
                let mut strukt = try!(serializer.serialize_struct("event", 3));
                try!(strukt.serialize_field("name", &self.to_string()));
                try!(strukt.serialize_field("user", u));
                try!(strukt.serialize_field("account", a));
                strukt
            }
            Event::OriginKeyUpload { origin: ref o, version: ref v, account: ref a } => {
                let mut strukt = try!(serializer.serialize_struct("event", 4));
                try!(strukt.serialize_field("name", &self.to_string()));
                try!(strukt.serialize_field("origin", o));
                try!(strukt.serialize_field("version", v));
                try!(strukt.serialize_field("account", a));
                strukt
            }
            Event::OriginSecretKeyUpload { origin: ref o, version: ref v, account: ref a } => {
                let mut strukt = try!(serializer.serialize_struct("event", 4));
                try!(strukt.serialize_field("name", &self.to_string()));
                try!(strukt.serialize_field("origin", o));
                try!(strukt.serialize_field("version", v));
                try!(strukt.serialize_field("account", a));
                strukt
            }
        };
        strukt.end()
    }
}

#[derive(Debug, Clone)]
pub struct Envelope {
    pub version: u32,
    pub timestamp: String,
    pub event: Event,
}

impl Envelope {
    pub fn new(event: &Event) -> Self {
        Envelope {
            version: SCHEMA_VERSION,
            timestamp: timestamp(),
            event: event.clone(),
        }
    }
}

impl Serialize for Envelope {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("envelope", 3));
        try!(strukt.serialize_field("version", &self.version));
        try!(strukt.serialize_field("timestamp", &self.timestamp));
        try!(strukt.serialize_field("event", &self.event));
        strukt.end()
    }
}

fn write_file<T: ?Sized>(parent_dir: &Path, file_path: &Path, val: &T)
    where T: Serialize
{
    fs::create_dir_all(parent_dir).expect("Unable to create directory");
    let mut file = File::create(&file_path).expect("Unable to create file");
    serde_json::ser::to_writer(&mut file, val).expect("Unable to write file");
}

fn timestamp() -> String {
    let (secs, subsec_nanos) = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => (duration.as_secs(), duration.subsec_nanos()),
        Err(e) => {
            error!("Cannot generate system time: {}", e);
            return "0.0".to_string();
        }
    };
    format!("{}.{}", secs, subsec_nanos)
}

pub struct EventLogger {
    log_dir: PathBuf,
    enabled: bool,
}

impl EventLogger {
    pub fn new<T: Into<PathBuf>>(log_dir: T, enabled: bool) -> Self {
        EventLogger {
            log_dir: log_dir.into(),
            enabled: enabled,
        }
    }

    pub fn record_event(&self, event: Event) {
        if self.enabled {
            let envelope = Envelope::new(&event);
            let file_path = self.log_dir.join(format!("event-{}.json", &envelope.timestamp));
            write_file(&self.log_dir, &file_path, &envelope);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn event_logger_path() {
        let event_logger: EventLogger = EventLogger::new("/hab/svc/foo/var", true);
        let expected = r#"foo"#;
        match event_logger.log_dir.to_str() {
            Some(s) => assert!(s.contains(expected)),
            None => assert!(false),
        }
    }
}
