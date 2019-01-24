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

use std::fmt;
use std::result;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

use ansi_term::Colour::{Cyan, Green, White};
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use serde_json;

use crate::PROGRAM_NAME;

static mut VERBOSE: AtomicBool = ATOMIC_BOOL_INIT;
// I am sorry this isn't named the other way; I can't get an atomic initializer that defaults to
// true. Them's the breaks.
static mut NO_COLOR: AtomicBool = ATOMIC_BOOL_INIT;

static mut JSON: AtomicBool = ATOMIC_BOOL_INIT;

/// True if verbose output is on.
pub fn is_verbose() -> bool {
    unsafe { VERBOSE.load(Ordering::Relaxed) }
}

/// Turn verbose output on or off.
pub fn set_verbose(booly: bool) {
    unsafe {
        VERBOSE.store(booly, Ordering::Relaxed);
    }
}

/// True if color is enabled
pub fn is_color() -> bool {
    unsafe { !NO_COLOR.load(Ordering::Relaxed) }
}

/// Set to true if you want color to turn off.
pub fn set_no_color(booly: bool) {
    unsafe {
        NO_COLOR.store(booly, Ordering::Relaxed);
    }
}

/// True if JSON formatting is enabled
pub fn is_json() -> bool {
    unsafe { JSON.load(Ordering::Relaxed) }
}

/// Set to true if you want JSON-formatted logs
pub fn set_json(booly: bool) {
    unsafe { JSON.store(booly, Ordering::Relaxed) }
}

/// Adds structure to printed output. Stores a preamble, a logkey, line, file, column, and content
/// to print.
pub struct StructuredOutput<'a> {
    preamble: &'a str,
    logkey: &'static str,
    line: u32,
    file: &'static str,
    column: u32,
    content: &'a str,
    /// Whether or not to render additional logging details such as
    /// file name and line numbers.
    pub verbose: Option<bool>,
    /// Whether or not to render output with embedded ANSI color
    /// codes. Ignored if serializing to JSON.
    pub color: Option<bool>,
    /// Whether or not to render as structured JSON logging output.
    pub json: Option<bool>,
}

impl<'a> StructuredOutput<'a> {
    /// Return a new StructuredOutput struct.
    pub fn new(
        preamble: &'a str,
        logkey: &'static str,
        line: u32,
        file: &'static str,
        column: u32,
        content: &'a str,
    ) -> StructuredOutput<'a> {
        StructuredOutput {
            preamble: preamble,
            logkey: logkey,
            line: line,
            file: file,
            column: column,
            content: content,
            verbose: None,
            color: None,
            json: None,
        }
    }
}

// Custom implementation of Serialize to ensure that we can
// appropriately represent both verbose and non-verbose output, a
// behavior which isn't otherwise possible to derive.
impl<'a> Serialize for StructuredOutput<'a> {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let verbose = self.verbose.unwrap_or_else(is_verbose);

        // Focused on JSON serialization right now, so the length hint
        // isn't needed; it might be later if we target other formats.
        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("preamble", &self.preamble)?;
        map.serialize_entry("logkey", &self.logkey)?;
        if verbose {
            map.serialize_entry("file", &self.file)?;
            map.serialize_entry("line", &self.line)?;
            map.serialize_entry("column", &self.column)?;
        }
        map.serialize_entry("content", &self.content)?;

        map.end()
    }
}

// If we ever want to create multiple output formats in the future, we would do it here -
// essentially create a flag we check to see what output you want, then call a different formatting
// function. Viola!
impl<'a> fmt::Display for StructuredOutput<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.json.unwrap_or_else(is_json) {
            // Our JSON serialization handles verbosity itself, and
            // color is ignored anyway, so there's no reason to check
            // those settings here.

            // unwrap is safe, as we control the inputs
            let as_json = serde_json::to_string(&self).unwrap();
            write!(f, "{}", as_json)
        } else {
            let verbose = self.verbose.unwrap_or_else(is_verbose);
            let color = self.color.unwrap_or_else(is_color);

            let preamble_color = if self.preamble == PROGRAM_NAME.as_str() {
                Cyan
            } else {
                Green
            };

            if verbose {
                if color {
                    write!(
                        f,
                        "{}({})[{}]: {}",
                        preamble_color.paint(self.preamble),
                        White.bold().paint(self.logkey),
                        White
                            .underline()
                            .paint(format!("{}:{}:{}", self.file, self.line, self.column)),
                        self.content
                    )
                } else {
                    write!(
                        f,
                        "{}({})[{}:{}:{}]: {}",
                        self.preamble, self.logkey, self.file, self.line, self.column, self.content
                    )
                }
            } else if color {
                write!(
                    f,
                    "{}({}): {}",
                    preamble_color.paint(self.preamble),
                    White.bold().paint(self.logkey),
                    self.content
                )
            } else {
                write!(f, "{}({}): {}", self.preamble, self.logkey, self.content)
            }
        }
    }
}

#[macro_export]
/// Works the same as println!, but uses our structured output formatter.
macro_rules! outputln {
    ($content: expr) => {
        {
            use $crate::output::StructuredOutput;
            use $crate::PROGRAM_NAME;
            let so = StructuredOutput::new(PROGRAM_NAME.as_str(),
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           $content);
            println!("{}", so);
        }
    };
    (preamble $preamble:expr, $content: expr) => {
        {
            use $crate::output::StructuredOutput;
            let so = StructuredOutput::new(&$preamble,
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           $content);
            println!("{}", so);
        }
    };
    ($content: expr, $($arg:tt)*) => {
        {
            use $crate::output::StructuredOutput;
            use $crate::PROGRAM_NAME;
            let content = format!($content, $($arg)*);
            let so = StructuredOutput::new(PROGRAM_NAME.as_str(),
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           &content);
            println!("{}", so);
        }
    };
    (preamble $preamble: expr, $content: expr, $($arg:tt)*) => {
        {
            use $crate::output::StructuredOutput;
            let content = format!($content, $($arg)*);
            let so = StructuredOutput::new(&$preamble,
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           &content);
            println!("{}", so);
        }
    };
}

#[macro_export]
/// Works the same as format!, but uses our structured output formatter.
macro_rules! output_format {
    (preamble $preamble:expr,logkey $logkey:expr, $content:expr) => {{
        use $crate::output::StructuredOutput;
        let trimmed_content = &$content.trim_right_matches('\n');
        let so = StructuredOutput::new(
            &$preamble,
            $logkey,
            line!(),
            file!(),
            column!(),
            trimmed_content,
        );
        format!("{}", so)
    }};
}

#[cfg(test)]
mod tests {
    use super::StructuredOutput;
    use ansi_term::Colour::{Cyan, White};
    use serde_json;

    use crate::PROGRAM_NAME;

    static LOGKEY: &'static str = "SOT";

    fn so<'a>(preamble: &'a str, content: &'a str) -> StructuredOutput<'a> {
        StructuredOutput::new(preamble, LOGKEY, 1, file!(), 2, content)
    }

    #[test]
    fn new() {
        let so = so("soup", "opeth is amazing");
        assert_eq!(so.logkey, "SOT");
        assert_eq!(so.preamble, "soup");
        assert_eq!(so.content, "opeth is amazing");
    }

    #[test]
    fn format() {
        let mut so = so("soup", "opeth is amazing");
        so.verbose = Some(false);
        so.color = Some(false);
        assert_eq!(format!("{}", so), "soup(SOT): opeth is amazing");
    }

    #[test]
    fn format_color() {
        let progname = PROGRAM_NAME.as_str();
        let mut so = so(progname, "opeth is amazing");
        so.verbose = Some(false);
        so.color = Some(true);
        assert_eq!(
            format!("{}", so),
            format!(
                "{}({}): opeth is amazing",
                Cyan.paint(progname),
                White.bold().paint("SOT")
            )
        );
    }

    #[test]
    fn json_formatting() {
        let mut so = so("monkeys", "I love monkeys");
        so.json = Some(true);

        let actual: serde_json::Value =
            serde_json::from_str(&(format!("{}", so))).expect("Couldn't parse from JSON");

        assert_eq!(
            actual,
            serde_json::json!({
                "preamble": "monkeys",
                "logkey": LOGKEY,
                "content": "I love monkeys"
            })
        );
    }

    #[test]
    fn verbose_json_formatting() {
        let mut so = so("monkeys", "I love verbose monkeys");
        so.json = Some(true);
        so.verbose = Some(true);

        let actual: serde_json::Value =
            serde_json::from_str(&(format!("{}", so))).expect("Couldn't parse from JSON");

        assert_eq!(
            actual,
            serde_json::json!({
                "preamble": "monkeys",
                "logkey": LOGKEY,
                "line": 1,
                "file": file!(),
                "column": 2,
                "content": "I love verbose monkeys"
            })
        );
    }

    #[test]
    fn json_formatting_ignores_color() {
        let mut with_color = so("monkeys", "I love drab monkeys");
        with_color.json = Some(true);
        with_color.color = Some(true);

        let actual: serde_json::Value =
            serde_json::from_str(&(format!("{}", with_color))).expect("Couldn't parse from JSON");

        assert_eq!(
            actual,
            serde_json::json!({
                "preamble": "monkeys",
                "logkey": LOGKEY,
                "content": "I love drab monkeys"
            }),
            "JSON output shouldn't have color, even if the colorized flag was set"
        );
    }
}
