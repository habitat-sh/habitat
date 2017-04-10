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
//! Most of this module is used via the `output!`, `outputln!`, and `output_format!` macros. They
//! create a `StructuredOutput` struct, which includes the line number, file name, and column it
//! was called on. Additionally, it uses a standard constant called `LOGKEY` as a short hint as to
//! where the output was generated within the Supervisor. Also supported is a `preamble`, which is
//! used to denote when output comes from a running service rather than the Supervisor itself.
//!
//! The `StructuredOutput` struct supports two global options - verbosity and coloring. If verbose
//! is turned on, then every line printed is annotated with its preamble, logkey, and precise
//! location. Without verbose, it prints simply the preamble and logkey. Coloring does what it says
//! on the tin :)

use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};
use std::fmt;

use ansi_term::Colour::{White, Cyan, Green};

use PROGRAM_NAME;

static mut VERBOSE: AtomicBool = ATOMIC_BOOL_INIT;
// I am sorry this isn't named the other way; I can't get an atomic initializer that defaults to
// true. Them's the breaks.
static mut NO_COLOR: AtomicBool = ATOMIC_BOOL_INIT;

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
    unsafe {
        if NO_COLOR.load(Ordering::Relaxed) {
            false
        } else {
            true
        }
    }
}

/// Set to true if you want color to turn off.
pub fn set_no_color(booly: bool) {
    unsafe {
        NO_COLOR.store(booly, Ordering::Relaxed);
    }
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
    pub verbose: Option<bool>,
    pub color: Option<bool>,
}

impl<'a> StructuredOutput<'a> {
    /// Return a new StructuredOutput struct.
    pub fn new(preamble: &'a str,
               logkey: &'static str,
               line: u32,
               file: &'static str,
               column: u32,
               content: &'a str)
               -> StructuredOutput<'a> {
        StructuredOutput {
            preamble: preamble,
            logkey: logkey,
            line: line,
            file: file,
            column: column,
            content: content,
            verbose: None,
            color: None,
        }
    }
}

// If we ever want to create multiple output formats in the future, we would do it here -
// essentially create a flag we check to see what output you want, then call a different formatting
// function. Viola!
impl<'a> fmt::Display for StructuredOutput<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let verbose = self.verbose.unwrap_or(is_verbose());
        let color = self.color.unwrap_or(is_color());
        let preamble_color = if self.preamble == PROGRAM_NAME.as_str() {
            Cyan
        } else {
            Green
        };
        if verbose {
            if color {
                write!(f,
                       "{}({})[{}]: {}",
                       preamble_color.paint(self.preamble),
                       White.bold().paint(self.logkey),
                       White
                           .underline()
                           .paint(format!("{}:{}:{}", self.file, self.line, self.column)),
                       self.content)
            } else {
                write!(f,
                       "{}({})[{}:{}:{}]: {}",
                       self.preamble,
                       self.logkey,
                       self.file,
                       self.line,
                       self.column,
                       self.content)
            }
        } else {
            if color {
                write!(f,
                       "{}({}): {}",
                       preamble_color.paint(self.preamble),
                       White.bold().paint(self.logkey),
                       self.content)
            } else {
                write!(f, "{}({}): {}", self.preamble, self.logkey, self.content)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StructuredOutput;
    use ansi_term::Colour::{White, Cyan};

    use PROGRAM_NAME;

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
        assert_eq!(format!("{}", so),
                   format!("{}({}): opeth is amazing",
                           Cyan.paint(progname),
                           White.bold().paint("SOT")));
    }
}
