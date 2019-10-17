//! Contains the cross-platform signal behavior.
// If signal handling ever becomes part of the rust stdlib, consider removing
// our homespun implementation. Check for status of that here:
// https://github.com/rust-lang/rfcs/issues/1368

use std::sync::atomic::{AtomicBool,
                        Ordering};

#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use self::unix::{pending_sigchld,
                     pending_sighup,
                     init};

static SHUTDOWN: AtomicBool = AtomicBool::new(false);

// NOTE: The Unix implementation of `init` also establishes a similar
// handler for shutdown signals, but also does some other stuff, as
// well. Seems best for now to keep all those implementation details
// in the `unix` module.
#[cfg(windows)]
pub fn init() {
    ctrlc::set_handler(move || {
        SHUTDOWN.store(true, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
}

/// Returns `true` if we have received a signal to shut down.
pub fn pending_shutdown() -> bool { SHUTDOWN.compare_and_swap(true, false, Ordering::SeqCst) }
