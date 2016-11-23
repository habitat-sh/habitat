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

use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{UNIX_EPOCH, SystemTime};
use std::fmt;
use rustc_serialize::json::{ToJson, Json};
use fs::svc_var_path;

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

// Supported events
#[derive(Debug, Clone)]
pub enum Event<'a> {
    ProjectCreate {
        origin: &'a str,
        package: &'a str,
        account: &'a str,
    },
    PackageUpload {
        origin: &'a str,
        package: &'a str,
        version: &'a str,
        release: &'a str,
        account: &'a str,
    },
    OriginKeyUpload {
        origin: &'a str,
        version: &'a str,
        account: &'a str,
    },
    OriginSecretKeyUpload {
        origin: &'a str,
        version: &'a str,
        account: &'a str,
    },
    OriginInvitationSend {
        origin: &'a str,
        user: &'a str,
        id: &'a str,
        account: &'a str,
    },
    OriginInvitationAccept { id: &'a str, account: &'a str },
    OriginInvitationIgnore { id: &'a str, account: &'a str },
    JobCreate { package: &'a str, account: &'a str },
    GithubAuthenticate { user: &'a str, account: &'a str },
}

impl<'a> fmt::Display for Event<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Event::ProjectCreate { origin: _, package: _, account: _ } => "project-create",
            Event::PackageUpload { origin: _, package: _, version: _, release: _, account: _ } => {
                "package-upload"
            }
            Event::OriginKeyUpload { origin: _, version: _, account: _ } => "origin-key-upload",
            Event::OriginSecretKeyUpload { origin: _, version: _, account: _ } => "origin-secret-key-upload",
            Event::OriginInvitationSend { origin: _, user: _, id: _, account: _ } => "origin-invitation-send",
            Event::OriginInvitationAccept { id: _, account: _ } => "origin-invitation-accept",
            Event::OriginInvitationIgnore { id: _, account: _ } => "origin-invitation-ignore",
            Event::JobCreate { package: _, account: _ } => "job-create",
            Event::GithubAuthenticate { user: _, account: _ } => "github-authenticate",
        };

        write!(f, "{}", msg)
    }
}

impl<'a> ToJson for Event<'a> {
    fn to_json(&self) -> Json {
        let mut m = BTreeMap::new();
        m.insert("name".to_string(), self.to_string().to_json());

        match *self {
            Event::ProjectCreate { origin: ref o, package: ref p, account: ref a } => {
                m.insert("origin".to_string(), o.to_json());
                m.insert("package".to_string(), p.to_json());
                m.insert("account".to_string(), a.to_json());
            }
            Event::PackageUpload { origin: ref o,
                                   package: ref p,
                                   version: ref v,
                                   release: ref r,
                                   account: ref a } => {
                m.insert("origin".to_string(), o.to_json());
                m.insert("package".to_string(), p.to_json());
                m.insert("version".to_string(), v.to_json());
                m.insert("release".to_string(), r.to_json());
                m.insert("account".to_string(), a.to_json());
            }
            Event::OriginInvitationSend { origin: ref o,
                                          user: ref u,
                                          id: ref i,
                                          account: ref a } => {
                m.insert("origin".to_string(), o.to_json());
                m.insert("user".to_string(), u.to_json());
                m.insert("id".to_string(), i.to_json());
                m.insert("account".to_string(), a.to_json());
            }
            Event::OriginInvitationAccept { id: ref i, account: ref a } => {
                m.insert("id".to_string(), i.to_json());
                m.insert("account".to_string(), a.to_json());
            }
            Event::OriginInvitationIgnore { id: ref i, account: ref a } => {
                m.insert("id".to_string(), i.to_json());
                m.insert("account".to_string(), a.to_json());
            }
            Event::JobCreate { package: ref p, account: ref a } => {
                m.insert("package".to_string(), p.to_json());
                m.insert("account".to_string(), a.to_json());
            }
            Event::GithubAuthenticate { user: ref u, account: ref a } => {
                m.insert("user".to_string(), u.to_json());
                m.insert("account".to_string(), a.to_json());
            }
            Event::OriginKeyUpload { origin: ref o, version: ref v, account: ref a } => {
                m.insert("origin".to_string(), o.to_json());
                m.insert("version".to_string(), v.to_json());
                m.insert("account".to_string(), a.to_json());
            }
            Event::OriginSecretKeyUpload { origin: ref o, version: ref v, account: ref a } => {
                m.insert("origin".to_string(), o.to_json());
                m.insert("version".to_string(), v.to_json());
                m.insert("account".to_string(), a.to_json());
            }
        };
        Json::Object(m)
    }
}

// Sample envelope JSON payload
// {
//   "timestamp": "1479330000.13442404",
//   "version": 1,
//   "event": {
//     "name": "project-create",
//     "origin" : "myorigin"
//     "package" : "mypackage"
//     "account" : "133508078967455744"
//   }
// }

pub const SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone)]
pub struct Envelope<'a> {
    version: u32,
    timestamp: String,
    event: Event<'a>,
}

impl<'a> Envelope<'a> {
    pub fn new(event: &Event<'a>) -> Self {
        Envelope {
            version: SCHEMA_VERSION,
            timestamp: timestamp(),
            event: event.clone(),
        }
    }
}

impl<'a> ToJson for Envelope<'a> {
    fn to_json(&self) -> Json {
        let mut m = BTreeMap::new();
        m.insert("version".to_string(), self.version.to_json());
        m.insert("timestamp".to_string(), self.timestamp.to_json());
        m.insert("event".to_string(), self.event.to_json());

        Json::Object(m)
    }
}

fn write_file(parent_dir: &Path, file_path: &Path, content: &str) {
    fs::create_dir_all(parent_dir).expect("Unable to create directory");
    let mut file = File::create(&file_path).expect("Unable to create file");
    file.write_all(content.as_bytes()).expect("Unable to write file");
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
    pub fn new(service_name: &str, enabled: bool) -> Self {
        EventLogger {
            log_dir: svc_var_path(&service_name),
            enabled: enabled,
        }
    }

    pub fn record_event(&self, event: Event) {
        if self.enabled {
            let envelope = Envelope::new(&event);
            let file_path = self.log_dir.join(format!("event-{}.json", &envelope.timestamp));
            write_file(&self.log_dir, &file_path, &envelope.to_json().to_string());
        }
    }
}

#[cfg(test)]
mod test {
    use super::{EventLogger, Envelope, Event};
    use rustc_serialize::json::ToJson;

    #[test]
    fn event_to_json() {
        let event: Event = Event::ProjectCreate {
            origin: "myorigin",
            package: "mypackage",
            account: "myaccount",
        };

        let expected = r#"{"account":"myaccount","name":"project-create","origin":"myorigin","package":"mypackage"}"#;
        assert!(event.to_json().to_string() == expected.to_string());
    }

    #[test]
    fn envelope_to_json() {
        let event: Event = Event::ProjectCreate {
            origin: "myorigin",
            package: "mypackage",
            account: "myaccount",
        };

        let envelope = Envelope::new(&event);
        let expected =
            r#"{"event":{"account":"myaccount","name":"project-create","origin":"myorigin","package":"mypackage"}"#;
        assert!(envelope.to_json().to_string().starts_with(expected));
    }

    #[test]
    fn event_logger_path() {
        let event_logger: EventLogger = EventLogger::new("foo", true);
        let expected = r#"foo"#;
        match event_logger.log_dir.to_str() {
            Some(s) => assert!(s.contains(expected)),
            None => assert!(false),
        }
    }
}
