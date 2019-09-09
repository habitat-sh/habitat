//! Main interface for a stream of events the Supervisor can send out
//! in the course of its operations.
//!
//! Currently, the Supervisor is able to send events to a [NATS][1]
//! server. The `init_stream` function must be called
//! before sending events to initialize the publishing thread in the
//! background. Thereafter, you can pass "event" structs to the
//! `event` function, which will publish the event to the stream.
//!
//! All events are published under the "habitat" subject.
//!
//! [1]:https://github.com/nats-io/nats-server

mod error;
mod stream;
mod types;

pub(crate) use self::types::ServiceMetadata;
use self::types::{EventMessage,
                  EventMetadata,
                  HealthCheckEvent,
                  ServiceStartedEvent,
                  ServiceStoppedEvent,
                  ServiceUpdateStartedEvent};
use crate::{manager::{service::{HealthCheckHookStatus,
                                HealthCheckResult,
                                ProcessOutput,
                                Service,
                                StandardStreams},
                      sys::Sys},
            sup_futures::FutureHandle};
use clap::ArgMatches;
pub use error::{Error,
                Result};
use futures::sync::mpsc::Sender;
use habitat_common::types::{AutomateAuthToken,
                            EventStreamConnectMethod,
                            EventStreamMetadata};
use habitat_core::package::ident::PackageIdent;
use parking_lot::Mutex;
use state::Container;
use std::{net::SocketAddr,
          sync::Once,
          time::Duration};
use tokio::runtime::Runtime;

const SERVICE_STARTED_SUBJECT: &str = "habitat.event.service_started";
const SERVICE_STOPPED_SUBJECT: &str = "habitat.event.service_stopped";
const SERVICE_UPDATE_STARTED_SUBJECT: &str = "habitat.event.service_update_started";
const HEALTHCHECK_SUBJECT: &str = "habitat.event.healthcheck";

static INIT: Once = Once::new();
lazy_static! {
    // TODO (CM): When const fn support lands in stable, we can ditch
    // this lazy_static call.

    /// Reference to the event stream.
    static ref EVENT_STREAM: Container = Container::new();
    /// Core information that is shared between all events.
    static ref EVENT_CORE: Container = Container::new();
}
type EventStreamContainer = Mutex<EventStream>;

/// Starts a new thread for sending events to a NATS Streaming
/// server. Stashes the handle to the stream, as well as the core
/// event information that will be a part of all events, in a global
/// static reference for access later.
pub fn init_stream(config: EventStreamConfig,
                   event_core: EventCore,
                   runtime: &mut Runtime)
                   -> Result<()> {
    // call_once can't return a Result (or anything), so we'll fake it
    // by hanging onto any error we might receive.
    let mut return_value: Result<()> = Ok(());

    INIT.call_once(|| {
            let conn_info = EventStreamConnectionInfo::new(&event_core.supervisor_id, config);
            match stream::init_stream(conn_info, runtime) {
                Ok(event_stream) => {
                    EVENT_STREAM.set(EventStreamContainer::new(event_stream));
                    EVENT_CORE.set(event_core);
                }
                Err(e) => return_value = Err(e),
            }
        });

    return_value
}

/// Stop the event stream future so we can gracefully shutdown the runtime.
///
/// `init_stream` and `stop_stream` cannot be called more than once. The singleton event stream can
/// only be set once. If `init_stream` is called after calling `stop_stream` no new event stream
/// will be started.
pub fn stop_stream() {
    if let Some(e) = EVENT_STREAM.try_get::<EventStreamContainer>() {
        if let Some(h) = e.lock().handle.take() {
            h.terminate();
        }
    }
}

/// Captures all event stream-related configuration options that would
/// be passed in by a user
#[derive(Clone, Debug)]
pub struct EventStreamConfig {
    environment:    String,
    application:    String,
    site:           Option<String>,
    meta:           EventStreamMetadata,
    token:          AutomateAuthToken,
    url:            String,
    connect_method: EventStreamConnectMethod,
}

impl<'a> From<&'a ArgMatches<'a>> for EventStreamConfig {
    fn from(m: &ArgMatches) -> Self {
        EventStreamConfig { environment:    m.value_of("EVENT_STREAM_ENVIRONMENT")
                                             .map(str::to_string)
                                             .expect("Required option for EventStream feature"),
                            application:    m.value_of("EVENT_STREAM_APPLICATION")
                                             .map(str::to_string)
                                             .expect("Required option for EventStream feature"),
                            site:           m.value_of("EVENT_STREAM_SITE").map(str::to_string),
                            meta:           EventStreamMetadata::from(m),
                            token:          AutomateAuthToken::from(m),
                            url:            m.value_of("EVENT_STREAM_URL")
                                             .map(str::to_string)
                                             .expect("Required option for EventStream feature"),
                            connect_method: EventStreamConnectMethod::from(m), }
    }
}

/// All the information needed to establish a connection to a NATS
/// Streaming server.
pub struct EventStreamConnectionInfo {
    pub name:           String,
    pub verbose:        bool,
    pub cluster_uri:    String,
    pub auth_token:     AutomateAuthToken,
    pub connect_method: EventStreamConnectMethod,
}

impl EventStreamConnectionInfo {
    pub fn new(supervisor_id: &str, config: EventStreamConfig) -> Self {
        EventStreamConnectionInfo { name:           format!("hab_client_{}", supervisor_id),
                                    verbose:        true,
                                    cluster_uri:    config.url,
                                    auth_token:     config.token,
                                    connect_method: config.connect_method, }
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
        publish(SERVICE_STARTED_SUBJECT,
                ServiceStartedEvent { service_metadata: Some(service.to_service_metadata()),
                                      event_metadata:   None, });
    }
}

/// Send an event for the stop of a Service.
pub fn service_stopped(service: &Service) {
    if stream_initialized() {
        publish(SERVICE_STOPPED_SUBJECT,
                ServiceStoppedEvent { service_metadata: Some(service.to_service_metadata()),
                                      event_metadata:   None, });
    }
}

/// Send an event at the start of a Service update.
pub fn service_update_started(service: &Service, update: &PackageIdent) {
    if stream_initialized() {
        publish(SERVICE_UPDATE_STARTED_SUBJECT,
                ServiceUpdateStartedEvent { event_metadata:       None,
                                            service_metadata:
                                                Some(service.to_service_metadata()),
                                            update_package_ident: update.clone().to_string(), });
    }
}

// Takes metadata directly, rather than a `&Service` like other event
// functions, because of how the asynchronous health checking
// currently works. Revisit when async/await + Pin is all stabilized.
pub fn health_check(metadata: ServiceMetadata,
                    health_check_result: HealthCheckResult,
                    health_check_hook_status: HealthCheckHookStatus) {
    if stream_initialized() {
        let health_check_result: types::HealthCheckResult = health_check_result.into();
        let maybe_duration = health_check_hook_status.maybe_duration();
        let maybe_process_output = health_check_hook_status.maybe_process_output();
        let exit_status = maybe_process_output.as_ref()
                                              .and_then(|o| o.exit_status().code());
        let StandardStreams { stdout, stderr } =
            maybe_process_output.map(ProcessOutput::standard_streams)
                                .unwrap_or_default();
        publish(HEALTHCHECK_SUBJECT,
                HealthCheckEvent { service_metadata: Some(metadata),
                                   event_metadata: None,
                                   result: i32::from(health_check_result),
                                   execution: maybe_duration.map(Duration::into),
                                   exit_status,
                                   stdout,
                                   stderr });
    }
}

////////////////////////////////////////////////////////////////////////

/// Internal helper function to know whether or not to go to the trouble of
/// creating event structures. If the event stream hasn't been
/// initialized, then we shouldn't need to do anything.
fn stream_initialized() -> bool { EVENT_STREAM.try_get::<EventStreamContainer>().is_some() }

/// Publish an event. This is the main interface that client code will
/// use.
///
/// If `init_stream` has not been called already, this function will
/// be a no-op.
fn publish(subject: &'static str, mut event: impl EventMessage) {
    if let Some(e) = EVENT_STREAM.try_get::<EventStreamContainer>() {
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

        let packet = EventPacket::new(subject, event.to_bytes());
        e.lock().send(packet);
    }
}

/// The subject and payload of an event message.
#[derive(Debug)]
struct EventPacket {
    subject: &'static str,
    payload: Vec<u8>,
}

impl EventPacket {
    fn new(subject: &'static str, payload: Vec<u8>) -> Self { Self { subject, payload } }
}

/// A lightweight handle for the event stream. All events get to the
/// event stream through this.
struct EventStream {
    sender: Sender<EventPacket>,
    handle: Option<FutureHandle>,
}

impl EventStream {
    fn new(sender: Sender<EventPacket>, handle: FutureHandle) -> Self {
        Self { sender,
               handle: Some(handle) }
    }

    /// Queues an event to be sent out.
    fn send(&mut self, event_packet: EventPacket) {
        trace!("About to queue an event: {:?}", event_packet);
        // If the server is down, the event stream channel may fill up. If this happens we are ok
        // dropping events.
        if let Err(e) = self.sender.try_send(event_packet) {
            error!("Failed to queue event: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prost::Message;
    use futures::{future::Future,
                  stream::Stream,
                  sync::mpsc as futures_mpsc};
    #[cfg(windows)]
    use habitat_core::os::process::windows_child::ExitStatus;
    #[cfg(unix)]
    use std::{os::unix::process::ExitStatusExt,
              process::ExitStatus};

    #[test]
    #[cfg(any(unix, windows))]
    fn health_check_event() {
        let (tx, rx) = futures_mpsc::channel(4);
        let event_stream = EventStream { sender: tx,
                                         handle: None, };
        EVENT_STREAM.set(EventStreamContainer::new(event_stream));
        EVENT_CORE.set(EventCore { supervisor_id: String::from("supervisor_id"),
                                   ip_address:    "127.0.0.1:8080".parse().unwrap(),
                                   fqdn:          String::from("fqdn"),
                                   application:   String::from("application"),
                                   environment:   String::from("environment"),
                                   site:          None,
                                   meta:          EventStreamMetadata::default(), });
        health_check(ServiceMetadata::default(),
                     HealthCheckResult::Ok,
                     HealthCheckHookStatus::NoHook);
        health_check(ServiceMetadata::default(),
                     HealthCheckResult::Warning,
                     HealthCheckHookStatus::FailedToRun(Duration::from_secs(5)));
        #[cfg(windows)]
        let exit_status = ExitStatus::from(2);
        #[cfg(unix)]
        let exit_status = ExitStatus::from_raw(2);
        let process_output =
            ProcessOutput::from_raw(StandardStreams { stdout: Some(String::from("stdout")),
                                                      stderr: Some(String::from("stderr")), },
                                    exit_status);
        health_check(ServiceMetadata::default(),
                     HealthCheckResult::Critical,
                     HealthCheckHookStatus::Ran(process_output, Duration::from_secs(10)));
        #[cfg(windows)]
        let exit_status = ExitStatus::from(3);
        #[cfg(unix)]
        let exit_status = ExitStatus::from_raw(3);
        let process_output =
            ProcessOutput::from_raw(StandardStreams { stdout: None,
                                                      stderr: Some(String::from("stderr")), },
                                    exit_status);
        health_check(ServiceMetadata::default(),
                     HealthCheckResult::Unknown,
                     HealthCheckHookStatus::Ran(process_output, Duration::from_secs(15)));
        let events = rx.take(4).collect().wait().unwrap();

        let event = HealthCheckEvent::decode(&events[0].payload).unwrap();
        assert_eq!(event.result, 0);
        assert_eq!(event.execution, None);
        assert_eq!(event.exit_status, None);
        assert_eq!(event.stdout, None);
        assert_eq!(event.stderr, None);

        let event = HealthCheckEvent::decode(&events[1].payload).unwrap();
        assert_eq!(event.result, 1);
        assert_eq!(event.execution.unwrap().seconds, 5);
        assert_eq!(event.exit_status, None);
        assert_eq!(event.stdout, None);
        assert_eq!(event.stderr, None);

        let event = HealthCheckEvent::decode(&events[2].payload).unwrap();
        assert_eq!(event.result, 2);
        assert_eq!(event.execution.unwrap().seconds, 10);
        #[cfg(windows)]
        assert_eq!(event.exit_status, Some(2));
        // `ExitStatus::from_raw` sets the signal not the code
        #[cfg(unix)]
        assert_eq!(event.exit_status, None);
        assert_eq!(event.stdout, Some(String::from("stdout")));
        assert_eq!(event.stderr, Some(String::from("stderr")));

        let event = HealthCheckEvent::decode(&events[3].payload).unwrap();
        assert_eq!(event.result, 3);
        assert_eq!(event.execution.unwrap().seconds, 15);
        #[cfg(windows)]
        assert_eq!(event.exit_status, Some(3));
        // `ExitStatus::from_raw` sets the signal not the code
        #[cfg(unix)]
        assert_eq!(event.exit_status, None);
        assert_eq!(event.stdout, None);
        assert_eq!(event.stderr, Some(String::from("stderr")));
    }
}
