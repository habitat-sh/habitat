// Copyright (c) 2019 Chef Software Inc. and/or applicable contributors
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

//! Main interface for a stream of events the Supervisor can send out
//! in the course of its operations.
//!
//! Currently, the Supervisor is able to send events to a [NATS
//! Streaming][1] server. The `init_stream` function must be called
//! before sending events to initialize the publishing thread in the
//! background. Thereafter, you can pass "event" structs to the
//! `event` function, which will publish the event to the stream.
//!
//! All events are published under the "habitat" subject.
//!
//! [1]:https://github.com/nats-io/nats-streaming-server

mod nats;
mod types;

use futures::sync::mpsc::UnboundedSender;
pub use nats::EventConnectionInfo;
use state::Container;
use std::{fmt::Debug,
          sync::Once};
pub use types::*;

static INIT: Once = Once::new();
lazy_static! {
    // TODO (CM): When const fn support lands in stable, we can ditch
    // this lazy_static call.

    /// Reference to the event stream.
    static ref EVENT_STREAM: Container = Container::new();
    /// Core information that is shared between all events.
    static ref EVENT_CORE: Container = Container::new();
}

/// Starts a new thread for sending events to a NATS Streaming
/// server. Stashes the handle to the stream, as well as the core
/// event information that will be a part of all events, in a global
/// static reference for access later.
pub fn init_stream(conn_info: EventConnectionInfo, event_core: EventCore) {
    INIT.call_once(|| {
            let event_stream = nats::init_stream(conn_info).expect("Could not start NATS thread");
            EVENT_STREAM.set(event_stream);
            EVENT_CORE.set(event_core);
        });
}

/// Publish an event. This is the main interface that client code will
/// use.
///
/// If `init_stream` has not been called already, this function will
/// be a no-op.
// NOTE: we can take advantage of this to "disable" the event
// subsystem if users don't wish to send events out; just don't call
// `init_stream` if they don't want it.
pub fn event<E>(event: E)
where
    E: Event,
{
    // TODO: incorporate the current timestamp into the rendered event
    // (which will require tweaks to the rendering logic, but we know
    // that'll need to be updated anyway).

    // We render the event to bytes here, rather than over in the
    // publication thread, because it allows our Event types to deal
    // with references, which means less allocations and unnecessary
    // copying (otherwise, we'd have to have copies of everything to
    // send it over to the other thread, or much more complicated
    // thread-safe types!). It also makes the publication thread logic
    // a bit simpler; it just takes bytes and sends them out.
    if let Some(e) = EVENT_STREAM.try_get::<EventStream>() {
        e.send(event.render(EVENT_CORE.get::<EventCore>()));
    }
}

/// A collection of data that will be present in all events. Rather
/// than baking this into the structure of each event, we represent it
/// once and merge the information into the final rendered form of the
/// event.
///
/// This prevents us from having to thread information throughout the
/// system, just to get it to the places where the events are
/// generated (e.g., not all code has direct access to the
/// Supervisor's ID).
#[derive(Clone, Debug)]
pub struct EventCore {
    /// The unique identifier of the Supervisor sending the event.
    pub supervisor_id: String,
}

/// A lightweight handle for the event stream. All events get to the
/// event stream through this.
pub(self) struct EventStream(UnboundedSender<Vec<u8>>);

impl EventStream {
    /// Queues an event to be sent out.
    fn send(&self, event: Vec<u8>) {
        trace!("About to queue an event: {:?}", event);
        if let Err(e) = self.0.unbounded_send(event) {
            error!("Failed to queue event: {:?}", e);
        }
    }
}

/// Defines the logic for transforming concrete event into a
/// byte representation to publish to the event stream.
// TODO: The ultimate format we need is not well defined yet, so we're
// using the absolute simplest thing that could possibly "work". Maybe
// it'll be JSON, maybe it'll be protobuf, maybe it'll be something
// else. Whatever it ultimately becomes, this is where the
// transformation logic will live.
pub trait Event: Debug {
    fn render(&self, core: &EventCore) -> Vec<u8> {
        format!("{:?} - {:?}", core, self).into_bytes()
    }
}
