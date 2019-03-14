// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

//! Formats user-visible output for the Supervisor.
//!
//! Most of this module is used via the `outputln!` and
//! `output_format!` macros. They create a `StructuredOutput` struct,
//! which includes the line number, file name, and column it was
//! called on. Additionally, it uses a standard constant called
//! `LOGKEY` as a short hint as to where the output was generated
//! within the Supervisor. Also supported is a `preamble`, which is
//! used to denote when output comes from a running service rather
//! than the Supervisor itself.
//!
//! The `StructuredOutput` struct supports three global options -
//! dealing with verbosity, coloring, and structured JSON output. If
//! verbose is turned on, then every line printed is annotated with
//! its preamble, logkey, and precise location; without verbose, it
//! prints simply the preamble and logkey. Coloring does what it says
//! on the tin :) JSON-formatted output emits this information as a
//! JSON object. It ignores the coloring option, and does _not_ ever log
//! with ANSI color codes, but does honor the verbose flag.

use crate::PROGRAM_NAME;
use serde::{ser::SerializeMap,
            Serialize,
            Serializer};
use serde_json;
use std::{fmt,
          io::{self,
               Write},
          result,
          sync::{atomic::{AtomicBool,
                          Ordering,
                          ATOMIC_BOOL_INIT},
                 Mutex}};
use termcolor::{BufferWriter,
                Color,
                ColorChoice,
                ColorSpec,
                WriteColor};

static VERBOSITY: AtomicBool = ATOMIC_BOOL_INIT;

lazy_static! {
    static ref FORMAT: Mutex<OutputFormat> = Mutex::new(OutputFormat::Color(ColorSpec::default()));
}

/// Get the OutputFormat for which content is to be rendered
// Maybe return a &'static instead of cloning?
pub fn get_format() -> OutputFormat { FORMAT.lock().expect("FORMAT lock poisoned").clone() }

/// Set the OutputFormat for which content is to be rendered
pub fn set_format(format: OutputFormat) { *FORMAT.lock().expect("FORMAT lock poisoned") = format }

/// Get the OutputVerbosity for which content is to be rendered
pub fn get_verbosity() -> OutputVerbosity {
    if VERBOSITY.load(Ordering::Relaxed) {
        OutputVerbosity::Verbose
    } else {
        OutputVerbosity::Normal
    }
}

/// Set the OutputVerbosity for which content is to be rendered
pub fn set_verbosity(format: OutputVerbosity) {
    VERBOSITY.store(match format {
                        OutputVerbosity::Verbose => true,
                        OutputVerbosity::Normal => false,
                    },
                    Ordering::Relaxed)
}

/// Adds structure to printed output. Stores a preamble, a logkey, line, file, column, and content
/// to print.
pub struct StructuredOutput<'a> {
    preamble: &'a str,
    logkey: &'static str,
    content: &'a str,
    /// The verbosity level of rendered content
    verbosity: OutputVerbosityInternal,
    /// How should output be formatted
    format: OutputFormat,
}

impl<'a> StructuredOutput<'a> {
    /// Return a new StructuredOutput struct.
    pub fn new(preamble: &'a str,
               logkey: &'static str,
               context: OutputContext,
               format: OutputFormat,
               verbosity: OutputVerbosity,
               content: &'a str)
               -> StructuredOutput<'a> {
        let verbosity = match verbosity {
            OutputVerbosity::Normal => OutputVerbosityInternal::Normal,
            OutputVerbosity::Verbose => OutputVerbosityInternal::Verbose(context),
        };
        StructuredOutput { preamble,
                           logkey,
                           content,
                           verbosity,
                           format }
    }

    pub fn succinct(preamble: &'a str,
                    logkey: &'static str,
                    format: OutputFormat,
                    content: &'a str)
                    -> StructuredOutput<'a> {
        StructuredOutput { preamble,
                           logkey,
                           content,
                           verbosity: OutputVerbosityInternal::Normal,
                           format }
    }

    pub fn print(&self) -> io::Result<()> {
        self.print_to_writer(&BufferWriter::stdout(self.format.color_choice()))
    }

    pub fn eprint(&self) -> io::Result<()> {
        self.print_to_writer(&BufferWriter::stderr(self.format.color_choice()))
    }

    pub fn println(&self) -> io::Result<()> {
        self.println_to_writer(&BufferWriter::stdout(self.format.color_choice()))
    }

    pub fn eprintln(&self) -> io::Result<()> {
        self.println_to_writer(&BufferWriter::stderr(self.format.color_choice()))
    }

    fn print_to_writer(&self, writer: &BufferWriter) -> io::Result<()> {
        let mut buffer = writer.buffer();
        self.format(&mut buffer)?;
        writer.print(&buffer)
    }

    fn println_to_writer(&self, writer: &BufferWriter) -> io::Result<()> {
        let mut buffer = writer.buffer();
        self.format(&mut buffer)?;
        buffer.write_all(b"\n")?;
        buffer.flush()?;
        writer.print(&buffer)
    }

    // If we ever want to create multiple output formats in the future, we would do it here -
    // essentially create a flag we check to see what output you want, then call a different
    // formatting function. Viola!
    fn format(&self, writer: &mut WriteColor) -> io::Result<()> {
        writer.reset()?;
        match self.format {
            OutputFormat::JSON => {
                // Our JSON serialization handles verbosity itself, and
                // color is ignored anyway, so there's no reason to check
                // those settings here.

                // unwrap is safe, as we control the inputs
                let as_json = serde_json::to_string(&self).unwrap();
                write!(writer, "{}", as_json)
            }
            _ => {
                let preamble_color = if self.preamble == PROGRAM_NAME.as_str() {
                    Color::Cyan
                } else {
                    Color::Green
                };

                writer.set_color(ColorSpec::new().set_fg(Some(preamble_color)))?;
                writer.write_all(self.preamble.as_bytes())?;
                writer.reset()?;
                writer.write_all(b"(")?;
                writer.set_color(ColorSpec::new().set_fg(Some(Color::White)).set_bold(true))?;
                writer.write_all(self.logkey.as_bytes())?;
                writer.reset()?;
                writer.write_all(b")")?;
                if let OutputVerbosityInternal::Verbose(OutputContext { line, file, column }) =
                    self.verbosity
                {
                    writer.write_all(b"[")?;
                    writer.set_color(ColorSpec::new().set_fg(Some(Color::White))
                                                     .set_underline(true))?;
                    write!(writer, "{}:{}:{}", file, line, column)?;
                    writer.reset()?;
                    writer.write_all(b"]")?;
                }
                writer.write_all(b": ")?;
                if let OutputFormat::Color(ref color_spec) = self.format {
                    writer.set_color(color_spec)?;
                }
                writer.write_all(self.content.as_bytes())?;
                writer.reset()?;
                writer.flush()
            }
        }
    }
}

// Custom implementation of Serialize to ensure that we can
// appropriately represent both verbose and non-verbose output, a
// behavior which isn't otherwise possible to derive.
impl<'a> Serialize for StructuredOutput<'a> {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        // Focused on JSON serialization right now, so the length hint
        // isn't needed; it might be later if we target other formats.
        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("preamble", &self.preamble)?;
        map.serialize_entry("logkey", &self.logkey)?;
        if let OutputVerbosityInternal::Verbose(OutputContext { line, file, column }) =
            self.verbosity
        {
            map.serialize_entry("file", &file)?;
            map.serialize_entry("line", &line)?;
            map.serialize_entry("column", &column)?;
        }
        map.serialize_entry("content", &self.content)?;

        map.end()
    }
}

impl<'a> fmt::Display for StructuredOutput<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bufwtr = BufferWriter::stdout(self.format.color_choice());
        let mut buffer = bufwtr.buffer();
        match self.format(&mut buffer) {
            Ok(_) => {
                f.write_str(std::str::from_utf8(buffer.as_slice()).expect("termcolor buffer \
                                                                           valid utf8"))
            }
            Err(e) => write!(f, "Error formatting StructuredOutput: {}", e),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum OutputFormat {
    Color(ColorSpec),
    NoColor,
    JSON,
}

impl OutputFormat {
    pub fn color_choice(&self) -> ColorChoice {
        match self {
            OutputFormat::Color(_) => ColorChoice::Auto,
            OutputFormat::NoColor | OutputFormat::JSON => ColorChoice::Never,
        }
    }

    pub fn is_color(&self) -> bool {
        match self {
            OutputFormat::Color(_) => true,
            OutputFormat::NoColor | OutputFormat::JSON => false,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum OutputVerbosity {
    Normal,
    Verbose,
}

#[derive(Clone, Copy)]
pub struct OutputContext {
    pub line:   u32,
    pub file:   &'static str,
    pub column: u32,
}

#[derive(Clone, Copy)]
enum OutputVerbosityInternal {
    Normal,
    Verbose(OutputContext),
}

#[macro_export]
/// Works the same as println!, but uses our structured output formatter.
macro_rules! outputln {
    ($content: expr) => {
        {
            use $crate::output::{get_format, get_verbosity, OutputContext, StructuredOutput};
            use $crate::PROGRAM_NAME;
            StructuredOutput::new(PROGRAM_NAME.as_str(),
                                           LOGKEY,
                                           OutputContext { line:   line!(),
                                                           file:   file!(),
                                                           column: column!() },
                                           get_format(),
                                           get_verbosity(),
                                           $content).println().expect("failed to write output to stdout");
        }
    };
    (preamble $preamble:expr, $content: expr) => {
        {
            use $crate::output::{get_format, get_verbosity, OutputContext, StructuredOutput};
            StructuredOutput::new(&$preamble,
                                           LOGKEY,
                                           OutputContext { line:   line!(),
                                                           file:   file!(),
                                                           column: column!() },
                                           get_format(),
                                           get_verbosity(),
                                           $content).println().expect("failed to write output to stdout");
        }
    };
    ($content: expr, $($arg:tt)*) => {
        {
            use $crate::output::{get_format, get_verbosity, OutputContext, StructuredOutput};
            use $crate::PROGRAM_NAME;
            let content = format!($content, $($arg)*);
            StructuredOutput::new(PROGRAM_NAME.as_str(),
                                           LOGKEY,
                                           OutputContext { line:   line!(),
                                                           file:   file!(),
                                                           column: column!() },
                                           get_format(),
                                           get_verbosity(),
                                           &content).println().expect("failed to write output to stdout");
        }
    };
    (preamble $preamble: expr, $content: expr, $($arg:tt)*) => {
        {
            use $crate::output::{get_format, get_verbosity, OutputContext, StructuredOutput};
            let content = format!($content, $($arg)*);
            StructuredOutput::new(&$preamble,
                                           LOGKEY,
                                           OutputContext { line:   line!(),
                                                           file:   file!(),
                                                           column: column!() },
                                           get_format(),
                                           get_verbosity(),
                                           &content).println().expect("failed to write output to stdout");
        }
    };
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::{OutputContext,
                OutputFormat,
                OutputVerbosity,
                StructuredOutput};
    use serde_json;
    use termcolor::{BufferWriter,
                    Color,
                    ColorChoice,
                    ColorSpec,
                    WriteColor};

    use crate::PROGRAM_NAME;

    static LOGKEY: &'static str = "SOT";

    fn so<'a>(preamble: &'a str,
              content: &'a str,
              format: OutputFormat,
              verbosity: OutputVerbosity)
              -> StructuredOutput<'a> {
        StructuredOutput::new(preamble,
                              LOGKEY,
                              OutputContext { line:   1,
                                              file:   file!(),
                                              column: 2, },
                              format,
                              verbosity,
                              content)
    }

    #[test]
    fn new() {
        let so = so("soup",
                    "opeth is amazing",
                    OutputFormat::NoColor,
                    OutputVerbosity::Normal);
        assert_eq!(so.logkey, "SOT");
        assert_eq!(so.preamble, "soup");
        assert_eq!(so.content, "opeth is amazing");
    }

    #[test]
    fn format() {
        let so = so("soup",
                    "opeth is amazing",
                    OutputFormat::NoColor,
                    OutputVerbosity::Normal);
        assert_eq!(format!("{}", so), "soup(SOT): opeth is amazing");
    }

    #[test]
    fn format_color() {
        let progname = PROGRAM_NAME.as_str();
        let content = "opeth is amazing";
        let mut cs = ColorSpec::new();
        cs.set_underline(true);
        let so = StructuredOutput::new(progname,
                                       LOGKEY,
                                       OutputContext { line:   1,
                                                       file:   file!(),
                                                       column: 2, },
                                       OutputFormat::Color(cs.clone()),
                                       OutputVerbosity::Normal,
                                       content);
        let writer = BufferWriter::stdout(ColorChoice::Auto);
        let mut buffer = writer.buffer();
        buffer.reset().unwrap();
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
              .unwrap();
        buffer.write_all(progname.as_bytes()).unwrap();
        buffer.reset().unwrap();
        buffer.write_all(b"(").unwrap();
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)).set_bold(true))
              .unwrap();
        buffer.write_all(b"SOT").unwrap();
        buffer.reset().unwrap();
        buffer.write_all(b"): ").unwrap();
        buffer.set_color(&cs).unwrap();
        buffer.write_all(content.as_bytes()).unwrap();
        buffer.reset().unwrap();
        assert_eq!(format!("{}", so),
                   String::from_utf8_lossy(buffer.as_slice()));
    }

    #[test]
    fn json_formatting() {
        let so = so("monkeys",
                    "I love monkeys",
                    OutputFormat::JSON,
                    OutputVerbosity::Normal);

        let actual: serde_json::Value =
            serde_json::from_str(&(format!("{}", so))).expect("Couldn't parse from JSON");

        assert_eq!(actual,
                   serde_json::json!({
                       "preamble": "monkeys",
                       "logkey": LOGKEY,
                       "content": "I love monkeys"
                   }));
    }

    #[test]
    fn verbose_json_formatting() {
        let so = so("monkeys",
                    "I love verbose monkeys",
                    OutputFormat::JSON,
                    OutputVerbosity::Verbose);

        let actual: serde_json::Value =
            serde_json::from_str(&(format!("{}", so))).expect("Couldn't parse from JSON");

        assert_eq!(actual,
                   serde_json::json!({
                       "preamble": "monkeys",
                       "logkey": LOGKEY,
                       "line": 1,
                       "file": file!(),
                       "column": 2,
                       "content": "I love verbose monkeys"
                   }));
    }

    #[test]
    fn json_formatting_ignores_color() {
        let with_color = so("monkeys",
                            "I love drab monkeys",
                            OutputFormat::JSON,
                            OutputVerbosity::Normal);

        let actual: serde_json::Value =
            serde_json::from_str(&(format!("{}", with_color))).expect("Couldn't parse from JSON");

        assert_eq!(actual,
                   serde_json::json!({
                       "preamble": "monkeys",
                       "logkey": LOGKEY,
                       "content": "I love drab monkeys"
                   }),
                   "JSON output shouldn't have color, even if the colorized flag was set");
    }
}
