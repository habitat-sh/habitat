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

use std::fmt;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
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

// Supported operations
#[derive(Debug, Clone)]
enum Operation {
    Increment,
}

// Helper type
type CounterOp = (String, Operation);

use std::sync::{Once, ONCE_INIT};

static mut SENDER: *const Sender<CounterOp> = 0 as *const Sender<CounterOp>;

static INIT: Once = ONCE_INIT;

fn get_sender() -> Sender<CounterOp> {
    unsafe {
        INIT.call_once(|| {
            SENDER = Box::into_raw(Box::new(do_init()));
        });
        (*SENDER).clone()
    }
}

fn do_init() -> Sender<CounterOp> {
    let (tx, rx): (Sender<CounterOp>, Receiver<CounterOp>) = channel();
    let mut statsd_client = statsd_client();
    thread::spawn(move || process_receives(rx, &mut statsd_client));
    tx
}

fn process_receives(rx: Receiver<CounterOp>, statsd_client: &mut Option<Client>) {
    loop {
        let (counter, op): (String, Operation) = rx.recv().unwrap();
        match *statsd_client {
            Some(ref mut client) => {
                match op {
                    Operation::Increment => {
                        println!("******* INCREMENTING COUNTER");
                        client.incr(&counter)
                    }
                }
            }
            None => {
                println!("******* RECEIVED OP, NO STATSD CLIENT");
                ()
            }
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
            .send((self.to_string(), Operation::Increment))
            .unwrap();
    }
}

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Counter::SearchPackages => "search-packages",
        };

        write!(f, "{}", msg)
    }
}

#[cfg(test)]
mod test {
    use super::Counter;
    use std::time::Duration;
    use std::thread;

    #[test]
    fn display_counter() {
        let expected = r#"search-packages"#;
        let disp = format!("{}", Counter::SearchPackages);
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
