use crate::error::{Error,
                   Result};
use habitat_common::liveliness_checker;
use habitat_core::os::ffi::OsStrExt;
use log::{debug,
          trace};
use notify::{self,
             event::{AccessKind,
                     AccessMode,
                     CreateKind,
                     DataChange,
                     EventKind,
                     MetadataKind,
                     ModifyKind,
                     RemoveKind,
                     RenameMode},
             Config,
             Event,
             RecursiveMode,
             Watcher};

use std::{collections::HashSet,
          fmt::Debug,
          path::{Component,
                 Path,
                 PathBuf},
          sync::mpsc::{channel,
                       Receiver,
                       TryRecvError},
          thread,
          time::Duration};

use super::sup_watcher::SupWatcher;

pub const WATCHER_DELAY_MS: u64 = 2_000;

#[derive(Clone, Debug, PartialEq)]
/// WatchedFileState is used to indicate first if a WatchedFile is valid or not.  The Invalid
/// variant signals that a file is Invalid and contains a string that provides the reasoning.  All
/// other variants are understood to be "valid paths" to a file where valid is understood to mean
/// that to the best of the codes ability the path looks like it name a file that could exist.
/// Beyond "validity" that the other variants indicate that
/// * the file, and thus its enclosing directory, exist on the file system
/// * the file does not exist the file system but enclosing directory does exist
/// * neither the file nor its enclosing directory exists
enum WatchedFileState {
    /// Indicates that both the file and enclosing directory exists and are visible.  
    ExistentFile,
    /// Indicates that while the file does not exist on the file system the enclosing directory
    /// does exist and since we've passed the watchability checks we should be able to trust that
    /// the path is valid.  What's tricky about this state is that you have to remember even though
    /// we are concerned with watching files (existent or nonexistent) the details of using the
    /// notify crate well means that we might sometime need to watch the enclosing directory we're
    /// still watching files so the path should indicate a single file that we are watching to see
    /// if it comes into existence.
    ExistentDirectory,
    /// Indicates that neither the file nor the enclosing directory exists but to the best of our
    /// ability we believe that the path is valid for the system and indicate a file that could
    /// exist on the platform's file system.  This also implies that the path of the enclosing
    /// directory would be valid path if it were to exist.
    NonExistent,
    /// Invalid indicates that the path is something that we have determined cannot indicate a file
    /// on the system. The path may be wholly malformed or it might indicate a directory instead of
    /// a file. The associated String will contain an explanation of why the path is valid
    Invalid(String),
    // I feel as though I should be making a Schrodinger's WatchedFile joke here... or not.
}

mod errmsg {
    use super::*;
    pub fn invalid_ending(path: &Path) -> String {
        format!("{:?} appears invalid because it ends with {:?} thus appearing to be a directory \
                 instead of a file",
                path,
                std::path::MAIN_SEPARATOR)
    }
    pub fn invalid_root(path: &Path) -> String {
        format!("{:?} appears invalid as the root is different than expected for the platform \
                 we're running on.",
                path)
    }
    pub fn is_directory(path: &Path) -> String {
        format!("{:?} is a directory instead of a file.", path)
    }
    pub fn file_not_found(path: &Path) -> String {
        format!("Neither File nor Enclosing Directory Found: {:?}", path)
    }
}

#[derive(Clone, Debug)]
/// WatchedFile is a struct bundling the essential data together for a file that
/// is to be watched.  It may or may not exist when file begins to be watched
/// and it may come into or go out of existence while the watch is in effect.
struct WatchedFile {
    /// Owned PathBuf that is the WatchedFile's path
    path:               PathBuf,
    /// This is the enclosing directory broken out for easier reference
    directory:          PathBuf,
    /// Used to signal if an initial Callbacks::file_appeared should be sent
    /// when creating the watched file.  The file must exist on the file system
    /// and this must be set true.  After the first execution this value will
    /// set back to false as the intial true has fulfilled its purpose.
    send_initial_event: bool,
    /// trackes the WatchedFileState of the WatchedFile
    state:              WatchedFileState,
}

impl WatchedFile {
    fn new<P>(path: P, send_initial_event: bool) -> Result<Self>
        where P: AsRef<Path> + Debug
    {
        trace!("WatchedFile::new({:?},{:?})", path, send_initial_event);
        let path = path.as_ref();
        let state = Self::discern_state_of(path);
        let enclosing_directory = match state {
            WatchedFileState::ExistentFile
            | WatchedFileState::ExistentDirectory
            | WatchedFileState::NonExistent => Self::get_enclosing_directory(path),
            WatchedFileState::Invalid(_) => {
                let s = path.as_os_str().to_string_lossy().to_string();
                return Err(Error::FileNotFound(s));
            }
        };
        let directory = match enclosing_directory {
            Some(s) => s,
            None => {
                let s = path.as_os_str().to_string_lossy().to_string();
                return Err(Error::FileNotFound(s));
            }
        };
        debug!("WatchedFile::path = {:?}", path);
        debug!("WatchedFile::state = {:?}", state);
        debug!("WatchedFile::directory= {:?}", directory);
        debug!("WatchedFile::send_initial_event = {:?}", send_initial_event);
        let path = path.to_path_buf();
        Ok(Self { path,
                  directory,
                  send_initial_event,
                  state })
    }

    // For clarity and testing this was extracted from new() and its not really intended to be used
    // stand alone. Its also written such that you should know the path you're passing in is valid
    // and trustworthy meaning whatever path you pass in should be one that would be valid according
    // to the new method
    fn get_enclosing_directory(path: &Path) -> Option<PathBuf> {
        trace!("WatchedFile::get_enclosing_directory({:?})", path);
        match path.parent() {
            Some(d) => Some(PathBuf::from(d)),
            None => {
                trace!("None Match in WatchedFile::get_enclosing_directory({:?})",
                       path);
                path.components()
                    .next()
                    .map(|c| PathBuf::from(c.as_os_str()))
            }
        }
    }

    // First thing we do is call is_file which will resolve symlinks and tell if a file exists on
    // the file system. If true, we have an existent file.  Second thing we do is call is_dir() and
    // if true we return an error because FileWatcher watched individual files, not directories.
    //
    // Now since both is_file() and is_dir() traverse symlinks its reasonable to assume that if
    // we're still looking then we're at this point we're being asked to look for a file that
    // doesn't exist yet.  So if the path is not an existent file and its not a existent directory
    // then the question we're left with is "does this path look like it could be a file doesn't
    // exist yet?" In order to determine that we're going to do two things.
    //
    // First, we're going to see if the path begins with an appropriate looking "path prefix". That
    // will eliminate things which couldn't possibly a valid path.  Testing the prefix is requires
    // different consideration on unix vs windows but Rust provides std::path::Prefix really helps
    // check what we need to check on windows and unix just begins with a std::path::MAIN_SEPARATOR
    // character.
    //
    // Second, we're going to check that it does not end with a platform specific path separator
    // which would indicate a directory if the path existed on the file system and we're going to
    // error again as we did previously.  Since we're testing with std::path::MAIN_SEPARATOR this
    // will hopefully prove to be platform agnostic but in testing.
    //
    // There is also the possibility that we're being asked to watch for something that exists
    // but for which there is an ownership or permission issue. Ownership and permission checking
    // is a something that the previous versions of FileWatcher didn't try to solve. It's also
    // something that the notify crate backends have bail on trying to report. So ownership and
    // permission are really not problems we want to take on either so effectively they will present
    // as nonexistent files.  Which... close enough.  :)
    //
    // Supporting reasoning for these decision is if the path was an existent file it would have
    // platform appropriate "beginnings and endings", that is whatever prefix is expected for the
    // and it wouldn't end in the platforms component separator.  As both are true in the case of
    // existent files we shouldn't have different expectations for files that exist when we begin
    // watching vs when we're being asked to watch for a file that doesn't currently exist.
    //
    // Finally, in the documentation for PeerWatcher the sample path '/path/to/file' is
    // used so a user might reasonably expect that a file extension isn't required. I think that
    // prevents a good test around presences of the file extension in the case of NonExistent files.
    fn discern_state_of(path: &Path) -> WatchedFileState {
        trace!("WatchedFile::discern_state_of({:?})", path);
        if path.is_file() {
            // if this is file existing on the file system then everything is as good as we could
            // hope. This is a condition we can expect to see during construction or when assessing
            // during runtime after construction
            WatchedFileState::ExistentFile
        } else if path.is_dir() {
            // if this is directory existing on the file system then its invalid because we watch
            // files, not directories.  We should only encounter this during construction.
            WatchedFileState::Invalid(errmsg::is_directory(path))
        } else if Self::ends_with_a_path_separator(path) {
            // if the file ends in a path separator then its appears to be a directory and we watch
            // files, not directories.  We should only encounter this during construction.
            WatchedFileState::Invalid(errmsg::invalid_ending(path))
        } else if !Self::begins_with_a_root_dir_or_prefix(path) {
            // if the file doesn't being appropriately for the platform we're running on then this
            // can't be a valid path to a file.  We should only encounter this during construction.
            WatchedFileState::Invalid(errmsg::invalid_root(path))
        } else if let Some(d) = Self::get_enclosing_directory(path) {
            // however, if we have still something that appears as though it could be a file...
            if d.is_dir() {
                // and the enclosing directory exists then we're in a state where the enclosing
                // directory exists but not the file
                WatchedFileState::ExistentDirectory
            } else {
                // or the enclosing directory does not exist and and we really don't have anything
                // that we can can watch.
                WatchedFileState::NonExistent
            }
        } else {
            // if we've fallen through to here then whatever the path we've been provided is isn't
            // valid. Whatever the path is it doesn't actually exist on the file system as a file
            // and based on our checks it doesn't appear to be a potentially valid path to a file
            // where the enclosing directory exists on the file system.
            WatchedFileState::Invalid(errmsg::file_not_found(path))
        }
    }

    pub fn assess_state(&self) -> WatchedFileState {
        trace!("WatchedFile::assess_state()");
        let cs = Self::discern_state_of(&self.path);
        debug!("CURRENT STATE = {:?}", cs);
        cs
    }

    // JAH: Altering the code this way I believe that this is now platform agnostic.  However, it
    // will require tests that at least windows specific and I'm not sure if it won't require tests
    // for unix to be run on unix only.
    fn begins_with_a_root_dir_or_prefix(path: &Path) -> bool {
        trace!("WatchedFile::begins_with_a_root_dir_or_prefix({:?})", path);
        match path.components().next() {
            Some(Component::Prefix(_)) => true,
            Some(Component::RootDir) => true,
            Some(Component::CurDir) => false,
            Some(Component::ParentDir) => false,
            Some(Component::Normal(_)) => false,
            None => false,
        }
    }

    // JAH: this might be platform agnostic but will need to confirm
    fn ends_with_a_path_separator(path: &Path) -> bool {
        trace!("WatchedFile::ends_with_a_path_separator({:?})", path);
        let last_byte: u8 = *(path.as_os_str().as_bytes().last().take().unwrap_or(&0));
        let main_separator_byte: u8 = std::path::MAIN_SEPARATOR.to_string().as_bytes()[0];
        last_byte == main_separator_byte
    }
}

/// A set of callbacks for the watched file events.
pub trait Callbacks {
    /// A function that gets called when the watched file shows up.
    ///
    /// `real_path` will contain a path to the watched file, which may
    /// be different from the one that was passed to `FileWatcher`,
    /// because of symlinks.
    fn file_appeared(&mut self, real_path: &Path);
    /// A function that gets called when the watched file is written to.
    ///
    /// Note that this is called only when the real file is
    /// modified. In case when some symlink in the watched paths is
    /// atomically changed to point to something else,
    /// `file_disappeared` followed by `file_appeared` will be
    /// actually called.
    ///
    /// `real_path` will contain a path to the watched file, which may
    /// be different from the one that was passed to `FileWatcher`,
    /// because of symlinks.
    fn file_modified(&mut self, real_path: &Path);
    /// A function that gets called when the watched file goes away.
    ///
    /// `real_path` will contain a path to the watched file, which may
    /// be different from the one that was passed to `FileWatcher`,
    /// because of symlinks.
    fn file_disappeared(&mut self, real_path: &Path);
}

/// FileWatcher watches for a regular file at any path. The file does not have to exist when the
/// FileWatcher is created.
///
/// FileWatcher uses callbacks to notify about events occuring to the watched file. Note that it
/// will call file_appeared callback during its first run if the file existed when the watcher was
/// created and ignore_initial is set false.
#[derive(Debug)]
pub struct FileWatcher<C: Callbacks, W: Watcher> {
    /// The Callback implementation to be used by this FileWatcher
    callbacks:      C,
    /// The Watcher implementation to be used by this FileWatcher.
    watcher:        W,
    /// An std::sync::mpsc::channel used to receive events.
    rx:             Receiver<notify::Result<Event>>,
    /// The actual file to be watched.
    watched_file:   WatchedFile,
    active_watches: HashSet<PathBuf>,
}

/// Convenience function for returning a new FileWatcher
pub fn create_file_watcher<P, C>(path: P,
                                 callbacks: C,
                                 ignore_initial: bool)
                                 -> Result<FileWatcher<C, SupWatcher>>
    where P: Into<PathBuf>,
          C: Callbacks
{
    trace!("create_file_watcher(path,callbacks,{:?})", ignore_initial);
    FileWatcher::<C, SupWatcher>::new(path, callbacks, ignore_initial)
}

impl<C: Callbacks, W: Watcher> FileWatcher<C, W> {
    /// Creates an instance of `W` and creates a WatchedFile using the supplied path.  If
    /// ignore_initial is passed as false and the WatchedFile exists when this is called then the
    /// first loop will emit an initial "file_appeared" event.  
    fn new<P>(path: P, callbacks: C, ignore_initial: bool) -> Result<Self>
        where P: Into<PathBuf>
    {
        trace!("FileWatcher::new(path,callbacks,{:?})", ignore_initial);
        let (tx, rx) = channel::<notify::Result<Event>>();
        let config = Config::default().with_poll_interval(Duration::from_millis(WATCHER_DELAY_MS));
        let watcher = W::new(tx, config).map_err(Error::NotifyCreateError)?;
        let p = path.into();
        let watched_file = WatchedFile::new(&p, !ignore_initial)?;
        let active_watches = HashSet::new();
        let mut file_watcher = Self { callbacks,
                                      watcher,
                                      rx,
                                      watched_file,
                                      active_watches };
        file_watcher.manage_watches()?;
        Ok(file_watcher)
    }

    /// manage_watches() wraps up the essential functions around "the watch" for the WatchedFile
    /// being watched by the FileWatcher.  Its really meant to be called in two places.  First,
    /// during construction of FileWatcher instances.  Second, as late as possible in the event
    /// handling process. Its called during construction because we need to assess "the state of
    /// the world" when the watch of the file begins so that we know how to respond to the next
    /// event received. We then defer calling this again until as late as possible in the process
    /// of responding to an event because our response needs to be within the context of the last
    /// action we took.
    fn manage_watches(&mut self) -> Result<()> {
        trace!("FileWatcher::manage_watches()");
        self.clear_active_watches()?;
        self.watched_file.state = self.watched_file.assess_state();
        self.set_new_watches()
    }

    fn set_new_watches(&mut self) -> Result<()> {
        trace!("FileWatcher::set_new_watches()");
        match &self.watched_file.state {
            WatchedFileState::ExistentFile => {
                Ok(self.watch(&self.watched_file.path.clone())
                       .and(self.watch(&self.watched_file.directory.clone()))?)
            }
            WatchedFileState::ExistentDirectory => Ok(self.watch(&self.watched_file.directory.clone())?),
            WatchedFileState::NonExistent => Ok(()),
            WatchedFileState::Invalid(_) => {
                let s = self.watched_file
                            .path
                            .as_os_str()
                            .to_string_lossy()
                            .to_string();
                Err(Error::FileNotFound(s))
            }
        }
    }

    fn watch(&mut self, path: &Path) -> Result<()> {
        trace!("FileWatcher::watch({:?})", path);
        self.active_watches.insert(path.to_path_buf());
        self.watcher
            .watch(path, RecursiveMode::NonRecursive)
            .map_err(Error::NotifyError)
    }

    fn clear_active_watches(&mut self) -> Result<()> {
        trace!("FileWatcher::clear_active_watches()");
        let laws = &mut self.active_watches.clone();
        for aw in laws.iter() {
            if laws.contains(aw) {
                self.unwatch(aw.as_ref())?;
            }
        }
        laws.clear();
        Ok(())
    }

    fn unwatch(&mut self, path: &Path) -> Result<()> {
        trace!("FileWatcher::unwatch({:?})", path);
        let r = self.watcher.unwatch(path);
        match r {
            Ok(_) => Ok(()),
            Err(e) => {
                match e.kind {
                    notify::ErrorKind::WatchNotFound => Ok(()),
                    _ => Err(Error::NotifyError(e)),
                }
            }
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let loop_value: liveliness_checker::ThreadUnregistered<_, _> = loop {
            let checked_thread = liveliness_checker::mark_thread_alive();
            if let result @ Err(_) = self.single_iteration() {
                break checked_thread.unregister(result);
            }
            thread::sleep(Duration::from_secs(1));
        };
        loop_value.into_result()
    }

    pub fn single_iteration(&mut self) -> Result<()> {
        trace!("FileWatcher::single_iteration()");

        if self.watched_file.send_initial_event {
            self.callbacks.file_appeared(&self.watched_file.path);
            self.watched_file.send_initial_event = false;
            // this should only ever trigger the one time
        }

        if self.watched_file.state == WatchedFileState::NonExistent {
            // if we've found ourselves with a WatchedFile value that has passed our validation but
            // doesn't seem to exist then we we have something to watch for but nothing that we can
            // pass to the notify crate to notify about.  So, here we assess the current state to
            // see if that has changed.
            let previous_state = self.watched_file.state.clone(); // JAH: unnecessary clone?
            self.watched_file.state = self.watched_file.assess_state();
            match self.watched_file.state {
                WatchedFileState::ExistentFile => {
                    self.set_new_watches()?;
                    if previous_state == WatchedFileState::NonExistent {
                        self.callbacks.file_appeared(&self.watched_file.path);
                    }
                    return Ok(());
                }
                WatchedFileState::ExistentDirectory => {
                    self.set_new_watches()?;
                    return Ok(());
                }
                WatchedFileState::NonExistent => return Ok(()),
                WatchedFileState::Invalid(_) => {
                    let s = self.watched_file
                                .path
                                .as_os_str()
                                .to_string_lossy()
                                .to_string();
                    return Err(Error::FileNotFound(s)); // JAH: Think about how we would get here
                }
            }
        }

        match self.rx.try_recv() {
            Ok(notify_result) => self.handle_event(&notify_result?),
            Err(TryRecvError::Empty) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn handle_event(&mut self, notify_event: &Event) -> Result<()> {
        debug!("FileWatcher::handle_event(..)\n{:?}\n", notify_event);
        if notify_event.need_rescan() {
            // JAH: think about rescans some more
            self.handle_file_modification(notify_event)
        } else {
            // ModifyKind below doesn't derive Copy which is what drives this call to clone.  It
            // seems like the easiest decision to make that solves several clippy warnings because
            // if the notify project ever does derive copy then we can just remove this.
            let notify_event_kind = notify_event.kind.clone();
            match notify_event_kind {
                EventKind::Any => {
                    // v5.0.0: debounced, FSEvent, and Windows backends
                    self.log_ignored_event(notify_event);
                    // debounced: debounced mode in v5 seems to throw away all event kind info
                    // FSEvents: used in "imprecise mode" which may be debounced mode
                    // Windows: Used as a default value and then appears to be reliably changed to
                    // the "real kind" before the event is used.
                }
                EventKind::Access(access_kind) => {
                    self.handle_access_event(access_kind, notify_event)
                }
                EventKind::Create(create_kind) => {
                    self.handle_create_event(create_kind, notify_event);
                }
                EventKind::Modify(modify_kind) => {
                    self.handle_modify_event(&modify_kind, notify_event)
                }
                EventKind::Remove(remove_kind) => {
                    self.handle_remove_event(remove_kind, notify_event)
                }
                EventKind::Other => {
                    // v5.0.0: FSEvents, inotify, and kqueue
                    self.log_ignored_event(notify_event);
                    // FSEvents: used in conjunction with need_rescan() to signal that FSEvents is
                    // signaling to rescan subdirectories so will be covered by need_rescan flag
                    // inotify: Used in conjunction with need_rescan to signal event queue overflow
                    // and I believe that it will also be covered by need rescan check above.
                    // kqueue: used as a catch because "on different BSD variants, different extra
                    // events may be present"
                }
            }
        }
        Ok(())
    }

    fn handle_access_event(&mut self, access_kind: AccessKind, notify_event: &Event) {
        trace!("FileWatch::handle_access_event");
        match access_kind {
            AccessKind::Close(AccessMode::Write) => {
                // v5.0.0: inotify
                self.handle_file_modification(notify_event);
            }
            _ => self.log_ignored_event(notify_event),
            // We really don't care about mere file access unless its a write on file close.
            // There's AccessKind::Open(AccessMode::Write) covered in this but as of v5.0.0 its not
            // in use and I'm not sure we would need to handle opening for write anyway
        }
    }

    fn handle_create_event(&mut self, create_kind: CreateKind, notify_event: &Event) {
        trace!("FileWatch::handle_create_event");
        match create_kind {
            CreateKind::Any // v5.0.0: FSEvent, poll, and windows
            | CreateKind::File => { // v5.0.0: FSEvent and inotify
                self.handle_file_appearance(notify_event);
                // JAH: Will "Any" require special handling on poll and windows to account for 
                // directories?
            }
            CreateKind::Folder // v5.0.0: FSEvent and inotify
            | CreateKind::Other => { // v5.0.0: FSEvent
                self.log_ignored_event(notify_event);
                // We're ignoring folder creation because we're watching files and a requirement
                // of FileWatcher is that either the file or its enclosing folder exists.  So if the
                // folder creation doesn't matter in that case.
            }
        }
    }

    fn handle_data_change_event(&mut self, data_change: DataChange, notify_event: &Event) {
        trace!("FileWatch::handle_data_change_event");
        match data_change {
            DataChange::Any => { // v5.0.0: inotify, kqueue, and poll
                // This case is also trigged by being called in the following cases
                // - ModifyKind::Any 
                // - ModifyKind::Metadata(MetadataKind::WriteTime)
                self.handle_file_modification(notify_event);
                // If we know a modification has happened but don't know exactly what happened then 
                // we will notify of a file modification. Similarly, if we are notified that a file 
                // write time was modified (which is how the poll backend works) then we will notify
                // of a file modification. 
            }
            DataChange::Size // v5.0.0: kqueue
            | DataChange::Content // v5.0.0: FSEvent 
            | DataChange::Other => { // v5.0.0: not implemented by any backend
                self.log_ignored_event(notify_event);
                // These are platforms we don't target today or unimplemented in notify
            }
        }
    }

    fn handle_modify_event(&mut self, modify_kind: &ModifyKind, notify_event: &Event) {
        trace!("FileWatch::handle_modify_event");
        match modify_kind {
            ModifyKind::Any // v5.0.0: kqueue and windows (winnt::FILE_ACTION_MODIFIED)
            | ModifyKind::Metadata(MetadataKind::WriteTime) => { // v5.0.0: poll (file was written)
                self.handle_data_change_event(DataChange::Any, notify_event)
            }
            ModifyKind::Data(data_change) => {
                self.handle_data_change_event(*data_change, notify_event)
            }
            ModifyKind::Name(rename_mode) => {
                self.handle_rename_event(*rename_mode, notify_event)
            }
            ModifyKind::Metadata(MetadataKind::Any)
            | ModifyKind::Metadata(MetadataKind::Ownership)
            | ModifyKind::Metadata(MetadataKind::Permissions) => {
                self.log_ignored_event(notify_event)
                // Changes to ownership and permissions can make files and directories appear or
                // disappear. However, in v5.0.0 Ownership is only used for FSEvents and Permissions
                // isn't implemented by anything only being found in a single test.
                //
                // This section also account for MetadataKind::Any because in notify v5.0.0 all uses
                // of MetadataKind::Any seem to be related to metadata/attrs broadly but in an way 
                // that we can't know if its ownership, permissions, or something else without going
                // deeper than we probably want to.
                // - FSEVents: StreamFlags::INODE_META_MOD 'only described as "metadata changed"'
                // - inotify: EventMask::ATTRIB which might be permissions and ownership
                // - kqueue: kqueue::Vnode::Attrib might also be permissions and ownership
            }
            ModifyKind::Metadata(MetadataKind::AccessTime) // we don't care about mere file access
            | ModifyKind::Metadata(MetadataKind::Extended) // FSEvents: macOS Extended Attributes
            | ModifyKind::Metadata(MetadataKind::Other) // FSEvents: Finder info
            | ModifyKind::Other => { // v5.0.0 - Not Implemented 
                self.log_ignored_event(notify_event)
            }
        }
    }

    fn handle_remove_event(&mut self, remove_kind: RemoveKind, notify_event: &Event) {
        trace!("FileWatch::handle_remove_event");
        match remove_kind {
            RemoveKind::Any // v5.0.0: FSEvent, poll, and windows
            // JAH: Does Any require special handling on poll or windows?
            | RemoveKind::File => { // v5.0.0: FSEvent and inotify
                self.handle_file_disappearance(notify_event);
            }
            RemoveKind::Folder => { // v5.0.0: FSEvent and inotify
                self.handle_folder_disappearance(notify_event);
            }
            RemoveKind::Other => { // v5.0.0: FSEvent
                self.log_ignored_event(notify_event);
            }
        }
    }

    fn handle_rename_event(&mut self, rename_mode: RenameMode, notify_event: &Event) {
        trace!("FileWatch::handle_rename_event");
        match rename_mode {
            RenameMode::To => { // v5.0.0: inotify and windows
                // Per notify crate doc "An event emitted on the file or folder resulting from a
                // rename." This case aligns well with the handle_file_appearance requirements.
                self.handle_file_appearance(notify_event);
            }
            RenameMode::From => { // v5.0.0: FSEvents, inotify, and windows
                // Per notify crate doc "An event emitted on the file or folder that was renamed."
                // This case aligns well with handle_file_disappearance
                self.handle_file_disappearance(notify_event);
            }
            RenameMode::Both => { // v5.0.0: inotify
                // Per notify crate doc "A single event emitted with both the From and To paths. 
                // This event should be emitted when both source and target are known. The paths 
                // should be provided in this exact order (from, to)." We should be able to leverage
                // handle_file_appearance and handle_file_disappearance here.  Its a bit hackish 
                // and wasteful but given that what is known about the state of everything we can 
                // just call handle_file_appearance and handle_file_disappearance back to back with 
                // the same argument because its reasonable to assume that the values in the paths
                // Vec are unique paths so only one of them should trigger.
                self.handle_file_appearance(notify_event);
                self.handle_file_disappearance(notify_event);
            }
            RenameMode::Any // v5.0.0: FSEvent and kqueue
            | RenameMode::Other => { // v5.0.0: not implemented
                self.log_ignored_event(notify_event);
            }
        }
    }

    /// File Appearance means
    /// * a valid WatchedFile
    /// * in WatchedFileState::ExistentDirectory
    /// * receiving an creation or rename event matching one of the events we watch
    /// * where WatchedFile.path matches one of the paths in the event
    fn handle_file_appearance(&mut self, notify_event: &Event) {
        trace!("FileWatcher::handle_file_appearance\n-=> {:?}\n-=> {:?}\n",
               notify_event,
               self.watched_file);
        for path_buf in notify_event.paths.iter() {
            let pb = match path_buf.canonicalize() {
                Ok(v) => v,
                Err(e) => {
                    debug!("failed canonicalization: {:?}", e);
                    path_buf.to_path_buf()
                }
            };
            if self.watched_file.state == WatchedFileState::ExistentDirectory {
                if self.watched_file.path == pb || self.watched_file.directory == pb {
                    match self.manage_watches() {
                        Ok(_) => {
                            self.callbacks.file_appeared(&self.watched_file.path);
                            break;
                        }
                        Err(e) => {
                            debug!("While managing watches: {:?}", e);
                            self.log_ignored_path(notify_event, path_buf)
                        }
                    }
                } else {
                    debug!("fell through");
                    self.log_ignored_path(notify_event, path_buf);
                }
            }
        }
    }

    /// File Modification means
    /// * a valid WatchedFile
    /// * in WatchedFileState::ExistentFile
    /// * receiving an event matching one of the various modification events we watch
    /// * where WatchedFile.path matches one of the paths in the event
    fn handle_file_modification(&mut self, notify_event: &Event) {
        trace!("FileWatcher::handle_file_modification\n-=> {:?}\n-=> {:?}\n",
               notify_event,
               self.watched_file);
        for path_buf in notify_event.paths.iter() {
            let pb = match path_buf.canonicalize() {
                Ok(v) => v,
                Err(e) => {
                    debug!("failed canonicalization: {:?}", e);
                    path_buf.to_path_buf()
                }
            };
            if self.watched_file.state == WatchedFileState::ExistentFile {
                if self.watched_file.path == pb || self.watched_file.directory == pb {
                    match self.manage_watches() {
                        Ok(_) => {
                            self.callbacks.file_modified(&self.watched_file.path);
                            break;
                        }
                        Err(e) => {
                            debug!("While managing watches: {:?}", e);
                            self.log_ignored_path(notify_event, path_buf)
                        }
                    }
                } else {
                    debug!("fell through");
                    self.log_ignored_path(notify_event, path_buf);
                }
            }
        }
    }

    /// File Disappearance means
    /// * a valid WatchedFile
    /// * in WatchedFileState::ExistentFile
    /// * receiving an remove or rename event matching one of the events we watch
    /// * where WatchedFile.path matches one of the paths in the event
    fn handle_file_disappearance(&mut self, notify_event: &Event) {
        trace!("FileWatcher::handle_file_disappearance\n-=> {:?}\n-=> {:?}\n",
               notify_event,
               self.watched_file);
        for path_buf in notify_event.paths.iter() {
            let pb = match path_buf.canonicalize() {
                Ok(v) => v,
                Err(e) => {
                    debug!("failed canonicalization: {:?}", e);
                    path_buf.to_path_buf()
                }
            };
            if self.watched_file.state == WatchedFileState::ExistentFile {
                if self.watched_file.path == pb || self.watched_file.directory == pb {
                    match self.manage_watches() {
                        Ok(_) => {
                            self.callbacks.file_disappeared(&self.watched_file.path);
                            break;
                        }
                        Err(e) => {
                            debug!("While managing watches: {:?}", e);
                            self.log_ignored_path(notify_event, path_buf)
                        }
                    }
                } else {
                    debug!("fell through");
                    self.log_ignored_path(notify_event, path_buf);
                }
            }
        }
    }

    fn handle_folder_disappearance(&mut self, notify_event: &Event) {
        trace!("FileWatcher::handle_folder_disappearance\n-=> {:?}\n-=> {:?}\n",
               notify_event,
               self.watched_file);
        if self.watched_file.state != WatchedFileState::NonExistent {
            for path_buf in notify_event.paths.iter() {
                let pb = match path_buf.canonicalize() {
                    Ok(v) => v,
                    Err(e) => {
                        debug!("failed canonicalization: {:?}", e);
                        path_buf.to_path_buf()
                    }
                };
                if self.watched_file.path == pb || self.watched_file.directory == pb {
                    match self.manage_watches() {
                        Ok(_) => {
                            break;
                        }
                        Err(e) => {
                            debug!("While managing watches: {:?}", e);
                            self.log_ignored_path(notify_event, path_buf)
                        }
                    }
                } else {
                    debug!("fell through");
                    self.log_ignored_path(notify_event, path_buf);
                }
            }
        }
    }

    // Logging that a particular event was ignored.  Ignoring an event may be
    // the correct thing to do so the purpose of logging that a particular event
    // was ignored event is to highlight the fact that an event was ignored in
    // case the decision made today to ignore that event needs to be changed
    // due to design changes, lessons learned (bugs) or notify crate updates.
    fn log_ignored_event(&mut self, notify_event: &Event) { trace!("IGNORING {:?}", notify_event) }

    // Logging the fact that a particular path was ignored.  Ignoring a given
    // path may be the correct thing to do so the purpose of logging that a
    // particular path was ignored is to highlight that the path was ignored in
    // case the decision made today to ignore the event needs to be changed
    // due to design changes, lessons learned (bugs) or notify crate updates.
    fn log_ignored_path(&mut self, notify_event: &Event, path_buf: &Path) {
        trace!("IGNORING {:?} of {:?}", path_buf, notify_event)
    }
}

// scenario, that involves symlinks.
#[cfg(test)]
mod tests {

    use super::*;
    use std::path::{Path,
                    PathBuf};

    use log::trace;
    use multimap::MultiMap;
    use tempfile::tempdir;

    use crate::error::Result;

    use super::Callbacks;

    mod watched_file {

        use super::*;

        #[test]
        fn get_enclosing_directory() {
            // logger::init();

            // testing a possible file that has a file extension
            let mut path = Path::new("/some/path/foo.txt");
            let mut expectation = Some(PathBuf::from("/some/path"));
            let mut result = WatchedFile::get_enclosing_directory(path);
            assert_eq!(result, expectation);

            // testing a possible file that does not have a file extension
            path = Path::new("/some/path/foo");
            expectation = Some(PathBuf::from("/some/path"));
            result = WatchedFile::get_enclosing_directory(path);
            assert_eq!(result, expectation);

            // testing a directory path. While correct behavior this is sketchy usage.  Study the
            // function we're testing before you leverage it elsewhere in the code base.
            path = Path::new("/some/path/");
            expectation = Some(PathBuf::from("/some"));
            result = WatchedFile::get_enclosing_directory(path);
            assert_eq!(result, expectation);

            // this is testing the None case where root is passed in.
            path = Path::new("/");
            expectation = Some(PathBuf::from("/"));
            result = WatchedFile::get_enclosing_directory(path);
            assert_eq!(result, expectation);

            // testing a pathological case
            path = Path::new("");
            expectation = None;
            result = WatchedFile::get_enclosing_directory(path);
            assert_eq!(result, expectation);
        }

        #[test]
        fn discern_state_of() -> Result<()> {
            // crate::logger::init();

            // what to test
            // (1a) existing file with extension
            // (1b) existing file without extension
            // (2) existent directory / is_dir() violated
            // (3a) ends with a path separator violation
            // (3b) ends with a path separator acceptance (file without extension)
            // (4) begins with a root dir or prefix violation
            // (5a) parsable enclosing directory that exists
            // (5b) parsable enclosing directory that does not exist
            // (6) valid path but neither file nor directory exist
            // (7) else case: pathlogical case of empty string passed in
            //      (7x) else case: xxx

            // TESTING (4)
            let mut p = Path::new("path/without/root/foo.txt");
            assert_eq!(WatchedFile::discern_state_of(p),
                       WatchedFileState::Invalid(errmsg::invalid_root(p)));

            // TESTING (3a)
            p = Path::new("/path/ending/in/file/separator/");
            assert_eq!(WatchedFile::discern_state_of(p),
                       WatchedFileState::Invalid(errmsg::invalid_ending(p)));

            let tempdir = tempdir()?;
            let dir_a = tempdir.path().join("a");
            fs::create_dir(&dir_a)?;

            // TESTING (2)
            assert_eq!(WatchedFile::discern_state_of(dir_a.as_path()),
                       WatchedFileState::Invalid(errmsg::is_directory(dir_a.as_path())));

            // TESTING(5a)
            let file_foo_txt = dir_a.join("foo.txt");
            let mut state = WatchedFile::discern_state_of(file_foo_txt.as_path());
            assert_eq!(state, WatchedFileState::ExistentDirectory);

            // TESTING (1a)
            fs::create_file(&file_foo_txt)?;
            state = WatchedFile::discern_state_of(file_foo_txt.as_path());
            assert_eq!(state, WatchedFileState::ExistentFile);

            // TESTING (5b), (6)
            let dir_b = tempdir.path().join("b");
            let file_bar = dir_b.join("bar");
            state = WatchedFile::discern_state_of(file_bar.as_path());
            assert_eq!(state, WatchedFileState::NonExistent);

            // TESTING (1b), (3b)
            fs::create_dir(&dir_b)?;
            fs::create_file(&file_bar)?;
            state = WatchedFile::discern_state_of(file_bar.as_path());
            assert_eq!(state, WatchedFileState::ExistentFile);

            // TESTING: (7)
            p = Path::new("");
            state = WatchedFile::discern_state_of(p);
            assert_eq!(state, WatchedFileState::Invalid(errmsg::invalid_root(p)));

            Ok(())
        }

        #[test]
        #[cfg(not(windows))]
        fn begins_with_a_root_dir_or_prefix() {
            let a = Path::new("/path/beginning/with/separator/");
            assert!(WatchedFile::begins_with_a_root_dir_or_prefix(a));

            let b = Path::new("path/not/beginning/with/separator/");
            assert!(!WatchedFile::begins_with_a_root_dir_or_prefix(b));

            let c = Path::new("/path/beginning/with/separator/but/not/ending/with/one");
            assert!(WatchedFile::begins_with_a_root_dir_or_prefix(c));

            let d = Path::new("path/not/beginning/with/separator/and/not/ending/with/one");
            assert!(!WatchedFile::begins_with_a_root_dir_or_prefix(d));
        }

        #[test]
        #[cfg(windows)]
        fn windows_begins_with_a_root_dir_or_prefix() {
            // These test are modelled after and extend from the Example section in
            // https://doc.rust-lang.org/stable/std/path/enum.Prefix.html
            assert!(WatchedFile::begins_with_a_root_dir_or_prefix(Path::new(r"C:\Users\Rust\Pictures\Ferris")));
            assert!(WatchedFile::begins_with_a_root_dir_or_prefix(Path::new(r"\\server\share")));
            assert!(WatchedFile::begins_with_a_root_dir_or_prefix(Path::new(r"\\?\pictures\kittens")));
            assert!(WatchedFile::begins_with_a_root_dir_or_prefix(Path::new(r"\\?\pictures\kittens")));
            assert!(WatchedFile::begins_with_a_root_dir_or_prefix(Path::new(r"\\?\UNC\server\share")));
            assert!(WatchedFile::begins_with_a_root_dir_or_prefix(Path::new(r"\\?\c:\")));
            assert!(WatchedFile::begins_with_a_root_dir_or_prefix(Path::new(r"\\.\BrainInterface")));
            assert!(WatchedFile::begins_with_a_root_dir_or_prefix(Path::new(r"\\server\share")));
            // windows style backslash with and with out trailing slash but lacking preceding \
            assert!(!WatchedFile::begins_with_a_root_dir_or_prefix(Path::new(r"Users\Rust\Pictures\Ferris")));
            assert!(!WatchedFile::begins_with_a_root_dir_or_prefix(Path::new(r"Users\Rust\Pictures\Ferris\")));
            // windows style backslash with and with out trailing slash with preceding \
            assert!(!WatchedFile::begins_with_a_root_dir_or_prefix(Path::new(r"\Users\Rust\Pictures\Ferris")));
            assert!(!WatchedFile::begins_with_a_root_dir_or_prefix(Path::new(r"\Users\Rust\Pictures\Ferris\")));
        }

        #[test]
        #[cfg(not(windows))]
        fn ends_with_a_path_separator() {
            let a = Path::new("/path/ending/with/separator/");
            assert!(WatchedFile::ends_with_a_path_separator(a));

            let b = Path::new("/path/not/ending/with/separator");
            assert!(!WatchedFile::ends_with_a_path_separator(b));

            let c = Path::new("path/not/beginning/with/separator/but/ending/with/one/");
            assert!(WatchedFile::ends_with_a_path_separator(c));

            let d = Path::new("path/not/beginning/with/separator/and/not/ending/with/one");
            assert!(!WatchedFile::ends_with_a_path_separator(d));
        }

        #[test]
        #[cfg(windows)]
        fn windows_ends_with_a_path_separator() {
            let a = Path::new(r"\\server\share\path\ending\with\separator\");
            assert!(WatchedFile::ends_with_a_path_separator(a));

            let b = Path::new(r"\\server\share\path\not\ending\with\separator");
            assert!(!WatchedFile::ends_with_a_path_separator(b));
        }
    }

    mod file_watcher {

        use super::*;

        #[test]
        #[allow(unused_must_use)] // Just testing instantion ability so this is OK
        fn create_file_watcher() -> Result<()> {
            crate::logger::init();

            // testing the case that neither the file nor the enclosing directory exists
            let p = Path::new("/some/thing/that/does/not/exist.txt");
            let mut fw = super::create_file_watcher(&p, TestCallbacks::default(), true)?;
            assert_eq!(fw.watched_file.state, WatchedFileState::NonExistent);

            let tempdir = tempdir()?;
            let pb = PathBuf::new();
            let tdpath = tempdir.path();
            let dir_a = pb.join(tdpath).join("a");
            let file_foo_txt = dir_a.join("foo.txt");
            let file_foo = dir_a.join("foo");

            fs::create_dir(dir_a.as_ref());

            // existing directory, non-existent file with extension, false ignore_initial
            fw = super::create_file_watcher(&file_foo_txt, TestCallbacks::default(), false)?;
            assert_eq!(fw.watched_file.state, WatchedFileState::ExistentDirectory);

            // existing directory, non-existent file with extension, true ignore_initial
            fw = super::create_file_watcher(&file_foo_txt, TestCallbacks::default(), true)?;
            assert_eq!(fw.watched_file.state, WatchedFileState::ExistentDirectory);

            // existing directory, non-existent file lacking an extension, false ignore_initial
            fw = super::create_file_watcher(&file_foo, TestCallbacks::default(), false)?;
            assert_eq!(fw.watched_file.state, WatchedFileState::ExistentDirectory);

            // existing directory, non-existent file lacking an extension, true ignore_initial
            fw = super::create_file_watcher(&file_foo, TestCallbacks::default(), true)?;
            assert_eq!(fw.watched_file.state, WatchedFileState::ExistentDirectory);

            fs::create_file(&file_foo);

            // existing directory, existing file lacking an extension, false ignore_initial
            fw = super::create_file_watcher(&file_foo, TestCallbacks::default(), false)?;
            assert_eq!(fw.watched_file.state, WatchedFileState::ExistentFile);

            // existing directory, existing file lacking an extension, true ignore_initial
            fw = super::create_file_watcher(&file_foo, TestCallbacks::default(), true)?;
            assert_eq!(fw.watched_file.state, WatchedFileState::ExistentFile);

            fs::create_file(&file_foo_txt);

            // existing directory, existing file with an extension, false ignore_initial
            fw = super::create_file_watcher(&file_foo_txt, TestCallbacks::default(), false)?;
            assert_eq!(fw.watched_file.state, WatchedFileState::ExistentFile);

            // existing directory, existing file with an extension, true ignore_initial
            fw = super::create_file_watcher(&file_foo_txt, TestCallbacks::default(), true)?;
            assert_eq!(fw.watched_file.state, WatchedFileState::ExistentFile);

            // while the previous test included the ignore_initial in the permutations the following
            // tests explicitly test the behavior for correctness

            // If ignore_initial is passed as false and the WatchedFile exists when this is called
            // then the first loop will emit an initial "file_appeared" event.

            // let tempdir = tempdir()?;
            // let dir_a = tempdir.path().join("a");
            // fs::create_dir(&dir_a)?;
            // let file_foo = dir_a.join("foo.txt");
            // fs::create_file(&file_foo)?;

            // Test the case that the file exists, false is passed, and the event is sent
            let mut tc = TestCallbacks::default();
            fw = super::create_file_watcher(&file_foo, tc, false)?;
            fw.single_iteration()?;
            assert_eq!(1, fw.callbacks.all_callbacks_count());
            assert_eq!(1, fw.callbacks.path_instance_count(&file_foo));

            // Test the case that the file exists, true is passed, and the event is not sent
            tc = TestCallbacks::default();
            fw = super::create_file_watcher(&file_foo, tc, true)?;
            fw.single_iteration()?;
            assert_eq!(0, fw.callbacks.all_callbacks_count());
            assert_eq!(0, fw.callbacks.path_instance_count(&file_foo));

            // JAH: do something to test that improved error message again
            // the reasoning for the following is that I found that the messages were janky as I
            // transition to Sup's FileNotFound error.  This tests a sound use of the message in
            // at least one case and while not exhaustive will hopefully serve to keep things sane.
            // let s = format!("File not found at: {}", p.display());
            // assert_eq!(r.unwrap_err().to_string(), s);

            Ok(())
        }

        #[test]
        fn handle_create_event() -> Result<()> {
            // TEST HARNESS
            // crate::logger::init();
            let tempdir = tempdir()?;
            let dir_a = tempdir.path().join("a");
            let file_foo_txt = dir_a.join("foo.txt");
            let mut tc = TestCallbacks::default();
            let mut fw = super::create_file_watcher(&file_foo_txt, tc, true)?;
            assert_eq!(fw.watched_file.state, WatchedFileState::NonExistent);

            // TESTING: directory creation triggering ExistentDirectory from NonExistent
            tc = TestCallbacks::default();
            fw = super::create_file_watcher(&file_foo_txt, tc, true)?;
            fs::create_dir(&dir_a)?;
            while fw.watched_file.state == WatchedFileState::NonExistent {
                fw.single_iteration()?;
                thread::sleep(Duration::from_millis(1000));
            }
            assert_eq!(fw.watched_file.state, WatchedFileState::ExistentDirectory);
            assert_eq!(fw.callbacks.callback_type_count(CallbackType::Appeared), 0);
            assert_eq!(fw.callbacks.path_instance_count(&file_foo_txt), 0);

            // TESTING: file creation triggering ExistentFile from ExistentDirectory
            tc = TestCallbacks::default();
            fw = super::create_file_watcher(&file_foo_txt, tc, true)?;
            fs::create_file(&file_foo_txt)?;
            while fw.callbacks.all_callbacks_count() < 1 {
                fw.single_iteration()?;
                thread::sleep(Duration::from_millis(1000));
            }
            assert_eq!(fw.watched_file.state, WatchedFileState::ExistentFile);
            assert_eq!(fw.callbacks.callback_type_count(CallbackType::Appeared), 1);
            assert_eq!(fw.callbacks.path_instance_count(&file_foo_txt), 1);

            Ok(())
        }

        #[test]
        fn handle_modify_event() -> Result<()> {
            // TEST HARNESS
            // crate::logger::init();
            let tempdir = tempdir()?;
            let dir_a = tempdir.path().join("a");
            let file_foo = dir_a.join("foo");

            fs::create_dir(&dir_a)?;
            fs::create_file(&file_foo)?;

            let tc = TestCallbacks::default();
            let mut fw = super::create_file_watcher(&file_foo, tc, true)?;
            assert_eq!(WatchedFileState::ExistentFile, fw.watched_file.state);

            // TEST EXECUTION
            fs::write_to_file(&file_foo)?;
            while fw.callbacks.all_callbacks_count() < 1 {
                fw.single_iteration()?;
                thread::sleep(Duration::from_millis(1000));
            }

            // TEST EVALUATION
            assert_eq!(1, fw.callbacks.callback_type_count(CallbackType::Modified));
            assert_eq!(1, fw.callbacks.path_instance_count(&file_foo));
            assert_eq!(WatchedFileState::ExistentFile, fw.watched_file.state);

            Ok(())
        }

        #[test]
        fn handle_remove_event() -> Result<()> {
            // TEST HARNESS
            // crate::logger::init();
            let tempdir = tempdir()?;
            let dir_a = tempdir.path().join("a");
            let file_foo = dir_a.join("foo");

            fs::create_dir(&dir_a)?;
            fs::create_file(&file_foo)?;

            let tc = TestCallbacks::default();
            let mut fw = super::create_file_watcher(&file_foo, tc, true)?;
            assert_eq!(WatchedFileState::ExistentFile, fw.watched_file.state);

            // TEST EXECUTION
            fs::remove(&file_foo)?;

            while fw.callbacks.all_callbacks_count() < 1 {
                fw.single_iteration()?;
                thread::sleep(Duration::from_millis(1000));
            }
            // Event { kind: Modify(Metadata(Any)), paths: ["/tmp/.tempDir/a/foo"],
            // attr:tracker: None, attr:flag: None, attr:info: None, attr:source: None }
            //  will be ignored
            // Event { kind: Remove(File), paths: ["/tmp/.tempDir/a/foo"], attr:tracker: None,
            // attr:flag: None, attr:info: None, attr:source: None }
            //  will be fired
            assert_eq!(1,
                       fw.callbacks.callback_type_count(CallbackType::Disappeared));
            assert_eq!(1, fw.callbacks.path_instance_count(&file_foo));
            assert_eq!(WatchedFileState::ExistentDirectory, fw.watched_file.state);

            Ok(())
        }
    }

    #[derive(Debug, Default)]
    struct TestCallbacks {
        type_occurrences: MultiMap<CallbackType, PathBuf>,
        path_occurrences: MultiMap<PathBuf, CallbackType>,
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum CallbackType {
        Appeared,
        Modified,
        Disappeared,
    }

    impl Callbacks for TestCallbacks {
        fn file_appeared(&mut self, real_path: &Path) {
            trace!("TestCallbacks::file_appeared({:?})", real_path);
            self.capture_callback(CallbackType::Appeared, real_path)
        }

        fn file_modified(&mut self, real_path: &Path) {
            trace!("TestCallbacks::file_modified({:?})", real_path);
            self.capture_callback(CallbackType::Modified, real_path)
        }

        fn file_disappeared(&mut self, real_path: &Path) {
            trace!("TestCallbacks::file_disappeared({:?})", real_path);
            self.capture_callback(CallbackType::Disappeared, real_path)
        }
    }

    impl TestCallbacks {
        fn capture_callback<P>(&mut self, callback_type: CallbackType, path: P)
            where P: AsRef<Path>
        {
            self.type_occurrences
                .insert(callback_type, path.as_ref().to_path_buf());
            self.path_occurrences
                .insert(path.as_ref().to_path_buf(), callback_type);
        }

        fn all_callbacks_count(&self) -> usize {
            #[allow(unused_variables)] // k isn't used below
            self.type_occurrences
                .iter_all()
                .fold(0, |acc, (k, v)| acc + v.len())
        }

        fn callback_type_count(&self, callback_type: CallbackType) -> usize {
            self.type_occurrences.get(&callback_type).iter().count()
        }

        fn path_instance_count<P>(&self, path: P) -> usize
            where P: AsRef<Path>
        {
            self.path_occurrences
                .get(&path.as_ref().to_path_buf())
                .iter()
                .count()
        }

        fn _path_count(&self) -> usize {
            #[allow(unused_variables)] // k isn't used below
            self.path_occurrences
                .iter_all()
                .fold(0, |acc, (k, v)| acc + v.len())
        }
    }

    mod fs {
        use log::trace;
        use std::{fmt::Debug,
                  fs::File,
                  io,
                  io::Write,
                  path::Path};

        /// Creates a file, writes some "Lorem ipsem", and syncs the file.  You should create the
        /// containing directory before calling it
        // pub fn create_file<P>(path: P) -> io::Result<File>
        // where P: AsRef<Path> + Debug
        pub fn create_file(path: &Path) -> io::Result<File> {
            trace!("fs::create_file({:?})", path);
            let mut file = std::fs::File::create(&path)?;
            file.write_all(b"Lorem ipsum dolor sit amet,....")?;
            file.sync_all()?;
            Ok(file)
        }

        /// Create a directory.  This is assumes "a complete path" from root to final directory.  
        /// This function creates intermediate directories.
        // pub fn create_dir<T>(path: T) -> io::Result<()>
        //     where T: AsRef<Path> + Debug
        pub fn create_dir(path: &Path) -> io::Result<()> {
            trace!("fs::create_dir({:?})", path);
            std::fs::create_dir_all(path)
        }

        /// if the path passed in
        pub fn remove<T>(path: T) -> io::Result<()>
            where T: AsRef<Path> + Debug
        {
            trace!("fs::remove({:?})", path);
            let p = path.as_ref();
            if p.is_dir() {
                std::fs::remove_dir_all(&path)
                // Errors when path doesn’t exist, isn’t a directory,  we lack permissions,
                // or directory isn’t empty.
            } else {
                std::fs::remove_file(&path)
                // Errors when file is a dir, doesn't exist, or we lack permissions to delete.
            }
        }

        pub fn write_to_file(path: &Path) -> io::Result<File> {
            trace!("fs::write_to_file({:?})", path);
            let mut file = std::fs::OpenOptions::new().write(true)
                                                      .create(true)
                                                      .open(path)?;
            file.write_all(b"Lorem ipsum dolor sit amet,....")?;
            file.sync_all()?;
            Ok(file)
        }

        // pub fn mv<T>(from: T, to: T) -> io::Result<()>
        //     where T: AsRef<Path> + Debug
        // {
        //     trace!("fs::mv {:?} {:?} ", from, to);
        //     std::fs::rename(from, to)
        // }

        // pub fn ln_s<T>(target: T, link: T) -> io::Result<()>
        //     where T: AsRef<Path> + Debug
        // {
        //     trace!("fs::ln_s {:?} {:?} ", target, link);
        //     std::os::unix::fs::symlink(&target, &link)
        // }
    }
}

// These are all of the various scenarios I could think of that could impact file appearance and
// disappearance from the command line. captured using unix commands I think they all have windows
// equivalents but I'm not as comfortable "thinking windows" as I am "thinking unix".  An entry with
// a + is one that I'm going to try and test and one with a - means that testing will be skipped
// because of knowledge of what the notify crate does. Currently that means no testing of ownership
// or permissions changes because they aren't exposed well in the notify crate because due to
// implementation issues with.  Also, neither concept seems to have been a concern in the previous
// version of the code so this massive update shouldn't be breaking anything as regards permissions
// and ownership that wasn't broken before.
//
// single file
// + creation (touch foo)
// + deletion (rm foo)
// + rename (mv foo bar)
// + move (mv foo ../a/foo)
// + move and rename (mv foo ../a/bar)
// multiple file
// + creation (touch foo bar baz)
// + deletion (rm foo bar baz)
// + move (mv foo bar baz ../dir/)
// single directory
// + create (mkdir a/)
// + deletion (rmdir a/)
// + rename (mv a/ b/)
// + move (mv a/ ../b/)
// + move and rename (mv a/ ../b/c)
// multiple directories
// + creation (mkdir a/ b/ c/)
// + deletion (rmdir a/ b/ c/)
// + rename (mv a/ b/)
// + move (mv a/ ../b/)
// + move and rename (mv a/ ../b/c)
// links
// + file symlink creation (ln -s foo bar)
// + file symlink deletion (rm bar@)
// + file symlink move (mv @bar baz)
// + file hard link creation (ln foo baz)
// + file hard link deletion (rm baz)
// + file hard link move (mv baz qux)
// permissions
// - permissions changed to make file visible
// - permissions changed to make file invisible
// ownership
// - ownership changed to make file visible
// - ownership changed to make file invisible
//
// Besides file appearance and disappearance we are concerned with file modifications which are well
// covered by the notify crate. At the time of this commenting what we should test  can be reviewed
// by considering the follow cases in the code.
// * need_rescan()
// * AccessKind::Close(AccessMode::Write)
// * DataChange events as documented in handle_data_change_events
//
// One final thought is that the previous version of the code used "#[cfg(all(unix, test))]" to
// begin its test section so this version of the code will be explicitly testing windows for the
// first time since whenever that was added and may uncover more things. Also, it would be ideal to
// add explicit testing for the polling back end alongside the windows one.
//
// More information may be found in in the comments embedded inline with the code.
//
// Some random bits of doc collected along the way
// https://man7.org/linux/man-pages/man7/inotify.7.html
// https://www.freebsd.org/cgi/man.cgi?kqueue
//
// What Watches What Using FileWatcher
// 1) UserConfigWatch watches individual files with names of the general form
//    '/hab/user/myservice/config/user.toml'
// 2) PeerWatcher watches a file for the purposes of "connecting to the ring"
//    with an expected naming convention like peer_watch_file = "/path/to/file".
//
// What about SupWatcher and SpecWatcher?
// SpecWatcher users SuperWatch which uses the notify crate to watch files in
// SpecDir. The spec directory is /hab/sup/default/specs and this is a distinct
// use of the notify crate separate from the usage in the FileWatcher hierarchy.
//
// Things to consider:
// 1) Is the file valid? If its TOML, YAML, JSON, etc. we could validate before passing on a notice
//

// What follows are tests that may be completed or tossed depending on exactly what test
// implementation strategy shakes out as this goes along

// // #[test]
// // fn file_appeared_due_to_directory_create() {}
// // doesn't make sense in practice, directory has to be generating its
// // own event and then files are created in the directory with their
// // own events

// #[test]
// fn file_appeared_due_to_symlink_create() {}

// #[test]
// fn file_appeared_due_to_permissions_change() {}

// #[test]
// fn file_appeared_due_to_ownership_change() {}

// #[test]
// fn file_appeared_due_to_file_rename() {}

// #[test]
// fn file_disappeared_due_to_file_delete() {}

// #[test]
// fn file_disappeared_due_to_directory_delete() {}

// #[test]
// fn file_disappeared_due_to_symlink_delete() {}

// #[test]
// fn file_disappeared_due_to_permissions_change() {}

// // fn file_disappeared_due_to_ownership_change() {}

// #[test]
// fn file_disappeared_due_to_file_rename() {}

// #[test]
// fn file_appeared_and_file_disappeared_on_symlink_move() {}
// // think this one through
// // is it enough to have the separate tests above?

// #[test]
// fn file_modified_due_to_file_write() {}

// #[test]
// fn file_modified_due_to_() {}

// #[test]
// fn file_modified_due_to_xxx() {}
//
// #[test]
// `mv foo.txt foo.txt` results in "mv: 'foo.txt' and 'foo.txt' are the same file" but let's
// explicitly test this to make sure that it works across platforms, etc.

// Things to Test:
// Q: what happens when you have a properly constructed WatchedFile instance and the file is
// deleted? A: This feels like normal usage and should be covered but think about it a moment longer
// later
//
// Q: what happens when you have a properly constructed WatchedFile instance and the file and
// enclosing directory are deleted? A: This feels like an exceptional path.  Currently I think this
// would througn an invalid file exception but this needs to be worked through.
