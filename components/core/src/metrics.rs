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

use std::thread;
use std::sync::mpsc::{channel, sync_channel, Sender, Receiver, SyncSender};
use std::sync::{Once, ONCE_INIT};
use statsd::Client;
use env;

// Statsd Application name
pub const APP_NAME: &'static str = "bldr";

// Statsd Listener Address
pub const STATS_ENV: &'static str = "HAB_STATS_ADDR";

// Supported metrics
#[derive(Debug, Clone)]
pub enum Counter {
    SearchPackages,
}

// Helper types
#[derive(Debug, Clone, Copy)]
enum MetricType {
    Counter,
}

#[derive(Debug, Clone, Copy)]
enum MetricOperation {
    Increment,
}

type MetricId = &'static str;
type MetricValue = f32;
type MetricTuple = (MetricType, MetricOperation, MetricId, Option<MetricValue>);

trait Metric {
    fn id(&self) -> &'static str;
}

// One-time initialization
static mut SENDER: *const Sender<MetricTuple> = 0 as *const Sender<MetricTuple>;

static INIT: Once = ONCE_INIT;

fn get_sender() -> Sender<MetricTuple> {
    unsafe {
        INIT.call_once(|| {
            SENDER = Box::into_raw(Box::new(init()));
        });
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
        Ok(addr) => Some(Client::new(&addr, APP_NAME).unwrap()),
        Err(_) => None,
    }
}

impl Counter {
    pub fn increment(&self) {
        get_sender()
            .send((MetricType::Counter, MetricOperation::Increment, &self.id(), None))
            .unwrap();
    }
}

impl Metric for Counter {
    fn id(&self) -> &'static str {
        match *self {
            Counter::SearchPackages => "search-packages",
        }
    }
}

#[cfg(test)]
mod test {
    use super::Counter;
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
    fn increment_counter() {
        Counter::SearchPackages.increment();
    }

    #[test]
    fn increment_counter_multiple_threads() {
        for _ in 0..10 {
            thread::spawn(move || {
                Counter::SearchPackages.increment();
            });
        }

        thread::sleep(Duration::from_millis(50))
    }
}
