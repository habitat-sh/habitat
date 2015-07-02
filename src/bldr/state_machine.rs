//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::fmt;

pub struct StateMachine<T, X, E> {
    pub state: T,
    pub delay: u32,
    pub dispatch: HashMap<T, fn(&mut X)-> Result<(T, u32), E>>,
    pub run: Option<fn(&mut StateMachine<T, X, E>, &mut X) -> Result<(), E>>
}

impl<T: Eq + Hash + fmt::Debug, X, E> StateMachine<T, X, E> {
    pub fn new(state: T) -> StateMachine<T, X, E> {
        StateMachine{
            state: state,
            delay: 0,
            dispatch: HashMap::new(),
            run: None
        }
    }

    pub fn add_run(&mut self, funk: fn(&mut StateMachine<T, X, E>, &mut X) -> Result<(), E>) {
        self.run = Some(funk);
    }

    pub fn add_dispatch(&mut self, state: T, funk: fn(&mut X) -> Result<(T, u32), E>) {
        self.dispatch.insert(state, funk);
    }

    fn set_state(&mut self, state: T, delay: u32) {
        self.state = state;
        self.delay = delay;
    }

    pub fn next(&mut self, worker: &mut X) -> Result<(), E> {
        if self.dispatch.contains_key(&self.state) {
            let (next_state, delay) = try!(self.dispatch.get(&self.state).unwrap()(worker));
            self.set_state(next_state, delay);
        } else {
            panic!("Cannot dispatch to {:?} - perhaps you need to call add_dispatch?", self.state);
        }
        Ok(())
    }

    pub fn run(&mut self, worker: &mut X) -> Result<(), E> {
        if self.run.is_some() {
            try!(self.run.unwrap()(self, worker));
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
        Done
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
        fn state_init(&mut self) -> Result<(State, u32), WorkerError> {
            self.value = String::from("init");
            Ok((State::Done, 0))
        }

        fn state_done(&mut self) -> Result<(State, u32), WorkerError> {
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
        let mut worker = Worker{ value: String::from("nothing") };
        let mut sm: StateMachine<State, Worker, WorkerError> = StateMachine::new(State::Init);
        sm.add_dispatch(State::Init, Worker::state_init);
        assert!(sm.dispatch.contains_key(&State::Init));
        let _ = sm.dispatch.get(&State::Init).unwrap()(&mut worker);
        assert_eq!(worker.value, String::from("init"));
    }

    #[test]
    fn next_works() {
        let mut worker = Worker{ value: String::from("nothing") };
        let mut sm: StateMachine<State, Worker, WorkerError> = StateMachine::new(State::Init);
        sm.add_dispatch(State::Init, Worker::state_init);
        sm.add_dispatch(State::Done, Worker::state_done);
        let _ = sm.next(&mut worker);
        assert_eq!(worker.value, String::from("init"));
        let _ = sm.next(&mut worker);
        assert_eq!(worker.value, String::from("done"));
    }
}

