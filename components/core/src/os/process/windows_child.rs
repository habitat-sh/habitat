// This contains code largely copy/pasted from the rust-lang/rust repo
// We will need to create processes using different windows API calls in
// order to be able to start habitat Supervisor services as different users.

use super::super::{super::crypto::dpapi::decrypt,
                   users::get_current_username};
use crate::error::{Error,
                   Result};
use habitat_win_users::sid::{self,
                             Sid};
use log::debug;
use rand::{self,
           Rng};
use std::{cmp,
          collections::HashMap,
          env,
          ffi::{OsStr,
                OsString},
          fmt,
          fs,
          io::{self,
               ErrorKind,
               Read},
          mem,
          ops::Deref,
          os::windows::ffi::OsStrExt,
          path::Path,
          ptr,
          slice::from_raw_parts_mut,
          sync::Mutex};
use widestring::WideCString;
use winapi::{shared::{minwindef::{BOOL,
                                  DWORD,
                                  FALSE,
                                  HWINSTA,
                                  LPVOID,
                                  TRUE},
                      windef::HDESK,
                      winerror::{ERROR_ACCESS_DENIED,
                                 ERROR_BROKEN_PIPE,
                                 ERROR_HANDLE_EOF,
                                 ERROR_INVALID_PARAMETER,
                                 ERROR_IO_PENDING,
                                 ERROR_LOGON_TYPE_NOT_GRANTED,
                                 ERROR_PRIVILEGE_NOT_HELD}},
             um::{fileapi::{self,
                            CREATE_ALWAYS,
                            CREATE_NEW,
                            OPEN_ALWAYS,
                            OPEN_EXISTING,
                            TRUNCATE_EXISTING},
                  handleapi::{self,
                              INVALID_HANDLE_VALUE},
                  ioapiset,
                  minwinbase::{LPSECURITY_ATTRIBUTES,
                               OVERLAPPED,
                               SECURITY_ATTRIBUTES},
                  namedpipeapi,
                  processthreadsapi::{self,
                                      LPPROCESS_INFORMATION,
                                      LPSTARTUPINFOW,
                                      PROCESS_INFORMATION,
                                      STARTUPINFOW},
                  synchapi,
                  userenv,
                  winbase::{CREATE_NEW_PROCESS_GROUP,
                            CREATE_UNICODE_ENVIRONMENT,
                            FILE_FLAG_FIRST_PIPE_INSTANCE,
                            FILE_FLAG_OPEN_REPARSE_POINT,
                            FILE_FLAG_OVERLAPPED,
                            INFINITE,
                            PIPE_ACCESS_INBOUND,
                            PIPE_ACCESS_OUTBOUND,
                            PIPE_READMODE_BYTE,
                            PIPE_REJECT_REMOTE_CLIENTS,
                            PIPE_TYPE_BYTE,
                            PIPE_WAIT,
                            SECURITY_SQOS_PRESENT,
                            STARTF_USESTDHANDLES,
                            STD_ERROR_HANDLE,
                            STD_INPUT_HANDLE,
                            STD_OUTPUT_HANDLE,
                            WAIT_OBJECT_0},
                  winnt::{ACCESS_MASK,
                          FILE_GENERIC_WRITE,
                          FILE_SHARE_DELETE,
                          FILE_SHARE_READ,
                          FILE_SHARE_WRITE,
                          FILE_WRITE_DATA,
                          GENERIC_READ,
                          GENERIC_WRITE,
                          HANDLE,
                          LPCWSTR,
                          LPWSTR,
                          MAXDWORD,
                          PHANDLE,
                          READ_CONTROL,
                          WRITE_DAC}}};

lazy_static::lazy_static! {
    static ref CREATE_PROCESS_LOCK: Mutex<()> = Mutex::new(());
}

#[link(name = "user32")]
extern "system" {
    fn LogonUserW(lpszUsername: LPCWSTR,
                  lpszDomain: LPCWSTR,
                  lpszPassword: LPCWSTR,
                  dwLogonType: DWORD,
                  dwLogonProvider: DWORD,
                  phToken: PHANDLE)
                  -> BOOL;

    fn CreateProcessAsUserW(hToken: HANDLE,
                            lpApplicationName: LPCWSTR,
                            lpCommandLine: LPWSTR,
                            lpProcessAttributes: LPSECURITY_ATTRIBUTES,
                            lpThreadAttributes: LPSECURITY_ATTRIBUTES,
                            bInheritHandles: BOOL,
                            dwCreationFlags: DWORD,
                            lpEnvironment: LPVOID,
                            lpCurrentDirectory: LPCWSTR,
                            lpStartupInfo: LPSTARTUPINFOW,
                            lpProcessInformation: LPPROCESS_INFORMATION)
                            -> BOOL;

    fn GetProcessWindowStation() -> HWINSTA;

    fn OpenDesktopW(lpszDesktop: LPWSTR,
                    dwFlags: DWORD,
                    fInherit: BOOL,
                    dwDesiredAccess: ACCESS_MASK)
                    -> HDESK;
}

const HANDLE_FLAG_INHERIT: DWORD = 0x0000_0001;

const LOGON32_LOGON_SERVICE: DWORD = 5;

enum ParsePart {
    Key,
    Value,
}

struct ServiceCredential {
    pub user:     String,
    pub domain:   String,
    pub password: String,
}

impl ServiceCredential {
    // TODO JB: fix this allow
    #[allow(clippy::needless_pass_by_value)]
    pub fn new<U, P>(svc_user: U, svc_encrypted_password: Option<P>) -> Result<Self>
        where U: ToString,
              P: ToString
    {
        let mut full_user = svc_user.to_string();
        let (domain, user) = match full_user.find('\\') {
            Some(idx) => {
                let u = full_user.split_off(idx);
                (full_user, u.trim_matches('\\').to_string())
            }
            None => (".".to_string(), full_user),
        };
        let pass = match svc_encrypted_password {
            Some(password) => decrypt(&password.to_string())?,
            None => String::new(),
        };
        Ok(Self { user,
                  domain,
                  password: pass })
    }

    pub fn is_current_user(&self) -> bool {
        self.user == get_current_username().ok().flatten().unwrap_or_default()
    }

    pub fn user_wide(&self) -> WideCString { WideCString::from_str(self.user.as_str()).unwrap() }

    pub fn domain_wide(&self) -> WideCString {
        WideCString::from_str(self.domain.as_str()).unwrap()
    }

    pub fn password_wide(&self) -> WideCString {
        WideCString::from_str(self.password.as_str()).unwrap()
    }
}

pub struct Child {
    pub handle: Handle,
    pub stdout: Option<ChildStdout>,
    pub stderr: Option<ChildStderr>,
}

impl Child {
    pub fn spawn<U, P>(program: &str,
                       args: &[&str],
                       env: &HashMap<String, String>,
                       svc_user: U,
                       svc_encrypted_password: Option<P>)
                       -> Result<Child>
        where U: ToString,
              P: ToString
    {
        debug!("in spawn");
        let mut os_env: HashMap<OsString, OsString> =
            env::vars_os().map(|(key, val)| (mk_key(key.to_str().unwrap()), val))
                          .collect();
        for (k, v) in env {
            os_env.insert(mk_key(k.as_str()), OsString::from(v));
        }

        let program_path = {
            let mut res = None;
            for (key, v) in &os_env {
                if &OsString::from("PATH") != key {
                    continue;
                }
                // Split the value and test each path to see if the
                // program exists.
                for path in env::split_paths(&v) {
                    let path = path.join(program)
                                   .with_extension(env::consts::EXE_EXTENSION);
                    if fs::metadata(&path).is_ok() {
                        res = Some(path.into_os_string());
                        break;
                    }
                }
                break;
            }
            res
        };
        debug!("in spawn2");

        let mut si = zeroed_startupinfo();
        si.cb = mem::size_of::<STARTUPINFOW>() as DWORD;
        si.dwFlags = STARTF_USESTDHANDLES;

        let program_path = program_path.unwrap_or_else(|| OsStr::new(program).to_os_string());
        let mut cmd_str = make_command_line(&program_path, args)?;
        cmd_str.push(0); // add null terminator

        let mut pi = zeroed_process_information();

        // Prepare all stdio handles to be inherited by the child. This
        // currently involves duplicating any existing ones with the ability to
        // be inherited by child processes. Note, however, that once an
        // inheritable handle is created, *any* spawned child will inherit that
        // handle. We only want our own child to inherit this handle, so we wrap
        // the remaining portion of this spawn in a mutex.
        //
        // For more information, msdn also has an article about this race:
        // http://support.microsoft.com/kb/315939
        //
        // The Rust standard library uses a similar lock[1] when spawning a process. Race
        // conditions can easily occur because these two implementations do not share the same
        // lock. We should be careful that spawning processes using the standard library and our
        // custom implementation never occur simultaneously.
        //
        // [1] https://github.com/rust-lang/rust/blob/1bd30ce2aac40c7698aa4a1b9520aa649ff2d1c5/src/libstd/sys/windows/process.rs#L179
        let _lock = CREATE_PROCESS_LOCK.lock().unwrap();

        let mut pipes = StdioPipes { stdin:  None,
                                     stdout: None,
                                     stderr: None, };

        let stdin = null_stdio_handle()?;
        let stdout = stdio_piped_handle(STD_OUTPUT_HANDLE, &mut pipes.stdout)?;
        let stderr = stdio_piped_handle(STD_ERROR_HANDLE, &mut pipes.stderr)?;
        si.hStdInput = stdin.raw();
        si.hStdOutput = stdout.raw();
        si.hStdError = stderr.raw();
        let flags = CREATE_UNICODE_ENVIRONMENT | CREATE_NEW_PROCESS_GROUP;

        let cred = ServiceCredential::new(svc_user, svc_encrypted_password)?;
        debug!("in spawn: create process");
        if cred.is_current_user() {
            create_process(cmd_str.as_mut_ptr(), flags, &os_env, &mut si, &mut pi)?;
        } else {
            create_process_as_user(&cred, cmd_str.as_mut_ptr(), flags, env, &mut si, &mut pi)?;
        }
        debug!("in spawn: process created");
        // We close the thread handle because we don't care about keeping
        // the thread id valid, and we aren't keeping the thread handle
        // around to be able to close it later.
        unsafe { handleapi::CloseHandle(pi.hThread) };
        debug!("in spawn finishing");
        Ok(Child { handle: Handle::new(pi.hProcess),
                   stdout: pipes.stdout.map(ChildStdout::from_inner),
                   stderr: pipes.stderr.map(ChildStderr::from_inner), })
    }

    pub fn id(&self) -> u32 { unsafe { processthreadsapi::GetProcessId(self.handle.raw()) } }

    pub fn kill(&mut self) -> io::Result<()> {
        cvt(unsafe { processthreadsapi::TerminateProcess(self.handle.raw(), 1) })?;
        Ok(())
    }

    pub fn wait(&mut self) -> Result<ExitStatus> {
        unsafe {
            let res = synchapi::WaitForSingleObject(self.handle.raw(), INFINITE);
            if res != WAIT_OBJECT_0 {
                return Err(Error::WaitForSingleObjectFailed(format!(
                    "Failed calling WaitForSingleObjectFailed: {}",
                    io::Error::last_os_error()
                )));
            }
            let mut status = 0;
            cvt(processthreadsapi::GetExitCodeProcess(self.handle.raw(), &mut status))?;
            Ok(ExitStatus(status))
        }
    }
}

pub trait AsInner<Inner: ?Sized> {
    fn as_inner(&self) -> &Inner;
}

pub trait IntoInner<Inner> {
    fn into_inner(self) -> Inner;
}

pub trait FromInner<Inner> {
    fn from_inner(inner: Inner) -> Self;
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ExitStatus(DWORD);

impl ExitStatus {
    // TODO JB: fix this allow
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn success(&self) -> bool { self.0 == 0 }

    // TODO JB: fix this allow
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn code(&self) -> Option<i32> { Some(self.0 as i32) }
}

impl From<DWORD> for ExitStatus {
    fn from(u: DWORD) -> ExitStatus { ExitStatus(u) }
}

impl fmt::Display for ExitStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "exit code: {}", self.0) }
}

pub struct ChildStdout {
    inner: AnonPipe,
}

impl Read for ChildStdout {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { self.inner.read(buf) }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.inner.read_to_end(buf)
    }
}

impl AsInner<AnonPipe> for ChildStdout {
    fn as_inner(&self) -> &AnonPipe { &self.inner }
}

impl IntoInner<AnonPipe> for ChildStdout {
    fn into_inner(self) -> AnonPipe { self.inner }
}

impl FromInner<AnonPipe> for ChildStdout {
    fn from_inner(pipe: AnonPipe) -> ChildStdout { ChildStdout { inner: pipe } }
}

impl fmt::Debug for ChildStdout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { f.pad("ChildStdout { .. }") }
}

pub struct ChildStderr {
    inner: AnonPipe,
}

impl Read for ChildStderr {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { self.inner.read(buf) }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.inner.read_to_end(buf)
    }
}

impl AsInner<AnonPipe> for ChildStderr {
    fn as_inner(&self) -> &AnonPipe { &self.inner }
}

impl IntoInner<AnonPipe> for ChildStderr {
    fn into_inner(self) -> AnonPipe { self.inner }
}

impl FromInner<AnonPipe> for ChildStderr {
    fn from_inner(pipe: AnonPipe) -> ChildStderr { ChildStderr { inner: pipe } }
}

impl fmt::Debug for ChildStderr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { f.pad("ChildStderr { .. }") }
}

pub struct AnonPipe {
    inner: Handle,
}

pub struct Pipes {
    pub ours:   AnonPipe,
    pub theirs: AnonPipe,
}

pub fn anon_pipe(ours_readable: bool) -> io::Result<Pipes> {
    // Note that we specifically do *not* use `CreatePipe` here because
    // unfortunately the anonymous pipes returned do not support overlapped
    // operations. Instead, we create a "hopefully unique" name and create a
    // named pipe which has overlapped operations enabled.
    //
    // Once we do this, we connect do it as usual via `CreateFileW`, and then
    // we return those reader/writer halves. Note that the `ours` pipe return
    // value is always the named pipe, whereas `theirs` is just the normal file.
    // This should hopefully shield us from child processes which assume their
    // stdout is a named pipe, which would indeed be odd!
    unsafe {
        let ours;
        let mut name;
        let mut tries = 0;
        let mut reject_remote_clients_flag = PIPE_REJECT_REMOTE_CLIENTS;
        loop {
            tries += 1;
            let key: u64 = rand::thread_rng().gen();
            name = format!(r"\\.\pipe\__rust_anonymous_pipe1__.{}.{}",
                           processthreadsapi::GetCurrentProcessId(),
                           key);
            let wide_name = OsStr::new(&name).encode_wide()
                                             .chain(Some(0))
                                             .collect::<Vec<_>>();
            let mut flags = FILE_FLAG_FIRST_PIPE_INSTANCE | FILE_FLAG_OVERLAPPED;
            if ours_readable {
                flags |= PIPE_ACCESS_INBOUND;
            } else {
                flags |= PIPE_ACCESS_OUTBOUND;
            }

            let handle = namedpipeapi::CreateNamedPipeW(wide_name.as_ptr(),
                                                        flags,
                                                        PIPE_TYPE_BYTE
                                                        | PIPE_READMODE_BYTE
                                                        | PIPE_WAIT
                                                        | reject_remote_clients_flag,
                                                        1,
                                                        4096,
                                                        4096,
                                                        0,
                                                        ptr::null_mut());

            // We pass the FILE_FLAG_FIRST_PIPE_INSTANCE flag above, and we're
            // also just doing a best effort at selecting a unique name. If
            // ERROR_ACCESS_DENIED is returned then it could mean that we
            // accidentally conflicted with an already existing pipe, so we try
            // again.
            //
            // Don't try again too much though as this could also perhaps be a
            // legit error.
            // If ERROR_INVALID_PARAMETER is returned, this probably means we're
            // running on pre-Vista version where PIPE_REJECT_REMOTE_CLIENTS is
            // not supported, so we continue retrying without it. This implies
            // reduced security on Windows versions older than Vista by allowing
            // connections to this pipe from remote machines.
            // Proper fix would increase the number of FFI imports and introduce
            // significant amount of Windows XP specific code with no clean
            // testing strategy
            // for more info see https://github.com/rust-lang/rust/pull/37677
            if handle == INVALID_HANDLE_VALUE {
                let err = io::Error::last_os_error();
                let raw_os_err = err.raw_os_error();
                if tries < 10 {
                    if raw_os_err == Some(ERROR_ACCESS_DENIED as i32) {
                        continue;
                    } else if reject_remote_clients_flag != 0
                              && raw_os_err == Some(ERROR_INVALID_PARAMETER as i32)
                    {
                        reject_remote_clients_flag = 0;
                        tries -= 1;
                        continue;
                    }
                }
                return Err(err);
            }
            ours = Handle::new(handle);
            break;
        }

        // Connect to the named pipe we just created. This handle is going to be
        // returned in `theirs`, so if `ours` is readable we want this to be
        // writable, otherwise if `ours` is writable we want this to be
        // readable.
        //
        // Additionally we don't enable overlapped mode on this because most
        // client processes aren't enabled to work with that.
        let mut opts = OpenOptions::new();
        opts.write(ours_readable);
        opts.read(!ours_readable);
        opts.share_mode(0);
        let theirs = File::open(Path::new(&name), &opts)?;
        let theirs = AnonPipe { inner: theirs.into_handle(), };

        Ok(Pipes { ours:   AnonPipe { inner: ours },
                   theirs: AnonPipe { inner: theirs.into_handle(), }, })
    }
}

impl AnonPipe {
    pub fn handle(&self) -> &Handle { &self.inner }

    pub fn into_handle(self) -> Handle { self.inner }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> { self.inner.read(buf) }

    pub fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.inner.read_to_end(buf)
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> { self.inner.write(buf) }
}

struct StdioPipes {
    #[allow(dead_code)]
    pub stdin:  Option<AnonPipe>,
    pub stdout: Option<AnonPipe>,
    pub stderr: Option<AnonPipe>,
}

#[derive(Clone)]
pub struct OpenOptions {
    // generic
    read:                bool,
    write:               bool,
    append:              bool,
    truncate:            bool,
    create:              bool,
    create_new:          bool,
    // system-specific
    custom_flags:        u32,
    access_mode:         Option<DWORD>,
    attributes:          DWORD,
    share_mode:          DWORD,
    security_qos_flags:  DWORD,
    security_attributes: usize, // FIXME: should be a reference
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions { // generic
                      read:                false,
                      write:               false,
                      append:              false,
                      truncate:            false,
                      create:              false,
                      create_new:          false,
                      // system-specific
                      custom_flags:        0,
                      access_mode:         None,
                      share_mode:          FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
                      attributes:          0,
                      security_qos_flags:  0,
                      security_attributes: 0, }
    }

    pub fn read(&mut self, read: bool) { self.read = read; }

    pub fn write(&mut self, write: bool) { self.write = write; }

    pub fn share_mode(&mut self, share_mode: u32) { self.share_mode = share_mode; }

    pub fn attributes(&mut self, attrs: u32) { self.attributes = attrs; }

    pub fn security_attributes(&mut self, attrs: LPSECURITY_ATTRIBUTES) {
        self.security_attributes = attrs as usize;
    }

    fn get_access_mode(&self) -> io::Result<DWORD> {
        const ERROR_INVALID_PARAMETER: i32 = 87;

        match (self.read, self.write, self.append, self.access_mode) {
            (_, _, _, Some(mode)) => Ok(mode),
            (true, false, false, None) => Ok(GENERIC_READ),
            (false, true, false, None) => Ok(GENERIC_WRITE),
            (true, true, false, None) => Ok(GENERIC_READ | GENERIC_WRITE),
            (false, _, true, None) => Ok(FILE_GENERIC_WRITE & !FILE_WRITE_DATA),
            (true, _, true, None) => Ok(GENERIC_READ | (FILE_GENERIC_WRITE & !FILE_WRITE_DATA)),
            (false, false, false, None) => {
                Err(io::Error::from_raw_os_error(ERROR_INVALID_PARAMETER))
            }
        }
    }

    fn get_creation_mode(&self) -> io::Result<DWORD> {
        const ERROR_INVALID_PARAMETER: i32 = 87;

        match (self.write, self.append) {
            (true, false) => {}
            (false, false) => {
                if self.truncate || self.create || self.create_new {
                    return Err(io::Error::from_raw_os_error(ERROR_INVALID_PARAMETER));
                }
            }
            (_, true) => {
                if self.truncate && !self.create_new {
                    return Err(io::Error::from_raw_os_error(ERROR_INVALID_PARAMETER));
                }
            }
        }

        Ok(match (self.create, self.truncate, self.create_new) {
            (false, false, false) => OPEN_EXISTING,
            (true, false, false) => OPEN_ALWAYS,
            (false, true, false) => TRUNCATE_EXISTING,
            (true, true, false) => CREATE_ALWAYS,
            (_, _, true) => CREATE_NEW,
        })
    }

    fn get_flags_and_attributes(&self) -> DWORD {
        self.custom_flags
        | self.attributes
        | self.security_qos_flags
        | if self.security_qos_flags != 0 {
            SECURITY_SQOS_PRESENT
        } else {
            0
        }
        | if self.create_new {
            FILE_FLAG_OPEN_REPARSE_POINT
        } else {
            0
        }
    }
}

pub struct File {
    handle: Handle,
}

impl File {
    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
        let path = to_u16s(path)?;
        let handle = unsafe {
            fileapi::CreateFileW(path.as_ptr(),
                                 opts.get_access_mode()?,
                                 opts.share_mode,
                                 opts.security_attributes as *mut _,
                                 opts.get_creation_mode()?,
                                 opts.get_flags_and_attributes(),
                                 ptr::null_mut())
        };
        if handle == INVALID_HANDLE_VALUE {
            Err(io::Error::last_os_error())
        } else {
            Ok(File { handle: Handle::new(handle), })
        }
    }

    pub fn into_handle(self) -> Handle { self.handle }
}

pub struct Handle(RawHandle);

#[derive(Copy, Clone)]
pub struct RawHandle(HANDLE);

unsafe impl Send for RawHandle {}
unsafe impl Sync for RawHandle {}

impl Handle {
    pub fn new(handle: HANDLE) -> Handle { Handle(RawHandle::new(handle)) }

    pub fn new_event(manual: bool, init: bool) -> io::Result<Handle> {
        unsafe {
            let event = synchapi::CreateEventW(ptr::null_mut(),
                                               i32::from(manual),
                                               i32::from(init),
                                               ptr::null());
            if event.is_null() {
                Err(io::Error::last_os_error())
            } else {
                Ok(Handle::new(event))
            }
        }
    }

    pub fn into_raw(self) -> HANDLE {
        let ret = self.raw();
        mem::forget(self);
        ret
    }
}

impl Deref for Handle {
    type Target = RawHandle;

    fn deref(&self) -> &RawHandle { &self.0 }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            let _ = handleapi::CloseHandle(self.raw());
        }
    }
}

impl RawHandle {
    pub fn new(handle: HANDLE) -> RawHandle { RawHandle(handle) }

    // TODO JB: fix this allow
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn raw(&self) -> HANDLE { self.0 }

    // TODO JB: fix this allow
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let mut read = 0;
        let len = cmp::min(buf.len(), <DWORD>::MAX as usize) as DWORD;
        let res = cvt(unsafe {
            fileapi::ReadFile(self.0,
                              buf.as_mut_ptr() as LPVOID,
                              len,
                              &mut read,
                              ptr::null_mut())
        });

        match res {
            Ok(_) => Ok(read as usize),

            // The special treatment of BrokenPipe is to deal with Windows
            // pipe semantics, which yields this error when *reading* from
            // a pipe after the other end has closed; we interpret that as
            // EOF on the pipe.
            Err(ref e) if e.kind() == ErrorKind::BrokenPipe => Ok(0),

            Err(e) => Err(e),
        }
    }

    // TODO JB: fix this allow
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        let mut read = 0;
        let len = cmp::min(buf.len(), <DWORD>::MAX as usize) as DWORD;
        let res = unsafe {
            let mut overlapped: OVERLAPPED = mem::zeroed();
            overlapped.u.s_mut().Offset = offset as u32;
            overlapped.u.s_mut().OffsetHigh = (offset >> 32) as u32;
            cvt(fileapi::ReadFile(self.0,
                                  buf.as_mut_ptr() as LPVOID,
                                  len,
                                  &mut read,
                                  &mut overlapped))
        };
        match res {
            Ok(_) => Ok(read as usize),
            Err(ref e) if e.raw_os_error() == Some(ERROR_HANDLE_EOF as i32) => Ok(0),
            Err(e) => Err(e),
        }
    }

    // TODO JB: fix this allow
    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn read_overlapped(&self,
                                  buf: &mut [u8],
                                  overlapped: *mut OVERLAPPED)
                                  -> io::Result<Option<usize>> {
        let len = cmp::min(buf.len(), <DWORD>::MAX as usize) as DWORD;
        let mut amt = 0;
        let res = cvt(fileapi::ReadFile(self.0,
                                        buf.as_ptr() as LPVOID,
                                        len,
                                        &mut amt,
                                        overlapped));
        match res {
            Ok(_) => Ok(Some(amt as usize)),
            Err(e) => {
                if e.raw_os_error() == Some(ERROR_IO_PENDING as i32) {
                    Ok(None)
                } else if e.raw_os_error() == Some(ERROR_BROKEN_PIPE as i32) {
                    Ok(Some(0))
                } else {
                    Err(e)
                }
            }
        }
    }

    // TODO JB: fix this allow
    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn overlapped_result(&self,
                                    overlapped: *mut OVERLAPPED,
                                    wait: bool)
                                    -> io::Result<usize> {
        let mut bytes = 0;
        let wait = if wait { TRUE } else { FALSE };
        let res = cvt(ioapiset::GetOverlappedResult(self.raw(),
                                                    overlapped,
                                                    &mut bytes,
                                                    wait));
        match res {
            Ok(_) => Ok(bytes as usize),
            Err(e) => {
                if e.raw_os_error() == Some(ERROR_HANDLE_EOF as i32)
                   || e.raw_os_error() == Some(ERROR_BROKEN_PIPE as i32)
                {
                    Ok(0)
                } else {
                    Err(e)
                }
            }
        }
    }

    // TODO JB: fix this allow
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn cancel_io(&self) -> io::Result<()> {
        unsafe { cvt(ioapiset::CancelIo(self.raw())).map(|_| ()) }
    }

    // TODO JB: fix this allow
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let mut me = self;
        (&mut me).read_to_end(buf)
    }

    // TODO JB: fix this allow
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let mut amt = 0;
        let len = cmp::min(buf.len(), <DWORD>::MAX as usize) as DWORD;
        cvt(unsafe {
            fileapi::WriteFile(self.0,
                               buf.as_ptr() as LPVOID,
                               len,
                               &mut amt,
                               ptr::null_mut())
        })?;
        Ok(amt as usize)
    }

    // TODO JB: fix this allow
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
        let mut written = 0;
        let len = cmp::min(buf.len(), <DWORD>::MAX as usize) as DWORD;
        unsafe {
            let mut overlapped: OVERLAPPED = mem::zeroed();
            overlapped.u.s_mut().Offset = offset as u32;
            overlapped.u.s_mut().OffsetHigh = (offset >> 32) as u32;
            cvt(fileapi::WriteFile(self.0,
                                   buf.as_ptr() as LPVOID,
                                   len,
                                   &mut written,
                                   &mut overlapped))?;
        }
        Ok(written as usize)
    }
}

impl<'a> Read for &'a RawHandle {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { (**self).read(buf) }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        unsafe { read_to_end_uninitialized(self, buf) }
    }
}

fn create_process(command: LPWSTR,
                  flags: DWORD,
                  env: &HashMap<OsString, OsString>,
                  si: LPSTARTUPINFOW,
                  pi: LPPROCESS_INFORMATION)
                  -> io::Result<i32> {
    let (envp, _data) = make_envp(env)?;

    unsafe {
        cvt(processthreadsapi::CreateProcessW(ptr::null(),
                                              command,
                                              ptr::null_mut(),
                                              ptr::null_mut(),
                                              TRUE,
                                              flags,
                                              envp,
                                              ptr::null(),
                                              si,
                                              pi))
    }
}

fn create_process_as_user(credential: &ServiceCredential,
                          command: LPWSTR,
                          flags: DWORD,
                          env: &HashMap<String, String>,
                          si: LPSTARTUPINFOW,
                          pi: LPPROCESS_INFORMATION)
                          -> Result<i32> {
    unsafe {
        debug!("in create_process_as_user");
        let mut token = ptr::null_mut();

        match cvt(LogonUserW(credential.user_wide().as_ptr(),
                             credential.domain_wide().as_ptr(),
                             credential.password_wide().as_ptr(),
                             LOGON32_LOGON_SERVICE,
                             0,
                             &mut token))
        {
            Ok(_) => {}
            Err(ref err) if err.raw_os_error() == Some(ERROR_LOGON_TYPE_NOT_GRANTED as i32) => {
                return Err(Error::LogonTypeNotGranted);
            }
            Err(_) => return Err(Error::LogonUserFailed(io::Error::last_os_error())),
        };

        debug!("in create_process_as_user: done with logon");

        let station = GetProcessWindowStation();

        let desktop = WideCString::from_str("default").unwrap();
        let hdesk = OpenDesktopW(desktop.into_raw(),
                                 0,
                                 FALSE,
                                 READ_CONTROL
                                 | WRITE_DAC
                                 | sid::DESKTOP_WRITEOBJECTS
                                 | sid::DESKTOP_READOBJECTS);
        if hdesk.is_null() {
            return Err(Error::OpenDesktopFailed(format!("Failed calling \
                                                         OpenDesktopW: {}",
                                                        io::Error::last_os_error())));
        }

        let sid = Sid::logon_sid_from_token(token)?;
        debug!("in create_process_as_user: got sid");
        sid.add_to_user_object(station as HANDLE,
                               sid::CONTAINER_INHERIT_ACE
                               | sid::INHERIT_ONLY_ACE
                               | sid::OBJECECT_INHERIT_ACE,
                               sid::GENERIC_READ
                               | sid::GENERIC_WRITE
                               | sid::GENERIC_EXECUTE
                               | sid::GENERIC_ALL)?;
        sid.add_to_user_object(station as HANDLE,
                               sid::NO_PROPAGATE_INHERIT_ACE,
                               sid::WINSTA_ALL_ACCESS
                               | sid::DELETE
                               | sid::READ_CONTROL
                               | sid::WRITE_DAC
                               | sid::WRITE_OWNER)?;
        sid.add_to_user_object(hdesk as HANDLE,
                               0,
                               sid::DESKTOP_CREATEMENU
                               | sid::DESKTOP_CREATEWINDOW
                               | sid::DESKTOP_ENUMERATE
                               | sid::DESKTOP_HOOKCONTROL
                               | sid::DESKTOP_JOURNALPLAYBACK
                               | sid::DESKTOP_JOURNALRECORD
                               | sid::DESKTOP_READOBJECTS
                               | sid::DESKTOP_SWITCHDESKTOP
                               | sid::DESKTOP_WRITEOBJECTS
                               | sid::DELETE
                               | sid::READ_CONTROL
                               | sid::WRITE_DAC
                               | sid::WRITE_OWNER)?;
        debug!("in create_process_as_user: added to user obj");

        let mut os_env = create_user_environment(token, &mut env.clone())?;
        debug!("in create_process_as_user: created env");

        match cvt(CreateProcessAsUserW(token,
                                       ptr::null(),
                                       command,
                                       ptr::null_mut(),
                                       ptr::null_mut(),
                                       TRUE,
                                       flags,
                                       os_env.as_mut_ptr() as LPVOID,
                                       ptr::null(),
                                       si,
                                       pi))
        {
            Ok(process) => Ok(process),
            Err(ref err) if err.raw_os_error() == Some(ERROR_PRIVILEGE_NOT_HELD as i32) => {
                Err(Error::PrivilegeNotHeld)
            }
            Err(_) => Err(Error::CreateProcessAsUserFailed(io::Error::last_os_error())),
        }
    }
}

fn create_user_environment(token: HANDLE,
                           env: &mut HashMap<String, String>)
                           -> io::Result<Vec<u16>> {
    unsafe {
        debug!("in create_user_environment");
        let mut new_env: Vec<u16> = Vec::new();
        let mut block: LPVOID = ptr::null_mut();
        cvt(userenv::CreateEnvironmentBlock(&mut block, token, FALSE))?;
        debug!("in create_user_environment: created block");
        let mut tail: u32 = MAXDWORD;
        let mut offset = 0;
        let mut part = ParsePart::Key;
        let mut cur_key: Vec<u16> = Vec::new();
        let mut cur_val: Vec<u16> = Vec::new();

        // we keep shifting the last u16 char into tail's u32 space
        // when it is 0 we know we are done because a propper environment
        // block ends in \0\0
        while tail != 0 {
            debug!("in create_user_environment:current env is {}", &String::from_utf16_lossy(&cur_key));

            tail <<= 16;
            let next_char = *(block.offset(offset) as *mut u16);
            tail |= u32::from(next_char);
            offset += 2;

            match part {
                ParsePart::Key => {
                    new_env.push(next_char);
                    if next_char == ('=' as u16) {
                        part = ParsePart::Value;
                    } else {
                        cur_key.push(next_char);
                    }
                }
                ParsePart::Value => {
                    if next_char == 0 {
                        part = ParsePart::Key;
                        match env.remove(&String::from_utf16_lossy(&cur_key).to_uppercase()) {
                            Some(val) => {
                                new_env.extend(OsStr::new(&val).encode_wide());
                            }
                            None => new_env.append(&mut cur_val),
                        }
                        new_env.push(next_char);
                        cur_key = Vec::new();
                        cur_val = Vec::new();
                    } else {
                        cur_val.push(next_char);
                    }
                }
            }
        }
        debug!("in create_user_environment: parsed tail");

        cvt(userenv::DestroyEnvironmentBlock(block))?;
        debug!("in create_user_environment: destroyed block");

        let len = new_env.len();
        new_env.truncate(len - 1);
        debug!("in create_user_environment:truncated env");

        for (k, v) in env {
            new_env.extend(OsStr::new(k).encode_wide());
            new_env.push('=' as u16);
            new_env.extend(OsStr::new(v).encode_wide());
            new_env.push(0);
        }
        new_env.push(0);
        Ok(new_env)
    }
}

fn cvt(i: i32) -> io::Result<i32> {
    if i == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(i)
    }
}

fn ensure_no_nuls<T: AsRef<OsStr>>(str: T) -> io::Result<T> {
    if str.as_ref().encode_wide().any(|b| b == 0) {
        Err(io::Error::new(ErrorKind::InvalidInput, "nul byte found in provided data"))
    } else {
        Ok(str)
    }
}

// Produces a wide string *without terminating null*; returns an error if
// `prog` or any of the `args` contain a nul.
fn make_command_line(prog: &OsStr, args: &[&str]) -> io::Result<Vec<u16>> {
    // Encode the command and arguments in a command line string such
    // that the spawned process may recover them using CommandLineToArgvW.
    let mut cmd: Vec<u16> = Vec::new();
    append_arg(&mut cmd, prog)?;
    for arg in args {
        cmd.push(' ' as u16);
        append_arg(&mut cmd, OsStr::new(arg))?;
    }
    return Ok(cmd);

    fn append_arg(cmd: &mut Vec<u16>, arg: &OsStr) -> io::Result<()> {
        // If an argument has 0 characters then we need to quote it to ensure
        // that it actually gets passed through on the command line or otherwise
        // it will be dropped entirely when parsed on the other end.
        ensure_no_nuls(arg)?;
        let arg_bytes = &arg.to_str().unwrap().as_bytes();
        let quote = arg_bytes.iter().any(|c| *c == b' ' || *c == b'\t') || arg_bytes.is_empty();
        if quote {
            cmd.push('"' as u16);
        }

        let iter = arg.encode_wide();
        let mut backslashes: usize = 0;

        for x in iter {
            if x == '\\' as u16 {
                backslashes += 1;
            } else {
                if x == '"' as u16 {
                    // Add n+1 backslashes to total 2n+1 before internal '"'.
                    for _ in 0..=backslashes {
                        cmd.push('\\' as u16);
                    }
                }
                backslashes = 0;
            }
            cmd.push(x);
        }

        if quote {
            // Add n backslashes to total 2n before ending '"'.
            for _ in 0..backslashes {
                cmd.push('\\' as u16);
            }
            cmd.push('"' as u16);
        }
        Ok(())
    }
}

fn make_envp(env: &HashMap<OsString, OsString>) -> io::Result<(LPVOID, Vec<u16>)> {
    // On Windows we pass an "environment block" which is not a char**, but
    // rather a concatenation of null-terminated k=v\0 sequences, with a final
    // \0 to terminate.
    let mut blk = Vec::new();

    for pair in env {
        blk.extend(ensure_no_nuls(pair.0)?.encode_wide());
        blk.push('=' as u16);
        blk.extend(ensure_no_nuls(pair.1)?.encode_wide());
        blk.push(0);
    }
    blk.push(0);
    Ok((blk.as_mut_ptr() as LPVOID, blk))
}

fn mk_key(s: &str) -> OsString { OsString::from(s.to_ascii_uppercase()) }

fn null_stdio_handle() -> Result<Handle> {
    let size = mem::size_of::<SECURITY_ATTRIBUTES>();
    let mut sa = SECURITY_ATTRIBUTES { nLength:              size as DWORD,
                                       lpSecurityDescriptor: ptr::null_mut(),
                                       bInheritHandle:       1, };
    let mut opts = OpenOptions::new();
    opts.read(true);
    opts.write(false);
    opts.security_attributes(&mut sa);
    Ok(File::open(Path::new("NUL"), &opts).map(File::into_handle)?)
}

unsafe fn read_to_end_uninitialized(r: &mut impl Read, buf: &mut Vec<u8>) -> io::Result<usize> {
    let start_len = buf.len();
    buf.reserve(16);

    // Always try to read into the empty space of the vector (from the length to the capacity).
    // If the vector ever fills up then we reserve an extra byte which should trigger the normal
    // reallocation routines for the vector, which will likely double the size.
    //
    // This function is similar to the read_to_end function in std::io, but the logic about
    // reservations and slicing is different enough that this is duplicated here.
    loop {
        if buf.len() == buf.capacity() {
            buf.reserve(1);
        }

        let buf_slice =
            from_raw_parts_mut(buf.as_mut_ptr().add(buf.len()), buf.capacity() - buf.len());

        match r.read(buf_slice) {
            Ok(0) => {
                return Ok(buf.len() - start_len);
            }
            Ok(n) => {
                let len = buf.len() + n;
                buf.set_len(len);
            }
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) => {
                return Err(e);
            }
        }
    }
}

fn stdio_piped_handle(stdio_id: DWORD, pipe: &mut Option<AnonPipe>) -> io::Result<Handle> {
    let ours_readable = stdio_id != STD_INPUT_HANDLE;
    let pipes = anon_pipe(ours_readable)?;
    *pipe = Some(pipes.ours);
    cvt(unsafe {
        handleapi::SetHandleInformation(pipes.theirs.handle().raw(),
                                        HANDLE_FLAG_INHERIT,
                                        HANDLE_FLAG_INHERIT)
    })?;
    Ok(pipes.theirs.into_handle())
}

fn to_u16s<S: AsRef<OsStr>>(s: S) -> io::Result<Vec<u16>> {
    fn inner(s: &OsStr) -> io::Result<Vec<u16>> {
        let mut maybe_result: Vec<u16> = s.encode_wide().collect();
        if maybe_result.iter().any(|&u| u == 0) {
            return Err(io::Error::new(io::ErrorKind::InvalidInput,
                                      "strings passed to WinAPI cannot contain \
                                       NULs"));
        }
        maybe_result.push(0);
        Ok(maybe_result)
    }
    inner(s.as_ref())
}

fn zeroed_startupinfo() -> STARTUPINFOW {
    STARTUPINFOW { cb:              0,
                   lpReserved:      ptr::null_mut(),
                   lpDesktop:       ptr::null_mut(),
                   lpTitle:         ptr::null_mut(),
                   dwX:             0,
                   dwY:             0,
                   dwXSize:         0,
                   dwYSize:         0,
                   dwXCountChars:   0,
                   dwYCountChars:   0,
                   dwFillAttribute: 0,
                   dwFlags:         0,
                   wShowWindow:     0,
                   cbReserved2:     0,
                   lpReserved2:     ptr::null_mut(),
                   hStdInput:       INVALID_HANDLE_VALUE,
                   hStdOutput:      INVALID_HANDLE_VALUE,
                   hStdError:       INVALID_HANDLE_VALUE, }
}

fn zeroed_process_information() -> PROCESS_INFORMATION {
    PROCESS_INFORMATION { hProcess:    ptr::null_mut(),
                          hThread:     ptr::null_mut(),
                          dwProcessId: 0,
                          dwThreadId:  0, }
}
