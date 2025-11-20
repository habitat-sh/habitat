//! Pidfile / Lockfile of the Launcher
//!
//! When the Supervisor starts, it is given the PID of the Launcher that started
//! it. It then writes this PID to `/hab/sup/default/LOCK` (see `path()` below)
//! to serve as a record of the currently-running process. This is used to
//! discover which process to terminate (when running `hab sup term`), and also
//! helps ensure that only one Supervisor process is running at a time.
//!
//! To maintain this exclusivity, we hold a shared file lock on the file for the
//! duration of the Supervisor process. As a Supervisor starts, it attempts to
//! acquire an _exclusive_ lock to that file. It will fail if a Supervisor is
//! already running, because that process' shared lock will prevent acquisition
//! of the exclusive lock.
//!
//! In an ideal world, this file would probably be written by the Launcher
//! itself. However, the file currently resides in the `/hab/sup` directory
//! hierarchy, which the Supervisor is responsible for maintaining.
use crate::manager::PROC_LOCK_FILE;
use fs2::FileExt;
use habitat_core::{env,
                   os::process::Pid};
use habitat_launcher_client::LAUNCHER_PID_ENV;
use log::error;
use std::{fmt,
          fs::{File,
               OpenOptions},
          io::{Read,
               Write},
          path::{Path,
                 PathBuf},
          str::FromStr};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error manipulating lock file '{}'", lock_file_path().display())]
    IOError(#[source] std::io::Error),

    #[error("Could not open lock file '{}'", lock_file_path().display())]
    CannotOpen(#[source] std::io::Error),

    // File lock-related error conditions
    #[error("Could not acquire exclusive lock on lock file '{}'. Is another Supervisor process running?", lock_file_path().display())]
    CannotAcquireExclusiveLock(#[source] std::io::Error),

    /// This could happen on *nix because changing the lock type is actually a
    /// non-atomic release-followed-by-acquisition. Thus, we call it out
    /// specifically.
    #[error("Could not downgrade to shared lock on '{}'. Possible race condidition with another process acquiring an exclusive lock.", lock_file_path().display())]
    LockDowngrade(#[source] std::io::Error),

    // Windows-only
    #[error("Could not acquire shared lock on lock file '{}'", lock_file_path().display())]
    CannotAcquireSharedLock(#[source] std::io::Error),

    // Windows-only
    #[error("Could not release exclusive lock on lock file '{}'", lock_file_path().display())]
    CannotReleaseExclusiveLock(#[source] std::io::Error),

    #[error("The lock file '{}' appears to be leftover from a prior Supervisor process", lock_file_path().display())]
    StaleLockFile,

    #[error("Contents of '{}' are corrupt", lock_file_path().display())]
    CorruptLockFile(#[source] Box<Self>),

    #[error("The '{}' environment variable must be present!", LAUNCHER_PID_ENV)]
    InvalidEnvironment(#[source] std::env::VarError),

    #[error("Could not parse '{0}' as a non-zero positive PID")]
    PidParse(String),
}

/// Returns the path of the lock file.
///
/// Also used directly in error implementations to provide context. Everything
/// in this module deals with the same file, and manually wiring it through in
/// `map_err()` calls is tedious.
fn lock_file_path() -> PathBuf { habitat_sup_protocol::sup_root(None).join(PROC_LOCK_FILE) }

///////////////////////////////////////////////////////////////////////

/// For our purposes, a PID must address a single process (to wit: the
/// Launcher). On both Linux and Windows, this means that the PID must be
/// non-zero and positive. (Negative PIDs and PID 0 both have specific meaning
/// and uses on Linux; while different on Windows, a positive PID is the
/// important characteristic on both platforms.) As a result, we encapsulate
/// this logic in one place in this type.
///
/// Note: For the purposes of this module, the only way to create a
/// `PositiveNonZeroPid` is to parse it from a String.
#[derive(Clone, Copy, Debug)]
struct PositiveNonZeroPid(Pid);

impl FromStr for PositiveNonZeroPid {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.parse() {
            Ok(pid) => {
                if pid > 0 {
                    Ok(PositiveNonZeroPid(pid))
                } else {
                    Err(Error::PidParse(s.to_string()))
                }
            }
            Err(_e) => Err(Error::PidParse(s.to_string())),
        }
    }
}

impl From<PositiveNonZeroPid> for Pid {
    fn from(src: PositiveNonZeroPid) -> Self { src.0 }
}

impl fmt::Display for PositiveNonZeroPid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.0) }
}

///////////////////////////////////////////////////////////////////////

/// A running Supervisor writes out the PID of its Launcher to a file, and then
/// holds it open with a shared lock for the duration of the process in order to
/// ensure only a single Supervisor process is running on a host at a time.
pub struct LockFile {
    /// An open file handle to the Launcher's lock file. The file will have a
    /// shared file lock on it, allowing other processes to read it, but not to
    /// write to it.
    ///
    /// An instance of `LockFile` can only be obtained if no other process has a
    /// lock on the file. Thus, only one instance of a Supervisor may be running
    /// on a host at a given time.
    file: File,
}

impl LockFile {
    /// Creates a lockfile, writes the Launcher's PID into it (after obtaining
    /// an exclusive file lock on it), and holds the filehandle open for the
    /// lifetime of this type instance. A shared file lock is held on the file,
    /// preventing another instance from being created. Ultimately, this is used
    /// to prevent multiple Supervisors from running on a host at the same time.
    pub fn acquire() -> Result<Self> {
        let pid: PositiveNonZeroPid =
            env::var(LAUNCHER_PID_ENV).map_err(Error::InvalidEnvironment)?
                                      .parse()?;

        let path = lock_file_path();
        Self::acquire_impl(path, pid)
    }

    /// Implementation of lock file creation / acquisition logic, separating it
    /// from the logic of determining the file path and the PID to write to that
    /// file.
    fn acquire_impl<P>(path: P, pid: PositiveNonZeroPid) -> Result<Self>
        where P: AsRef<Path>
    {
        // Create the file in a block simply to contain the mutability.
        let file = {
            // We use `create`, rather than `create_new`, because the lock file may
            // exist already if it were not cleaned up properly. Whether or not we
            // can get an exclusive lock on the file is the real test.
            //
            // We explicitly DO NOT truncate at first, because this open operation
            // is not limited by file locks on Linux, which are advisory*. If we
            // were to truncate, a second Supervisor could try to start up and
            // obliterate the PID of the running Supervisor. As a result, we'll
            // manually truncate later on, once we have obtained an exclusive lock.
            //
            // * Well, Linux file locks _can_ be mandatory, but it requries mounting
            // filesystems in a particular way, setting specific permission bits
            // on files, and comes with a lot of other caveats. To quote the Linux
            // Programming Interface, Section 55.4, "the use of mandatory locks is
            // best avoided".
            let mut file = OpenOptions::new().write(true)
                                              .create(true)
                                              .truncate(false) // NEVER TRUE!
                                              .open(path)
                                              .map_err(Error::CannotOpen)?;

            // If we can't get an exclusive lock, then something else has locked the
            // file. Assume it's another Supervisor and bail.
            file.try_lock_exclusive()
                .map_err(Error::CannotAcquireExclusiveLock)?;

            // Now that we know we have an exclusive lock, we can truncate the file
            // (in case it existed before!) and write the value of the new PID to
            // it.
            file.set_len(0).map_err(Error::IOError)?;
            write!(&mut file, "{}", pid).map_err(Error::IOError)?;
            file
        };

        // Now that we have written the PID to the file, we need to downgrade
        // our exclusive lock to a shared one. This will ensure that other
        // processes can read the file to find out the PID, but that no one else
        // will be able to write the file. This also means that no other process
        // (like another Supervisor) can create another instance of `LockFile`,
        // because doing so requires an exclusive file lock.
        //
        // On Linux, only one lock can be held on a file by a process, so by
        // taking out a shared lock, we effectively downgrade the lock, as
        // desired.
        //
        // The relevant documentation from
        // [flock(2)](https://linux.die.net/man/2/flock):
        //
        // > A process may only hold one type of lock (shared or exclusive) on a
        // > file. Subsequent flock() calls on an already locked file will
        // > convert an existing lock to the new lock mode.
        //
        // Note, however, that on Linux, this "downgrading" operation is
        // unfortunately *NOT* atomic, so there is a small chance that another
        // process attempting to acquire an exclusive lock on the file could
        // succeed in this window (See `read_lock_file` below).
        //
        // On Windows, however, we can hold multiple locks; since we're using
        // the same file handle, we're allowed to take out a shared lock _while
        // still holding our exclusive lock_. To release the locks requires two
        // `unlock` operations, the first of which releases the _exclusive_
        // lock. Thus, we can downgrade to a shared lock by acquiring a shared
        // lock and then unlocking once.
        //
        // See
        // [LockFileEx](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfileex)
        // for more.
        fs2::FileExt::try_lock_shared(&file).map_err(|e| {
                                                if cfg!(windows) {
                                                    Error::CannotAcquireSharedLock(e)
                                                } else {
                                                    // Unix, Linux, et al.
                                                    Error::LockDowngrade(e)
                                                }
                                            })?;
        if cfg!(windows) {
            fs2::FileExt::unlock(&file).map_err(Error::CannotReleaseExclusiveLock)?;
        }

        // Now, anyone should be able to read the file, but another Supervisor
        // can't start up as long as we hold onto this file.
        Ok(Self { file })
    }
}

impl Drop for LockFile {
    /// Though the operating system will drop file locks when the file handle is
    /// closed, it is good to explicitly drop them when we can.
    ///
    /// In particular, this is [recommended][1] on Windows, where such automatic
    /// lock dropping is dependent on currently available system resources.
    ///
    /// **WE DO NOT DELETE THE LOCK FILE!** Because opening a file and acquiring
    /// a lock do not atomically occur together, there are sequences of events
    /// that could play out that would lead to multiple Supervisors being able
    /// to run simultaneously, thus defeating the entire purpose of the
    /// `LockFile`, _but only if we delete the file_. (This arises because both
    /// processes would acquire exclusive locks on filehandles that point to
    /// different instances of the file in question. [See further
    /// discussion.][2])
    ///
    /// (This shouldn't be taken to mean that users can't clean up the file on
    /// their own, if they choose; it just means that we're not going to
    /// intentionally undermine the exclusivity that the lock file is supposed
    /// to provide. The rest of this implementation is robust against the
    /// presence of stale lock files, so leaving them on disk poses no harm.)
    ///
    /// [1]:
    /// https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-unlockfile#remarks
    /// [2]: http://www.guido-flohr.net/never-delete-your-pid-file/
    fn drop(&mut self) {
        if let Err(e) = fs2::FileExt::unlock(&self.file) {
            error!("Error unlocking '{}'; proceeding anyway: {:?}",
                   lock_file_path().display(),
                   e);
        }
    }
}

///////////////////////////////////////////////////////////////////////

/// Reads the contents of the Supervisor / Launcher lock file to obtain the PID
/// inside.
///
/// This is intended to support the `hab sup term` use case, where we use this
/// function to determine which process to terminate.
///
/// If the following are true:
///
/// - the file is already locked by a process, and
/// - the file contains a valid PID
///
/// then it is assumed that:
///
/// - the process identified by that PID holds the lock, and
/// - the process identified by the PID is the Launcher.
pub fn read_lock_file() -> Result<Pid> { read_lock_file_impl(lock_file_path()) }

/// Implementation of main lockfile reading logic, separate from the specific
/// file being read. Done to facilitate testing.
fn read_lock_file_impl<P>(path: P) -> Result<Pid>
    where P: AsRef<Path>
{
    let mut file = OpenOptions::new().read(true)
                                     .open(path)
                                     .map_err(Error::IOError)?;

    // This function is expected to be called in a context where a Supervisor is
    // already running. As a result, we should *NOT* be able to acquire an
    // exclusive lock on the lock file. If we _can_, it means that whatever
    // Supervisor process that originally had the lock has released it, so we
    // should consider any contents of the file to be garbage.
    if file.try_lock_exclusive().is_ok() {
        return Err(Error::StaleLockFile);
    }

    let pid: PositiveNonZeroPid = {
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).map_err(Error::IOError)?;
        buffer.trim()
              .parse()
              .map_err(|e| Error::CorruptLockFile(Box::new(e)))?
    };

    Ok(pid.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path,
                    PathBuf};
    use tempfile::{TempDir,
                   tempdir};

    /// Create a directory to put lock files in, and create a `PathBuf` that
    /// points to a location in that directory.
    ///
    /// The dirctory is temporary, so hang on to it as long as you need it!
    fn setup() -> (PathBuf, TempDir) {
        let dir = tempdir().unwrap();
        let lock_path = dir.path().join("LOCK");
        (lock_path, dir)
    }

    /// Given a path and a string, write the string to the path and return an
    /// open file handle.
    fn write_to_file<P, C>(path: P, content: C) -> File
        where P: AsRef<Path>,
              C: AsRef<str>
    {
        let mut file = std::fs::OpenOptions::new().write(true)
                                                  .truncate(true)
                                                  .create(true)
                                                  .open(path.as_ref())
                                                  .unwrap();
        write!(file, "{}", content.as_ref()).unwrap();
        file
    }

    fn assert_file_contents<P>(path: P, expected_content: &str)
        where P: AsRef<Path>
    {
        assert!(path.as_ref().exists());
        let actual_content = std::fs::read_to_string(path.as_ref()).unwrap();
        assert_eq!(actual_content, expected_content);
    }

    #[test]
    fn lock_acquisition_creates_file_if_necessary() {
        let (lock_path, _dir) = setup();

        assert!(!lock_path.exists());

        let _lock_file = LockFile::acquire_impl(&lock_path, PositiveNonZeroPid(2112)).unwrap();

        assert_file_contents(lock_path, "2112");
    }

    #[test]
    fn lock_acquisition_overwrites_existing_file() {
        let (lock_path, _dir) = setup();

        // This is obviously some bogus content; it's just going to act as a
        // sentinel, since a successful lock acquisition should overwrite it
        // with correct information.
        write_to_file(&lock_path, "Rusty McRustface");
        assert_file_contents(&lock_path, "Rusty McRustface");

        let _lock_file = LockFile::acquire_impl(&lock_path, PositiveNonZeroPid(2112)).unwrap();

        assert_file_contents(&lock_path, "2112");
    }

    #[test]
    #[should_panic(expected = "CannotAcquireExclusiveLock")]
    fn cannot_have_more_than_one_instance() {
        let (lock_path, _dir) = setup();

        let lock_file_1 = LockFile::acquire_impl(&lock_path, PositiveNonZeroPid(2112));
        assert!(lock_file_1.is_ok());

        // Acquiring a second lock will fail because the first one is still in
        // effect.
        LockFile::acquire_impl(&lock_path, PositiveNonZeroPid(2113)).unwrap();
    }

    #[test]
    fn dropping_releases_lock() {
        let (lock_path, _dir) = setup();

        let lock_file_1 = LockFile::acquire_impl(&lock_path, PositiveNonZeroPid(2112)).unwrap();

        // We can't acquire the lock a second time, as tested above.
        let lock_file_2 = LockFile::acquire_impl(&lock_path, PositiveNonZeroPid(2112));
        assert!(lock_file_2.is_err());

        // However, once we get rid of the old lock, though, we can reacquire it.
        drop(lock_file_1);
        let lock_file_2 = LockFile::acquire_impl(&lock_path, PositiveNonZeroPid(2112));
        assert!(lock_file_2.is_ok());
    }

    #[test]
    fn dropping_a_lock_does_not_delete_the_backing_file() {
        let (lock_path, _dir) = setup();

        assert!(!lock_path.exists());

        let lock_file = LockFile::acquire_impl(&lock_path, PositiveNonZeroPid(2112)).unwrap();

        // Just asserting that the file really gets created
        assert_file_contents(&lock_path, "2112");

        drop(lock_file);

        // Dropping the lock MUST LEAVE THE FILE BEHIND (see the `Drop`
        // implementation for more).
        assert_file_contents(&lock_path, "2112");
    }

    #[test]
    #[should_panic(expected = "StaleLockFile")]
    fn cannot_read_a_stale_lockfile() {
        let (lock_path, _dir) = setup();
        write_to_file(&lock_path, "1234");

        // There's no lock on the file, thus it is stale.
        read_lock_file_impl(lock_path).unwrap();
    }

    #[test]
    #[should_panic(expected = "CorruptLockFile")]
    fn corrupt_lock_files_cannot_be_read() {
        let (lock_path, _dir) = setup();

        // Write some corrupt data to the file
        let file = write_to_file(&lock_path, "LOLWUT");

        // Lock the file so we can get past that check.
        fs2::FileExt::lock_shared(&file).unwrap();

        // Trying to get the PID out will fail now.
        read_lock_file_impl(lock_path).unwrap();
    }
}
