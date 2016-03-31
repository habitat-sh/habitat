// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! A generic state machine.

use std::collections::HashMap;
use std::hash::Hash;
use std::fmt;
use std::time::Duration;
use std::thread;

/// The StateMachine struct.
///
/// * T: the states your machine can be in
/// * X: the worker we pass between states
/// * E: the Error type you want to return
pub struct StateMachine<T, X, E> {
    /// The states our machine supports
    pub state: T,
    /// How long to wait between state transitions
    pub delay: u64,
    /// The dispatch table of states to functions
    pub dispatch: HashMap<T, fn(&mut X) -> Result<(T, u64), E>>,
}

impl<T: Eq + Hash + fmt::Debug, X, E> StateMachine<T, X, E> {
    /// Create a new StateMachine
    pub fn new(state: T) -> StateMachine<T, X, E> {
        StateMachine {
            state: state,
            delay: 0,
            dispatch: HashMap::new(),
        }
    }

    /// Add a function to run for a given state.
    pub fn add_dispatch(&mut self, state: T, funk: fn(&mut X) -> Result<(T, u64), E>) {
        self.dispatch.insert(state, funk);
    }

    /// Set the next state, and a delay.
    fn set_state(&mut self, state: T, delay: u64) {
        self.state = state;
        self.delay = delay;
    }

    /// Call the current state function, then update the next state to its return value.
    pub fn next(&mut self, worker: &mut X) -> Result<(), E> {
        if self.dispatch.contains_key(&self.state) {
            if self.delay != 0 {
                thread::sleep(Duration::from_millis(self.delay));
            }
            let (next_state, delay) = try!(self.dispatch.get(&self.state).unwrap()(worker));
            self.set_state(next_state, delay);
        } else {
            panic!("Cannot dispatch to {:?} - perhaps you need to call add_dispatch?",
                   self.state);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::StateMachine;
    use std::error::Error;
    use std::fmt;

    #[derive(PartialEq, Eq, Debug, Hash)]
    enum State {
        Init,
        Done,
    }

    #[derive(PartialEq)]
    struct Worker {
        value: String,
    }

    #[derive(Debug)]
    pub enum WorkerError {
    }

    impl fmt::Display for WorkerError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "get the funk out")
        }
    }

    impl Error for WorkerError {
        fn description(&self) -> &str {
            "what the funk"
        }
    }

    impl Worker {
        fn state_init(&mut self) -> Result<(State, u64), WorkerError> {
            self.value = String::from("init");
            Ok((State::Done, 0))
        }

        fn state_done(&mut self) -> Result<(State, u64), WorkerError> {
            self.value = String::from("done");
            Ok((State::Init, 0))
        }
    }

    #[test]
    fn new_works() {
        let sm: StateMachine<State, Worker, WorkerError> = StateMachine::new(State::Init);
        assert_eq!(sm.state, State::Init);
        assert_eq!(sm.delay, 0);
    }

    #[test]
    fn add_dispatch() {
        let mut worker = Worker { value: String::from("nothing") };
        let mut sm: StateMachine<State, Worker, WorkerError> = StateMachine::new(State::Init);
        sm.add_dispatch(State::Init, Worker::state_init);
        assert!(sm.dispatch.contains_key(&State::Init));
        let _ = sm.dispatch.get(&State::Init).unwrap()(&mut worker);
        assert_eq!(worker.value, String::from("init"));
    }

    #[test]
    fn next_works() {
        let mut worker = Worker { value: String::from("nothing") };
        let mut sm: StateMachine<State, Worker, WorkerError> = StateMachine::new(State::Init);
        sm.add_dispatch(State::Init, Worker::state_init);
        sm.add_dispatch(State::Done, Worker::state_done);
        let _ = sm.next(&mut worker);
        assert_eq!(worker.value, String::from("init"));
        let _ = sm.next(&mut worker);
        assert_eq!(worker.value, String::from("done"));
    }
}
