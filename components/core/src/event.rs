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

// Supported events
#[derive(Debug, Clone)]
pub enum Event<'a> {
    ProjectCreate { origin: &'a str, package: &'a str },
}

impl<'a> fmt::Display for Event<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Event::ProjectCreate { origin: _, package: _ } => "project-create",
        };
        write!(f, "{}", msg)
    }
}

impl<'a> ToJson for Event<'a> {
    fn to_json(&self) -> Json {
        let mut m = BTreeMap::new();
        m.insert("name".to_string(), self.to_string().to_json());

        match *self {
            Event::ProjectCreate { origin: ref o, package: ref p } => {
                m.insert("origin".to_string(), o.to_json());
                m.insert("package".to_string(), p.to_json());
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
}

impl EventLogger {
    pub fn new(service_name: &str) -> Self {
        EventLogger { log_dir: svc_var_path(&service_name) }
    }

    pub fn record_event(&self, event: Event) {
        let envelope = Envelope::new(&event);
        let file_path = self.log_dir.join(format!("event-{}.json", &envelope.timestamp));
        write_file(&self.log_dir, &file_path, &envelope.to_json().to_string());
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
        };

        let expected = r#"{"name":"project-create","origin":"myorigin","package":"mypackage"}"#;
        assert!(event.to_json().to_string() == expected.to_string());
    }

    #[test]
    fn envelope_to_json() {
        let event: Event = Event::ProjectCreate {
            origin: "myorigin",
            package: "mypackage",
        };

        let envelope = Envelope::new(&event);
        let expected =
            r#"{"event":{"name":"project-create","origin":"myorigin","package":"mypackage"}"#;
        assert!(envelope.to_json().to_string().starts_with(expected));
    }

    #[test]
    fn event_logger_path() {
        let event_logger: EventLogger = EventLogger::new("foo");
        let expected = r#"foo"#;
        match event_logger.log_dir.to_str() {
            Some(s) => assert!(s.contains(expected)),
            None => assert!(false),
        }
    }
}
