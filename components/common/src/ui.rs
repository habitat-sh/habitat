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

use std::{env,
          fmt,
          fs::{self,
               File},
          io::{self,
               BufRead,
               BufReader,
               Read,
               Stdout,
               Write},
          process::{self,
                    Command},
          str::FromStr};
use uuid::Uuid;

use crate::api_client::DisplayProgress;
use pbr;
use termcolor::{self,
                ColorChoice,
                ColorSpec,
                StandardStream,
                WriteColor};

use self::tty::StdStream;
use crate::error::{Error,
                   Result};

pub const NONINTERACTIVE_ENVVAR: &str = "HAB_NONINTERACTIVE";

pub const NOCOLORING_ENVVAR: &str = "HAB_NOCOLORING";

pub const GLYPH_STYLE_ENVVAR: &str = "HAB_GLYPH_STYLE";

#[derive(Clone, Copy)]
pub enum Color {
    Plain,
    Info,
    Important,
    Warn,
    Critical,
    End,
}

impl From<Color> for termcolor::Color {
    fn from(color: Color) -> Self {
        match color {
            Color::Plain => termcolor::Color::White,
            Color::Info => termcolor::Color::Green,
            Color::Important => termcolor::Color::Cyan,
            Color::Critical => termcolor::Color::Red,
            Color::End => termcolor::Color::Magenta,
            Color::Warn => termcolor::Color::Yellow,
        }
    }
}

#[derive(Clone, Copy)]
pub enum GlyphStyle {
    Full,
    Limited,
    Ascii,
}

impl Default for GlyphStyle {
    #[cfg(windows)]
    fn default() -> GlyphStyle { GlyphStyle::Limited }

    #[cfg(unix)]
    fn default() -> GlyphStyle { GlyphStyle::Full }
}

impl FromStr for GlyphStyle {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        match value.to_lowercase().as_ref() {
            "full" => Ok(GlyphStyle::Full),
            "limited" => Ok(GlyphStyle::Limited),
            "ascii" => Ok(GlyphStyle::Ascii),
            _ => Err(Error::BadGlyphStyle(value.to_string())),
        }
    }
}

impl fmt::Display for GlyphStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match *self {
            GlyphStyle::Full => "full",
            GlyphStyle::Limited => "limited",
            GlyphStyle::Ascii => "ascii",
        };
        write!(f, "{}", msg)
    }
}

#[derive(Clone, Copy)]
pub enum Glyph {
    UpArrow,
    FingerPoint,
    CheckMark,
    BoxedCheckMark,
    Omega,
    BoxedX,
    RightArrow,
    Cloud,
    DownArrow,
    Elipses,
    Because,
    RightShift,
    Star,
    SlashedZero,
    ErrorX,
}

impl Glyph {
    pub fn to_str(&self) -> &str {
        let style = if let Ok(s) = env::var(GLYPH_STYLE_ENVVAR) {
            match GlyphStyle::from_str(&s) {
                Ok(style) => style,
                Err(e) => {
                    eprintln!("{}\nSetting GlyphStyle to {}", e, GlyphStyle::default());
                    GlyphStyle::default()
                }
            }
        } else {
            GlyphStyle::default()
        };

        match style {
            GlyphStyle::Ascii => {
                match *self {
                    Glyph::UpArrow => "/^\\",
                    Glyph::FingerPoint => "-->",
                    Glyph::CheckMark => "[x]",
                    Glyph::BoxedCheckMark => "[#]",
                    Glyph::Omega => "-->",
                    Glyph::BoxedX => "-X-",
                    Glyph::RightArrow => "-->",
                    Glyph::Cloud => "-->",
                    Glyph::DownArrow => "  >",
                    Glyph::Elipses => "...",
                    Glyph::Because => "???",
                    Glyph::RightShift => " >>",
                    Glyph::Star => "***",
                    Glyph::SlashedZero => "  0",
                    Glyph::ErrorX => "XXX",
                }
            }
            GlyphStyle::Limited => {
                match *self {
                    Glyph::UpArrow => "↑",
                    Glyph::FingerPoint => "→",
                    Glyph::CheckMark => "√",
                    Glyph::BoxedCheckMark => "⌂",
                    Glyph::Omega => "Ω",
                    Glyph::BoxedX => "░",
                    Glyph::RightArrow => "→",
                    Glyph::Cloud => "⌂",
                    Glyph::DownArrow => "↓",
                    Glyph::Elipses => "…",
                    Glyph::Because => "‼",
                    Glyph::RightShift => "»",
                    Glyph::Star => "≡",
                    Glyph::SlashedZero => "Ø",
                    Glyph::ErrorX => "XXX",
                }
            }
            GlyphStyle::Full => {
                match *self {
                    Glyph::UpArrow => "↑",
                    Glyph::FingerPoint => "☛",
                    Glyph::CheckMark => "✓",
                    Glyph::BoxedCheckMark => "☑",
                    Glyph::Omega => "Ω",
                    Glyph::BoxedX => "☒",
                    Glyph::RightArrow => "→",
                    Glyph::Cloud => "☁",
                    Glyph::DownArrow => "↓",
                    Glyph::Elipses => "…",
                    Glyph::Because => "∵",
                    Glyph::RightShift => "»",
                    Glyph::Star => "★",
                    Glyph::SlashedZero => "Ø",
                    Glyph::ErrorX => "✗✗✗",
                }
            }
        }
    }
}

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
    DryRunDeleting,
    Encrypting,
    Encrypted,
    Executing,
    Found,
    Generated,
    Generating,
    Installed,
    Missing,
    Promoted,
    Promoting,
    Signed,
    Signing,
    Skipping,
    Uploaded,
    Uploading,
    Using,
    Verified,
    Verifying,
    Custom(Glyph, String),
}

impl Status {
    pub fn parts(&self) -> (Glyph, String, Color) {
        match *self {
            Status::Applying => (Glyph::UpArrow, "Applying".into(), Color::Info),
            Status::Added => (Glyph::UpArrow, "Added".into(), Color::Info),
            Status::Adding => (Glyph::FingerPoint, "Adding".into(), Color::Info),
            Status::Canceled => (Glyph::CheckMark, "Canceled".into(), Color::Info),
            Status::Canceling => (Glyph::FingerPoint, "Canceling".into(), Color::Info),
            Status::Cached => (Glyph::BoxedCheckMark, "Cached".into(), Color::Info),
            Status::Created => (Glyph::CheckMark, "Created".into(), Color::Info),
            Status::Creating => (Glyph::Omega, "Creating".into(), Color::Info),
            Status::Deleted => (Glyph::CheckMark, "Deleted".into(), Color::Info),
            Status::Deleting => (Glyph::BoxedX, "Deleting".into(), Color::Info),
            Status::Demoted => (Glyph::CheckMark, "Demoted".into(), Color::Info),
            Status::Demoting => (Glyph::RightArrow, "Demoting".into(), Color::Info),
            Status::Determining => (Glyph::Cloud, "Determining".into(), Color::Info),
            Status::Downloading => (Glyph::DownArrow, "Downloading".into(), Color::Info),
            Status::DryRunDeleting => {
                (Glyph::BoxedX, "Would be deleted (Dry run)".into(), Color::Critical)
            }
            Status::Encrypting => (Glyph::FingerPoint, "Encrypting".into(), Color::Info),
            Status::Encrypted => (Glyph::CheckMark, "Encrypted".into(), Color::Info),
            Status::Executing => (Glyph::FingerPoint, "Executing".into(), Color::Info),
            Status::Found => (Glyph::RightArrow, "Found".into(), Color::Important),
            Status::Generated => (Glyph::RightArrow, "Generated".into(), Color::Important),
            Status::Generating => (Glyph::FingerPoint, "Generating".into(), Color::Info),
            Status::Installed => (Glyph::CheckMark, "Installed".into(), Color::Info),
            Status::Missing => (Glyph::Because, "Missing".into(), Color::Critical),
            Status::Promoted => (Glyph::CheckMark, "Promoted".into(), Color::Info),
            Status::Promoting => (Glyph::RightArrow, "Promoting".into(), Color::Info),
            Status::Signed => (Glyph::CheckMark, "Signed".into(), Color::Important),
            Status::Signing => (Glyph::FingerPoint, "Signing".into(), Color::Important),
            Status::Skipping => (Glyph::Elipses, "Skipping".into(), Color::Info),
            Status::Uploaded => (Glyph::CheckMark, "Uploaded".into(), Color::Info),
            Status::Uploading => (Glyph::UpArrow, "Uploading".into(), Color::Info),
            Status::Using => (Glyph::RightArrow, "Using".into(), Color::Info),
            Status::Verified => (Glyph::CheckMark, "Verified".into(), Color::Info),
            Status::Verifying => (Glyph::FingerPoint, "Verifying".into(), Color::Info),
            Status::Custom(c, ref s) => (c, s.to_string(), Color::Info),
        }
    }
}

/// Functions applied to an IO stream for receiving input for a UI.
pub trait UIReader {
    fn edit<T>(&mut self, contents: &[T]) -> Result<String>
        where T: fmt::Display;
    /// Returns true if message reads should expect the source as a tty.
    fn is_a_tty(&self) -> bool;
    fn prompt_ask(&mut self, question: &str, default: Option<&str>) -> Result<String>;
    fn prompt_yes_no(&mut self, question: &str, default: Option<bool>) -> Result<bool>;
}

/// Functions applied to an IO stream for sending information to a UI.
pub trait UIWriter {
    type ProgressBar: DisplayProgress;

    /// IO Stream for sending error messages to.
    fn err(&mut self) -> &mut dyn WriteColor;
    /// IO Stream for sending normal or informational messages to.
    fn out(&mut self) -> &mut dyn WriteColor;

    /// Messages sent to the normal or informational IO stream will be formatted for a terminal if
    /// true.
    fn is_out_a_terminal(&self) -> bool;
    /// Messages sent to the error IO stream will be formatted for a terminal if true.
    fn is_err_a_terminal(&self) -> bool;
    /// Returns a progress bar widget implementation for writing operation's progress to.
    fn progress(&self) -> Option<Self::ProgressBar>;

    /// Write a message formatted with `begin`.
    fn begin<T>(&mut self, message: T) -> io::Result<()>
        where T: fmt::Display
    {
        let symbol = Glyph::RightShift.to_str();
        println(self.out(),
                format!("{} {}", symbol, message).as_bytes(),
                ColorSpec::new().set_fg(Some(Color::Warn.into()))
                                .set_bold(true))
    }

    /// Write a message formatted with `end`.
    fn end<T>(&mut self, message: T) -> io::Result<()>
        where T: fmt::Display
    {
        let symbol = Glyph::Star.to_str();
        println(self.out(),
                format!("{} {}", symbol, message).as_bytes(),
                ColorSpec::new().set_fg(Some(Color::End.into()))
                                .set_bold(true))
    }

    /// Write a message formatted with `status`.
    fn status<T>(&mut self, status: Status, message: T) -> io::Result<()>
        where T: fmt::Display
    {
        let (symbol, status_str, color) = status.parts();
        print(self.out(),
              format!("{} {}", symbol.to_str(), status_str).as_bytes(),
              ColorSpec::new().set_fg(Some(color.into())).set_bold(true))?;
        self.out().write_all(format!(" {}\n", message).as_bytes())?;
        self.out().flush()
    }

    /// Write a message formatted with `info`.
    fn info<T>(&mut self, text: T) -> io::Result<()>
        where T: fmt::Display
    {
        self.out().write_all(format!("{}\n", text).as_bytes())?;
        self.out().flush()
    }

    /// Write a message formatted with `warn`.
    fn warn<T>(&mut self, message: T) -> io::Result<()>
        where T: fmt::Display
    {
        println(self.err(),
                format!("{} {}", Glyph::SlashedZero.to_str(), message).as_bytes(),
                ColorSpec::new().set_fg(Some(Color::Warn.into()))
                                .set_bold(true))
    }

    /// Write a message formatted with `fatal`.
    fn fatal<T>(&mut self, message: T) -> io::Result<()>
        where T: fmt::Display
    {
        println(self.err(),
                Glyph::ErrorX.to_str().as_bytes(),
                ColorSpec::new().set_fg(Some(Color::Critical.into()))
                                .set_bold(true))?;
        for line in message.to_string().lines() {
            println(self.err(),
                    format!("{} {}", Glyph::ErrorX.to_str(), line).as_bytes(),
                    ColorSpec::new().set_fg(Some(Color::Critical.into()))
                                    .set_bold(true))?;
        }
        println(self.err(),
                Glyph::ErrorX.to_str().as_bytes(),
                ColorSpec::new().set_fg(Some(Color::Critical.into()))
                                .set_bold(true))
    }

    /// Write a message formatted with `title`.
    fn title<T>(&mut self, text: T) -> io::Result<()>
        where T: AsRef<str>
    {
        println(self.out(),
                format!("{}\n{:=<width$}\n",
                        text.as_ref(),
                        "",
                        width = text.as_ref().chars().count()).as_bytes(),
                ColorSpec::new().set_fg(Some(Color::Info.into()))
                                .set_bold(true))
    }

    /// Write a message formatted with `heading`.
    fn heading<T>(&mut self, text: T) -> io::Result<()>
        where T: AsRef<str>
    {
        println(self.out(),
                format!("{}\n", text.as_ref()).as_bytes(),
                ColorSpec::new().set_fg(Some(Color::Info.into()))
                                .set_bold(true))
    }

    /// Write a message formatted with `para`.
    fn para(&mut self, text: &str) -> io::Result<()> { print_wrapped(self.out(), text, 75, 2) }

    /// Write a line break message`.
    fn br(&mut self) -> io::Result<()> {
        self.out().write_all(b"\n")?;
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
    pub fn new(shell: Shell) -> Self { UI { shell } }

    /// Creates a new default `UI` with a coloring strategy and tty hinting.
    pub fn default_with(coloring: ColorChoice, isatty: Option<bool>) -> Self {
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
        let coloring = if env::var(NOCOLORING_ENVVAR).map(|val| val == "1" || val == "true")
                                                     .unwrap_or(false)
        {
            ColorChoice::Never
        } else {
            ColorChoice::Auto
        };

        let ui = UI::default_with(coloring, isatty);
        debug!("{:?}", &ui);
        ui
    }

    /// Creates a new `UI` from generic `Read` and `Write` streams.
    ///
    /// The standard input stream needs to implement `Read` and both the standard output and
    /// standard error streams need to implement `Write`.
    pub fn with_streams<O, E>(stdin: Box<dyn Read + Send>,
                              stdout_fn: O,
                              stderr_fn: E,
                              coloring: ColorChoice,
                              isatty: bool)
                              -> Self
        where O: FnMut() -> Box<dyn Write + Send>,
              E: FnMut() -> Box<dyn Write + Send>
    {
        Self::new(Shell::new(InputStream::new(stdin, isatty),
                             OutputStream::new(WriteStream::from_write(stdout_fn),
                                               coloring,
                                               isatty),
                             OutputStream::new(WriteStream::from_write(stderr_fn),
                                               coloring,
                                               isatty)))
    }

    /// Creates a new `UI` which an empty standard input and sinks (i.e. a `/dev/null`-like stream)
    /// for standard output and standard error.
    pub fn with_sinks() -> Self {
        Self::with_streams(Box::new(io::empty()),
                           || Box::new(io::sink()),
                           || Box::new(io::sink()),
                           ColorChoice::Never,
                           false)
    }
}

impl Default for UI {
    fn default() -> Self { UI::default_with(ColorChoice::Auto, None) }
}

impl UIWriter for UI {
    type ProgressBar = ConsoleProgressBar;

    fn out(&mut self) -> &mut dyn WriteColor { &mut self.shell.out }

    fn err(&mut self) -> &mut dyn WriteColor { &mut self.shell.err }

    fn is_out_a_terminal(&self) -> bool { self.shell.out.is_a_terminal() }

    fn is_err_a_terminal(&self) -> bool { self.shell.err.is_a_terminal() }

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
        let stream = &mut self.shell.out;
        let (prefix, default_text, suffix) = match default {
            Some(true) => ("[", "Yes", "/no/quit]"),
            Some(false) => ("[yes/", "No", "/quit]"),
            None => ("[yes/no/quit]", "", ""),
        };
        loop {
            print(stream,
                  question.as_bytes(),
                  ColorSpec::new().set_fg(Some(Color::Important.into())))?;
            print(stream,
                  format!(" {}", prefix).as_bytes(),
                  ColorSpec::new().set_fg(Some(Color::Plain.into())))?;
            print(stream,
                  default_text.as_bytes(),
                  ColorSpec::new().set_fg(Some(Color::Plain.into()))
                                  .set_bold(true))?;
            print(stream,
                  format!("{} ", suffix).as_bytes(),
                  ColorSpec::new().set_fg(Some(Color::Plain.into())))?;
            let mut response = String::new();
            {
                let reference = self.shell.input.by_ref();
                BufReader::new(reference).read_line(&mut response)?;
            }
            match response.trim().chars().next().unwrap_or('\n') {
                'y' | 'Y' => return Ok(true),
                'n' | 'N' => return Ok(false),
                'q' | 'Q' => process::exit(0),
                '\n' => {
                    match default {
                        Some(default) => return Ok(default),
                        None => continue,
                    }
                }
                _ => continue,
            }
        }
    }

    fn prompt_ask(&mut self, question: &str, default: Option<&str>) -> Result<String> {
        let stream = &mut self.shell.out;
        loop {
            print(stream,
                  question.as_bytes(),
                  ColorSpec::new().set_fg(Some(Color::Important.into())))?;
            stream.write_all(b": ")?;
            if let Some(d) = default {
                print(stream,
                      b"[default: ",
                      ColorSpec::new().set_fg(Some(Color::Plain.into())))?;
                print(stream,
                      d.as_bytes(),
                      ColorSpec::new().set_fg(Some(Color::Plain.into()))
                                      .set_bold(true))?;
                print(stream,
                      b"]",
                      ColorSpec::new().set_fg(Some(Color::Plain.into())))?;
            }
            stream.write_all(b" ")?;
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
        where T: fmt::Display
    {
        let editor = env::var("EDITOR").map_err(Error::EditorEnv)?;

        let mut tmp_file_path = env::temp_dir();
        tmp_file_path.push(format!("_hab_{}.tmp", Uuid::new_v4()));

        let mut tmp_file = File::create(&tmp_file_path)?;

        if !contents.is_empty() {
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
    out:   OutputStream,
    err:   OutputStream,
}

impl Shell {
    pub fn new(input: InputStream, out: OutputStream, err: OutputStream) -> Self {
        Shell { input, out, err }
    }

    pub fn default_with(coloring: ColorChoice, isatty: Option<bool>) -> Self {
        let stdin = InputStream::from_stdin(isatty);
        let stdout = OutputStream::from_stdout(coloring, isatty);
        let stderr = OutputStream::from_stderr(coloring, isatty);
        Shell::new(stdin, stdout, stderr)
    }

    pub fn input(&mut self) -> &mut InputStream { &mut self.input }

    pub fn out(&mut self) -> &mut OutputStream { &mut self.out }

    pub fn err(&mut self) -> &mut OutputStream { &mut self.err }
}

impl Default for Shell {
    fn default() -> Self { Shell::default_with(ColorChoice::Auto, None) }
}

pub struct InputStream {
    inner:  Box<dyn Read + Send>,
    isatty: bool,
}

impl InputStream {
    pub fn new(inner: Box<dyn Read + Send>, isatty: bool) -> Self { InputStream { inner, isatty } }

    pub fn from_stdin(isatty: Option<bool>) -> Self {
        Self::new(Box::new(io::stdin()), match isatty {
            Some(val) => val,
            None => tty::isatty(StdStream::Stdin),
        })
    }

    pub fn is_a_terminal(&self) -> bool { self.isatty }
}

impl Read for InputStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { self.inner.read(buf) }
}

impl fmt::Debug for InputStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InputStream {{ isatty: {} }}", self.isatty)
    }
}

pub struct OutputStream {
    inner:    WriteStream,
    coloring: ColorChoice,
    isatty:   bool,
}

impl OutputStream {
    pub fn new(inner: WriteStream, coloring: ColorChoice, isatty: bool) -> Self {
        OutputStream { inner,
                       coloring,
                       isatty }
    }

    pub fn from_stdout(coloring: ColorChoice, isatty: Option<bool>) -> Self {
        Self::new(WriteStream::from_stdout(coloring), coloring, match isatty {
            Some(val) => val,
            None => tty::isatty(StdStream::Stdout),
        })
    }

    pub fn from_stderr(coloring: ColorChoice, isatty: Option<bool>) -> Self {
        Self::new(WriteStream::from_stderr(coloring), coloring, match isatty {
            Some(val) => val,
            None => tty::isatty(StdStream::Stderr),
        })
    }

    pub fn is_a_terminal(&self) -> bool { self.isatty }
}

impl WriteColor for OutputStream {
    fn supports_color(&self) -> bool {
        match self.inner {
            WriteStream::Stream(ref stream) => stream.supports_color(),
            _ => false,
        }
    }

    fn reset(&mut self) -> io::Result<()> {
        match self.inner {
            WriteStream::Stream(ref mut stream) => stream.reset(),
            _ => Ok(()),
        }
    }

    fn set_color(&mut self, spec: &ColorSpec) -> io::Result<()> {
        match self.inner {
            WriteStream::Stream(ref mut stream) => stream.set_color(spec),
            _ => Ok(()),
        }
    }
}

impl Write for OutputStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.inner {
            WriteStream::Stream(ref mut stream) => stream.write(buf),
            WriteStream::Write(ref mut w) => w.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self.inner {
            WriteStream::Stream(ref mut stream) => stream.flush(),
            WriteStream::Write(ref mut w) => w.flush(),
        }
    }
}

impl fmt::Debug for OutputStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "OutputStream {{ coloring: {:?}, isatty: {} }}",
               self.coloring, self.isatty,)
    }
}

pub enum WriteStream {
    /// A plain write object without color support
    Write(Box<dyn Write + Send>),
    /// Color-enabled stdio, with information on whether color should be used
    Stream(StandardStream),
}

impl WriteStream {
    // Implementation heavily inspired and based on the Cargo `shell.rs` implementation. Source:
    // https://github.com/rust-lang/cargo/blob/5c6aa46e6f28661270979696e7b4c2f0dff8628f/
    // src/cargo/core/shell.rs

    pub fn from_stdout(coloring: ColorChoice) -> Self {
        WriteStream::Stream(StandardStream::stdout(coloring))
    }

    pub fn from_stderr(coloring: ColorChoice) -> Self {
        WriteStream::Stream(StandardStream::stderr(coloring))
    }

    /// Create a shell from a plain writable object, with no color, and max verbosity.
    pub fn from_write<T: FnMut() -> Box<dyn Write + Send>>(mut writable_fn: T) -> Self {
        WriteStream::Write(writable_fn())
    }
}

mod tty {
    #[derive(Clone, Copy)]
    pub enum StdStream {
        Stdin,
        Stdout,
        Stderr,
    }

    #[cfg(unix)]
    pub fn isatty(output: StdStream) -> bool {
        use libc;

        let fd = match output {
            StdStream::Stdin => libc::STDIN_FILENO,
            StdStream::Stdout => libc::STDOUT_FILENO,
            StdStream::Stderr => libc::STDERR_FILENO,
        };

        unsafe { libc::isatty(fd) != 0 }
    }
    #[cfg(windows)]
    pub fn isatty(output: StdStream) -> bool {
        use winapi::um::{consoleapi,
                         processenv,
                         winbase};

        let handle = match output {
            StdStream::Stdin => winbase::STD_INPUT_HANDLE,
            StdStream::Stdout => winbase::STD_OUTPUT_HANDLE,
            StdStream::Stderr => winbase::STD_ERROR_HANDLE,
        };

        unsafe {
            let handle = processenv::GetStdHandle(handle);
            let mut out = 0;
            consoleapi::GetConsoleMode(handle, &mut out) != 0
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
    bar:     pbr::ProgressBar<Stdout>,
    total:   u64,
    current: u64,
}

impl Default for ConsoleProgressBar {
    fn default() -> Self {
        ConsoleProgressBar { bar:     pbr::ProgressBar::new(0),
                             total:   0,
                             current: 0, }
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
        println!();
        io::stdout().flush().expect("flush() fail");
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

    fn flush(&mut self) -> io::Result<()> { self.bar.flush() }
}

pub fn print_wrapped<U>(stream: &mut dyn WriteColor,
                        text: U,
                        wrap_width: usize,
                        left_indent: usize)
                        -> io::Result<()>
    where U: AsRef<str>
{
    for line in text.as_ref().split("\n\n") {
        let mut buffer = String::new();
        let mut width = 0;
        for word in line.split_whitespace() {
            let wl = word.chars().count();
            if (width + wl + 1) > (wrap_width - left_indent) {
                stream.write_all(
                    format!("{:<width$}{}\n", " ", buffer, width = left_indent).as_bytes(),
                )?;
                buffer.clear();
                width = 0;
            }
            width = width + wl + 1;
            buffer.push_str(word);
            buffer.push(' ');
        }
        if !buffer.is_empty() {
            stream.write_all(
                format!("{:<width$}{}\n", " ", buffer, width = left_indent).as_bytes(),
            )?;
        }
        stream.write_all(b"\n")?;
    }
    stream.flush()
}

pub fn print(writer: &mut WriteColor, buf: &[u8], color_spec: &ColorSpec) -> io::Result<()> {
    writer.reset()?;
    writer.set_color(color_spec)?;
    writer.write_all(buf)?;
    writer.flush()?;
    writer.reset()
}

pub fn println(writer: &mut WriteColor, buf: &[u8], color_spec: &ColorSpec) -> io::Result<()> {
    print(writer, buf, color_spec)?;
    writer.write_all(b"\n")?;
    writer.flush()
}
