//! Traps and notifies UNIX signals.

use crate::os::process::{Signal,
                         SignalCode};
use std::{collections::VecDeque,
          mem,
          ptr,
          io,
          sync::{atomic::Ordering,
                 Mutex,
                 Once},
          thread};

static INIT: Once = Once::new();

lazy_static::lazy_static! {
    static ref CAUGHT_SIGNALS: Mutex<VecDeque<SignalCode>> = Mutex::new(VecDeque::new());
}

pub fn init() {
    INIT.call_once(|| {
            // TODO(ssd) 2019-10-16: We could bubble this error up
            // further if we want, but in either case this should be a
            // hard failure.
            self::start_signal_handler().expect("starting signal handler failed");
        });
}

pub enum SignalEvent {
    WaitForChild,
    Passthrough(Signal),
}

/// Consumers should call this function fairly frequently and since the vast
/// majority of the time there is at most one signal event waiting, we return
/// at most one. If multiple signals have been received since the last call,
/// they will be returned, one per call in the order they were received.
pub fn check_for_signal() -> Option<SignalEvent> {
    let mut signals = CAUGHT_SIGNALS.lock().expect("Signal mutex poisoned");

    if let Some(code) = signals.pop_front() {
        match from_signal_code(code) {
            Some(Signal::CHLD) => Some(SignalEvent::WaitForChild),
            Some(signal) => Some(SignalEvent::Passthrough(signal)),
            None => {
                println!("Received invalid signal: #{}", code);
                None
            }
        }
    } else {
        None
    }
}

fn start_signal_handler() -> io::Result<()> {
    let mut handled_signals = Sigset::empty()?;
    handled_signals.addsig(libc::SIGINT)?;
    handled_signals.addsig(libc::SIGTERM)?;
    handled_signals.addsig(libc::SIGHUP)?;
    handled_signals.addsig(libc::SIGQUIT)?;
    handled_signals.addsig(libc::SIGALRM)?;
    handled_signals.addsig(libc::SIGUSR1)?;
    handled_signals.addsig(libc::SIGUSR2)?;
    handled_signals.addsig(libc::SIGCHLD)?;
    handled_signals.block()?;
    thread::Builder::new().name("signal-handler".to_string())
                          .spawn(move || {
                              loop {
                                  // Using expect here seems reasonable because our understanding of wait() is that it only
                                  // returns an error if we called it with a bad signal.
                                  let signal = handled_signals.wait().expect("sigwait failed");
                                  debug!("signal-handler thread received signal {:?}!", signal);
                                  match signal {
                                      libc::SIGINT | libc::SIGTERM => {
                                          super::SHUTDOWN.store(true, Ordering::SeqCst);
                                      }
                                      _ => {
                                          CAUGHT_SIGNALS.lock()
                                                        .expect("Signal mutex poisoned")
                                                        .push_back(signal);
                                      }
                                  };
                              }
                          })?;
    Ok(())
}

// Sigset is a wrapper for the underlying libc type.
struct Sigset {
    inner: libc::sigset_t
}

impl Sigset {
    // empty returns an empty Sigset.
    //
    // For more information on the relevant libc function see:
    //
    // http://man7.org/linux/man-pages/man3/sigsetops.3.html
    //
    fn empty() -> io::Result<Sigset> {
        let mut set: libc::sigset_t = unsafe { mem::zeroed() };
        let ret = unsafe { libc::sigemptyset(&mut set) };
        if ret < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(Sigset{inner: set})
        }
    }

    // addsig adds the given signal to the Sigset.
    //
    // For more information on the relevant libc function see:
    //
    // http://man7.org/linux/man-pages/man3/sigsetops.3.html
    //
    fn addsig(&mut self, signal: SignalCode) -> io::Result<()> {
        let ret = unsafe { libc::sigaddset(&mut self.inner, signal) };
        if ret < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    // block sets the calling thread's signal mask to the current
    // sigmask, blocking delivery of all signals in the sigmask.
    //
    // This should be called before wait() to avoid race conditions.
    //
    // For more information on the relevant libc function see:
    //
    // http://man7.org/linux/man-pages/man3/pthread_sigmask.3.html
    //
    fn block(&self) -> io::Result<()> {
        let ret = unsafe { libc::pthread_sigmask(libc::SIG_SETMASK, &self.inner, ptr::null_mut()) };
        if ret < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    // wait blocks until a signal in the current sigset has been
    // delivered to the thread.
    //
    // Callers should call block() before this function to avoid race
    // conditions.
    //
    // For information on the relevant libc function see:
    //
    // http://man7.org/linux/man-pages/man3/sigwait.3.html
    //
    // The manual page on linux only lists a single failure case:
    //
    // > EINVAL set contains an invalid signal number.
    //
    // thus most callers should be able to expect success.
    //
    fn wait(&self) -> io::Result<SignalCode> {
        let mut signal: libc::c_int = 0;
        let ret = unsafe { libc::sigwait(&self.inner, &mut signal) };
        if ret != 0 {
            Err(io::Error::from_raw_os_error(ret))
        } else {
            Ok(signal)
        }
    }
}


/// These are the signals that we can eventually translate into
/// some kind of event
fn from_signal_code(code: SignalCode) -> Option<Signal> {
    match code {
        libc::SIGHUP => Some(Signal::HUP),
        libc::SIGCHLD => Some(Signal::CHLD),
        _ => None,
    }
}
