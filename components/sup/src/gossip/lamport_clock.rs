// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! This is an implementation of a [lamport
//! clock](https://en.wikipedia.org/wiki/Lamport_timestamps), which we use to track incarnations.
//!
//! LamportClocks deref to u64.

use std::cmp::Ordering;
use std::ops::Deref;

/// A struct representing a lamport clock; a simple unsigned 64 bit integer.
#[derive(Debug, RustcDecodable, RustcEncodable, Clone, Eq)]
pub struct LamportClock {
    pub counter: u64,
}

impl LamportClock {
    /// Create a new LamportClock; its counter is set to 0.
    pub fn new() -> LamportClock {
        LamportClock { counter: 0 }
    }

    /// Increment the clock.
    pub fn increment(&mut self) {
        self.counter += 1;
    }

    /// Set the clock based on a peer; if the peer is later than you, update the clock to refer to
    /// its time.
    pub fn set_by_peer_clock(&mut self, peer_clock: &LamportClock) {
        let peer_counter: u64 = peer_clock.counter + 1;
        if peer_counter > self.counter {
            self.counter = peer_counter;
        }
    }

    /// Return the current time.
    pub fn time(&self) -> &u64 {
        &self.counter
    }
}

impl PartialEq for LamportClock {
    fn eq(&self, other: &LamportClock) -> bool {
        if self.counter == other.counter {
            true
        } else {
            false
        }
    }
}

impl PartialOrd for LamportClock {
    fn partial_cmp(&self, other: &LamportClock) -> Option<Ordering> {
        let my_counter = self.counter;
        let their_counter = other.counter;
        if my_counter > their_counter {
            Some(Ordering::Greater)
        } else if my_counter < their_counter {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Deref for LamportClock {
    type Target = u64;

    fn deref(&self) -> &u64 {
        &self.counter
    }
}

#[cfg(test)]
mod test {
    use super::LamportClock;
    use std::thread::{self, JoinHandle};
    use std::sync::{Arc, RwLock};

    #[test]
    fn lamport_clock_increment() {
        let lc = Arc::new(RwLock::new(LamportClock::new()));

        assert_eq!(*lc.read().unwrap().time(), 0);

        fn spawn_threads(lc: &Arc<RwLock<LamportClock>>) -> Vec<JoinHandle<()>> {
            let mut children = Vec::new();
            for _ in 0..10 {
                let lc_clone = lc.clone();
                let child = thread::spawn(move || {
                    &lc_clone.write().unwrap().increment();
                });
                children.push(child);
            }
            children
        }

        let children = spawn_threads(&lc);

        for child in children {
            &child.join();
        }

        assert_eq!(*lc.read().unwrap().time(), 10);
    }

    #[test]
    fn lamport_clock_peer() {
        let lc = Arc::new(RwLock::new(LamportClock::new()));

        let peer = LamportClock { counter: 665 };

        {
            let mut lc_write = lc.write().unwrap();
            lc_write.set_by_peer_clock(&peer);
        }

        assert_eq!(*lc.read().unwrap().time(), 666);
    }

    #[test]
    fn lamport_clock_peer_younger() {
        let lc = Arc::new(RwLock::new(LamportClock { counter: 665 }));

        let peer = LamportClock { counter: 1 };

        {
            let mut lc_write = lc.write().unwrap();
            lc_write.set_by_peer_clock(&peer);
        }

        assert_eq!(*lc.read().unwrap().time(), 665);
    }

    #[test]
    fn lamport_clock_partialord() {
        let olc = LamportClock { counter: 666 };
        let ylc = LamportClock { counter: 555 };
        let elc = LamportClock { counter: 666 };

        assert!(olc > ylc);
        assert!(ylc < olc);
        assert!(olc == elc);
    }
}
