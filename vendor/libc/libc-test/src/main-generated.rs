// Note that this main file is meant to mirror `main.rs` but is only used on the
// BSDs where the generated location of `all.rs` is slightly different

#![allow(bad_style, improper_ctypes)]
extern crate libc;

use libc::*;

include!("../all.rs");
