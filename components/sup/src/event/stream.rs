use crate::event::{Error,
                   EventStream,
                   EventStreamConnectionInfo,
                   Result};
use futures::sync::mpsc as futures_mpsc;
use std::{sync::mpsc as std_mpsc,
          thread};

/// All messages are published under this subject.
const HABITAT_SUBJECT: &str = "habitat";

pub(super) fn init_stream(conn_info: EventStreamConnectionInfo) -> Result<EventStream> {
    let (event_tx, event_rx) = futures_mpsc::unbounded();
    let (sync_tx, sync_rx) = std_mpsc::sync_channel(0); // rendezvous channel

    let EventStreamConnectionInfo { name,
                                    verbose,
                                    cluster_uri,
                                    cluster_id,
                                    auth_token,
                                    connect_method, } = conn_info;
    let connect_method_is_timeout = connect_method.is_timeout();

    thread::Builder::new().name("events".to_string())
                          .spawn(move || {})
                          .map_err(Error::SpawnEventThreadError)?;

    if let Some(connect_method) = connect_method.into() {
        sync_rx.recv_timeout(connect_method)
               .map_err(Error::ConnectEventServerError)?;
    }
    Ok(EventStream(event_tx))
}
