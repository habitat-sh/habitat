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

mod error;
// ratsio_stream is the default, but setting it as a default in Cargo.toml
// makes it trickier to use nitox instead.
#[cfg(feature = "nitox_stream")]
#[path = "event/nitox.rs"]
mod stream_impl;
#[cfg(any(feature = "ratsio_stream", not(feature = "nitox_stream")))]
#[path = "event/ratsio.rs"]
mod stream_impl;
mod types;

pub(crate) use self::types::ServiceMetadata;
use self::types::{EventMessage,
                  EventMetadata,
                  HealthCheckEvent,
                  ServiceStartedEvent,
                  ServiceStoppedEvent,
                  ServiceUpdateStartedEvent};
use crate::manager::{service::{HealthCheckResult,
                               Service},
                     sys::Sys};
use clap::ArgMatches;
pub use error::{Error,
                Result};
use futures::sync::mpsc::UnboundedSender;
use habitat_common::types::{AutomateAuthToken,
                            EventStreamConnectTimeout,
                            EventStreamMetadata};
use habitat_core::package::ident::PackageIdent;
use state::Container;
use std::{net::SocketAddr,
          sync::Once,
          time::Duration};

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
pub fn init_stream(config: EventStreamConfig, event_core: EventCore) -> Result<()> {
    // call_once can't return a Result (or anything), so we'll fake it
    // by hanging onto any error we might receive.
    let mut return_value: Result<()> = Ok(());

    INIT.call_once(|| {
            let conn_info = EventStreamConnectionInfo::new(&event_core.supervisor_id, config);
            match stream_impl::init_stream(conn_info) {
                Ok(event_stream) => {
                    EVENT_STREAM.set(event_stream);
                    EVENT_CORE.set(event_core);
                }
                Err(e) => return_value = Err(e),
            }
        });

    return_value
}

/// Captures all event stream-related configuration options that would
/// be passed in by a user
#[derive(Clone, Debug)]
pub struct EventStreamConfig {
    environment:     String,
    application:     String,
    site:            Option<String>,
    meta:            EventStreamMetadata,
    token:           AutomateAuthToken,
    url:             String,
    connect_timeout: EventStreamConnectTimeout,
}

impl<'a> From<&'a ArgMatches<'a>> for EventStreamConfig {
    fn from(m: &ArgMatches) -> Self {
        EventStreamConfig { environment:     m.value_of("EVENT_STREAM_ENVIRONMENT")
                                              .map(str::to_string)
                                              .expect("Required option for EventStream feature"),
                            application:     m.value_of("EVENT_STREAM_APPLICATION")
                                              .map(str::to_string)
                                              .expect("Required option for EventStream feature"),
                            site:            m.value_of("EVENT_STREAM_SITE").map(str::to_string),
                            meta:            EventStreamMetadata::from(m),
                            token:           AutomateAuthToken::from(m),
                            url:             m.value_of("EVENT_STREAM_URL")
                                              .map(str::to_string)
                                              .expect("Required option for EventStream feature"),
                            connect_timeout: EventStreamConnectTimeout::from(m), }
    }
}

/// All the information needed to establish a connection to a NATS
/// Streaming server.
pub struct EventStreamConnectionInfo {
    pub name:            String,
    pub verbose:         bool,
    pub cluster_uri:     String,
    pub cluster_id:      String,
    pub auth_token:      AutomateAuthToken,
    pub connect_timeout: EventStreamConnectTimeout,
}

impl EventStreamConnectionInfo {
    pub fn new(supervisor_id: &str, config: EventStreamConfig) -> Self {
        EventStreamConnectionInfo { name:            format!("hab_client_{}", supervisor_id),
                                    verbose:         true,
                                    cluster_uri:     config.url,
                                    cluster_id:      "event-service".to_string(),
                                    auth_token:      config.token,
                                    connect_timeout: config.connect_timeout, }
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
    supervisor_id: String,
    ip_address: SocketAddr,
    fqdn: String,
    application: String,
    environment: String,
    site: Option<String>,
    meta: EventStreamMetadata,
}

impl EventCore {
    pub fn new(config: &EventStreamConfig, sys: &Sys, fqdn: String) -> Self {
        EventCore { supervisor_id: sys.member_id.clone(),
                    ip_address: sys.gossip_listen(),
                    fqdn,
                    environment: config.environment.clone(),
                    application: config.application.clone(),
                    site: config.site.clone(),
                    meta: config.meta.clone() }
    }
}

/// Send an event for the start of a Service.
pub fn service_started(service: &Service) {
    if stream_initialized() {
        publish(ServiceStartedEvent { service_metadata: Some(service.to_service_metadata()),
                                      event_metadata:   None, });
    }
}

/// Send an event for the stop of a Service.
pub fn service_stopped(service: &Service) {
    if stream_initialized() {
        publish(ServiceStoppedEvent { service_metadata: Some(service.to_service_metadata()),
                                      event_metadata:   None, });
    }
}

/// Send an event at the start of a Service update.
pub fn service_update_started(service: &Service, update: &PackageIdent) {
    if stream_initialized() {
        publish(ServiceUpdateStartedEvent { event_metadata:       None,
                                            service_metadata:
                                                Some(service.to_service_metadata()),
                                            update_package_ident: update.clone().to_string(), });
    }
}

// Takes metadata directly, rather than a `&Service` like other event
// functions, because of how the asynchronous health checking
// currently works. Revisit when async/await + Pin is all stabilized.
/// `execution` will be `Some` if the service had a hook to run, and
/// records how long it took that hook to execute completely.
pub fn health_check(metadata: ServiceMetadata,
                    check_result: HealthCheckResult,
                    execution: Option<Duration>) {
    if stream_initialized() {
        let check_result: types::HealthCheckResult = check_result.into();
        publish(HealthCheckEvent { service_metadata: Some(metadata),
                                   event_metadata:   None,
                                   result:           i32::from(check_result),
                                   execution:        execution.map(Duration::into), });
    }
}

////////////////////////////////////////////////////////////////////////

/// Internal helper function to know whether or not to go to the trouble of
/// creating event structures. If the event stream hasn't been
/// initialized, then we shouldn't need to do anything.
fn stream_initialized() -> bool { EVENT_STREAM.try_get::<EventStream>().is_some() }

/// Publish an event. This is the main interface that client code will
/// use.
///
/// If `init_stream` has not been called already, this function will
/// be a no-op.
fn publish(mut event: impl EventMessage) {
    if let Some(e) = EVENT_STREAM.try_get::<EventStream>() {
        // TODO (CM): Yeah... this is looking pretty gross. The
        // intention is to be able to timestamp the events right as
        // they go out.
        //
        // We *could* set the time when we convert the EventCore to a
        // EventMetadata struct, but that seems odd.
        //
        // It probably means that this structure just isn't the right
        // one.
        //
        // The ugliness is at least contained, though.
        debug!("Publishing to event stream: event {:?} ", event);
        event.event_metadata(EventMetadata { occurred_at:
                                                 Some(std::time::SystemTime::now().into()),
                                             ..EVENT_CORE.get::<EventCore>().to_event_metadata() });

        e.send(event.to_bytes());
    }
}

/// A lightweight handle for the event stream. All events get to the
/// event stream through this.
struct EventStream(UnboundedSender<Vec<u8>>);

impl EventStream {
    /// Queues an event to be sent out.
    fn send(&self, event: Vec<u8>) {
        trace!("About to queue an event: {:?}", event);
        if let Err(e) = self.0.unbounded_send(event) {
            error!("Failed to queue event: {:?}", e);
        }
    }
}
