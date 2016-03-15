// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! Formats user-visible output for Bldr.
//!
//! Most of this module is used via the `output!`, `outputln!`, and `output_format!` macros. They
//! create a `StructuredOutput` struct, which includes the line number, file name, and column it
//! was called on. Additionally, it uses a standard constant called `LOGKEY` as a short hint as to
//! where the output was generated within bldr. Also supported is a `preamble`, which is used to
//! denote when output comes from a running service rather than bldr itself.
//!
//! The `StructuredOutput` struct supports two global options - verbosity and coloring. If verbose
//! is turned on, then every line printed is annotated with its preamble, logkey, and precise
//! location. Without verbose, it prints simply the preamble and logkey. Coloring does what it says
//! on the tin :)

use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};
use std::fmt;

use ansi_term::Colour::{White, Cyan, Green};

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
        let preamble_color = match self.preamble {
            "bldr" => Cyan,
            _ => Green,
        };
        if verbose {
            if color {
                write!(f,
                       "{}({})[{}]: {}",
                       preamble_color.paint(self.preamble),
                       White.bold().paint(self.logkey),
                       White.underline()
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

    static LOGKEY: &'static str = "SOT";

    fn so<'a>(preamble: &'a str, content: &'a str) -> StructuredOutput<'a> {
        StructuredOutput::new(preamble, LOGKEY, 1, file!(), 2, content)
    }

    #[test]
    fn new() {
        let so = so("bldr", "opeth is amazing");
        assert_eq!(so.logkey, "SOT");
        assert_eq!(so.preamble, "bldr");
        assert_eq!(so.content, "opeth is amazing");
    }

    #[test]
    fn format() {
        let mut so = so("bldr", "opeth is amazing");
        so.verbose = Some(false);
        so.color = Some(false);
        assert_eq!(format!("{}", so), "bldr(SOT): opeth is amazing");
    }

    #[test]
    fn format_color() {
        let mut so = so("bldr", "opeth is amazing");
        so.verbose = Some(false);
        so.color = Some(true);
        assert_eq!(format!("{}", so),
                   format!("{}({}): opeth is amazing",
                           Cyan.paint("bldr"),
                           White.bold().paint("SOT")));
    }

    #[test]
    fn format_verbose() {
        let mut so = so("bldr", "opeth is amazing");
        so.verbose = Some(true);
        so.color = Some(false);
        assert_eq!(format!("{}", so),
                   "bldr(SOT)[components/bldr/src/output.rs:1:2]: opeth is amazing");
    }

    #[test]
    fn format_verbose_color() {
        let mut so = so("bldr", "opeth is amazing");
        so.verbose = Some(true);
        so.color = Some(true);
        assert_eq!(format!("{}", so),
                   format!("{}({})[{}]: opeth is amazing",
                   Cyan.paint("bldr"),
                   White.bold().paint("SOT"),
                   White.underline().paint("components/bldr/src/output.rs:1:2"),
                   ));
    }
}
