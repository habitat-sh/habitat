// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use std::env;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Stdout, Write};
use std::process::{self, Command};
use uuid::Uuid;

use ansi_term::Colour;
use api_client::DisplayProgress;
use pbr;
use term::terminfo::TermInfo;
use term::{Terminal, TerminfoTerminal};

use self::tty::StdStream;
use error::{Error, Result};

pub const NONINTERACTIVE_ENVVAR: &'static str = "HAB_NONINTERACTIVE";

pub const NOCOLORING_ENVVAR: &'static str = "HAB_NOCOLORING";

pub enum Status {
    Applying,
    Added,
    Adding,
    Cached,
    Canceled,
    Canceling,
    Created,
    Creating,
    Deleting,
    Deleted,
    Demoted,
    Demoting,
    Determining,
    Downloading,
    Encrypting,
    Encrypted,
    Found,
    Generated,
    Generating,
    Installed,
    Missing,
    Promoted,
    Promoting,
    Signed,
    Signing,
    Uploaded,
    Uploading,
    Using,
    Verified,
    Verifying,
    Custom(char, String),
}

impl Status {
    pub fn parts(&self) -> (char, String, Colour) {
        match *self {
            Status::Applying => ('↑', "Applying".into(), Colour::Green),
            Status::Added => ('↑', "Added".into(), Colour::Green),
            Status::Adding => ('☛', "Adding".into(), Colour::Green),
            Status::Canceled => ('✓', "Canceled".into(), Colour::Green),
            Status::Canceling => ('☛', "Canceling".into(), Colour::Green),
            Status::Cached => ('☑', "Cached".into(), Colour::Green),
            Status::Created => ('✓', "Created".into(), Colour::Green),
            Status::Creating => ('Ω', "Creating".into(), Colour::Green),
            Status::Deleted => ('✓', "Deleted".into(), Colour::Green),
            Status::Deleting => ('☒', "Deleting".into(), Colour::Green),
            Status::Demoted => ('✓', "Demoted".into(), Colour::Green),
            Status::Demoting => ('→', "Demoting".into(), Colour::Green),
            Status::Determining => ('☁', "Determining".into(), Colour::Green),
            Status::Downloading => ('↓', "Downloading".into(), Colour::Green),
            Status::Encrypting => ('☛', "Encrypting".into(), Colour::Green),
            Status::Encrypted => ('✓', "Encrypted".into(), Colour::Green),
            Status::Found => ('→', "Found".into(), Colour::Cyan),
            Status::Generated => ('→', "Generated".into(), Colour::Cyan),
            Status::Generating => ('☛', "Generating".into(), Colour::Green),
            Status::Installed => ('✓', "Installed".into(), Colour::Green),
            Status::Missing => ('∵', "Missing".into(), Colour::Red),
            Status::Promoted => ('✓', "Promoted".into(), Colour::Green),
            Status::Promoting => ('→', "Promoting".into(), Colour::Green),
            Status::Signed => ('✓', "Signed".into(), Colour::Cyan),
            Status::Signing => ('☛', "Signing".into(), Colour::Cyan),
            Status::Uploaded => ('✓', "Uploaded".into(), Colour::Green),
            Status::Uploading => ('↑', "Uploading".into(), Colour::Green),
            Status::Using => ('→', "Using".into(), Colour::Green),
            Status::Verified => ('✓', "Verified".into(), Colour::Green),
            Status::Verifying => ('☛', "Verifying".into(), Colour::Green),
            Status::Custom(c, ref s) => (c, s.to_string(), Colour::Green),
        }
    }
}

/// Functions applied to an IO stream for receiving input for a UI.
pub trait UIReader {
    fn edit<T>(&mut self, contents: &[T]) -> Result<String>
    where
        T: fmt::Display;
    /// Returns true if message reads should expect the source as a tty.
    fn is_a_tty(&self) -> bool;
    fn prompt_ask(&mut self, question: &str, default: Option<&str>) -> Result<String>;
    fn prompt_yes_no(&mut self, question: &str, default: Option<bool>) -> Result<bool>;
}

/// Functions applied to an IO stream for sending information to a UI.
pub trait UIWriter {
    type ProgressBar: DisplayProgress;

    /// IO Stream for sending error messages to.
    fn err(&mut self) -> &mut io::Write;
    /// IO Stream for sending normal or informational messages to.
    fn out(&mut self) -> &mut io::Write;
    /// Messages sent to the normal or informational IO stream will be formatted in color if true.
    fn is_out_colored(&self) -> bool;
    /// Messages sent to the error IO stream will be formatted in color if true.
    fn is_err_colored(&self) -> bool;
    /// Messages sent to the normal or informational IO stream will be formatted for a terminal if
    /// true.
    fn is_out_a_terminal(&self) -> bool;
    /// Messages sent to the error IO stream will be formatted for a terminal if true.
    fn is_err_a_terminal(&self) -> bool;
    /// Returns a progress bar widget implementation for writing operation's progress to.
    fn progress(&self) -> Option<Self::ProgressBar>;

    /// Write a message formatted with `begin`.
    fn begin<T>(&mut self, message: T) -> io::Result<()>
    where
        T: fmt::Display,
    {
        let symbol = '»';
        let formatted = if self.is_out_colored() {
            format!(
                "{}\n",
                Colour::Yellow
                    .bold()
                    .paint(format!("{} {}", symbol, message))
            )
        } else {
            format!("{} {}\n", symbol, message)
        };
        self.out().write(formatted.as_bytes())?;
        self.out().flush()
    }

    /// Write a message formatted with `end`.
    fn end<T>(&mut self, message: T) -> io::Result<()>
    where
        T: fmt::Display,
    {
        let symbol = '★';
        let formatted = if self.is_out_colored() {
            format!(
                "{}\n",
                Colour::Blue.bold().paint(format!("{} {}", symbol, message))
            )
        } else {
            format!("{} {}\n", symbol, message)
        };
        self.out().write(formatted.as_bytes())?;
        self.out().flush()
    }

    /// Write a message formatted with `status`.
    fn status<T>(&mut self, status: Status, message: T) -> io::Result<()>
    where
        T: fmt::Display,
    {
        let (symbol, status_str, color) = status.parts();
        let formatted = if self.is_out_colored() {
            format!(
                "{} {}\n",
                color.bold().paint(format!("{} {}", symbol, status_str)),
                message
            )
        } else {
            format!("{} {} {}\n", symbol, status_str, message)
        };
        self.out().write(formatted.as_bytes())?;
        self.out().flush()
    }

    /// Write a message formatted with `info`.
    fn info<T>(&mut self, text: T) -> io::Result<()>
    where
        T: fmt::Display,
    {
        self.out().write(format!("{}\n", text).as_bytes())?;
        self.out().flush()
    }

    /// Write a message formatted with `warn`.
    fn warn<T>(&mut self, message: T) -> io::Result<()>
    where
        T: fmt::Display,
    {
        let formatted = if self.is_err_colored() {
            format!(
                "{}\n",
                Colour::Yellow.bold().paint(format!("∅ {}", message))
            )
        } else {
            format!("∅ {}\n", message)
        };
        self.err().write(formatted.as_bytes())?;
        self.err().flush()
    }

    /// Write a message formatted with `fatal`.
    fn fatal<T>(&mut self, message: T) -> io::Result<()>
    where
        T: fmt::Display,
    {
        let color = Colour::Red;
        let formatted = if self.is_err_colored() {
            let mut buf = format!("{}\n", color.bold().paint("✗✗✗"));
            for line in message.to_string().lines() {
                buf.push_str(&format!(
                    "{}\n",
                    color.bold().paint(format!("✗✗✗ {}", line))
                ));
            }
            buf.push_str(&format!("{}\n", color.bold().paint("✗✗✗")));
            buf
        } else {
            let mut buf = format!("✗✗✗\n");
            for line in message.to_string().lines() {
                buf.push_str(&format!("✗✗✗ {}\n", line));
            }
            buf.push_str("✗✗✗\n");
            buf
        };
        self.err().write(formatted.as_bytes())?;
        self.err().flush()
    }

    /// Write a message formatted with `title`.
    fn title<T>(&mut self, text: T) -> io::Result<()>
    where
        T: AsRef<str>,
    {
        if self.is_out_colored() {
            write!(
                self.out(),
                "{}\n",
                Colour::Green.bold().paint(text.as_ref())
            )?;
            write!(
                self.out(),
                "{}\n\n",
                Colour::Green.bold().paint(format!(
                    "{:=<width$}",
                    "",
                    width = text.as_ref().chars().count()
                ))
            )?;
        } else {
            write!(self.out(), "{}\n", text.as_ref())?;
            write!(
                self.out(),
                "{}\n\n",
                format!("{:=<width$}", "", width = text.as_ref().chars().count())
            )?;
        }
        self.out().flush()
    }

    /// Write a message formatted with `heading`.
    fn heading<T>(&mut self, text: T) -> io::Result<()>
    where
        T: AsRef<str>,
    {
        let formatted = if self.is_out_colored() {
            format!("{}\n\n", Colour::Green.bold().paint(text.as_ref()))
        } else {
            format!("{}\n\n", text.as_ref())
        };
        self.out().write(formatted.as_bytes())?;
        self.out().flush()
    }

    /// Write a message formatted with `para`.
    fn para(&mut self, text: &str) -> io::Result<()> {
        print_wrapped(self.out(), text, 75, 2)
    }

    /// Write a line break message`.
    fn br(&mut self) -> io::Result<()> {
        self.out().write(b"\n")?;
        self.out().flush()
    }
}

/// Console (shell) backed UI.
#[derive(Debug)]
pub struct UI {
    shell: Shell,
}

impl UI {
    /// Creates a new `UI` from a `Shell`.
    pub fn new(shell: Shell) -> Self {
        UI { shell: shell }
    }

    /// Creates a new default `UI` with a coloring strategy and tty hinting.
    pub fn default_with(coloring: Coloring, isatty: Option<bool>) -> Self {
        Self::new(Shell::default_with(coloring, isatty))
    }

    /// Creates a new default `UI` with a coloring strategy and tty hinting.
    pub fn default_with_env() -> Self {
        let isatty = if env::var(NONINTERACTIVE_ENVVAR)
            // Keep string boolean for backwards-compatibility
            .map(|val| val == "1" || val == "true")
            .unwrap_or(false)
        {
            Some(false)
        } else {
            None
        };
        let coloring = if env::var(NOCOLORING_ENVVAR)
            .map(|val| val == "1" || val == "true")
            .unwrap_or(false)
        {
            Coloring::Never
        } else {
            Coloring::Auto
        };

        let ui = UI::default_with(coloring, isatty);
        debug!("{:?}", &ui);
        ui
    }

    /// Creates a new `UI` from generic `Read` and `Write` streams.
    ///
    /// The standard input stream needs to implement `Read` and both the standard output and
    /// standard error streams need to implement `Write`.
    pub fn with_streams<O, E>(
        stdin: Box<Read + Send>,
        stdout_fn: O,
        stderr_fn: E,
        coloring: Coloring,
        isatty: bool,
    ) -> Self
    where
        O: FnMut() -> Box<Write + Send>,
        E: FnMut() -> Box<Write + Send>,
    {
        Self::new(Shell::new(
            InputStream::new(stdin, isatty),
            OutputStream::new(WriteStream::create(stdout_fn), coloring, isatty),
            OutputStream::new(WriteStream::create(stderr_fn), coloring, isatty),
        ))
    }

    /// Creates a new `UI` which an empty standard input and sinks (i.e. a `/dev/null`-like stream)
    /// for standard output and standard error.
    pub fn with_sinks() -> Self {
        Self::with_streams(
            Box::new(io::empty()),
            || Box::new(io::sink()),
            || Box::new(io::sink()),
            Coloring::Never,
            false,
        )
    }
}

impl Default for UI {
    fn default() -> Self {
        UI::default_with(Coloring::Auto, None)
    }
}

impl UIWriter for UI {
    type ProgressBar = ConsoleProgressBar;

    fn out(&mut self) -> &mut io::Write {
        &mut self.shell.out
    }

    fn err(&mut self) -> &mut io::Write {
        &mut self.shell.err
    }

    fn is_out_colored(&self) -> bool {
        self.shell.out.is_colored()
    }

    fn is_err_colored(&self) -> bool {
        self.shell.err.is_colored()
    }

    fn is_out_a_terminal(&self) -> bool {
        self.shell.out.is_a_terminal()
    }

    fn is_err_a_terminal(&self) -> bool {
        self.shell.err.is_a_terminal()
    }

    fn progress(&self) -> Option<Self::ProgressBar> {
        if self.is_out_a_terminal() {
            Some(Self::ProgressBar::default())
        } else {
            None
        }
    }
}

impl UIReader for UI {
    fn is_a_tty(&self) -> bool {
        self.shell.input.isatty && self.shell.out.isatty && self.shell.err.isatty
    }

    fn prompt_yes_no(&mut self, question: &str, default: Option<bool>) -> Result<bool> {
        let ref mut stream = self.shell.out;
        let choice = match default {
            Some(yes) => {
                if yes {
                    match stream.is_colored() {
                        true => format!(
                            "{}{}{}",
                            Colour::White.paint("["),
                            Colour::White.bold().paint("Yes"),
                            Colour::White.paint("/no/quit]")
                        ),
                        false => format!("[Yes/no/quit]"),
                    }
                } else {
                    match stream.is_colored() {
                        true => format!(
                            "{}{}{}",
                            Colour::White.paint("[yes/"),
                            Colour::White.bold().paint("No"),
                            Colour::White.paint("/quit]")
                        ),
                        false => format!("[yes/No/quit]"),
                    }
                }
            }
            None => match stream.is_colored() {
                true => format!("{}", Colour::White.paint("[yes/no/quit]")),
                false => format!("[yes/no/quit]"),
            },
        };
        loop {
            stream.flush()?;
            match stream.is_colored() {
                true => {
                    write!(stream, "{} {} ", Colour::Cyan.paint(question), choice)?;
                }
                false => {
                    write!(stream, "{} {} ", question, choice)?;
                }
            }
            stream.flush()?;
            let mut response = String::new();
            {
                let reference = self.shell.input.by_ref();
                BufReader::new(reference).read_line(&mut response)?;
            }
            match response.trim().chars().next().unwrap_or('\n') {
                'y' | 'Y' => return Ok(true),
                'n' | 'N' => return Ok(false),
                'q' | 'Q' => process::exit(0),
                '\n' => match default {
                    Some(default) => return Ok(default),
                    None => continue,
                },
                _ => continue,
            }
        }
    }

    fn prompt_ask(&mut self, question: &str, default: Option<&str>) -> Result<String> {
        let ref mut stream = self.shell.out;
        let choice = match default {
            Some(d) => match stream.is_colored() {
                true => format!(
                    " {}{}{}",
                    Colour::White.paint("[default: "),
                    Colour::White.bold().paint(d),
                    Colour::White.paint("]")
                ),
                false => format!(" [default: {}]", d),
            },
            None => "".to_string(),
        };
        loop {
            stream.flush()?;
            match stream.is_colored() {
                true => {
                    write!(
                        stream,
                        "{}{} ",
                        Colour::Cyan.paint(format!("{}:", question)),
                        choice
                    )?;
                }
                false => {
                    write!(stream, "{}{} ", format!("{}:", question), choice)?;
                }
            }
            stream.flush()?;
            let mut response = String::new();
            {
                let reference = self.shell.input.by_ref();
                BufReader::new(reference).read_line(&mut response)?;
            }
            if response.trim().is_empty() {
                match default {
                    Some(d) => return Ok(d.to_string()),
                    None => continue,
                }
            }
            return Ok(response.trim().to_string());
        }
    }

    fn edit<T>(&mut self, contents: &[T]) -> Result<String>
    where
        T: fmt::Display,
    {
        let editor = env::var("EDITOR").map_err(|e| Error::EditorEnv(e))?;

        let mut tmp_file_path = env::temp_dir();
        tmp_file_path.push(format!("_hab_{}.tmp", Uuid::new_v4()));

        let mut tmp_file = File::create(&tmp_file_path)?;

        if contents.len() > 0 {
            for line in contents {
                write!(tmp_file, "{}", line)?;
            }
            tmp_file.sync_all()?;
        }

        let mut cmd = Command::new(editor);
        cmd.arg(tmp_file_path.display().to_string());
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            debug!("Failed edit with status: {:?}", status);
            return Err(Error::EditStatus);
        }

        let mut out = String::new();
        tmp_file = File::open(&tmp_file_path)?;
        tmp_file.read_to_string(&mut out)?;

        fs::remove_file(tmp_file_path)?;

        Ok(out)
    }
}

#[derive(Debug)]
pub struct Shell {
    input: InputStream,
    out: OutputStream,
    err: OutputStream,
}

impl Shell {
    pub fn new(input: InputStream, out: OutputStream, err: OutputStream) -> Self {
        Shell {
            input: input,
            out: out,
            err: err,
        }
    }

    pub fn default_with(coloring: Coloring, isatty: Option<bool>) -> Self {
        let stdin = InputStream::from_stdin(isatty);
        let stdout = OutputStream::from_stdout(coloring, isatty);
        let stderr = OutputStream::from_stderr(coloring, isatty);
        Shell::new(stdin, stdout, stderr)
    }

    pub fn input(&mut self) -> &mut InputStream {
        &mut self.input
    }

    pub fn out(&mut self) -> &mut OutputStream {
        &mut self.out
    }

    pub fn err(&mut self) -> &mut OutputStream {
        &mut self.err
    }
}

impl Default for Shell {
    fn default() -> Self {
        Shell::default_with(Coloring::Auto, None)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Coloring {
    Auto,
    Always,
    Never,
}

pub struct InputStream {
    inner: Box<Read + Send>,
    isatty: bool,
}

impl InputStream {
    pub fn new(inner: Box<Read + Send>, isatty: bool) -> Self {
        InputStream {
            inner: inner,
            isatty: isatty,
        }
    }

    pub fn from_stdin(isatty: Option<bool>) -> Self {
        Self::new(
            Box::new(io::stdin()),
            match isatty {
                Some(val) => val,
                None => tty::isatty(StdStream::Stdin),
            },
        )
    }

    pub fn is_a_terminal(&self) -> bool {
        self.isatty
    }
}

impl Read for InputStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl fmt::Debug for InputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InputStream {{ isatty: {} }}", self.isatty)
    }
}

pub struct OutputStream {
    inner: WriteStream,
    coloring: Coloring,
    isatty: bool,
}

impl OutputStream {
    pub fn new(inner: WriteStream, coloring: Coloring, isatty: bool) -> Self {
        OutputStream {
            inner: inner,
            coloring: coloring,
            isatty: isatty,
        }
    }

    pub fn from_stdout(coloring: Coloring, isatty: Option<bool>) -> Self {
        Self::new(
            WriteStream::create(|| Box::new(io::stdout())),
            coloring,
            match isatty {
                Some(val) => val,
                None => tty::isatty(StdStream::Stdout),
            },
        )
    }

    pub fn from_stderr(coloring: Coloring, isatty: Option<bool>) -> Self {
        Self::new(
            WriteStream::create(|| Box::new(io::stderr())),
            coloring,
            match isatty {
                Some(val) => val,
                None => tty::isatty(StdStream::Stderr),
            },
        )
    }

    pub fn supports_color(&self) -> bool {
        match self.inner {
            WriteStream::Color(_) => true,
            WriteStream::NoColor(_) => false,
        }
    }

    pub fn is_colored(&self) -> bool {
        self.supports_color()
            && (Coloring::Auto == self.coloring || Coloring::Always == self.coloring)
    }

    pub fn is_a_terminal(&self) -> bool {
        self.isatty
    }
}

impl Write for OutputStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.inner {
            WriteStream::Color(ref mut io) => io.write(buf),
            WriteStream::NoColor(ref mut io) => io.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self.inner {
            WriteStream::Color(ref mut io) => io.flush(),
            WriteStream::NoColor(ref mut io) => io.flush(),
        }
    }
}

impl fmt::Debug for OutputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "OutputStream {{ coloring: {:?}, isatty: {}, is_colored(): {}, supports_color(): {} }}",
            self.coloring,
            self.isatty,
            self.is_colored(),
            self.supports_color(),
        )
    }
}

pub enum WriteStream {
    NoColor(Box<Write + Send>),
    Color(Box<Terminal<Output = Box<Write + Send>> + Send>),
}

impl WriteStream {
    // Implementation heavily inspired and based on the Cargo `shell.rs` implementation. Source:
    // https://github.com/rust-lang/cargo/blob/d05ba53afec82308edcfeb778446010bf18e71ae/
    // src/cargo/core/shell.rs

    pub fn create<T: FnMut() -> Box<Write + Send>>(mut writable_fn: T) -> Self {
        match Self::get_term(writable_fn()) {
            Ok(t) => t,
            Err(_) => WriteStream::NoColor(writable_fn()),
        }
    }

    #[cfg(any(windows))]
    fn get_term(writeable: Box<Write + Send>) -> Result<Self> {
        // Check if the creation of a console will succeed
        if ::term::WinConsole::new(vec![0u8; 0]).is_ok() {
            let term = ::term::WinConsole::new(writeable)?;
            if !term.supports_color() {
                Ok(WriteStream::NoColor(Box::new(term)))
            } else {
                Ok(WriteStream::Color(Box::new(term)))
            }
        } else {
            // If we fail to get a windows console, we try to get a `TermInfo` one
            Ok(Self::get_terminfo_term(writeable))
        }
    }

    #[cfg(any(unix))]
    fn get_term(writeable: Box<Write + Send>) -> Result<Self> {
        Ok(Self::get_terminfo_term(writeable))
    }

    fn get_terminfo_term(writeable: Box<Write + Send>) -> Self {
        // Use `TermInfo::from_env()` and `TerminfoTerminal::supports_color()` to determine if
        // creation of a TerminfoTerminal is possible regardless of the tty status. --color options
        // are parsed after Shell creation so always try to create a terminal that supports color
        // output. Fall back to a no-color terminal regardless of whether or not a tty is present
        // and if color output is not possible.
        match TermInfo::from_env() {
            Ok(info) => {
                let term = TerminfoTerminal::new_with_terminfo(writeable, info);
                if !term.supports_color() {
                    WriteStream::NoColor(term.into_inner())
                } else {
                    WriteStream::Color(Box::new(term))
                }
            }
            Err(_) => WriteStream::NoColor(writeable),
        }
    }
}

mod tty {
    pub enum StdStream {
        Stdin,
        Stdout,
        Stderr,
    }

    #[cfg(unix)]
    pub fn isatty(output: StdStream) -> bool {
        extern crate libc;

        let fd = match output {
            StdStream::Stdin => libc::STDIN_FILENO,
            StdStream::Stdout => libc::STDOUT_FILENO,
            StdStream::Stderr => libc::STDERR_FILENO,
        };

        unsafe { libc::isatty(fd) != 0 }
    }
    #[cfg(windows)]
    pub fn isatty(output: StdStream) -> bool {
        extern crate kernel32;
        extern crate winapi;

        let handle = match output {
            StdStream::Stdin => winapi::winbase::STD_INPUT_HANDLE,
            StdStream::Stdout => winapi::winbase::STD_OUTPUT_HANDLE,
            StdStream::Stderr => winapi::winbase::STD_ERROR_HANDLE,
        };

        unsafe {
            let handle = kernel32::GetStdHandle(handle);
            let mut out = 0;
            kernel32::GetConsoleMode(handle, &mut out) != 0
        }
    }
}

/// A moving progress bar to track progress of a sized event, similar to wget, curl, npm, etc.
///
/// This is designed to satisfy a generic behavior which sets the size of the task (usually a
/// number of bytes representing the total download/upload/transfer size) and will be a generic
/// writer (i.e. implementing the `Write` trait) as a means to increase progress towards
/// completion.
pub struct ConsoleProgressBar {
    bar: pbr::ProgressBar<Stdout>,
    total: u64,
    current: u64,
}

impl Default for ConsoleProgressBar {
    fn default() -> Self {
        ConsoleProgressBar {
            bar: pbr::ProgressBar::new(0),
            total: 0,
            current: 0,
        }
    }
}

impl DisplayProgress for ConsoleProgressBar {
    fn size(&mut self, size: u64) {
        self.bar = pbr::ProgressBar::new(size);
        self.bar.set_units(pbr::Units::Bytes);
        self.bar.show_tick = true;
        self.bar.message("    ");
        self.total = size;
    }

    fn finish(&mut self) {
        println!("");
        io::stdout().flush().ok().expect("flush() fail");
    }
}

impl Write for ConsoleProgressBar {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.bar.write(buf) {
            Ok(n) => {
                self.current += n as u64;
                if self.current == self.total {
                    self.finish();
                }
                Ok(n)
            }
            Err(e) => Err(e),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        self.bar.flush()
    }
}

pub fn print_wrapped<T, U>(
    mut stream: T,
    text: U,
    wrap_width: usize,
    left_indent: usize,
) -> io::Result<()>
where
    T: io::Write,
    U: AsRef<str>,
{
    for line in text.as_ref().split("\n\n") {
        let mut buffer = String::new();
        let mut width = 0;
        for word in line.split_whitespace() {
            let wl = word.chars().count();
            if (width + wl + 1) > (wrap_width - left_indent) {
                write!(stream, "{:<width$}{}\n", " ", buffer, width = left_indent)?;
                buffer.clear();
                width = 0;
            }
            width = width + wl + 1;
            buffer.push_str(word);
            buffer.push(' ');
        }
        if !buffer.is_empty() {
            write!(stream, "{:<width$}{}\n", " ", buffer, width = left_indent)?;
        }
        write!(stream, "\n")?;
    }
    stream.flush()
}
