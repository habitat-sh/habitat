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

// This contains code largely copy/pasted from the rust-lang/rust repo
// We will need to create processes using different windows API calls in
// order to be able to start habitat supervisor services as different users.

use std::ascii::AsciiExt;
use std::cmp;
use std::collections::HashMap;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::fs;
use std::io::{self, ErrorKind, Read};
use std::mem;
use std::ops::Deref;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::ptr;
use std::slice::from_raw_parts_mut;
use std::sync::Mutex;

use kernel32;
use rand::{self, Rng};
use winapi;
use winapi::winbase;

use error::{Error, Result};

lazy_static! {
    static ref CREATE_PROCESS_LOCK: Mutex<()> = Mutex::new(());
}

const HANDLE_FLAG_INHERIT: winapi::DWORD = 0x00000001;

pub struct Child {
    handle: Handle,
    pub stdout: Option<ChildStdout>,
    pub stderr: Option<ChildStderr>,
}

impl Child {
    pub fn spawn(program: &str, args: Vec<&str>, env: &HashMap<String, String>) -> Result<Child> {
        let mut os_env: HashMap<OsString, OsString> = env::vars_os()
            .map(|(key, val)| (mk_key(key.to_str().unwrap()), val))
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
                    let path = path.join(program).with_extension(
                        env::consts::EXE_EXTENSION,
                    );
                    if fs::metadata(&path).is_ok() {
                        res = Some(path.into_os_string());
                    }
                }
                break;
            }
            res
        };

        let mut si = zeroed_startupinfo();
        si.cb = mem::size_of::<winapi::STARTUPINFOW>() as winapi::DWORD;
        si.dwFlags = winbase::STARTF_USESTDHANDLES;

        let program_path = program_path.unwrap_or(OsStr::new(program).to_os_string());
        let mut cmd_str = make_command_line(&program_path, &args)?;
        cmd_str.push(0); // add null terminator

        let (envp, _data) = make_envp(&os_env)?;
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
        let _ = CREATE_PROCESS_LOCK.lock().unwrap();

        let mut pipes = StdioPipes {
            stdin: None,
            stdout: None,
            stderr: None,
        };

        let stdin = null_stdio_handle()?;
        let stdout = stdio_piped_handle(winapi::STD_OUTPUT_HANDLE, &mut pipes.stdout)?;
        let stderr = stdio_piped_handle(winapi::STD_ERROR_HANDLE, &mut pipes.stderr)?;
        si.hStdInput = stdin.raw();
        si.hStdOutput = stdout.raw();
        si.hStdError = stderr.raw();
        let flags = winapi::CREATE_UNICODE_ENVIRONMENT | winapi::CREATE_NEW_PROCESS_GROUP;

        unsafe {
            cvt(kernel32::CreateProcessW(
                ptr::null(),
                cmd_str.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                winapi::TRUE,
                flags,
                envp,
                ptr::null(),
                &mut si,
                &mut pi,
            ))
        }?;

        // We close the thread handle because we don't care about keeping
        // the thread id valid, and we aren't keeping the thread handle
        // around to be able to close it later.
        unsafe { kernel32::CloseHandle(pi.hThread) };

        Ok(Child {
            handle: Handle::new(pi.hProcess),
            stdout: pipes.stdout.map(ChildStdout::from_inner),
            stderr: pipes.stderr.map(ChildStderr::from_inner),
        })
    }

    pub fn id(&self) -> u32 {
        unsafe { kernel32::GetProcessId(self.handle.raw()) as u32 }
    }

    pub fn kill(&mut self) -> io::Result<()> {
        cvt(unsafe { kernel32::TerminateProcess(self.handle.raw(), 1) })?;
        Ok(())
    }

    pub fn wait(&mut self) -> Result<ExitStatus> {
        unsafe {
            let res = kernel32::WaitForSingleObject(self.handle.raw(), winapi::INFINITE);
            if res != winapi::WAIT_OBJECT_0 {
                return Err(Error::WaitForSingleObjectFailed(format!(
                    "Failed calling WaitForSingleObjectFailed: {}",
                    io::Error::last_os_error()
                )));
            }
            let mut status = 0;
            cvt(kernel32::GetExitCodeProcess(self.handle.raw(), &mut status))?;
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
pub struct ExitStatus(winapi::DWORD);

impl ExitStatus {
    pub fn success(&self) -> bool {
        self.0 == 0
    }
    pub fn code(&self) -> Option<i32> {
        Some(self.0 as i32)
    }
}

impl From<winapi::DWORD> for ExitStatus {
    fn from(u: winapi::DWORD) -> ExitStatus {
        ExitStatus(u)
    }
}

impl fmt::Display for ExitStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "exit code: {}", self.0)
    }
}

pub struct ChildStdout {
    inner: AnonPipe,
}

impl Read for ChildStdout {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.inner.read_to_end(buf)
    }
}

impl AsInner<AnonPipe> for ChildStdout {
    fn as_inner(&self) -> &AnonPipe {
        &self.inner
    }
}

impl IntoInner<AnonPipe> for ChildStdout {
    fn into_inner(self) -> AnonPipe {
        self.inner
    }
}

impl FromInner<AnonPipe> for ChildStdout {
    fn from_inner(pipe: AnonPipe) -> ChildStdout {
        ChildStdout { inner: pipe }
    }
}

impl fmt::Debug for ChildStdout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("ChildStdout { .. }")
    }
}

pub struct ChildStderr {
    inner: AnonPipe,
}

impl Read for ChildStderr {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.inner.read_to_end(buf)
    }
}

impl AsInner<AnonPipe> for ChildStderr {
    fn as_inner(&self) -> &AnonPipe {
        &self.inner
    }
}

impl IntoInner<AnonPipe> for ChildStderr {
    fn into_inner(self) -> AnonPipe {
        self.inner
    }
}

impl FromInner<AnonPipe> for ChildStderr {
    fn from_inner(pipe: AnonPipe) -> ChildStderr {
        ChildStderr { inner: pipe }
    }
}

impl fmt::Debug for ChildStderr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("ChildStderr { .. }")
    }
}

pub struct AnonPipe {
    inner: Handle,
}

pub struct Pipes {
    pub ours: AnonPipe,
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
        let mut reject_remote_clients_flag = winapi::PIPE_REJECT_REMOTE_CLIENTS;
        loop {
            tries += 1;
            let key: u64 = rand::thread_rng().gen();
            name = format!(
                r"\\.\pipe\__rust_anonymous_pipe1__.{}.{}",
                kernel32::GetCurrentProcessId(),
                key
            );
            let wide_name = OsStr::new(&name)
                .encode_wide()
                .chain(Some(0))
                .collect::<Vec<_>>();
            let mut flags = winapi::FILE_FLAG_FIRST_PIPE_INSTANCE | winapi::FILE_FLAG_OVERLAPPED;
            if ours_readable {
                flags |= winapi::PIPE_ACCESS_INBOUND;
            } else {
                flags |= winapi::PIPE_ACCESS_OUTBOUND;
            }

            let handle = kernel32::CreateNamedPipeW(
                wide_name.as_ptr(),
                flags,
                winapi::PIPE_TYPE_BYTE | winapi::PIPE_READMODE_BYTE | winapi::PIPE_WAIT |
                    reject_remote_clients_flag,
                1,
                4096,
                4096,
                0,
                ptr::null_mut(),
            );

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
            if handle == winapi::INVALID_HANDLE_VALUE {
                let err = io::Error::last_os_error();
                let raw_os_err = err.raw_os_error();
                if tries < 10 {
                    if raw_os_err == Some(winapi::ERROR_ACCESS_DENIED as i32) {
                        continue;
                    } else if reject_remote_clients_flag != 0 &&
                               raw_os_err == Some(winapi::ERROR_INVALID_PARAMETER as i32)
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
        let theirs = AnonPipe { inner: theirs.into_handle() };

        Ok(Pipes {
            ours: AnonPipe { inner: ours },
            theirs: AnonPipe { inner: theirs.into_handle() },
        })
    }
}

impl AnonPipe {
    pub fn handle(&self) -> &Handle {
        &self.inner
    }
    pub fn into_handle(self) -> Handle {
        self.inner
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }

    pub fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.inner.read_to_end(buf)
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }
}

struct StdioPipes {
    pub stdin: Option<AnonPipe>,
    pub stdout: Option<AnonPipe>,
    pub stderr: Option<AnonPipe>,
}

#[derive(Clone)]
pub struct OpenOptions {
    // generic
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
    // system-specific
    custom_flags: u32,
    access_mode: Option<winapi::DWORD>,
    attributes: winapi::DWORD,
    share_mode: winapi::DWORD,
    security_qos_flags: winapi::DWORD,
    security_attributes: usize, // FIXME: should be a reference
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions {
            // generic
            read: false,
            write: false,
            append: false,
            truncate: false,
            create: false,
            create_new: false,
            // system-specific
            custom_flags: 0,
            access_mode: None,
            share_mode: winapi::FILE_SHARE_READ | winapi::FILE_SHARE_WRITE |
                winapi::FILE_SHARE_DELETE,
            attributes: 0,
            security_qos_flags: 0,
            security_attributes: 0,
        }
    }

    pub fn read(&mut self, read: bool) {
        self.read = read;
    }
    pub fn write(&mut self, write: bool) {
        self.write = write;
    }
    pub fn share_mode(&mut self, share_mode: u32) {
        self.share_mode = share_mode;
    }
    pub fn attributes(&mut self, attrs: u32) {
        self.attributes = attrs;
    }
    pub fn security_attributes(&mut self, attrs: winapi::LPSECURITY_ATTRIBUTES) {
        self.security_attributes = attrs as usize;
    }

    fn get_access_mode(&self) -> io::Result<winapi::DWORD> {
        const ERROR_INVALID_PARAMETER: i32 = 87;

        match (self.read, self.write, self.append, self.access_mode) {
            (_, _, _, Some(mode)) => Ok(mode),
            (true, false, false, None) => Ok(winapi::GENERIC_READ),
            (false, true, false, None) => Ok(winapi::GENERIC_WRITE),
            (true, true, false, None) => Ok(winapi::GENERIC_READ | winapi::GENERIC_WRITE),
            (false, _, true, None) => Ok(winapi::FILE_GENERIC_WRITE & !winapi::FILE_WRITE_DATA),
            (true, _, true, None) => {
                Ok(
                    winapi::GENERIC_READ | (winapi::FILE_GENERIC_WRITE & !winapi::FILE_WRITE_DATA),
                )
            }
            (false, false, false, None) => {
                Err(io::Error::from_raw_os_error(ERROR_INVALID_PARAMETER))
            }
        }
    }

    fn get_creation_mode(&self) -> io::Result<winapi::DWORD> {
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
            (false, false, false) => winapi::OPEN_EXISTING,
            (true, false, false) => winapi::OPEN_ALWAYS,
            (false, true, false) => winapi::TRUNCATE_EXISTING,
            (true, true, false) => winapi::CREATE_ALWAYS,
            (_, _, true) => winapi::CREATE_NEW,
        })
    }

    fn get_flags_and_attributes(&self) -> winapi::DWORD {
        self.custom_flags | self.attributes | self.security_qos_flags |
            if self.security_qos_flags != 0 {
                winapi::SECURITY_SQOS_PRESENT
            } else {
                0
            } |
            if self.create_new {
                winapi::FILE_FLAG_OPEN_REPARSE_POINT
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
            kernel32::CreateFileW(
                path.as_ptr(),
                opts.get_access_mode()?,
                opts.share_mode,
                opts.security_attributes as *mut _,
                opts.get_creation_mode()?,
                opts.get_flags_and_attributes(),
                ptr::null_mut(),
            )
        };
        if handle == winapi::INVALID_HANDLE_VALUE {
            Err(io::Error::last_os_error())
        } else {
            Ok(File { handle: Handle::new(handle) })
        }
    }

    pub fn into_handle(self) -> Handle {
        self.handle
    }
}

pub struct Handle(RawHandle);

#[derive(Copy, Clone)]
pub struct RawHandle(winapi::HANDLE);

unsafe impl Send for RawHandle {}
unsafe impl Sync for RawHandle {}

impl Handle {
    pub fn new(handle: winapi::HANDLE) -> Handle {
        Handle(RawHandle::new(handle))
    }

    pub fn new_event(manual: bool, init: bool) -> io::Result<Handle> {
        unsafe {
            let event = kernel32::CreateEventW(
                ptr::null_mut(),
                manual as winapi::BOOL,
                init as winapi::BOOL,
                ptr::null(),
            );
            if event.is_null() {
                Err(io::Error::last_os_error())
            } else {
                Ok(Handle::new(event))
            }
        }
    }

    pub fn into_raw(self) -> winapi::HANDLE {
        let ret = self.raw();
        mem::forget(self);
        return ret;
    }
}

impl Deref for Handle {
    type Target = RawHandle;
    fn deref(&self) -> &RawHandle {
        &self.0
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            let _ = kernel32::CloseHandle(self.raw());
        }
    }
}

impl RawHandle {
    pub fn new(handle: winapi::HANDLE) -> RawHandle {
        RawHandle(handle)
    }

    pub fn raw(&self) -> winapi::HANDLE {
        self.0
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let mut read = 0;
        let len = cmp::min(buf.len(), <winapi::DWORD>::max_value() as usize) as winapi::DWORD;
        let res = cvt(unsafe {
            kernel32::ReadFile(
                self.0,
                buf.as_mut_ptr() as winapi::LPVOID,
                len,
                &mut read,
                ptr::null_mut(),
            )
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

    pub fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        let mut read = 0;
        let len = cmp::min(buf.len(), <winapi::DWORD>::max_value() as usize) as winapi::DWORD;
        let res = unsafe {
            let mut overlapped: winapi::OVERLAPPED = mem::zeroed();
            overlapped.Offset = offset as u32;
            overlapped.OffsetHigh = (offset >> 32) as u32;
            cvt(kernel32::ReadFile(
                self.0,
                buf.as_mut_ptr() as winapi::LPVOID,
                len,
                &mut read,
                &mut overlapped,
            ))
        };
        match res {
            Ok(_) => Ok(read as usize),
            Err(ref e) if e.raw_os_error() == Some(winapi::ERROR_HANDLE_EOF as i32) => Ok(0),
            Err(e) => Err(e),
        }
    }

    pub unsafe fn read_overlapped(
        &self,
        buf: &mut [u8],
        overlapped: *mut winapi::OVERLAPPED,
    ) -> io::Result<Option<usize>> {
        let len = cmp::min(buf.len(), <winapi::DWORD>::max_value() as usize) as winapi::DWORD;
        let mut amt = 0;
        let res = cvt({
            kernel32::ReadFile(
                self.0,
                buf.as_ptr() as winapi::LPVOID,
                len,
                &mut amt,
                overlapped,
            )
        });
        match res {
            Ok(_) => Ok(Some(amt as usize)),
            Err(e) => {
                if e.raw_os_error() == Some(winapi::ERROR_IO_PENDING as i32) {
                    Ok(None)
                } else if e.raw_os_error() == Some(winapi::ERROR_BROKEN_PIPE as i32) {
                    Ok(Some(0))
                } else {
                    Err(e)
                }
            }
        }
    }

    pub fn overlapped_result(
        &self,
        overlapped: *mut winapi::OVERLAPPED,
        wait: bool,
    ) -> io::Result<usize> {
        unsafe {
            let mut bytes = 0;
            let wait = if wait { winapi::TRUE } else { winapi::FALSE };
            let res = cvt({
                kernel32::GetOverlappedResult(self.raw(), overlapped, &mut bytes, wait)
            });
            match res {
                Ok(_) => Ok(bytes as usize),
                Err(e) => {
                    if e.raw_os_error() == Some(winapi::ERROR_HANDLE_EOF as i32) ||
                        e.raw_os_error() == Some(winapi::ERROR_BROKEN_PIPE as i32)
                    {
                        Ok(0)
                    } else {
                        Err(e)
                    }
                }
            }
        }
    }

    pub fn cancel_io(&self) -> io::Result<()> {
        unsafe { cvt(kernel32::CancelIo(self.raw())).map(|_| ()) }
    }

    pub fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let mut me = self;
        (&mut me).read_to_end(buf)
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let mut amt = 0;
        let len = cmp::min(buf.len(), <winapi::DWORD>::max_value() as usize) as winapi::DWORD;
        cvt(unsafe {
            kernel32::WriteFile(
                self.0,
                buf.as_ptr() as winapi::LPVOID,
                len,
                &mut amt,
                ptr::null_mut(),
            )
        })?;
        Ok(amt as usize)
    }

    pub fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
        let mut written = 0;
        let len = cmp::min(buf.len(), <winapi::DWORD>::max_value() as usize) as winapi::DWORD;
        unsafe {
            let mut overlapped: winapi::OVERLAPPED = mem::zeroed();
            overlapped.Offset = offset as u32;
            overlapped.OffsetHigh = (offset >> 32) as u32;
            cvt(kernel32::WriteFile(
                self.0,
                buf.as_ptr() as winapi::LPVOID,
                len,
                &mut written,
                &mut overlapped,
            ))?;
        }
        Ok(written as usize)
    }
}

impl<'a> Read for &'a RawHandle {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (**self).read(buf)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        unsafe { read_to_end_uninitialized(self, buf) }
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
        Err(io::Error::new(
            ErrorKind::InvalidInput,
            "nul byte found in provided data",
        ))
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

        let mut iter = arg.encode_wide();
        let mut backslashes: usize = 0;
        while let Some(x) = iter.next() {
            if x == '\\' as u16 {
                backslashes += 1;
            } else {
                if x == '"' as u16 {
                    // Add n+1 backslashes to total 2n+1 before internal '"'.
                    for _ in 0..(backslashes + 1) {
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

fn make_envp(env: &HashMap<OsString, OsString>) -> io::Result<(winapi::LPVOID, Vec<u16>)> {
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
    Ok((blk.as_mut_ptr() as winapi::LPVOID, blk))
}

fn mk_key(s: &str) -> OsString {
    OsString::from(s.to_ascii_uppercase())
}

fn null_stdio_handle() -> Result<Handle> {
    let size = mem::size_of::<winapi::SECURITY_ATTRIBUTES>();
    let mut sa = winapi::SECURITY_ATTRIBUTES {
        nLength: size as winapi::DWORD,
        lpSecurityDescriptor: ptr::null_mut(),
        bInheritHandle: 1,
    };
    let mut opts = OpenOptions::new();
    opts.read(true);
    opts.write(false);
    opts.security_attributes(&mut sa);
    Ok(File::open(Path::new("NUL"), &opts).map(
        |file| file.into_handle(),
    )?)
}

unsafe fn read_to_end_uninitialized(r: &mut Read, buf: &mut Vec<u8>) -> io::Result<usize> {

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

        let buf_slice = from_raw_parts_mut(
            buf.as_mut_ptr().offset(buf.len() as isize),
            buf.capacity() - buf.len(),
        );

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

fn stdio_piped_handle(stdio_id: winapi::DWORD, pipe: &mut Option<AnonPipe>) -> io::Result<Handle> {
    let ours_readable = stdio_id != winapi::STD_INPUT_HANDLE;
    let pipes = anon_pipe(ours_readable)?;
    *pipe = Some(pipes.ours);
    cvt(unsafe {
        kernel32::SetHandleInformation(
            pipes.theirs.handle().raw(),
            HANDLE_FLAG_INHERIT,
            HANDLE_FLAG_INHERIT,
        )
    })?;
    Ok(pipes.theirs.into_handle())
}

fn to_u16s<S: AsRef<OsStr>>(s: S) -> io::Result<Vec<u16>> {
    fn inner(s: &OsStr) -> io::Result<Vec<u16>> {
        let mut maybe_result: Vec<u16> = s.encode_wide().collect();
        if maybe_result.iter().any(|&u| u == 0) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "strings passed to WinAPI cannot contain NULs",
            ));
        }
        maybe_result.push(0);
        Ok(maybe_result)
    }
    inner(s.as_ref())
}

fn zeroed_startupinfo() -> winapi::STARTUPINFOW {
    winapi::STARTUPINFOW {
        cb: 0,
        lpReserved: ptr::null_mut(),
        lpDesktop: ptr::null_mut(),
        lpTitle: ptr::null_mut(),
        dwX: 0,
        dwY: 0,
        dwXSize: 0,
        dwYSize: 0,
        dwXCountChars: 0,
        dwYCountChars: 0,
        dwFillAttribute: 0,
        dwFlags: 0,
        wShowWindow: 0,
        cbReserved2: 0,
        lpReserved2: ptr::null_mut(),
        hStdInput: winapi::INVALID_HANDLE_VALUE,
        hStdOutput: winapi::INVALID_HANDLE_VALUE,
        hStdError: winapi::INVALID_HANDLE_VALUE,
    }
}

fn zeroed_process_information() -> winapi::PROCESS_INFORMATION {
    winapi::PROCESS_INFORMATION {
        hProcess: ptr::null_mut(),
        hThread: ptr::null_mut(),
        dwProcessId: 0,
        dwThreadId: 0,
    }
}
