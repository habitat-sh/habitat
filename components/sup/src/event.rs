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
mod nats_message_stream;
mod types;

pub(crate) use self::types::ServiceMetadata;
use self::types::{EventMessage,
                  EventMetadata,
                  HealthCheckEvent,
                  ServiceStartedEvent,
                  ServiceStoppedEvent,
                  ServiceUpdateStartedEvent};
use crate::manager::{service::{HealthCheckHookStatus,
                               HealthCheckResult,
                               ProcessOutput,
                               Service,
                               StandardStreams},
                     sys::Sys};
use clap::ArgMatches;
pub use error::{Error,
                Result};
use habitat_common::types::{AutomateAuthToken,
                            EventStreamConnectMethod,
                            EventStreamMetadata,
                            EventStreamServerCertificate};
use habitat_core::{package::ident::PackageIdent,
                   service::HealthCheckInterval};
use nats_message_stream::{NatsMessage,
                          NatsMessageStream};
use prost_types::Duration as ProstDuration;
use rants::Subject;
use state::Storage;
use std::{net::SocketAddr,
          time::Duration};

lazy_static! {
    // TODO (CM): When const fn support lands in stable, we can ditch
    // this lazy_static call.

    // NATS subject names
    static ref SERVICE_STARTED_SUBJECT: Subject =
        "habitat.event.service_started".parse().expect("valid NATS subject");
    static ref SERVICE_STOPPED_SUBJECT: Subject =
        "habitat.event.service_stopped".parse().expect("valid NATS subject");
    static ref SERVICE_UPDATE_STARTED_SUBJECT: Subject =
        "habitat.event.service_update_started".parse().expect("valid NATS subject");
    static ref HEALTHCHECK_SUBJECT: Subject =
        "habitat.event.healthcheck".parse().expect("valid NATS subject");

    /// Reference to the event stream.
    static ref NATS_MESSAGE_STREAM: Storage<NatsMessageStream> = Storage::new();
    /// Core information that is shared between all events.
    static ref EVENT_CORE: Storage<EventCore> = Storage::new();
}

/// Starts a new task for sending events to a NATS Streaming
/// server. Stashes the handle to the stream, as well as the core
/// event information that will be a part of all events, in a global
/// static reference for access later.
pub async fn init(sys: &Sys, fqdn: String, config: EventStreamConfig) -> Result<()> {
    // Only initialize once
    if !initialized() {
        let supervisor_id = sys.member_id.clone();
        let ip_address = sys.gossip_listen();
        let event_core = EventCore::new(&supervisor_id, ip_address, &fqdn, &config);
        let stream = NatsMessageStream::new(&supervisor_id, config).await?;
        NATS_MESSAGE_STREAM.set(stream);
        EVENT_CORE.set(event_core);
    }
    Ok(())
}

/// Captures all event stream-related configuration options that would
/// be passed in by a user
#[derive(Clone, Debug)]
pub struct EventStreamConfig {
    environment:        String,
    application:        String,
    site:               Option<String>,
    meta:               EventStreamMetadata,
    token:              AutomateAuthToken,
    url:                String,
    connect_method:     EventStreamConnectMethod,
    server_certificate: Option<EventStreamServerCertificate>,
}

impl<'a> From<&'a ArgMatches<'a>> for EventStreamConfig {
    fn from(m: &ArgMatches) -> Self {
        EventStreamConfig { environment:        m.value_of("EVENT_STREAM_ENVIRONMENT")
                                                 .map(str::to_string)
                                                 .expect("Required option for EventStream feature"),
                            application:        m.value_of("EVENT_STREAM_APPLICATION")
                                                 .map(str::to_string)
                                                 .expect("Required option for EventStream feature"),
                            site:               m.value_of("EVENT_STREAM_SITE").map(str::to_string),
                            meta:               EventStreamMetadata::from(m),
                            token:              AutomateAuthToken::from(m),
                            url:                m.value_of("EVENT_STREAM_URL")
                                                 .map(str::to_string)
                                                 .expect("Required option for EventStream feature"),
                            connect_method:     EventStreamConnectMethod::from(m),
                            server_certificate: EventStreamServerCertificate::from_arg_matches(m), }
    }
}

/// Send an event for the start of a Service.
pub fn service_started(service: &Service) {
    if initialized() {
        publish(&SERVICE_STARTED_SUBJECT,
                ServiceStartedEvent { service_metadata: Some(service.to_service_metadata()),
                                      event_metadata:   None, });
    }
}

/// Send an event for the stop of a Service.
pub fn service_stopped(service: &Service) {
    if initialized() {
        publish(&SERVICE_STOPPED_SUBJECT,
                ServiceStoppedEvent { service_metadata: Some(service.to_service_metadata()),
                                      event_metadata:   None, });
    }
}

/// Send an event at the start of a Service update.
pub fn service_update_started(service: &Service, update: &PackageIdent) {
    if initialized() {
        publish(&SERVICE_UPDATE_STARTED_SUBJECT,
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
                    health_check_hook_status: HealthCheckHookStatus,
                    health_check_interval: HealthCheckInterval) {
    if initialized() {
        let health_check_result: types::HealthCheckResult = health_check_result.into();
        let maybe_duration = health_check_hook_status.maybe_duration();
        let maybe_process_output = health_check_hook_status.maybe_process_output();
        let exit_status = maybe_process_output.as_ref()
                                              .and_then(|o| o.exit_status().code());
        let StandardStreams { stdout, stderr } =
            maybe_process_output.map(ProcessOutput::standard_streams)
                                .unwrap_or_default();

        let prost_interval = ProstDuration::from(Duration::from(health_check_interval));

        publish(&HEALTHCHECK_SUBJECT,
                HealthCheckEvent { service_metadata: Some(metadata),
                                   event_metadata: None,
                                   result: i32::from(health_check_result),
                                   execution: maybe_duration.map(Duration::into),
                                   exit_status,
                                   stdout,
                                   stderr,
                                   interval: Some(prost_interval) });
    }
}

////////////////////////////////////////////////////////////////////////

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
struct EventCore {
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
    fn new(supervisor_id: &str,
           ip_address: SocketAddr,
           fqdn: &str,
           config: &EventStreamConfig)
           -> Self {
        EventCore { supervisor_id: String::from(supervisor_id),
                    ip_address,
                    fqdn: String::from(fqdn),
                    environment: config.environment.clone(),
                    application: config.application.clone(),
                    site: config.site.clone(),
                    meta: config.meta.clone() }
    }
}

/// Internal helper function to know whether or not to go to the trouble of
/// creating event structures. If the event stream hasn't been
/// initialized, then we shouldn't need to do anything.
fn initialized() -> bool { NATS_MESSAGE_STREAM.try_get().is_some() }

/// Publish an event. This is the main interface that client code will
/// use.
///
/// If `init_stream` has not been called already, this function will
/// be a no-op.
fn publish(subject: &'static Subject, mut event: impl EventMessage) {
    if let Some(stream) = NATS_MESSAGE_STREAM.try_get() {
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
                                             ..EVENT_CORE.get().to_event_metadata() });

        let packet = NatsMessage::new(subject, event.to_bytes());
        stream.send(packet);
    }
}

#[cfg(test)]
mod tests {
    use super::{nats_message_stream::NatsMessageStream,
                *};
    use crate::prost::Message;
    use futures::{channel::mpsc as futures_mpsc,
                  stream::StreamExt};
    #[cfg(windows)]
    use habitat_core::os::process::windows_child::ExitStatus;
    use habitat_core::service::HealthCheckInterval;
    #[cfg(unix)]
    use std::{os::unix::process::ExitStatusExt,
              process::ExitStatus};

    #[tokio::test]
    #[cfg(any(unix, windows))]
    async fn health_check_event() {
        let (tx, rx) = futures_mpsc::unbounded();
        NATS_MESSAGE_STREAM.set(NatsMessageStream(tx));
        EVENT_CORE.set(EventCore { supervisor_id: String::from("supervisor_id"),
                                   ip_address:    "127.0.0.1:8080".parse().unwrap(),
                                   fqdn:          String::from("fqdn"),
                                   application:   String::from("application"),
                                   environment:   String::from("environment"),
                                   site:          None,
                                   meta:          EventStreamMetadata::default(), });
        health_check(ServiceMetadata::default(),
                     HealthCheckResult::Ok,
                     HealthCheckHookStatus::NoHook,
                     HealthCheckInterval::default());
        health_check(ServiceMetadata::default(),
                     HealthCheckResult::Warning,
                     HealthCheckHookStatus::FailedToRun(Duration::from_secs(5)),
                     HealthCheckInterval::default());
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
                     HealthCheckHookStatus::Ran(process_output, Duration::from_secs(10)),
                     HealthCheckInterval::default());
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
                     HealthCheckHookStatus::Ran(process_output, Duration::from_secs(15)),
                     HealthCheckInterval::default());
        let events = rx.take(4).collect::<Vec<_>>().await;

        let event = HealthCheckEvent::decode(events[0].payload()).unwrap();
        assert_eq!(event.result, 0);
        assert_eq!(event.execution, None);
        assert_eq!(event.exit_status, None);
        assert_eq!(event.stdout, None);
        assert_eq!(event.stderr, None);

        let default_interval = HealthCheckInterval::default();
        let prost_interval = ProstDuration::from(Duration::from(default_interval));
        let prost_interval_option = Some(prost_interval);

        assert_eq!(event.interval, prost_interval_option);

        let event = HealthCheckEvent::decode(events[1].payload()).unwrap();
        assert_eq!(event.result, 1);
        assert_eq!(event.execution.unwrap().seconds, 5);
        assert_eq!(event.exit_status, None);
        assert_eq!(event.stdout, None);
        assert_eq!(event.stderr, None);

        let event = HealthCheckEvent::decode(events[2].payload()).unwrap();
        assert_eq!(event.result, 2);
        assert_eq!(event.execution.unwrap().seconds, 10);
        #[cfg(windows)]
        assert_eq!(event.exit_status, Some(2));
        // `ExitStatus::from_raw` sets the signal not the code
        #[cfg(unix)]
        assert_eq!(event.exit_status, None);
        assert_eq!(event.stdout, Some(String::from("stdout")));
        assert_eq!(event.stderr, Some(String::from("stderr")));

        let event = HealthCheckEvent::decode(events[3].payload()).unwrap();
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
