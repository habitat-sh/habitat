use std::any::Any;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::result;
use std::sync::mpsc;
use std::thread;

use time::{Duration, SteadyTime};

pub type InitResult<E> = result::Result<Option<u64>, E>;
pub type ActorResult<T> = result::Result<T, ActorError>;
pub type StartResult<T, E> = result::Result<Actor<T>, E>;
pub type ActorSender<T> = mpsc::Sender<Message<T>>;
pub type Receiver<T> = mpsc::Receiver<Message<T>>;

pub struct Actor<T> where T: Any + Send {
    pub sender: ActorSender<T>,
    pub receiver: Receiver<T>,
    pub handle: thread::JoinHandle<ActorResult<()>>,
    pub name: Option<String>,
}

impl<T> Actor<T> where T: Any + Send {
    /// Create a new actor handler struct.
    pub fn new(sender: ActorSender<T>, receiver: Receiver<T>,
        handle: thread::JoinHandle<ActorResult<()>>, name: Option<String>) -> Self {
        Actor {
            sender: sender,
            receiver: receiver,
            handle: handle,
            name: name,
        }
    }

    pub fn cast(&self, message: T) -> ActorResult<()> {
        self::cast(&self.sender, message)
    }

    pub fn call(&self, message: T) -> ActorResult<T> {
        self::call(&self.sender, &self.receiver, message)
    }
}

pub fn cast<T: Any + Send>(tx: &ActorSender<T>, message: T) -> ActorResult<()> {
    match tx.send(Message::Cast(message)) {
        Ok(()) => Ok(()),
        Err(err) => Err(ActorError::from(err)),
    }
}

pub fn call<T: Any + Send>(tx: &ActorSender<T>, rx: &Receiver<T>, message: T) -> ActorResult<T> {
    match tx.send(Message::Call(message)) {
        Ok(()) => {
            match rx.recv() {
                Ok(Message::Reply(msg)) => Ok(msg),
                Ok(_) => panic!("must reply from a call!"),
                Err(err) => Err(ActorError::from(err)),
            }
        },
        Err(err) => Err(ActorError::from(err)),
    }
}

pub struct Builder<T: GenServer> {
    name: Option<String>,
    spec: T,
}

impl<A: GenServer> Builder<A> {
    pub fn new(spec: A) -> Self {
        Builder {
            name: None,
            spec: spec,
        }
    }

    pub fn name(mut self, name: String) -> Builder<A> {
        self.name = Some(name);
        self
    }

    /// Start an actor on a new thread and return an Actor.
    pub fn start(self, mut state: A::S) -> StartResult<A::T, A::E> {
        let (otx, orx) = mpsc::channel::<Message<A::T>>();
        let (itx, irx) = mpsc::channel::<Message<A::T>>();
        let initial_wait_ms = match self.spec.init(&itx, &mut state) {
            Ok(result) => result,
            Err(err) => return Err(err),
        };
        let itx2 = itx.clone(); // clone inner receive loop's sender for actor struct
        let name = self.name.clone();
        let thread_name = name.clone().unwrap_or("GenServer".to_string());
        let handle = thread::Builder::new().name(thread_name.clone()).spawn(move || {
            let mut timeout: Option<SteadyTime> = None;
            if let Some(ms) = initial_wait_ms {
                set_timeout(ms, &mut timeout);
            }
            loop {
                if let Some(go_time) = timeout {
                    if go_time >= SteadyTime::now() {
                        match self.spec.handle_timeout(&otx, &itx, &mut state) {
                            HandleResult::Stop(reason, None) => return shutdown(reason, None, &otx),
                            HandleResult::NoReply(Some(0)) => {
                                set_timeout(0, &mut timeout);
                                continue;
                            },
                            HandleResult::NoReply(new_timeout) => {
                                if let Some(ms) = new_timeout {
                                    set_timeout(ms, &mut timeout);
                                }
                            },
                            hr => panic!("unexpected `HandleResult` returned from handle_timeout: {:?}", hr),
                        }
                    }
                }
                match irx.try_recv() {
                    Ok(Message::Call(msg)) => {
                        match self.spec.handle_call(msg, &otx, &itx, &mut state) {
                            HandleResult::Reply(msg, new_timeout) => {
                                try!(otx.send(Message::Reply(msg)));
                                if let Some(ms) = new_timeout {
                                    set_timeout(ms, &mut timeout);
                                }
                            },
                            HandleResult::NoReply(new_timeout) => {
                                if let Some(ms) = new_timeout {
                                    set_timeout(ms, &mut timeout);
                                }
                            },
                            HandleResult::Stop(reason, reply) => return shutdown(reason, reply, &otx),
                        }
                    },
                    Ok(Message::Cast(msg)) => {
                        match self.spec.handle_cast(msg, &otx, &itx, &mut state) {
                            HandleResult::Stop(reason, reply) => return shutdown(reason, reply, &otx),
                            HandleResult::NoReply(new_timeout) => {
                                if let Some(ms) = new_timeout {
                                    set_timeout(ms, &mut timeout);
                                }
                            },
                            hr => panic!("unexpected `HandleResult` returned from handle_cast: {:?}", hr),
                        }
                    },
                    Ok(hr) => panic!("received unexpected message type: {:?}", hr),
                    Err(mpsc::TryRecvError::Disconnected) => { break; },
                    Err(mpsc::TryRecvError::Empty) => { },
                }
                // This is absolutely the wrong solution. I need to park the thread or call
                // recv instead of try_recv and schedule the timeout mechanism another way.
                // This is a quick and dirty workaround that should be short lived while the API
                // stabilizes and is leveraged in our other applications.
                //
                // I'm so sorry for doing this.
                //      - Jamie
                thread::sleep_ms(30)
            }
            Ok(())
        }).unwrap();
        Ok(Actor::new(itx2, orx, handle, name))
    }
}

#[derive(Debug)]
pub enum ActorError {
    InitFailure(String),
    AbnormalShutdown(String),
    SendError,
    RecvError,
}

impl<T: Any + Send> From<mpsc::SendError<Message<T>>> for ActorError {
    fn from(_err: mpsc::SendError<Message<T>>) -> Self {
        ActorError::SendError
    }
}

impl From<mpsc::RecvError> for ActorError {
    fn from(_err: mpsc::RecvError) -> Self {
        ActorError::RecvError
    }
}

#[derive(Debug)]
pub enum StopReason {
    Normal,
    Fatal(String),
}

#[derive(Debug)]
pub enum HandleResult<T> where T: Any + Send {
    Reply(T, Option<u64>),
    NoReply(Option<u64>),
    Stop(StopReason, Option<T>),
}

pub enum Message<T> where T: Any + Send {
    Call(T),
    Cast(T),
    Reply(T),
}

impl<T> Debug for Message<T> where T: Any + Send + Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Message::Call(ref msg) => write!(f, "CALL: {:?}", msg),
            &Message::Cast(ref msg) => write!(f, "CAST: {:?}", msg),
            &Message::Reply(ref msg) => write!(f, "REPLY: {:?}", msg)
        }
    }
}

pub trait GenServer : Send + 'static {
    type T: Send + Any + Debug;
    type S: Send + Any;
    type E: Error + 'static;

    fn init(&self, _tx: &ActorSender<Self::T>, state: &mut Self::S) -> InitResult<Self::E>;

    fn handle_call(&self, message: Self::T, _tx: &ActorSender<Self::T>, _me: &ActorSender<Self::T>, _state: &mut Self::S) -> HandleResult<Self::T> {
        panic!("handle_call callback not implemented; received: {:?}", message);
    }

    fn handle_cast(&self, message: Self::T, _tx: &ActorSender<Self::T>, _me: &ActorSender<Self::T>, _state: &mut Self::S) -> HandleResult<Self::T> {
        panic!("handle_cast callback not implemented; received: {:?}", message);
    }

    fn handle_timeout(&self, _tx: &ActorSender<Self::T>, _me: &ActorSender<Self::T>, _state: &mut Self::S) -> HandleResult<Self::T> {
        HandleResult::NoReply(None)
    }
}

fn set_timeout(wait_ms: u64, current_timeout: &mut Option<SteadyTime>) {
    *current_timeout = Some(SteadyTime::now() + Duration::milliseconds(wait_ms as i64));
}

fn shutdown<T: Any + Send>(reason: StopReason, reply: Option<T>, sender: &ActorSender<T>) -> Result<(), ActorError> {
    if let Some(msg) = reply {
        let _result = sender.send(Message::Reply(msg));
    }
    match reason {
        StopReason::Normal => Ok(()),
        StopReason::Fatal(e) => Err(ActorError::AbnormalShutdown(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;
    use std::error::Error;

    struct Worker;

    struct MyState {
        pub initialized: bool,
    }

    impl MyState {
        pub fn new() -> Self {
            MyState {
                initialized: false,
            }
        }
    }

    #[derive(Debug)]
    enum MyError {
        DirtyState,
    }

    #[derive(Debug)]
    enum MyMessage {
        Ok,
        Stop,
        State(bool),
        GetState,
        SetState(bool),
    }

    impl GenServer for Worker {
        type T = MyMessage;
        type S = MyState;
        type E = MyError;

        fn init(&self, _tx: &ActorSender<Self::T>, state: &mut Self::S) -> InitResult<Self::E> {
            if state.initialized {
                Err(MyError::DirtyState)
            } else {
                state.initialized = true;
                Ok(None)
            }
        }

        fn handle_call(&self, msg: Self::T, _: &ActorSender<Self::T>, _: &ActorSender<Self::T>, state: &mut Self::S) -> HandleResult<Self::T> {
            match msg {
                MyMessage::Stop => HandleResult::Stop(StopReason::Normal, Some(MyMessage::Ok)),
                MyMessage::GetState => HandleResult::Reply(MyMessage::State(state.initialized), None),
                MyMessage::SetState(value) => {
                    state.initialized = value;
                    HandleResult::Reply(MyMessage::Ok, None)
                }
                _ => HandleResult::Stop(StopReason::Fatal(String::from("Nope")), None),
            }
        }

        fn handle_cast(&self, msg: Self::T, _: &ActorSender<Self::T>, _: &ActorSender<Self::T>, state: &mut Self::S) -> HandleResult<Self::T> {
            match msg {
                MyMessage::SetState(value) => {
                    state.initialized = value;
                    HandleResult::NoReply(None)
                },
                _ => HandleResult::NoReply(None)
            }
        }
    }

    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                MyError::DirtyState => write!(f, "state already initialized"),
            }
        }
    }

    impl Error for MyError {
        fn description(&self) -> &str {
            match *self {
                MyError::DirtyState => "state already initialized"
            }
        }
    }

    #[test]
    fn error_on_init() {
        let mut state = MyState::new();
        state.initialized = true;
        match Builder::new(Worker).start(state) {
            Err(MyError::DirtyState) => assert!(true),
            Ok(_) => assert!(false),
        }
    }

    #[test]
    fn call_set_get_state() {
        let state = MyState::new();
        let actor = Builder::new(Worker).start(state).unwrap();
        match actor.call(MyMessage::GetState) {
            Ok(MyMessage::State(true)) => assert!(true),
            _ => assert!(false),
        }
        assert!(actor.call(MyMessage::SetState(false)).is_ok());
        match actor.call(MyMessage::GetState) {
            Ok(MyMessage::State(false)) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn multiple_cast_and_call_ordered() {
        let state = MyState::new();
        let actor = Builder::new(Worker).start(state).unwrap();
        assert!(actor.cast(MyMessage::SetState(false)).is_ok());
        assert!(actor.cast(MyMessage::SetState(true)).is_ok());
        assert!(actor.cast(MyMessage::SetState(false)).is_ok());
        match actor.call(MyMessage::GetState) {
            Ok(MyMessage::State(result)) => assert_eq!(result, false),
            _ => assert!(false),
        }
    }

    #[test]
    fn stopping_an_actor() {
        let state = MyState::new();
        let actor = Builder::new(Worker).start(state).unwrap();
        match actor.call(MyMessage::Stop) {
            Ok(MyMessage::Ok) => assert!(true),
            _ => assert!(false),
        }
        match actor.handle.join() {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn explicitly_naming_actor() {
        let state = MyState::new();
        let actor = Builder::new(Worker).name("batman".to_string()).start(state).unwrap();
        assert!(actor.name.is_some());
        assert_eq!(actor.name.unwrap(), "batman".to_string());
    }

    #[test]
    fn default_named_actor() {
        let state = MyState::new();
        let actor = Builder::new(Worker).start(state).unwrap();
        assert!(actor.name.is_none());
    }
}
