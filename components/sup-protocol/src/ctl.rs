//! Specific request, responses, and types used to specifically communicate with the Supervisor's
//! Control Gateway.
//!
//! Note: See `protocols/ctl.proto` for type level documentation for generated types.

include!("generated/sup.ctl.rs");
include!("generated/sup.ctl.impl.rs");

use std::fmt;

impl fmt::Display for ConsoleLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.line) }
}
