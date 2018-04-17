// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

//! CltGateway, short for Control Gateway, is a TCP based client and server connection for
//! sending and receiving command and control requests to a running Supervisor. Operational tasks
//! such as starting, stopping, loading, and unloading services are exposed through the
//! CtlGateway.
//!
//! The [`ctl_gateway.client`] and [`ctl_gateway.server`] speak a streaming, multiplexed, binary
//! protocol defined in [`protocol.codec`].

pub mod server;

use std::fmt;
use std::io::{self, Write};
use std::fs::{self, File};
use std::path::Path;

use common::ui::UIWriter;
use depot_client::DisplayProgress;
use futures::prelude::*;
use hcore::util::perm;
use protocol;

use error::{Error, Result};

/// Time to wait in milliseconds for a client connection to timeout.
pub const REQ_TIMEOUT: u64 = 10_000;
static LOGKEY: &'static str = "AG";

/// Used by modules outside of the CtlGateway for seamlessly replying to transactional messages.
/// This type is used in functions which can be called by the CtlGateway such as
/// [`Manager::service_load`] and [`Manager::service_unload`].
///
/// A _bare request_ can be instantiated with `CtlRequest::default()` and used as a no-op stub when
/// calling functions which typically respond to the client. This is useful when the caller isn't
/// a client but needs to perform the same operation. For example, loading an initial service for
/// the Supervisor has no need to reply back to a networked client, so passing a bare request will
/// no-op any calls to send messages back to the client.
#[derive(Clone, Default)]
pub struct CtlRequest {
    /// The sending side of the CtlGateway's server. Replies are sent across this channel and then
    /// eventually over the network back to the client.
    tx: Option<server::CtlSender>,
    /// Transaction for the given request.
    transaction: Option<protocol::codec::SrvTxn>,
}

impl CtlRequest {
    /// Create a new CtlRequest from an optional [`server.CtlSender`] and
    /// [`protocol.codec.SrvTxn`].
    pub fn new(
        tx: Option<server::CtlSender>,
        transaction: Option<protocol::codec::SrvTxn>,
    ) -> Self {
        CtlRequest {
            tx: tx,
            transaction: transaction,
        }
    }

    /// Reply to the transaction with the given message but indicate to the receiver that this is
    /// not the final message for the transaction.
    pub fn reply_partial<T>(&mut self, msg: T)
    where
        T: Into<protocol::codec::SrvMessage> + fmt::Debug,
    {
        self.send_msg(msg, false);
    }

    /// Reply to the transaction with the given message and indicate to the receiver that this is
    /// the final message for the transaction.
    pub fn reply_complete<T>(&mut self, msg: T)
    where
        T: Into<protocol::codec::SrvMessage> + fmt::Debug,
    {
        self.send_msg(msg, true);
    }

    /// Returns true if the request is transactional and false if not.
    pub fn transactional(&self) -> bool {
        self.transaction.is_some() && self.tx.is_some()
    }

    fn send_msg<T>(&mut self, msg: T, complete: bool)
    where
        T: Into<protocol::codec::SrvMessage> + fmt::Debug,
    {
        if !self.transactional() {
            warn!(
                "Attempted to reply to a non-transactional message with {:?}",
                msg
            );
            return;
        }
        let mut wire: protocol::codec::SrvMessage = msg.into();
        wire.reply_for(self.transaction.unwrap(), complete);
        self.tx.as_ref().unwrap().start_send(wire).unwrap();
    }
}

impl UIWriter for CtlRequest {
    type ProgressBar = NetProgressBar;

    fn out(&mut self) -> &mut io::Write {
        self
    }

    fn err(&mut self) -> &mut io::Write {
        self
    }

    fn is_out_colored(&self) -> bool {
        true
    }

    fn is_err_colored(&self) -> bool {
        true
    }

    fn is_out_a_terminal(&self) -> bool {
        true
    }

    fn is_err_a_terminal(&self) -> bool {
        true
    }

    fn progress(&self) -> Option<Self::ProgressBar> {
        if self.is_out_a_terminal() {
            Some(Self::ProgressBar::new(self.clone()))
        } else {
            None
        }
    }
}

impl io::Write for CtlRequest {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let line = String::from_utf8_lossy(buf).into_owned();
        output!("{}", line);
        let mut msg = protocol::ctl::ConsoleLine::new();
        msg.set_line(line);
        self.reply_partial(msg);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// A wrapper around a [`protocol.ctl.NetProgress`] and [`CtlRequest`]. This type implements
/// traits for writing it's progress to the console.
pub struct NetProgressBar {
    inner: protocol::ctl::NetProgress,
    req: CtlRequest,
}

impl NetProgressBar {
    /// Create a new progress bar.
    pub fn new(req: CtlRequest) -> Self {
        NetProgressBar {
            inner: protocol::ctl::NetProgress::new(),
            req: req,
        }
    }
}

impl DisplayProgress for NetProgressBar {
    fn size(&mut self, size: u64) {
        self.inner.set_total(size);
    }

    fn finish(&mut self) {
        ()
    }
}

impl io::Write for NetProgressBar {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let position = self.inner.get_position() + buf.len() as u64;
        self.inner.set_position(position);
        self.req.reply_partial(self.inner.clone());
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// First attempts to read the secret key used to authenticate with the `CtlGateway` from disk
/// and, if not found, will generate a new key and write it to disk.
pub fn readgen_secret_key<T>(sup_root: T) -> Result<String>
where
    T: AsRef<Path>,
{
    let mut out = String::new();
    fs::create_dir_all(&sup_root)
        .map_err(|e| sup_error!(Error::CtlSecretIo(sup_root.as_ref().to_path_buf(), e)))?;
    if protocol::read_secret_key(&sup_root, &mut out)
        .ok()
        .unwrap_or(false)
    {
        Ok(out)
    } else {
        let secret_key_path = protocol::secret_key_path(sup_root);
        {
            let mut f = File::create(&secret_key_path)?;
            protocol::generate_secret_key(&mut out);
            f.write_all(out.as_bytes())?;
            f.sync_all()?;
        }
        perm::set_permissions(&secret_key_path, 0600)?;
        Ok(out)
    }
}
