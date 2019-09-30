use habitat_common::{error::{Error,
                             Result},
                     liveliness_checker,
                     outputln,
                     templating::package::Pkg};
use habitat_core::{env as henv,
                   os::process::windows_child::Child};
use mio::{Events,
          Poll,
          PollOpt,
          Ready,
          Token};
use mio_named_pipes::NamedPipe;
use std::{self,
          env,
          ffi::OsStr,
          fs::{File,
               OpenOptions},
          io::{self,
               prelude::*,
               BufRead,
               BufReader,
               Read},
          iter::once,
          os::windows::{ffi::OsStrExt,
                        fs::*,
                        io::*},
          path::PathBuf,
          thread,
          time::{Duration,
                 Instant}};
use uuid::Uuid;
use winapi::um::{namedpipeapi,
                 processthreadsapi,
                 winbase};

static LOGKEY: &str = "HK";
const PIPE_SERVICE_ENVVAR: &str = "HAB_HOOK_PIPE_SCRIPT";
const SIGNAL_EXEC_HOOK: [u8; 1] = [1];
const SIGNAL_QUIT: [u8; 1] = [0];

habitat_core::env_config_duration!(PipeStartTimeout,
                                    HAB_START_PIPE_TIMEOUT_SECS => from_secs,
                                    Duration::from_secs(5));

#[derive(Debug)]
pub struct PipeHookClient {
    hook_name:       String,
    hook_path:       PathBuf,
    pipe_name:       String,
    stdout_log_file: PathBuf,
    stderr_log_file: PathBuf,
}

impl PipeHookClient {
    pub fn new(hook_name: String,
               hook_path: PathBuf,
               stdout_log_file: PathBuf,
               stderr_log_file: PathBuf)
               -> Self {
        let pipe_name = format!("{}-hook-{}", hook_name, Uuid::new_v4());
        Self { hook_name,
               hook_path,
               pipe_name,
               stdout_log_file,
               stderr_log_file }
    }

    pub fn exec_hook<T>(&self,
                        service_group: &str,
                        pkg: &Pkg,
                        svc_encrypted_password: Option<T>)
                        -> Result<u32>
        where T: ToString
    {
        // Always check to see if the powershell named pipe server is running.
        // This should only be false the first time this function is called
        // or if someone explicitly terminates the powershell process
        if let Err(err) = self.pipe_wait() {
            debug!("Initiating Powershell named pipe server for {} {} hook. last err: {}",
                   service_group, self.hook_name, err);
            self.start_server(service_group, pkg, svc_encrypted_password)?;
        }

        // truncate log files before executing hook
        // the log should only contain output for a single invocation
        File::create(&self.stdout_log_file)?;
        File::create(&self.stderr_log_file)?;

        let (mut pipe, poll) = self.connect()?;
        debug!("connected to {} {} hook pipe",
               service_group, self.hook_name);

        // The powershell server takes a single byte as input which will be either
        // 0 to shut down (see drop below) or 1 to run the hook
        self.pipe_ready(&poll, Ready::writable())?;
        pipe.write_all(&SIGNAL_EXEC_HOOK)?;

        // Now we wait for the hook to run and the powershell service to
        // send back the hook's exit code over the pipe
        self.pipe_ready(&poll, Ready::readable())?;
        let mut exit_buf = [0; std::mem::size_of::<u32>()];
        pipe.read_exact(&mut exit_buf)?;
        Ok(u32::from_ne_bytes(exit_buf))
    }

    fn pipe_ready(&self, poll: &Poll, readiness: Ready) -> io::Result<bool> {
        let mut events = Events::with_capacity(1024);
        let loop_value = loop {
            let checked_thread = liveliness_checker::mark_thread_alive();
            let result =
                poll.poll(&mut events, None)
                    .map(|_| events.iter().any(|e| e.readiness().contains(readiness)));
            if let Ok(false) = result {
                continue;
            } else {
                break checked_thread.unregister(result);
            }
        };
        loop_value.into_result()
    }

    fn abs_pipe_name(&self) -> String { format!(r#"\\.\pipe\{}"#, self.pipe_name) }

    fn connect(&self) -> Result<(NamedPipe, Poll)> {
        let mut opts = OpenOptions::new();
        opts.read(true)
            .write(true)
            .custom_flags(winbase::FILE_FLAG_OVERLAPPED);
        let file = opts.open(self.abs_pipe_name())?;

        let pipe = unsafe { NamedPipe::from_raw_handle(file.into_raw_handle()) };
        let poll = Poll::new()?;
        poll.register(&pipe, Token(0), Ready::all(), PollOpt::edge())?;
        Ok((pipe, poll))
    }

    fn start_server<T>(&self,
                       service_group: &str,
                       pkg: &Pkg,
                       svc_encrypted_password: Option<T>)
                       -> Result<()>
        where T: ToString
    {
        let script_path = match henv::var(PIPE_SERVICE_ENVVAR) {
            Ok(script) => PathBuf::from(script),
            Err(_) => {
                env::current_exe()?.parent()
                                   .expect("unable to find sup binary parent dir")
                                   .join("named_pipe_service.ps1")
            }
        };
        let ps_cmd = format!("& '{}' -HookPath '{}' -PipeName {}",
                             script_path.to_string_lossy(),
                             self.hook_path.to_string_lossy(),
                             self.pipe_name);
        // Start instance of powershell to host named pipe server for this client

        let args = vec!["-NonInteractive", "-Command", ps_cmd.as_str()];
        let child = Child::spawn("pwsh.exe",
                                 &args,
                                 &pkg.env,
                                 &pkg.svc_user,
                                 svc_encrypted_password)?;
        debug!("spawned powershell server for {} {} hook on pipe: {}",
               service_group, self.hook_name, self.pipe_name);

        let out = child.stdout;
        let err = child.stderr;
        let preamble_out_str = format!("{} hook[{}]:", service_group, self.hook_name);
        let preamble_err_str = preamble_out_str.clone();
        let out_path = self.stdout_log_file.clone();
        let err_path = self.stderr_log_file.clone();
        let handle = child.handle;

        // The above powershell will run for the lifetime of this client which is the same
        // duration as the service itself and will thus service each hook run. So we will
        // stream its stdout/err in separate threads
        if let Some(o) = out {
            thread::Builder::new().name(format!("{}-{}-pipe-server-out",
                                                service_group, self.hook_name))
                                  .spawn(move || stream_output(o, &out_path, &preamble_out_str))
                                  .ok();
        }
        if let Some(e) = err {
            thread::Builder::new().name(format!("{}-{}-pipe-server-err",
                                                service_group, self.hook_name))
                                  .spawn(move || stream_output(e, &err_path, &preamble_err_str))
                                  .ok();
        }

        // We want to wait until we know the named pipe is up and running before returning OK
        // If we suspect anything is wrong with the pipe we should terminate the pwsh process
        let start = Instant::now();
        loop {
            match self.pipe_wait() {
                Ok(_) => return Ok(()),
                Err(err) => {
                    if start.elapsed() >= PipeStartTimeout::configured_value().into() {
                        self.win32_result(unsafe {
                                processthreadsapi::TerminateProcess(handle.raw(), 1)
                            })?;
                        return Err(Error::NamedPipeTimeoutOnStart(service_group.to_string(),
                                                                  self.hook_name.to_string(),
                                                                  err));
                    }
                }
            }
            thread::yield_now();
        }
    }

    // Simple call to win32 API to see if named pipe exists.
    // Note that we cannot use Path::exists for named pipes
    fn pipe_wait(&self) -> io::Result<()> {
        let pipe_path: Vec<u16> = OsStr::new(&self.abs_pipe_name()).encode_wide()
                                                                   .chain(once(0))
                                                                   .collect();
        // Note that WaitNamedPipeW will error immediately if the pipe has not yet
        // been created. The timeout is honored if the pipe exists and then the
        // waits for it to be ready for clients
        self.win32_result(unsafe { namedpipeapi::WaitNamedPipeW(pipe_path.as_ptr(), 1000) })?;
        Ok(())
    }

    fn quit(&self) -> Result<()> {
        if let Err(err) = self.pipe_wait() {
            debug!("error checking if pipe exists: {}", err);
        } else {
            debug!("Telling {} pipe server to quit", self.pipe_name);
            let (mut pipe, poll) = self.connect()?;
            self.pipe_ready(&poll, Ready::writable())?;
            pipe.write_all(&SIGNAL_QUIT)?;
        }
        Ok(())
    }

    fn win32_result(&self, i: i32) -> io::Result<i32> {
        if i == 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(i)
        }
    }
}

impl Drop for PipeHookClient {
    fn drop(&mut self) {
        if let Err(err) = self.quit() {
            error!("Unable to tell {} pipe server to quit. {}",
                   self.pipe_name, err);
        }
    }
}

fn stream_output<T>(out: T, log_file: &PathBuf, preamble_str: &str)
    where T: Read
{
    File::create(&log_file).unwrap_or_else(|_| {
                               panic!("{}: couldn't create log output file {}",
                                      preamble_str,
                                      &log_file.to_string_lossy())
                           });

    for line in BufReader::new(out).lines() {
        if let Ok(ref l) = line {
            outputln!(preamble preamble_str, l);
            // we append each line to the log file instead of continuously
            // streaming to an open file because the parent thread needs to
            // truncate the log on each hook execution so that the log only
            // holds the output of the last run. This mimics the behavior of
            // the HookOutput streaming.
            match OpenOptions::new().write(true).append(true).open(&log_file) {
                Ok(mut log) => {
                    if let Err(e) = writeln!(log, "{}", l) {
                        outputln!(preamble preamble_str, "couldn't write line. {}", e);
                    }
                }
                Err(err) => {
                    outputln!(preamble preamble_str, "unable to open log {} : {}",
                            &log_file.to_string_lossy(),
                            err);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use habitat_common::{locked_env_var,
                         templating::{package::Pkg,
                                      test_helpers::{create_with_content,
                                                     file_content}}};
    use habitat_core::{os::process,
                       package::{PackageIdent,
                                 PackageInstall},
                       service::ServiceGroup};
    use tempfile::TempDir;
    use winapi::um::synchapi;

    locked_env_var!(HAB_HOOK_PIPE_SCRIPT, pipe_service_path);

    fn named_pipe_service_ps1() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static")
                                                 .join("named_pipe_service.ps1")
    }

    fn pkg() -> Pkg {
        let service_group = ServiceGroup::new(None, "test_service", "test_group", None).unwrap();
        let pg_id = PackageIdent::new("testing",
                                      &service_group.service(),
                                      Some("1.0.0"),
                                      Some("20170712000000"));
        let pkg_install = PackageInstall::new_from_parts(pg_id.clone(),
                                                         PathBuf::from("/tmp"),
                                                         PathBuf::from("/tmp"),
                                                         PathBuf::from("/tmp"));
        Pkg::from_install(&pkg_install).unwrap()
    }

    #[test]
    fn pipe_hook_client_exec_hook_returns_exit_status() {
        let var = pipe_service_path();
        var.set(&named_pipe_service_ps1());
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("health-check");
        create_with_content(&path, "exit 5000");

        let client = PipeHookClient::new("test".to_string(),
                                         path,
                                         tmpdir.path().join("out.log"),
                                         tmpdir.path().join("err.log"));

        let exit = client.exec_hook("tg1", &pkg(), None::<String>).unwrap();

        assert_eq!(5000, exit);
    }

    #[test]
    fn pipe_hook_client_exec_hook_returns_exit_status_when_no_exit_in_script() {
        let var = pipe_service_path();
        var.set(&named_pipe_service_ps1());
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("health-check");
        create_with_content(&path, "write-host 'no exit here'");

        let client = PipeHookClient::new("test".to_string(),
                                         path,
                                         tmpdir.path().join("out.log"),
                                         tmpdir.path().join("err.log"));

        let exit = client.exec_hook("tg2", &pkg(), None::<String>).unwrap();

        assert_eq!(0, exit);
    }

    #[test]
    fn pipe_hook_client_exec_hook_logs_stdout() {
        let var = pipe_service_path();
        var.set(&named_pipe_service_ps1());
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("health-check");
        create_with_content(&path, "Write-Host 'you are my sunshine'");

        let client = PipeHookClient::new("test".to_string(),
                                         path,
                                         tmpdir.path().join("out.log"),
                                         tmpdir.path().join("err.log"));

        client.exec_hook("tg3", &pkg(), None::<String>).unwrap();

        // give stream a chance to write
        thread::sleep(Duration::from_millis(10));

        assert!(file_content(tmpdir.path().join("out.log")).contains("you are my sunshine\n"));
    }

    #[test]
    fn pipe_hook_client_exec_hook_logs_stderr() {
        let var = pipe_service_path();
        var.set(&named_pipe_service_ps1());
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("health-check");
        create_with_content(&path, "Write-Error 'you are not my sunshine'");

        let client = PipeHookClient::new("test".to_string(),
                                         path,
                                         tmpdir.path().join("out.log"),
                                         tmpdir.path().join("err.log"));

        client.exec_hook("tg4", &pkg(), None::<String>).unwrap();

        // give stream a chance to write
        thread::sleep(Duration::from_millis(10));

        assert!(file_content(tmpdir.path().join("err.log")).contains("you are not my sunshine\n"));
    }

    #[test]
    fn pipe_hook_client_exec_hook_shares_server_accross_calls() {
        let var = pipe_service_path();
        var.set(&named_pipe_service_ps1());
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("health-check");
        create_with_content(&path, "exit $PID");

        let client = PipeHookClient::new("test".to_string(),
                                         path,
                                         tmpdir.path().join("out.log"),
                                         tmpdir.path().join("err.log"));

        let pid1 = client.exec_hook("tg5", &pkg(), None::<String>).unwrap();
        let pid2 = client.exec_hook("tg5", &pkg(), None::<String>).unwrap();

        assert_eq!(pid1, pid2);
    }

    #[test]
    fn pipe_hook_client_start_server_terminates_failed_server() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("fake-server.ps1");
        create_with_content(&path,
                            "write-host $PID;while($true){Start-Sleep -Seconds 5}");
        let var = pipe_service_path();
        var.set(&path);
        let client = PipeHookClient::new("test".to_string(),
                                         tmpdir.path().join("health-check"),
                                         tmpdir.path().join("out.log"),
                                         tmpdir.path().join("err.log"));

        let result = client.exec_hook("tg6", &pkg(), None::<String>);

        // give stream a chance to write
        thread::sleep(Duration::from_millis(10));
        let pid_str = file_content(tmpdir.path().join("out.log"));

        assert!(result.is_err());
        assert!(!process::is_alive(pid_str.trim()
                                          .parse::<process::Pid>()
                                          .expect("could no parse pid")));
    }

    #[test]
    fn pipe_hook_client_start_server_logs_stderr_of_failed_server() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("fake-server.ps1");
        create_with_content(&path, "write-error 'I am not a real pipe server'");
        let var = pipe_service_path();
        var.set(&path);
        let client = PipeHookClient::new("test".to_string(),
                                         tmpdir.path().join("health-check"),
                                         tmpdir.path().join("out.log"),
                                         tmpdir.path().join("err.log"));

        let result = client.exec_hook("tg7", &pkg(), None::<String>);

        // give stream a chance to write
        thread::sleep(Duration::from_millis(10));

        assert!(result.is_err());
        assert!(file_content(tmpdir.path().join("err.log")).contains("I am not a real pipe \
                                                                      server"));
    }

    #[test]
    fn pipe_hook_client_start_server_logs_stdout_of_failed_server() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("fake-server.ps1");
        create_with_content(&path, "write-host 'I am not a real pipe server'");
        let var = pipe_service_path();
        var.set(&path);
        let client = PipeHookClient::new("test".to_string(),
                                         tmpdir.path().join("health-check"),
                                         tmpdir.path().join("out.log"),
                                         tmpdir.path().join("err.log"));

        let result = client.exec_hook("tg8", &pkg(), None::<String>);

        // give stream a chance to write
        thread::sleep(Duration::from_millis(10));

        assert!(result.is_err());
        assert_eq!("I am not a real pipe server\n",
                   file_content(tmpdir.path().join("out.log")));
    }

    #[test]
    fn pipe_hook_client_exec_hook_starts_new_service_if_current_instance_exits() {
        let var = pipe_service_path();
        var.set(&named_pipe_service_ps1());
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("health-check");
        create_with_content(&path, "exit $PID");

        let client = PipeHookClient::new("test".to_string(),
                                         path,
                                         tmpdir.path().join("out.log"),
                                         tmpdir.path().join("err.log"));

        let pid1 = client.exec_hook("tg9", &pkg(), None::<String>).unwrap();

        let handle = process::handle_from_pid(pid1).expect("unable to get handle to pipe server");
        unsafe {
            processthreadsapi::TerminateProcess(handle, 1);
        }

        let pid2 = client.exec_hook("tg9", &pkg(), None::<String>).unwrap();

        assert_ne!(pid1, pid2);
    }

    #[test]
    fn pipe_hook_client_exec_hook_restores_environment() {
        let var = pipe_service_path();
        var.set(&named_pipe_service_ps1());
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("health-check");
        create_with_content(&path,
                            "if($env:test_var) { exit 10 } else { $env:test_var=1;exit 0 }");

        let client = PipeHookClient::new("test".to_string(),
                                         path,
                                         tmpdir.path().join("out.log"),
                                         tmpdir.path().join("err.log"));

        let exit1 = client.exec_hook("tg10", &pkg(), None::<String>).unwrap();
        let exit2 = client.exec_hook("tg10", &pkg(), None::<String>).unwrap();

        assert_eq!(0, exit1);
        assert_eq!(0, exit2);
    }

    #[test]
    fn pipe_hook_client_drop_quits_service() {
        let var = pipe_service_path();
        var.set(&named_pipe_service_ps1());
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("health-check");
        create_with_content(&path, "exit $PID");

        let pid = {
            let client = PipeHookClient::new("test".to_string(),
                                             tmpdir.path().join("health-check"),
                                             tmpdir.path().join("out.log"),
                                             tmpdir.path().join("err.log"));

            client.exec_hook("tg11", &pkg(), None::<String>).unwrap()
        };
        let handle = process::handle_from_pid(pid).expect("unable to get handle to pipe server");
        unsafe {
            synchapi::WaitForSingleObject(handle, 100);
        }

        assert!(!process::is_alive(pid));
    }

    #[test]
    fn pipe_hook_client_exec_hook_will_return_3_if_hook_throws_exception() {
        let var = pipe_service_path();
        var.set(&named_pipe_service_ps1());
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("health-check");
        create_with_content(&path, "throw 'an exception'");

        let client = PipeHookClient::new("test".to_string(),
                                         path,
                                         tmpdir.path().join("out.log"),
                                         tmpdir.path().join("err.log"));

        let exit = client.exec_hook("tg12", &pkg(), None::<String>).unwrap();

        assert_eq!(3, exit);
    }

    #[test]
    fn pipe_hook_client_exec_hook_clears_stdout_log() {
        let var = pipe_service_path();
        var.set(&named_pipe_service_ps1());
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("health-check");
        create_with_content(&path, "write-host 'you should only see me once in the log'");

        let client = PipeHookClient::new("test".to_string(),
                                         path,
                                         tmpdir.path().join("out.log"),
                                         tmpdir.path().join("err.log"));

        client.exec_hook("tg13", &pkg(), None::<String>).unwrap();
        client.exec_hook("tg13", &pkg(), None::<String>).unwrap();

        // give stream a chance to write
        thread::sleep(Duration::from_millis(10));

        assert_eq!("you should only see me once in the log\n",
                   file_content(tmpdir.path().join("out.log")));
    }

    #[test]
    fn pipe_hook_client_exec_hook_clears_stderr_log() {
        let var = pipe_service_path();
        var.set(&named_pipe_service_ps1());
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("health-check");
        create_with_content(&path, "write-error 'I am the only error'");

        let client = PipeHookClient::new("test".to_string(),
                                         path,
                                         tmpdir.path().join("out.log"),
                                         tmpdir.path().join("err.log"));

        client.exec_hook("tg13", &pkg(), None::<String>).unwrap();
        client.exec_hook("tg13", &pkg(), None::<String>).unwrap();

        // give stream a chance to write
        thread::sleep(Duration::from_millis(10));

        let content = file_content(tmpdir.path().join("err.log"));
        assert_eq!(content.find("I am the only error"),
                   content.rfind("I am the only error"));
    }
}
