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

use std::sync::{Once, ONCE_INIT};
use std::sync::mpsc::{channel, sync_channel, Sender, Receiver, SyncSender};
use std::thread;
use statsd::Client;
use hab_core::env;

// Statsd Application name
pub const APP_NAME: &'static str = "bldr";

// Statsd Listener Address
pub const STATS_ENV: &'static str = "HAB_STATS_ADDR";

// Supported metrics
#[derive(Debug, Clone)]
pub enum Counter {
    SearchPackages,
}

// Supported metrics
#[derive(Debug, Clone)]
pub enum Gauge {
    PackageCount,
}

// Helper types
#[derive(Debug, Clone, Copy)]
enum MetricType {
    Counter,
    Gauge,
}

#[derive(Debug, Clone, Copy)]
enum MetricOperation {
    Increment,
    Decrement,
    SetValue,
}

type MetricId = &'static str;
type MetricValue = f64;
type MetricTuple = (MetricType, MetricOperation, MetricId, Option<MetricValue>);

trait Metric {
    fn id(&self) -> &'static str;
}

// One-time initialization
static mut SENDER: *const Sender<MetricTuple> = 0 as *const Sender<MetricTuple>;

static INIT: Once = ONCE_INIT;

fn sender() -> Sender<MetricTuple> {
    unsafe {
        INIT.call_once(|| { SENDER = Box::into_raw(Box::new(init())); });
        (*SENDER).clone()
    }
}

// init creates a worker thread ready to receive and process metric events,
// and returns a channel for use by metric senders
fn init() -> Sender<MetricTuple> {
    let (tx, rx) = channel::<MetricTuple>();
    let (rztx, rzrx) = sync_channel(0); // rendezvous channel

    thread::Builder::new()
        .name("metrics".to_string())
        .spawn(move || receive(rztx, rx))
        .expect("couldn't start metrics thread");

    match rzrx.recv() {
        Ok(()) => tx,
        Err(e) => panic!("metrics thread startup error, err={}", e),
    }
}

// receive runs in a separate thread and processes all metrics events
fn receive(rz: SyncSender<()>, rx: Receiver<MetricTuple>) {
    let mut client = statsd_client();
    rz.send(()).unwrap(); // Blocks until the matching receive is called

    loop {
        let (mtyp, mop, mid, mval): MetricTuple = rx.recv().unwrap();
        debug!("Received metrics tuple: {:?}", (mtyp, mop, mid, mval));

        match client {
            Some(ref mut cli) => {
                match mtyp {
                    MetricType::Counter => {
                        match mop {
                            MetricOperation::Increment => cli.incr(mid),
                            MetricOperation::Decrement => cli.decr(mid),
                            _ => error!("Unexpected metric operation: {:?}", mop),
                        }
                    }
                    MetricType::Gauge => {
                        match mop {
                            MetricOperation::SetValue => cli.gauge(mid, mval.unwrap()),
                            _ => error!("Unexpected metric operation: {:?}", mop),
                        }
                    }
                }
            }
            None => (),
        }
    }
}

fn statsd_client() -> Option<Client> {
    match env::var(STATS_ENV) {
        Ok(addr) => {
            match Client::new(&addr, APP_NAME) {
                Ok(c) => Some(c),
                Err(e) => {
                    debug!("Error creating statsd client: {:?}", e);
                    None
                }
            }
        }
        Err(_) => None,
    }
}

impl Counter {
    pub fn increment(&self) {
        match sender().send((
            MetricType::Counter,
            MetricOperation::Increment,
            &self.id(),
            None,
        )) {
            Ok(_) => (),
            Err(e) => error!("Failed to increment counter, error: {:?}", e),
        }
    }

    pub fn decrement(&self) {
        match sender().send((
            MetricType::Counter,
            MetricOperation::Decrement,
            &self.id(),
            None,
        )) {
            Ok(_) => (),
            Err(e) => error!("Failed to decrement counter, error: {:?}", e),
        }
    }
}

impl Gauge {
    pub fn set(&self, val: f64) {
        match sender().send((
            MetricType::Gauge,
            MetricOperation::SetValue,
            &self.id(),
            Some(val),
        )) {
            Ok(_) => (),
            Err(e) => error!("Failed to set gauge, error: {:?}", e),
        }
    }
}

impl Metric for Counter {
    fn id(&self) -> &'static str {
        match *self {
            Counter::SearchPackages => "search-packages",
        }
    }
}

impl Metric for Gauge {
    fn id(&self) -> &'static str {
        match *self {
            Gauge::PackageCount => "package-count",
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Counter, Gauge};
    use metrics::Metric;
    use std::time::Duration;
    use std::thread;

    #[test]
    fn counter_id() {
        let expected = r#"search-packages"#;
        let disp = Counter::SearchPackages.id();
        assert!(disp == expected);
    }

    #[test]
    fn guage_id() {
        let expected = r#"package-count"#;
        let disp = Gauge::PackageCount.id();
        assert!(disp == expected);
    }

    #[test]
    #[ignore]
    fn increment_counter() {
        Counter::SearchPackages.increment();
    }

    #[test]
    #[ignore]
    fn decrement_counter() {
        Counter::SearchPackages.decrement();
    }

    #[test]
    #[ignore]
    fn set_gauge() {
        Gauge::PackageCount.set(10.0);
    }

    #[test]
    #[ignore]
    fn calls_from_multiple_threads() {
        for n in 0..10 {
            thread::spawn(move || {
                Counter::SearchPackages.increment();
                Gauge::PackageCount.set(n as f64);
                Counter::SearchPackages.decrement();
            });
        }

        thread::sleep(Duration::from_millis(500))
    }
}
