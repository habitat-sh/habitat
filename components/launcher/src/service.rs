use crate::protocol;
#[cfg(windows)]
use core::os::process::windows_child::{ChildStderr,
                                       ChildStdout,
                                       ExitStatus};
use habitat_common::output::{self,
                             StructuredOutput};
#[cfg(unix)]
use std::process::{ChildStderr,
                   ChildStdout,
                   ExitStatus};
use std::{fmt,
          io::{self,
               BufRead,
               BufReader,
               Read},
          thread};

pub use crate::sys::service::*;

pub struct Service {
    args:    protocol::Spawn,
    process: Process,
}

impl Service {
    pub fn new(spawn: protocol::Spawn,
               process: Process,
               stdout: Option<ChildStdout>,
               stderr: Option<ChildStderr>)
               -> Self {
        if let Some(stdout) = stdout {
            let id = spawn.id.to_string();
            thread::Builder::new().name(format!("{}-out", spawn.id))
                                  .spawn(move || pipe_stdout(stdout, &id))
                                  .ok();
        }
        if let Some(stderr) = stderr {
            let id = spawn.id.to_string();
            thread::Builder::new().name(format!("{}-err", spawn.id))
                                  .spawn(move || pipe_stderr(stderr, &id))
                                  .ok();
        }
        Service { args: spawn,
                  process }
    }

    pub fn args(&self) -> &protocol::Spawn { &self.args }

    pub fn id(&self) -> u32 { self.process.id() }

    /// Attempt to gracefully terminate a proccess and then forcefully kill it after
    /// 8 seconds if it has not terminated.
    pub fn kill(&mut self) -> protocol::ShutdownMethod { self.process.kill() }

    pub fn name(&self) -> &str { &self.args.id }

    pub fn take_args(self) -> protocol::Spawn { self.args }

    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> { self.process.try_wait() }

    pub fn wait(&mut self) -> io::Result<ExitStatus> { self.process.wait() }
}

impl fmt::Debug for Service {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Service {{ pid: {:?} }}", self.process.id())
    }
}

/// Consume output from a child process until EOF, then finish
fn pipe_stdout<T>(out: T, id: &str)
    where T: Read
{
    let mut reader = BufReader::new(out);
    let mut buffer = String::new();
    while reader.read_line(&mut buffer).unwrap() > 0 {
        let content = &buffer.trim_end_matches('\n');
        let so = StructuredOutput::succinct(&id, "O", output::get_format(), content);
        if let Err(e) = so.println() {
            println!("printing output: '{}' to stdout resulted in error: {}",
                     content, e);
        }
        buffer.clear();
    }
}

/// Consume standard error from a child process until EOF, then finish
fn pipe_stderr<T>(err: T, id: &str)
    where T: Read
{
    let mut reader = BufReader::new(err);
    let mut buffer = String::new();
    while reader.read_line(&mut buffer).unwrap() > 0 {
        let content = &buffer.trim_end_matches('\n');
        let so = StructuredOutput::succinct(&id, "E", output::get_format(), content);
        if let Err(e) = so.eprintln() {
            eprintln!("printing output: '{}' to stderr resulted in error: {}",
                      content, e);
        }
        buffer.clear();
    }
}
