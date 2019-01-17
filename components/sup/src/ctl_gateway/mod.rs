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

//! CtlGateway, short for Control Gateway, is a TCP based client and server connection for
//! sending and receiving command and control requests to a running Supervisor. Operational tasks
//! such as starting, stopping, loading, and unloading services are exposed through the
//! CtlGateway.
//!
//! The [`ctl_gateway.client`] and [`ctl_gateway.server`] speak a streaming, multiplexed, binary
//! protocol defined in [`protocol.codec`].

pub mod server;

use std::borrow::Cow;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use regex::Regex;

use crate::api_client::DisplayProgress;
use crate::common::ui::UIWriter;
use crate::hcore::{self, output};
use crate::protocol;
use futures::prelude::*;

use crate::error::{Error, Result};

lazy_static! {
    /// Shamelessly stolen from https://github.com/chalk/ansi-regex/blob/master/index.js
    static ref STRIP_ANSI_CODES: Regex = Regex::new(
        r"[\x1b\x9b][\[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-PRZcf-nqry=><]")
        .unwrap();
}

/// Time to wait in milliseconds for a client connection to timeout.
pub const REQ_TIMEOUT: u64 = 10_000;
static LOGKEY: &'static str = "AG";

/// The control gateway secret should only be readable by the
/// Supervisor process
#[cfg(not(windows))]
pub const CTL_SECRET_PERMISSIONS: u32 = 0o600;

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
        self.tx.as_ref().unwrap().start_send(wire).ok(); // ignore Err return
    }
}

impl UIWriter for CtlRequest {
    type ProgressBar = NetProgressBar;

    fn out(&mut self) -> &mut dyn io::Write {
        self
    }

    fn err(&mut self) -> &mut dyn io::Write {
        self
    }

    // Whether output is colored, or the output is a terminal is,
    // technically, a bit more complicated than simply `true`, since
    // output from the CtlRequest ends up being streamed back to the
    // client (which may or may not be an interactive terminal, by the
    // way), as well as being rendered into the Supervisor's output
    // streams, which have their own, possibly opposing, formatting
    // constraints.
    //
    // For the time being, this is mostly addressed manually down in
    // the `io::Write` implementation. Furthermore, this is currently
    // the only implementation of UIWriter subject to these
    // constraints, so it's not clear that adding additional
    // complexity at the type / trait modeling level is worthwhile.

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

        // The protocol reply is destined for the client, so (for now,
        // at least), we'll retain any colored output.
        //
        // `line` will also have a newline character at the end, FYI.
        let mut msg = protocol::ctl::ConsoleLine::default();
        msg.line = line.clone();
        self.reply_partial(msg);

        // Down here, however, we're doing double-duty by *also*
        // sending the output to the Supervisor's output stream. In
        // _this_ case, we want to honor whatever global logging flags
        // have been set; in particular, if we're emitting
        // JSON-formatted logs or if we've been instructed to not
        // output ANSI color codes, we need to strip those codes out.
        //
        // TODO (CM): Obviously, this feels hacky to the extreme. It'd
        // be nice to find a cleaner way to model this, but the fact
        // that CtlRequest is sending output to two destinations with
        // different formatting requirements complicates things a bit.
        let maybe_stripped = if output::is_json() || !output::is_color() {
            STRIP_ANSI_CODES.replace_all(&line, "")
        } else {
            Cow::Owned(line)
        };
        // TODO (CM): Consider pulling this newline trimming up into
        // the macro
        outputln!("{}", maybe_stripped.trim_right_matches('\n'));

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
            inner: protocol::ctl::NetProgress::default(),
            req: req,
        }
    }
}

impl DisplayProgress for NetProgressBar {
    fn size(&mut self, size: u64) {
        self.inner.total = size;
    }

    fn finish(&mut self) {}
}

impl io::Write for NetProgressBar {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.position += buf.len() as u64;
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
        set_permissions(&secret_key_path)?;
        Ok(out)
    }
}

#[cfg(not(windows))]
fn set_permissions<T: AsRef<Path>>(path: T) -> hcore::error::Result<()> {
    use crate::hcore::util::posix_perm;

    posix_perm::set_permissions(path.as_ref(), CTL_SECRET_PERMISSIONS)
}

#[cfg(windows)]
fn set_permissions<T: AsRef<Path>>(path: T) -> hcore::error::Result<()> {
    use hcore::util::win_perm;

    win_perm::harden_path(path.as_ref())
}
