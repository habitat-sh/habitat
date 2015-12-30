// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! Manages runtime configuration provided by the users through a GenServer.
//!
//! The actor manages the watch on the config endpoint for the service and group.

use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};

use wonder::actor::{self, GenServer, HandleResult, InitResult, StopReason, ActorSender};
use discovery::etcd;

use error::{BldrError, BldrResult};

/// The timeout interval for the actor
const TIMEOUT_MS: u64 = 200;

/// The messages for our actor
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Message {
    /// The current config
    ConfigToml(Option<String>),
    /// Request the config
    Config,
    /// A generic Ok
    Ok,
    /// Stop the actor
    Stop,
}

/// The actor itself
#[derive(Debug)]
pub struct UserActor;

impl UserActor {
    /// Return the current user configuration.
    ///
    /// # Failures
    ///
    /// * If the actor call fails.
    pub fn config_string(actor: &actor::Actor<Message>) -> BldrResult<Option<String>> {
        match try!(actor.call(Message::Config)) {
            Message::ConfigToml(config_string) => Ok(config_string),
            _ => unreachable!(),
        }
    }
}

/// The state for our UserActor.
pub struct UserActorState {
    /// The etcd write channel
    ctx: Option<Sender<bool>>,
    /// The etcd read channel
    crx: Option<Receiver<Option<String>>>,
    /// The last configuration string
    config_string: Option<String>,
    /// The key we are watching in etcd
    watch_key: String,
}

impl UserActorState {
    /// Create a new UserActorState
    pub fn new(watch_key: String) -> UserActorState {
        UserActorState {
            ctx: None,
            crx: None,
            config_string: None,
            watch_key: watch_key,
        }
    }
}

impl GenServer for UserActor {
    type T = Message;
    type S = UserActorState;
    type E = BldrError;

    /// Set up the underlying etcd::watch, and store the channels in our state.
    fn init(&self, _tx: &ActorSender<Self::T>, state: &mut Self::S) -> InitResult<Self::E> {
        let (ctx, wrx) = channel();
        let (wtx, crx) = channel();
        etcd::watch(&state.watch_key, 1, true, true, wtx, wrx);
        state.ctx = Some(ctx);
        state.crx = Some(crx);
        Ok(Some(0))
    }

    /// Check the etcd::watch for updates. If we have data, update our states last known
    /// config_string.
    fn handle_timeout(&self,
                      _tx: &ActorSender<Self::T>,
                      _me: &ActorSender<Self::T>,
                      state: &mut Self::S)
                      -> HandleResult<Self::T> {
        if let Some(ref crx) = state.crx {
            match crx.try_recv() {
                Ok(Some(toml_string)) => {
                    state.config_string = Some(toml_string);
                }
                Ok(None) => {
                    state.config_string = None;
                }
                Err(TryRecvError::Empty) => {}
                Err(e) => {
                    return HandleResult::Stop(StopReason::Fatal(format!("User Actor caught \
                                                                         unexpected error: {:?}",
                                                                        e)),
                                              None);
                }
            }
        }
        HandleResult::NoReply(Some(TIMEOUT_MS))
    }

    /// Respond to messages, after checking for new data from etcd.
    fn handle_call(&self,
                   message: Self::T,
                   _caller: &ActorSender<Self::T>,
                   _me: &ActorSender<Self::T>,
                   state: &mut Self::S)
                   -> HandleResult<Self::T> {
        if let Some(ref crx) = state.crx {
            match crx.try_recv() {
                Ok(Some(toml_string)) => {
                    state.config_string = Some(toml_string);
                }
                Ok(None) => {
                    state.config_string = None;
                }
                Err(TryRecvError::Empty) => {}
                Err(e) => {
                    return HandleResult::Stop(StopReason::Fatal(format!("User Actor caught \
                                                                         unexpected error: {:?}",
                                                                        e)),
                                              Some(Message::Ok));
                }
            }
        }

        match message {
            Message::Stop => HandleResult::Stop(StopReason::Normal, Some(Message::Ok)),
            Message::Config => {
                match state.config_string {
                    Some(ref toml_string) => {
                        HandleResult::Reply(Message::ConfigToml(Some(toml_string.clone())),
                                            Some(TIMEOUT_MS))
                    }
                    None => HandleResult::Reply(Message::ConfigToml(None), Some(TIMEOUT_MS)),
                }
            }
            Message::Ok => {
                HandleResult::Stop(StopReason::Fatal(format!("You don't send me Ok! I send YOU \
                                                              Ok!")),
                                   Some(Message::Ok))
            }
            Message::ConfigToml(_) => {
                HandleResult::Stop(StopReason::Fatal(format!("You don't send me CensusToml(_)! \
                                                              I send YOU CensusToml(_)!")),
                                   Some(Message::Ok))
            }
        }
    }
}
