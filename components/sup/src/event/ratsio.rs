use super::EventStreamConnectTimeout;
use crate::event::{Error,
                   EventStream,
                   EventStreamConnectionInfo,
                   Result};
use futures::{sync::mpsc as futures_mpsc,
              Future,
              Stream};
use habitat_core::env::Config as _;
use ratsio::{nats_client::NatsClientOptions,
             stan_client::{StanClient,
                           StanMessage,
                           StanOptions}};
use std::{sync::mpsc as std_mpsc,
          thread};
use tokio::{executor,
            runtime::current_thread::Runtime as ThreadRuntime};

/// All messages are published under this subject.
const HABITAT_SUBJECT: &str = "habitat";

pub(super) fn init_stream(conn_info: EventStreamConnectionInfo) -> Result<EventStream> {
    let (event_tx, event_rx) = futures_mpsc::unbounded();
    let (sync_tx, sync_rx) = std_mpsc::sync_channel(0); // rendezvous channel

    // Disabling rustfmt on this... I think we might be running into
    // https://github.com/rust-lang/rustfmt/issues/1762
    #[rustfmt::skip]
    thread::Builder::new().name("events".to_string())
                          .spawn(move || {
                              let EventStreamConnectionInfo { name,
                                                        verbose,
                                                        cluster_uri,
                                                        cluster_id,
                                                        auth_token, } = conn_info;
                              let nats_options =
                                  NatsClientOptions::builder().cluster_uris(cluster_uri)
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
                                                               event);
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
                          .map_err(Error::SpawnEventThreadError)?;

    sync_rx.recv_timeout(EventStreamConnectTimeout::configured_value().into())
           .map_err(Error::ConnectEventServerError)?;
    Ok(EventStream(event_tx))
}
