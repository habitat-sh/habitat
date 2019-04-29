use crate::event::{Error,
                   EventConnectionInfo,
                   EventStream,
                   Result};
use futures::{sync::mpsc as futures_mpsc,
              Future,
              Stream};
use ratsio::{nats_client::NatsClientOptions,
             stan_client::{StanClient,
                           StanMessage,
                           StanOptions}};
use std::{sync::mpsc as std_mpsc,
          thread,
          time::Duration};
use tokio::{executor,
            runtime::current_thread::Runtime as ThreadRuntime};

/// All messages are published under this subject.
const HABITAT_SUBJECT: &str = "habitat";

pub(super) fn init_stream(conn_info: EventConnectionInfo) -> Result<EventStream> {
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
                              let nats_options =
                                  NatsClientOptions::builder().cluster_uris(cluster_uri)
                                  // TODO (CM): implement Into<String>?
                                  .auth_token(auth_token.to_string())
                                                              .verbose(verbose)
                                                              .build()
                                                              .unwrap();
                              let stan_options = StanOptions::builder().nats_options(nats_options)
                                                                       .cluster_id(cluster_id)
                                                                       .client_id(name)
                                                                       .build()
                                                                       .unwrap();

                              let publisher =
                                  StanClient::from_options(stan_options).map_err(|ratsio_error| {
                                      error!("Error upgrading to streaming NATS client: {}",
                                             ratsio_error)
                                  })
                                  .and_then(move |client| {
                                      sync_tx.send(())
                                             .expect("Couldn't synchronize event thread!");

                                      event_rx.for_each(move |event: Vec<u8>| {
                                                  let stan_msg =
                                                      StanMessage::new(HABITAT_SUBJECT.into(),
                                                                       event.into());
                                                  let publish_event = client
                            .send(stan_msg)
                            .map_err(|e| {
                                error!("Error publishing event: {}", e)
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
                          .map_err(Error::SpawnEventThreadError)?; // TODO (CM): ratsio error variant

    sync_rx.recv_timeout(Duration::from_secs(5))
           .map_err(Error::ConnectEventServerError)?;
    Ok(EventStream(event_tx))
}
