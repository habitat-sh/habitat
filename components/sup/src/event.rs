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

mod types;

use crate::error::Result;
use futures::{sync::{mpsc as futures_mpsc,
                     mpsc::UnboundedSender},
              Future,
              Stream};
use nitox::{commands::ConnectCommand,
            streaming::{client::NatsStreamingClient,
                        error::NatsStreamingError},
            NatsClient,
            NatsClientOptions};
use state::Container;
use std::{fmt::Debug,
          sync::{mpsc as std_mpsc,
                 Once},
          thread};
use tokio::{executor,
            runtime::current_thread::Runtime as ThreadRuntime};
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
            let event_stream = init_nats_stream(conn_info).expect("Could not start NATS thread");
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
pub fn publish(event: &impl Event) {
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

////////////////////////////////////////////////////////////////////////

/// All messages are published under this subject.
const HABITAT_SUBJECT: &str = "habitat";

/// All the information needed to establish a connection to a NATS
/// Streaming server.
// TODO: This will change as we firm up what the interaction between
// Habitat and A2 looks like.
pub struct EventConnectionInfo {
    pub name:        String,
    pub verbose:     bool,
    pub cluster_uri: String,
    pub cluster_id:  String,
}

/// Defines default connection information for a NATS Streaming server
/// running on localhost.
// TODO: As we become clear on the interaction between Habitat and A2,
// this implementation *may* disappear. It's useful for testing and
// prototyping, though.
impl Default for EventConnectionInfo {
    fn default() -> Self {
        EventConnectionInfo { name:        String::from("habitat"),
                              verbose:     true,
                              cluster_uri: String::from("127.0.0.1:4223"),
                              cluster_id:  String::from("test-cluster"), }
    }
}

fn init_nats_stream(conn_info: EventConnectionInfo) -> Result<EventStream> {
    // TODO (CM): Investigate back-pressure scenarios
    let (event_tx, event_rx) = futures_mpsc::unbounded();
    let (sync_tx, sync_rx) = std_mpsc::sync_channel(0); // rendezvous channel

    // TODO (CM): We could theoretically create this future and spawn
    // it in the Supervisor's Tokio runtime, but there's currently a
    // bug: https://github.com/YellowInnovation/nitox/issues/24

    thread::Builder::new().name("events".to_string())
                          .spawn(move || {
                              let EventConnectionInfo { name,
                                                        verbose,
                                                        cluster_uri,
                                                        cluster_id, } = conn_info;

                              let cc = ConnectCommand::builder()
                // .user(Some("nats".to_string()))
                // .pass(Some("S3Cr3TP@5w0rD".to_string()))
                .name(Some(name))
                .verbose(verbose)
                .build()
                .unwrap();
                              let opts =
                                  NatsClientOptions::builder().connect_command(cc)
                                                              .cluster_uri(cluster_uri.as_str())
                                                              .build()
                                                              .unwrap();

                              let publisher = NatsClient::from_options(opts)
                .map_err(Into::<NatsStreamingError>::into)
                .and_then(|client| {
                    NatsStreamingClient::from(client)
                        .cluster_id(cluster_id)
                        .connect()
                })
                .map_err(|streaming_error| error!("{}", streaming_error))
                .and_then(move |client| {
                    sync_tx.send(()).expect("Couldn't synchronize!");
                    event_rx.for_each(move |event: Vec<u8>| {
                        let publish_event = client
                            .publish(HABITAT_SUBJECT.into(), event.into())
                            .map_err(|e| {
                                error!("Error publishing event: {:?}", e);
                            });
                        executor::spawn(publish_event);
                        Ok(())
                    })
                });

                              ThreadRuntime::new().expect("Couldn't create event stream runtime!")
                                                  .spawn(publisher)
                                                  .run()
                                                  .expect("something seriously wrong has occurred");
                          })
                          .expect("Couldn't start events thread!");

    sync_rx.recv()?; // TODO (CM): nicer error message
    Ok(EventStream(event_tx))
}
