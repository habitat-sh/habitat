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
//! protocol defined in [`ctl_gateway.codec`].

pub mod client;
pub mod codec;
pub mod server;

use std::fmt;
use std::io::{self, Read, Write};
use std::fs::{self, File};
use std::net::{Ipv4Addr, SocketAddr};
use std::path::{Path, PathBuf};

use base64;
use common::ui::UIWriter;
use depot_client::DisplayProgress;
use futures::prelude::*;
use hcore;
use hcore::util::perm;
use rand::{self, Rng};

use error::{Result, Error};
use protocols;

/// Default listening port for the CtlGateway listener.
pub const DEFAULT_PORT: u16 = 9632;
/// Time to wait in milliseconds for a client connection to timeout.
pub const REQ_TIMEOUT: u64 = 10_000;
/// Name of file containing the CtlGateway secret key.
const CTL_SECRET_FILENAME: &'static str = "CTL_SECRET";
/// Length of characters in CtlGateway secret key.
const CTL_SECRET_LEN: usize = 64;
/// Environment variable optionally containing the CtlGateway secret key.
const CTL_SECRET_ENVVAR: &'static str = "HAB_CTL_SECRET";
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
    transaction: Option<codec::SrvTxn>,
}

impl CtlRequest {
    /// Create a new CtlRequest from an optional [`server.CtlSender`] and [`codec::SrvTxn`].
    pub fn new(tx: Option<server::CtlSender>, transaction: Option<codec::SrvTxn>) -> Self {
        CtlRequest {
            tx: tx,
            transaction: transaction,
        }
    }

    /// Reply to the transaction with the given message but indicate to the receiver that this is
    /// not the final message for the transaction.
    pub fn reply_partial<T>(&mut self, msg: T)
    where
        T: Into<codec::SrvMessage> + fmt::Debug,
    {
        self.send_msg(msg, false);
    }

    /// Reply to the transaction with the given message and indicate to the receiver that this is
    /// the final message for the transaction.
    pub fn reply_complete<T>(&mut self, msg: T)
    where
        T: Into<codec::SrvMessage> + fmt::Debug,
    {
        self.send_msg(msg, true);
    }

    /// Returns true if the request is transactional and false if not.
    pub fn transactional(&self) -> bool {
        self.transaction.is_some() && self.tx.is_some()
    }

    fn send_msg<T>(&mut self, msg: T, complete: bool)
    where
        T: Into<codec::SrvMessage> + fmt::Debug,
    {
        if !self.transactional() {
            warn!(
                "Attempted to reply to a non-transactional message with {:?}",
                msg
            );
            return;
        }
        let mut wire: codec::SrvMessage = msg.into();
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

    fn is_colored(&self) -> bool {
        true
    }

    fn is_a_terminal(&self) -> bool {
        true
    }

    fn progress(&self) -> Option<Self::ProgressBar> {
        if self.is_a_terminal() {
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
        let mut msg = protocols::ctl::ConsoleLine::new();
        msg.set_line(line);
        self.reply_partial(msg);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// A wrapper around a [`protocols.ctl.NetProgress`] and [`CtlRequest`]. This type implements
/// traits for writing it's progress to the console.
pub struct NetProgressBar {
    inner: protocols::ctl::NetProgress,
    req: CtlRequest,
}

impl NetProgressBar {
    /// Create a new progress bar.
    pub fn new(req: CtlRequest) -> Self {
        NetProgressBar {
            inner: protocols::ctl::NetProgress::new(),
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

/// Return a SocketAddr with the default listening address and port.
pub fn default_addr() -> SocketAddr {
    SocketAddr::from((Ipv4Addr::new(127, 0, 0, 1), DEFAULT_PORT))
}

/// Read the secret key used to authenticate connections to the `CtlGateway` from disk and write
/// it to the given out buffer. An `Ok` return value of `true` indicates a successful read while
/// `false` indicates the file was not found.
pub fn read_secret_key<T>(sup_root: T, out: &mut String) -> Result<bool>
where
    T: AsRef<Path>,
{
    // We attempt to read from environment variable before attempting to read from filesystem
    // because a remote client won't have the sup root on it's disk. The env var is set by the
    // `hab` binary and populated by it's config.
    if let Some(value) = hcore::env::var(CTL_SECRET_ENVVAR).ok() {
        *out = value;
        return Ok(true);
    }
    let secret_key_path = sup_root.as_ref().join(CTL_SECRET_FILENAME);
    if secret_key_path.exists() {
        if secret_key_path.is_dir() {
            return Err(sup_error!(Error::CtlSecretConflict(secret_key_path)));
        }
        File::open(&secret_key_path)
            .and_then(|mut f| f.read_to_string(out))
            .map_err(move |e| sup_error!(Error::CtlSecretIo(secret_key_path, e)))?;
        Ok(true)
    } else {
        Ok(false)
    }
}

/// First attempts to read the secret key used to authenticate with the `CtlGateway` from disk
/// and, if not found, will generate a new key and write it to disk.
pub fn readgen_secret_key<T>(sup_root: T) -> Result<String>
where
    T: AsRef<Path>,
{
    let mut out = String::new();
    fs::create_dir_all(&sup_root).map_err(|e| {
        sup_error!(Error::CtlSecretIo(sup_root.as_ref().to_path_buf(), e))
    })?;
    if read_secret_key(&sup_root, &mut out)? {
        Ok(out)
    } else {
        let secret_key_path = secret_key_path(sup_root);
        {
            let mut f = File::create(&secret_key_path)?;
            generate_secret_key(&mut out);
            f.write_all(out.as_bytes())?;
            f.sync_all()?;
        }
        perm::set_permissions(&secret_key_path, 0600)?;
        Ok(out)
    }
}

/// Returns the location of the CtlGateway Secret on disk for the given Supervisor root.
pub fn secret_key_path<T>(sup_root: T) -> PathBuf
where
    T: AsRef<Path>,
{
    sup_root.as_ref().join(CTL_SECRET_FILENAME)
}

/// Generate a new secret key used for authenticating clients to the `CtlGateway`.
fn generate_secret_key(out: &mut String) {
    let mut rng = rand::OsRng::new().unwrap();
    let mut result = vec![0u8; CTL_SECRET_LEN];
    rng.fill_bytes(&mut result);
    *out = base64::encode(&result);
}
