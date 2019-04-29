use crate::event::{Error,
                   EventConnectionInfo,
                   EventStream,
                   Result};
use futures::{sync::mpsc as futures_mpsc,
              Future,
              Stream};
use nitox::{commands::ConnectCommand,
            streaming::client::NatsStreamingClient,
            NatsClient,
            NatsClientOptions};
use std::{sync::mpsc as std_mpsc,
          thread,
          time::Duration};
use tokio::{executor,
            runtime::current_thread::Runtime as ThreadRuntime};
/// All messages are published under this subject.
const HABITAT_SUBJECT: &str = "habitat";

pub(super) fn init_stream(conn_info: EventConnectionInfo) -> Result<EventStream> {
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
                                                        cluster_id,
                                                        auth_token, } = conn_info;
                              let cc =
                                  ConnectCommand::builder().name(Some(name))
                                                           .verbose(verbose)
                                                           .auth_token(Some(auth_token.to_string()))
                                                           .tls_required(false)
                                                           .build()
                                                           .expect("Could not create NATS \
                                                                    ConnectCommand");
                              let opts =
                                  NatsClientOptions::builder().connect_command(cc)
                                                              .cluster_uri(cluster_uri.as_str())
                                                              .build()
                                                              .expect("Could not create \
                                                                       NatsClientOptions");

                              let publisher = NatsClient::from_options(opts).map_err(|e| {
                                                  error!("Error connecting to NATS: {}", e);
                                                  e.into()
                                              })
                                              .and_then(|client| {
                                                  NatsStreamingClient::from(client)
                        .cluster_id(cluster_id)
                        .connect()
                                              })
                                              .map_err(|streaming_error| {
                                                  error!("Error upgrading to streaming NATS \
                                                          client: {}",
                                                         streaming_error)
                                              })
                                              .and_then(move |client| {
                                                  sync_tx.send(()).expect("Couldn't synchronize \
                                                                           event thread!");
                                                  event_rx.for_each(move |event: Vec<u8>| {
                                                              let publish_event = client
                            .publish(HABITAT_SUBJECT.into(), event.into())
                            .map_err(|e| {
                                error!("Error publishing event: {}", e);
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
                          .map_err(Error::SpawnEventThreadError)?;

    sync_rx.recv_timeout(Duration::from_secs(5))
           .map_err(Error::ConnectEventServerError)?;
    Ok(EventStream(event_tx))
}
