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

pub mod acceptor;
pub mod handler;
pub mod server;

use crate::error::{Error,
                   Result};
use futures::prelude::*;
use habitat_api_client::DisplayProgress;
use habitat_common::{output::{self,
                              OutputContext,
                              OutputFormat,
                              StructuredOutput},
                     ui::UIWriter,
                     PROGRAM_NAME};
use habitat_core;
use habitat_sup_protocol;
use std::{fmt,
          fs::{self,
               File},
          io::{self,
               Write},
          path::Path};
use termcolor::{Color,
                ColorSpec,
                StandardStream,
                WriteColor};

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
// TODO (CM): The ONLY reason we're keeping Default, and thus the
// `Option` on the `tx` field, is because we're using `Default` to
// create a request for loading a service if you start a Supervisor
// with a package (i.e., `hab sup run core/redis`... that's the ONLY
// PLACE this "bare request" pattern is used.
//
// We should look for ways to refactor this so we can simplify this
// code.
#[derive(Clone, Default)]
pub struct CtlRequest {
    /// The sending side of the CtlGateway's server. Replies are sent across this channel and then
    /// eventually over the network back to the client.
    tx: Option<server::CtlSender>,
    /// Transaction for the given request.
    transaction: Option<habitat_sup_protocol::codec::SrvTxn>,
    current_color_spec: ColorSpec,
    is_new_line: bool,
}

impl CtlRequest {
    /// Create a new CtlRequest from [`server.CtlSender`] and optional
    /// [`protocol.codec.SrvTxn`].
    pub fn new(tx: server::CtlSender,
               transaction: Option<habitat_sup_protocol::codec::SrvTxn>)
               -> Self {
        CtlRequest { tx: Some(tx),
                     transaction,
                     current_color_spec: ColorSpec::new(),
                     is_new_line: true }
    }

    /// Reply to the transaction with the given message but indicate to the receiver that this is
    /// not the final message for the transaction.
    pub fn reply_partial<T>(&mut self, msg: T)
        where T: Into<habitat_sup_protocol::codec::SrvMessage> + fmt::Debug
    {
        self.send_msg(msg, false);
    }

    /// Reply to the transaction with the given message and indicate to the receiver that this is
    /// the final message for the transaction.
    pub fn reply_complete<T>(&mut self, msg: T)
        where T: Into<habitat_sup_protocol::codec::SrvMessage> + fmt::Debug
    {
        self.send_msg(msg, true);
    }

    /// Returns true if the request is transactional and false if not.
    pub fn transactional(&self) -> bool { self.transaction.is_some() && self.tx.is_some() }

    fn send_msg<T>(&mut self, msg: T, complete: bool)
        where T: Into<habitat_sup_protocol::codec::SrvMessage> + fmt::Debug
    {
        if !self.transactional() {
            warn!("Attempted to reply to a non-transactional message with {:?}",
                  msg);
            return;
        }
        let mut wire: habitat_sup_protocol::codec::SrvMessage = msg.into();
        wire.reply_for(self.transaction.unwrap(), complete);
        self.tx.as_ref().unwrap().start_send(wire).ok(); // ignore Err return
    }
}

impl UIWriter for CtlRequest {
    type ProgressBar = NetProgressBar;

    fn out(&mut self) -> &mut dyn WriteColor { self }

    fn err(&mut self) -> &mut dyn WriteColor { self }

    fn is_out_a_terminal(&self) -> bool { true }

    fn is_err_a_terminal(&self) -> bool { true }

    fn progress(&self) -> Option<Self::ProgressBar> {
        if self.is_out_a_terminal() {
            Some(Self::ProgressBar::new(self.clone()))
        } else {
            None
        }
    }
}

impl WriteColor for CtlRequest {
    fn supports_color(&self) -> bool { true }

    fn reset(&mut self) -> io::Result<()> {
        self.current_color_spec = ColorSpec::new();
        Ok(())
    }

    fn set_color(&mut self, spec: &ColorSpec) -> io::Result<()> {
        self.current_color_spec = spec.clone();
        Ok(())
    }
}

impl Write for CtlRequest {
    fn flush(&mut self) -> io::Result<()> { Ok(()) }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let line = String::from_utf8(buf.to_vec()).expect("CtlRequest buffer valid utf8");

        // The protocol reply is destined for the client, so (for now,
        // at least), we'll apply colored output.
        //
        // `line` will also have a newline character at the end, FYI.
        let mut msg = habitat_sup_protocol::ctl::ConsoleLine::default();
        msg.line = line.to_string();
        msg.color = color_to_string(self.current_color_spec.fg());
        msg.bold = self.current_color_spec.bold();
        self.reply_partial(msg);

        // Down here, however, we're doing double-duty by *also*
        // sending the output to the Supervisor's output stream.
        //
        // TODO (CM): Obviously, this feels hacky to the extreme. It'd
        // be nice to find a cleaner way to model this, but the fact
        // that CtlRequest is sending output to two destinations with
        // different formatting requirements complicates things a bit.
        //
        // (MW): to add to the hackiness here we need to determine if the
        // buffer is the beginning of a line. If it is OR if it is JSON
        // formatted, then we want the structured output metadata. If it
        // is a line fragment starting somewhere in the middle (this will
        // usually be the case when styling changes) of a line, then
        // non-json content should just be printed as-is.
        let is_line_ending = line.ends_with('\n');
        if self.is_new_line || output::get_format() == OutputFormat::JSON {
            let output_format =
                if !self.current_color_spec.is_none() && output::get_format().is_color() {
                    OutputFormat::Color(self.current_color_spec.clone())
                } else {
                    output::get_format()
                };
            let so = StructuredOutput::new(&PROGRAM_NAME,
                                           LOGKEY,
                                           OutputContext { line:   line!(),
                                                           file:   file!(),
                                                           column: column!(), },
                                           output_format,
                                           output::get_verbosity(),
                                           line.trim_end_matches('\n'));

            let print_func = if is_line_ending {
                StructuredOutput::println
            } else {
                StructuredOutput::print
            };
            print_func(&so).expect("failed to write output to stdout");
        } else {
            let mut stdout = StandardStream::stdout(output::get_format().color_choice());
            stdout.set_color(&self.current_color_spec)?;
            write!(&mut stdout, "{}", line)?;
        }

        self.is_new_line = is_line_ending;

        self.reset()?;
        Ok(buf.len())
    }
}

fn color_to_string(color: Option<&Color>) -> Option<String> {
    match color {
        Some(c) => Some(format!("{:?}", c).to_string()),
        None => None,
    }
}

/// A wrapper around a [`protocol.ctl.NetProgress`] and [`CtlRequest`]. This type implements
/// traits for writing it's progress to the console.
pub struct NetProgressBar {
    inner: habitat_sup_protocol::ctl::NetProgress,
    req:   CtlRequest,
}

impl NetProgressBar {
    /// Create a new progress bar.
    pub fn new(req: CtlRequest) -> Self {
        NetProgressBar { inner: habitat_sup_protocol::ctl::NetProgress::default(),
                         req }
    }
}

impl DisplayProgress for NetProgressBar {
    fn size(&mut self, size: u64) { self.inner.total = size; }

    fn finish(&mut self) {}
}

impl io::Write for NetProgressBar {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.position += buf.len() as u64;
        self.req.reply_partial(self.inner.clone());
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

/// First attempts to read the secret key used to authenticate with the `CtlGateway` from disk
/// and, if not found, will generate a new key and write it to disk.
pub fn readgen_secret_key<T>(sup_root: T) -> Result<String>
    where T: AsRef<Path>
{
    let mut out = String::new();
    fs::create_dir_all(&sup_root).map_err(|e| {
                                     sup_error!(Error::CtlSecretIo(sup_root.as_ref().to_path_buf(),
                                                                   e))
                                 })?;
    if habitat_sup_protocol::read_secret_key(&sup_root, &mut out).ok()
                                                                 .unwrap_or(false)
    {
        Ok(out)
    } else {
        let secret_key_path = habitat_sup_protocol::secret_key_path(sup_root);
        {
            let mut f = File::create(&secret_key_path)?;
            habitat_sup_protocol::generate_secret_key(&mut out);
            f.write_all(out.as_bytes())?;
            f.sync_all()?;
        }
        set_permissions(&secret_key_path)?;
        Ok(out)
    }
}

#[cfg(not(windows))]
fn set_permissions<T: AsRef<Path>>(path: T) -> habitat_core::error::Result<()> {
    use habitat_core::util::posix_perm;

    posix_perm::set_permissions(path.as_ref(), CTL_SECRET_PERMISSIONS)
}

#[cfg(windows)]
fn set_permissions<T: AsRef<Path>>(path: T) -> habitat_core::error::Result<()> {
    use habitat_core::util::win_perm;

    win_perm::harden_path(path.as_ref())
}
