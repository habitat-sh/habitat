// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::io::prelude::*;
use std::io;
use std::process::{Command, Child, Stdio, ExitStatus};
use std::fmt;
use std::error::Error;
use std::result;
use std::thread;
use std::collections::HashMap;

pub struct Cmd {
    pub child: Option<Child>,
    pub status: Option<ExitStatus>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

impl Cmd {
    pub fn stdout(&self) -> &str {
        match self.stdout {
            Some(ref stdout) => stdout,
            None => panic!("No stdout available - process needs a wait"),
        }
    }

    pub fn stderr(&self) -> &str {
        match self.stderr {
            Some(ref stderr) => stderr,
            None => panic!("No stderr available - process needs a wait"),
        }
    }

    pub fn status(&self) -> &ExitStatus {
        match self.status {
            Some(ref status) => status,
            None => panic!("No status available - process needs a wait or kill"),
        }
    }

    pub fn wait_with_output(&mut self) -> &Self {
        // The child is unavailable for more calls after this
        let child = self.child.take().unwrap();

        let output = match child.wait_with_output() {
            Ok(output) => output,
            Err(e) => panic!("{:?}", e),
        };
        self.status = Some(output.status);
        let stdout = String::from_utf8(output.stdout).unwrap_or_else(|x| panic!("{:?}", x));
        let stderr = String::from_utf8(output.stderr).unwrap_or_else(|x| panic!("{:?}", x));
        println!("OUT: {}", stdout);
        println!("ERR: {}", stderr);
        self.stdout = Some(stdout);
        self.stderr = Some(stderr);
        self
    }
}

#[derive(Debug)]
pub enum CmdError {
    Io(io::Error),
}

pub type CmdResult<T> = result::Result<T, CmdError>;

impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CmdError::Io(ref err) => err.fmt(f),
        }
    }
}

impl Error for CmdError {
    fn description(&self) -> &str {
        match *self {
            CmdError::Io(ref err) => err.description(),
        }
    }
}

impl From<io::Error> for CmdError {
    fn from(err: io::Error) -> CmdError {
        CmdError::Io(err)
    }
}

#[derive(Debug)]
pub struct CommandArgs {
    pub cmd: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    pub cwd: Option<String>,
}

impl CommandArgs {
    fn new<S: Into<String>>(cmd: S) -> CommandArgs {
        CommandArgs {
            cmd: cmd.into(),
            args: Vec::new(),
            env: HashMap::new(),
            cwd: None,
        }
    }

    fn arg<S: Into<String>>(&mut self, arg: S) -> &mut CommandArgs {
        self.args.push(arg.into());
        self
    }

    fn env<S: Into<String>>(&mut self, k: S, v: S) -> &mut CommandArgs {
        self.env.insert(k.into(), v.into());
        self
    }
}

impl fmt::Display for CommandArgs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Command: C: {} A: {:?} E: {:?} CWD: {:?}",
               self.cmd,
               self.args,
               self.env,
               self.cwd)
    }
}


pub fn command(cmd: &str, args: &[&str]) -> Command {
    command_with_env(cmd, args, None)
}

pub fn command_with_env(cmd: &str, args: &[&str], env: Option<&HashMap<&str, &str>>) -> Command {
    let mut cmd_args = CommandArgs::new(cmd);
    for a in args {
        cmd_args.arg(*a);
    }
    if let Some(real_env) = env {
        for (k, v) in real_env {
            cmd_args.env(*k, *v);
        }
    }
    run_command(cmd_args)
}

pub fn run_command(cmd_args: CommandArgs) -> Command {
    println!("{}: {}",
             thread::current().name().unwrap_or("main"),
             cmd_args);

    let mut command = Command::new(&cmd_args.cmd);
    command.args(&cmd_args.args);
    command.stdin(Stdio::null());
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    for (k, v) in cmd_args.env {
        command.env(k, v);
    }

    command
}

pub fn spawn(mut command: Command) -> CmdResult<Cmd> {
    let child = try!(command.spawn());
    Ok(Cmd {
        child: Some(child),
        status: None,
        stdout: None,
        stderr: None,
    })
}

pub fn studio_run(cmd: &str, args: &[&str]) -> CmdResult<Cmd> {
    let real_cmd = "studio";
    let mut real_args = vec!["-r", "/opt/studios/functional-tests", "run", cmd];
    real_args.extend_from_slice(args);
    let mut command = command(real_cmd, &real_args[..]);
    command.current_dir("/src");
    spawn(command)
}

pub fn run(cmd: &str, args: &[&str]) -> CmdResult<Cmd> {
    let command = command(cmd, args);
    spawn(command)
}

pub fn plan_build(to_build: &str) -> CmdResult<Cmd> {
    studio_run("/src/components/plan-build/bin/hab-plan-build.sh",
               &[to_build])
}

pub fn bldr(args: &[&str]) -> CmdResult<Cmd> {
    let bldr = super::path::bldr();
    let command = command(&bldr, args);
    spawn(command)
}
