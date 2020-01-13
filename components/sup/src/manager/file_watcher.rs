use crate::{error::{Error,
                    Result},
            manager::debug::{IndentedStructFormatter,
                             IndentedToString}};
use habitat_common::liveliness_checker;
use notify::{self,
             DebouncedEvent,
             RecommendedWatcher,
             RecursiveMode,
             Watcher};
use std::{collections::{hash_map::Entry,
                        HashMap,
                        HashSet,
                        VecDeque},
          env,
          ffi::OsString,
          mem::swap,
          path::{Component,
                 Path,
                 PathBuf},
          sync::mpsc::{channel,
                       Receiver,
                       TryRecvError},
          thread,
          time::Duration};

pub const WATCHER_DELAY_MS: u64 = 2_000;

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
    /// A function that gets called every time an event with paths
    /// happens.
    ///
    /// `paths` contains a list of paths the recent event is related
    /// to. Usually it will be just one, but in case of renames it may
    /// be two.
    // Keep the variable name for documentation purposes, so silence
    // compiler's complaints about unused variable.
    #[allow(unused_variables)]
    fn event_in_directories(&mut self, paths: &[PathBuf]) {}
}

// Essentially a pair of dirname and basename.
#[derive(Clone, Debug, Default)]
struct DirFileName {
    directory: PathBuf,
    file_name: OsString,
}

impl DirFileName {
    // split_path separates the dirname from the basename.
    fn split_path(path: &PathBuf) -> Option<Self> {
        let parent = match path.parent() {
            None => return None,
            Some(p) => p,
        };
        let file_name = match path.file_name() {
            None => return None,
            Some(f) => f,
        };
        Some(Self { directory: parent.to_owned(),
                    file_name: file_name.to_owned(), })
    }

    fn as_path(&self) -> PathBuf { self.directory.join(&self.file_name) }
}

impl IndentedToString for DirFileName {
    fn indented_to_string(&self, spaces: &str, repeat: usize) -> String {
        let mut formatter = IndentedStructFormatter::new("DirFileName", spaces, repeat);
        formatter.add("directory", &self.directory);
        formatter.add("file_name", &self.file_name);
        formatter.fmt()
    }
}

// TODO: handle mount events, we could use libc crate to get the
// select function that we could use to watch /proc/self/mountinfo for
// exceptional events - such event means that something was mounted or
// unmounted. For this to work, we would need to keep a mount state of
// the directories we are interested in and compare it to the current
// status in mountinfo, when some change there happens.

// Similar to DirFileName, but the file_name part is optional.
struct SplitPath {
    directory: PathBuf,
    file_name: Option<OsString>,
}

impl SplitPath {
    fn push(&mut self, path: OsString) -> DirFileName {
        if let Some(ref file_name) = self.file_name {
            self.directory.push(file_name);
        }
        self.file_name = Some(path.clone());

        DirFileName { directory: self.directory.clone(),
                      file_name: path, }
    }
}

impl IndentedToString for SplitPath {
    fn indented_to_string(&self, spaces: &str, repeat: usize) -> String {
        let mut formatter = IndentedStructFormatter::new("SplitPath", spaces, repeat);
        formatter.add("directory", &self.directory);
        formatter.add("file_name", &self.file_name);
        formatter.fmt()
    }
}

// Contents of this type are for `process_path`.
#[derive(Debug)]
struct ProcessPathArgs {
    // The beginning of the patch to process (very often it is root,
    // but not always).
    path: PathBuf,
    // The rest of the path as components ([habitat-operator, peers])
    path_rest: VecDeque<OsString>,
    // Describes the position in the chain, used for determining from
    // where we should start reprocessing the path in case of some
    // events.
    //
    // What is called "chain" here is a list of items we end up
    // watching.
    //
    // TODO(krnowak): Check if we can just remove it.
    index: u32,
    // Previous path in chain, usually a parent directory, but in case
    // of symlinks it becomes a bit more complicated.
    //
    // Item: directory, file or symlink.
    prev: Option<PathBuf>,
}

impl IndentedToString for ProcessPathArgs {
    fn indented_to_string(&self, spaces: &str, repeat: usize) -> String {
        let mut formatter = IndentedStructFormatter::new("ProcessPathArgs", spaces, repeat);
        formatter.add("path", &self.path);
        formatter.add_debug("path_rest", &self.path_rest);
        formatter.add_string("index", self.index.to_string());
        formatter.add("prev", &self.prev);
        formatter.fmt()
    }
}

// This struct tells that for `path` the previous item in chain is
// `prev`.
struct ChainLinkInfo {
    path: PathBuf,
    prev: Option<PathBuf>,
}

impl IndentedToString for ChainLinkInfo {
    fn indented_to_string(&self, spaces: &str, repeat: usize) -> String {
        let mut formatter = IndentedStructFormatter::new("ChainLinkInfo", spaces, repeat);
        formatter.add("path", &self.path);
        formatter.add("prev", &self.prev);
        formatter.fmt()
    }
}

// This struct is passed together with some event actions.
#[derive(Debug)]
struct PathsActionData {
    dir_file_name: DirFileName,
    args:          ProcessPathArgs,
}

impl IndentedToString for PathsActionData {
    fn indented_to_string(&self, spaces: &str, repeat: usize) -> String {
        let mut formatter = IndentedStructFormatter::new("PathsActionData", spaces, repeat);
        formatter.add("dir_file_name", &self.dir_file_name);
        formatter.add("args", &self.args);
        formatter.fmt()
    }
}

// This is stores information about the watched item
//
// TODO(krnowak): Rename it.
struct Common {
    // TODO: maybe drop this? we could also use dir_file_name.as_path()
    path:          PathBuf,
    dir_file_name: DirFileName,
    // Previous watched item in chain, is None for the first watched
    // item.
    prev: Option<PathBuf>,
    // Next watched item in chain, is None for the last watched item.
    next: Option<PathBuf>,
    // That is needed to make sure that the generated process args
    // with lower index will overwrite the generated process args with
    // higher index. Several generated process args can be generated
    // when several files or directories are removed.
    //
    // TODO(krnowak): Not sure if we need it anymore, since we have a
    // simple chain of watches, so the most recent removal event
    // should happen for the element in chain with lowest index.
    index: u32,
    // This is the rest of the components that were left to process at
    // the moment we were processing this path. Useful for
    // reprocessing the path, when the next item in the list was
    // removed/replaced.
    path_rest: VecDeque<OsString>,
}

impl Common {
    fn get_process_path_args(&self) -> ProcessPathArgs {
        let mut path_rest = VecDeque::new();
        path_rest.push_back(self.dir_file_name.file_name.clone());
        path_rest.extend(self.path_rest.iter().cloned());
        ProcessPathArgs { path: self.dir_file_name.directory.clone(),
                          path_rest,
                          index: self.index,
                          prev: self.prev.clone() }
    }

    fn get_chain_link_info(&self) -> ChainLinkInfo {
        ChainLinkInfo { path: self.path.clone(),
                        prev: self.prev.clone(), }
    }

    fn get_paths_action_data(&self) -> PathsActionData {
        PathsActionData { dir_file_name: self.dir_file_name.clone(),
                          args:          self.get_process_path_args(), }
    }

    // The path is at the end of the chain, which means we expect it
    // to be a file. This is because we always expect the watcher to
    // be started on a file.
    fn is_leaf(&self) -> bool { self.path_rest.is_empty() }
}

impl IndentedToString for Common {
    fn indented_to_string(&self, spaces: &str, repeat: usize) -> String {
        let mut formatter = IndentedStructFormatter::new("Common", spaces, repeat);
        formatter.add("path", &self.path);
        formatter.add("dir_file_name", &self.dir_file_name);
        formatter.add("prev", &self.prev);
        formatter.add("next", &self.next);
        formatter.add_string("index", self.index.to_string());
        formatter.add_debug("path_rest", &self.path_rest);
        formatter.fmt()
    }
}

// This is only used to generate `Common` for each item we have in the
// path, so for `/h-o/peers`, it will generate `Common` instance for
// `/h-o` and then for `/h-o/peers` subsequently.
//
// TODO(krnowak): Rename it.
struct CommonGenerator {
    // Usually describes what was the last item we processed. Bit more
    // complicated in case of symlinks.
    prev: Option<PathBuf>,
    // Normally `prev` is just a parent directory (previous processed
    // item), but this is not the case when we are dealing with
    // symlinks. `old_prev` is used then to override `prev` to the
    // previous processed item on next iteration.
    old_prev: Option<PathBuf>,
    // The path for the generated `Common`.
    //
    // TODO(krnowak): drop it? we likely have this information stored
    // in split_path, we could add a as_path() function to SplitPath.
    path: PathBuf,
    // Same as path, but splitted into dirname and basename
    split_path: SplitPath,
    // Index in the chain.
    //
    // TODO(krnowak): Check if this can be dropped.
    index: u32,
    // The rest of the path as components to be processed.
    path_rest: VecDeque<OsString>,
}

impl CommonGenerator {
    fn new(args: ProcessPathArgs) -> Self {
        let split_path = SplitPath { directory: args.path.clone(),
                                     file_name: None, };
        let s = Self { prev: args.prev,
                       old_prev: None,
                       path: args.path,
                       split_path,
                       index: args.index,
                       path_rest: args.path_rest };
        debug!("common generator created: {}", dits!(s));
        s
    }

    fn revert_previous(&mut self) { self.prev = self.old_prev.clone(); }

    fn set_path(&mut self, path: PathBuf) {
        self.path = path;
        self.split_path = SplitPath { directory: self.path.clone(),
                                      file_name: None, };
        debug!("new path in generator: {:?}", self.path);
    }

    fn prepend_to_path_rest(&mut self, mut path_rest: VecDeque<OsString>) {
        path_rest.extend(self.path_rest.drain(..));
        self.path_rest = path_rest;
        debug!("new path rest in generator: {:?}", self.path_rest);
    }

    // Extract a new component from the `path_rest` vec and create a
    // new `Common` instance. If there are not components left in
    // `path_rest`, returns `None`.
    fn get_new_common(&mut self) -> Option<Common> {
        debug!("common generator before new Common: {}", dits!(self));
        let c = if let Some(component) = self.path_rest.pop_front() {
            self.path.push(&component);
            let path = self.path.clone();
            let dir_file_name = self.split_path.push(component);
            let prev = self.prev.clone();

            // This is only used for symlinks. We want to make sure
            // that the previous item for the symlink's target is
            // either the symlink or target's parent directory if we
            // didn't watch the directory before.
            //
            // An example: we watch `/a/b/c`, `c` is a symlink to
            // `/a/x/c`, so after processing `/a/b/c` we want to have
            // a chain like `/a, `/a/b`, `/a/x`, `/a/x/c`.
            //
            // So we use `old_prev` to make sure that the proper
            // previous item in chain will be set for the paths coming
            // after following the symlink.
            swap(&mut self.old_prev, &mut self.prev);
            self.prev = Some(self.path.clone());

            let index = self.index;
            self.index += 1;
            Some(Common { path,
                          dir_file_name,
                          prev,
                          next: None,
                          index,
                          path_rest: self.path_rest.clone() })
        } else {
            None
        };
        debug!("generated common: {}", dits!(c));
        debug!("common generator after new Common: {}", dits!(self));
        c
    }
}

impl IndentedToString for CommonGenerator {
    fn indented_to_string(&self, spaces: &str, repeat: usize) -> String {
        let mut formatter = IndentedStructFormatter::new("CommonGenerator", spaces, repeat);
        formatter.add("prev", &self.prev);
        formatter.add("old_prev", &self.old_prev);
        formatter.add("path", &self.path);
        formatter.add("split_path", &self.split_path);
        formatter.add_string("index", self.index.to_string());
        formatter.add_debug("path_rest", &self.path_rest);
        formatter.fmt()
    }
}

// An item we are interested in.
enum WatchedFile {
    Regular(Common),
    MissingRegular(Common),
    Symlink(Common),
    Directory(Common),
    MissingDirectory(Common),
}

impl WatchedFile {
    fn get_common(&self) -> &Common {
        match self {
            WatchedFile::Regular(ref c)
            | WatchedFile::MissingRegular(ref c)
            | WatchedFile::Symlink(ref c)
            | WatchedFile::Directory(ref c)
            | WatchedFile::MissingDirectory(ref c) => c,
        }
    }

    fn get_mut_common(&mut self) -> &mut Common {
        match self {
            WatchedFile::Regular(ref mut c)
            | WatchedFile::MissingRegular(ref mut c)
            | WatchedFile::Symlink(ref mut c)
            | WatchedFile::Directory(ref mut c)
            | WatchedFile::MissingDirectory(ref mut c) => c,
        }
    }

    fn steal_common(self) -> Common {
        match self {
            WatchedFile::Regular(c)
            | WatchedFile::MissingRegular(c)
            | WatchedFile::Symlink(c)
            | WatchedFile::Directory(c)
            | WatchedFile::MissingDirectory(c) => c,
        }
    }
}

impl IndentedToString for WatchedFile {
    fn indented_to_string(&self, spaces: &str, repeat: usize) -> String {
        let name = match self {
            WatchedFile::Regular(_) => "Regular",
            WatchedFile::MissingRegular(_) => "MissingRegular",
            WatchedFile::Symlink(_) => "Symlink",
            WatchedFile::Directory(_) => "Directory",
            WatchedFile::MissingDirectory(_) => "MissingDirectory",
        };
        format!("{}({})", name, its!(self.get_common(), spaces, repeat))
    }
}

// Similar to std::fs::canonicalize, but without resolving symlinks.
//
// I'm not sure if this is entirely correct, consider:
//
// pwd # displays /some/abs/path
// mkdir -p foo/bar
// ln -s foo/bar baz
// realpath baz/.. # displays /some/abs/path/foo
// cd baz/.. # stays in the same directory instead of going to foo
//
// Basically, realpath says that "baz/.." == "foo" and cd says that
// "baz/.." == ".".
//
// I went here with the "cd" way. Likely less surprising.
fn simplify_abs_path(abs_path: &PathBuf) -> PathBuf {
    let mut simple = PathBuf::new();
    for c in abs_path.components() {
        match c {
            Component::CurDir => (),
            Component::ParentDir => {
                simple.pop();
            }
            _ => simple.push(c.as_os_str()),
        };
    }
    simple
}

// `EventAction`s are high-level actions to be performed in response to
// filesystem events.
//
// We translate `DebouncedEvent` to `EventAction`, and `EventAction`
// to a list of `PathsAction`s.
#[derive(Debug)]
enum EventAction {
    Ignore,
    PlainChange(PathBuf),
    RestartWatching,
    AddRegular(PathsActionData),
    DropRegular(PathsActionData),
    AddDirectory(PathsActionData),
    DropDirectory(PathsActionData),
    RewireSymlink(PathsActionData),
    DropSymlink(PathsActionData),
    SettlePath(PathBuf),
}

impl IndentedToString for EventAction {
    fn indented_to_string(&self, spaces: &str, repeat: usize) -> String {
        match self {
            EventAction::Ignore => "Ignore".to_string(),
            EventAction::PlainChange(ref p) => format!("PlainChange({})", its!(p, spaces, repeat)),
            EventAction::RestartWatching => "RestartWatching".to_string(),
            EventAction::AddRegular(ref pad) => {
                format!("AddRegular({})", its!(pad, spaces, repeat))
            }
            EventAction::DropRegular(ref pad) => {
                format!("DropRegular({})", its!(pad, spaces, repeat))
            }
            EventAction::AddDirectory(ref pad) => {
                format!("AddDirectory({})", its!(pad, spaces, repeat))
            }
            EventAction::DropDirectory(ref pad) => {
                format!("DropDirectory({})", its!(pad, spaces, repeat))
            }
            EventAction::RewireSymlink(ref pad) => {
                format!("RewireSymlink({})", its!(pad, spaces, repeat))
            }
            EventAction::DropSymlink(ref pad) => {
                format!("DropSymlink({})", its!(pad, spaces, repeat))
            }
            EventAction::SettlePath(ref p) => format!("SettlePath({})", its!(p, spaces, repeat)),
        }
    }
}

// Lower-level actions, created to execute `EventAction`s.
#[derive(Debug)]
enum PathsAction {
    NotifyFileAppeared(PathBuf),
    NotifyFileModified(PathBuf),
    NotifyFileDisappeared(PathBuf),
    DropWatch(PathBuf),
    AddPathToSettle(PathBuf),
    SettlePath(PathBuf),
    ProcessPathAfterSettle(ProcessPathArgs),
    RestartWatching,
}

impl IndentedToString for PathsAction {
    fn indented_to_string(&self, spaces: &str, repeat: usize) -> String {
        match self {
            PathsAction::NotifyFileAppeared(ref p) => {
                format!("NotifyFileAppeared({})", its!(p, spaces, repeat + 1))
            }
            PathsAction::NotifyFileModified(ref p) => {
                format!("NotifyFileModified({})", its!(p, spaces, repeat + 1))
            }
            PathsAction::NotifyFileDisappeared(ref p) => {
                format!("NotifyFileDisappeared({})", its!(p, spaces, repeat + 1))
            }
            PathsAction::DropWatch(ref p) => format!("DropWatch({})", its!(p, spaces, repeat + 1)),
            PathsAction::AddPathToSettle(ref p) => {
                format!("AddPathToSettle({})", its!(p, spaces, repeat + 1))
            }
            PathsAction::SettlePath(ref p) => {
                format!("SettlePath({})", its!(p, spaces, repeat + 1))
            }
            PathsAction::ProcessPathAfterSettle(ref a) => {
                format!("ProcessPathAfterSettle({})", its!(a, spaces, repeat + 1))
            }
            PathsAction::RestartWatching => "RestartWatching".to_string(),
        }
    }
}

// Both branch result and leaf result are about the status of adding
// new path to be watched. Branch is about symlinks and directories,
// leaves - about regular files, missing regular files and missing
// directories.
//
// TODO(asymmetric): This could be renamed to `BranchStatus`.
enum BranchResult {
    // The path already existed - may happen when dealing with
    // symlinks in the path.
    AlreadyExists,
    // New path in a known directory.
    NewInOldDirectory(ChainLinkInfo),
    // New path in an unknown directory.
    NewInNewDirectory(ChainLinkInfo, PathBuf),
}

impl IndentedToString for BranchResult {
    fn indented_to_string(&self, spaces: &str, repeat: usize) -> String {
        match self {
            BranchResult::AlreadyExists => "AlreadyExists".to_string(),
            BranchResult::NewInOldDirectory(ref i) => {
                format!("NewInOldDirectory({})", its!(i, spaces, repeat))
            }
            BranchResult::NewInNewDirectory(ref i, ref p) => {
                format!("NewInNewDirectory({}, {})",
                        its!(i, spaces, repeat),
                        p.to_string_lossy())
            }
        }
    }
}

// See `BranchResult`.
enum LeafResult {
    // New path in a known directory.
    NewInOldDirectory(ChainLinkInfo),
    // New path in an unknown directory.
    NewInNewDirectory(ChainLinkInfo, PathBuf),
}

impl IndentedToString for LeafResult {
    fn indented_to_string(&self, spaces: &str, repeat: usize) -> String {
        match self {
            LeafResult::NewInOldDirectory(ref i) => {
                format!("NewInOldDirectory({})", its!(i, spaces, repeat))
            }
            LeafResult::NewInNewDirectory(ref i, ref p) => {
                format!("NewInNewDirectory({}, {})",
                        its!(i, spaces, repeat),
                        p.to_string_lossy())
            }
        }
    }
}

// Used when we settle a path, so we know if we processed a path
// because all the paths were already settled, or not if there were
// still some left to settle.
enum ProcessPathStatus {
    // Holds a vector of new directories to watch (a result of
    // `process_path` function)
    Executed(Vec<PathBuf>),
    NotExecuted(ProcessPathArgs),
}

// Paths holds the state with regards to watching.
struct Paths {
    // A map of paths to file info of items. If something happens to
    // them, we react.
    paths: HashMap<PathBuf, WatchedFile>,
    // A map of directories to use count we actually watch with the
    // OS-specific watcher. These are parent directories of the items
    // in `paths`. Use count can be greater than 1 in case of
    // symlinks, for example when we watch `/a`, which is a symlink
    // that points to `b`, so we end up with two paths (`/a` and
    // `/b`), but only one directory (`/`) to watch with use count
    // being two.
    dirs: HashMap<// Path to watched directory.
                  PathBuf,
                  // Watched files count in the directory.
                  u32>,
    // A path we are interested in.
    start_path: PathBuf,
    // A map used to detect symlink loops.
    //
    // TODO(krnowak): Figure out if we can perform loop detection
    // without this hash map, but only using whatever data we have in
    // `Paths`.
    symlink_loop_catcher: HashMap<// Symlink path
                                  PathBuf,
                                  // Path + path rest
                                  PathBuf>,
    // A real path to the file from `start_path`.
    real_file: Option<PathBuf>,
    // A set of paths to settle after the items there got removed.
    paths_to_settle: HashSet<PathBuf>,
    // These args are used to pass them to `process_path`, when
    // `paths_to_settle` becomes empty.
    process_args_after_settle: Option<ProcessPathArgs>,
}

impl IndentedToString for Paths {
    fn indented_to_string(&self, spaces: &str, repeat: usize) -> String {
        let mut formatter = IndentedStructFormatter::new("Paths", spaces, repeat);
        formatter.add("paths", &self.paths);
        formatter.add("dirs", &self.dirs);
        formatter.add("start_path", &self.start_path);
        formatter.add("symlink_loop_catcher", &self.symlink_loop_catcher);
        formatter.add("real_file", &self.real_file);
        formatter.add("paths_to_settle", &self.paths_to_settle);
        formatter.add("process_args_after_settle", &self.process_args_after_settle);
        formatter.fmt()
    }
}

impl Paths {
    fn new(simplified_abs_path: &PathBuf) -> Self {
        Self { paths: HashMap::new(),
               dirs: HashMap::new(),
               start_path: simplified_abs_path.clone(),
               symlink_loop_catcher: HashMap::new(),
               real_file: None,
               paths_to_settle: HashSet::new(),
               process_args_after_settle: None, }
    }

    // Get a list of paths to watch, based on the configured
    // `start_path`.
    fn generate_watch_paths(&mut self) -> Vec<PathBuf> {
        let process_args = Self::path_for_processing(&self.start_path);

        self.process_path(process_args)
    }

    // Separate the root from the `simplified_abs_path` rest and store
    // them in a `ProcessPathArgs` instance.
    fn path_for_processing(simplified_abs_path: &PathBuf) -> ProcessPathArgs {
        // Holds the `/` component of a path, or the path prefix and
        // root on Windows (e.g. `C:\`).
        let mut path = PathBuf::new();
        // Holds all other components of a path.
        let mut path_rest = VecDeque::new();

        // components are substrings between path separators ('/' or '\')
        for component in simplified_abs_path.components() {
            match component {
                Component::Prefix(_) | Component::RootDir => {
                    path.push(component.as_os_str().to_owned());
                }
                Component::Normal(c) => path_rest.push_back(c.to_owned()),
                // Respectively the `.`. and `..` components of a path.
                Component::CurDir | Component::ParentDir => panic!("the path should be simplified"),
            };
        }

        ProcessPathArgs { path,
                          path_rest,
                          index: 0,
                          prev: None }
    }

    // Navigates through each of the components of the watched paths,
    // deciding what action to take for each of them.
    fn process_path(&mut self, args: ProcessPathArgs) -> Vec<PathBuf> {
        let mut common_generator = CommonGenerator::new(args);
        let mut new_watches = Vec::new();

        self.real_file = None;

        while let Some(common) = common_generator.get_new_common() {
            let dir_file_name = common.dir_file_name.clone();

            match common.path.symlink_metadata() {
                // The error can be triggered when the underlying path
                // does not exist, or when the user lacks permission
                // to access the metadata.
                //
                // TODO(asymmetric): should we handle this last case as well?
                // TODO(krnowak): see TODO in get_event_actions for Chmod event.
                Err(_) => {
                    let leaf_result = if common.is_leaf() {
                        debug!("add missing regular because error and is a leaf");
                        self.add_missing_regular(common)
                    } else {
                        debug!("add missing directory because error and is not a leaf");
                        self.add_missing_directory(common)
                    };

                    self.handle_leaf_result(leaf_result, &mut new_watches);

                    break;
                }
                Ok(metadata) => {
                    let file_type = metadata.file_type();

                    if !file_type.is_symlink() {
                        if common.is_leaf() {
                            let leaf_result = if file_type.is_file() {
                                debug!("add regular because is not a symlink, is a file and is a \
                                        leaf");
                                self.real_file = Some(common.path.clone());
                                self.add_regular(common)
                            } else {
                                debug!("{}{}",
                                       "add missing regular because is not a symlink,",
                                       " not a file and is a leaf",);
                                // It is not a symlink nor a file, so
                                // it is either a directory or
                                // something OS-specific. We expected
                                // a file here (common is a leaf - has
                                // an empty path rest).
                                self.add_missing_regular(common)
                            };

                            self.handle_leaf_result(leaf_result, &mut new_watches);
                            // TODO(krnowak): restructure the code, so
                            // the function has a simpler code
                            // flow. The break is needed to avoid
                            // running the rest of the code.
                            break;
                        }

                        if file_type.is_dir() {
                            debug!("add directory because is not a symlink and is not a leaf");
                            let branch_result = self.get_or_add_directory(common);

                            self.handle_branch_result(&mut common_generator,
                                                      branch_result,
                                                      &mut new_watches);

                            continue;
                        }

                        // Not a symlink, not a dir, and not a last
                        // component - this means that we got some
                        // file in the middle of the path. For example
                        // when we watch for `/a/b/c` and apparently
                        // `/a/b` is a file, not a directory. So add a
                        // missing directory item here and stop
                        // processing the rest of the path - we need
                        // to wait for the directory to show up first.
                        debug!("{}{}",
                               "add missing directory because is not a symlink ",
                               "and is not a dir and is not a leaf",);
                        let leaf_result = self.add_missing_directory(common);

                        self.handle_leaf_result(leaf_result, &mut new_watches);
                        break;
                    } else {
                        let target = match dir_file_name.as_path().read_link() {
                            Ok(target) => target,
                            Err(_) => {
                                // The path does not exist.
                                let leaf_result = if common.is_leaf() {
                                    debug!("{}{}",
                                           "add missing regular because is a symlink and",
                                           " is a leaf, but failed to read link",);
                                    self.add_missing_regular(common)
                                } else {
                                    debug!("{}{}",
                                           "add missing directory because is a symlink",
                                           " and is not a leaf, but failed to read link",);
                                    self.add_missing_directory(common)
                                };

                                self.handle_leaf_result(leaf_result, &mut new_watches);

                                break;
                            }
                        };

                        debug!("target: {:?}", &target);
                        let target_path = if target.is_absolute() {
                            target
                        } else {
                            debug!("directory for target: {:?}", &dir_file_name.directory);
                            dir_file_name.directory.join(target)
                        };

                        let simplified_target = simplify_abs_path(&target_path);
                        debug!("simplified target: {:?}", &simplified_target);
                        let process_args = Self::path_for_processing(&simplified_target);

                        if self.symlink_loop(&common.path,
                                             &common.path_rest,
                                             &process_args.path,
                                             &process_args.path_rest)
                        {
                            debug!("symlink loop");
                            // Symlink loop, nothing to watch here - hopefully
                            // later some symlink will be rewired to the real
                            // file.
                            break;
                        }

                        debug!("add symlink");
                        let branch_result = self.get_or_add_symlink(common);

                        self.handle_branch_result(&mut common_generator,
                                                  branch_result,
                                                  &mut new_watches);

                        common_generator.set_path(process_args.path);
                        common_generator.prepend_to_path_rest(process_args.path_rest);
                    }
                }
            }
        }

        new_watches
    }

    fn handle_leaf_result(&mut self, leaf_result: LeafResult, new_watches: &mut Vec<PathBuf>) {
        debug!("handle leaf result: {}", dits!(leaf_result));
        match leaf_result {
            LeafResult::NewInNewDirectory(chain_link_info, directory) => {
                new_watches.push(directory);
                self.setup_chain_link(chain_link_info);
            }
            LeafResult::NewInOldDirectory(chain_link_info) => {
                self.setup_chain_link(chain_link_info);
            }
        }
    }

    fn handle_branch_result(&mut self,
                            common_generator: &mut CommonGenerator,
                            branch_result: BranchResult,
                            new_watches: &mut Vec<PathBuf>) {
        debug!("handle branch result: {}", dits!(branch_result));
        match branch_result {
            // This is a part of the symlink handling, where we want
            // to establish a chain link between a symlink and the
            // first not-yet-watched item in the path the symlink is
            // pointing to.
            BranchResult::AlreadyExists => {
                common_generator.revert_previous();
            }
            BranchResult::NewInNewDirectory(chain_link_info, directory) => {
                new_watches.push(directory);
                self.setup_chain_link(chain_link_info);
            }
            BranchResult::NewInOldDirectory(chain_link_info) => {
                self.setup_chain_link(chain_link_info);
            }
        }
    }

    // Adds a new directory/file to a chain of watched paths.
    fn setup_chain_link(&mut self, chain_link_info: ChainLinkInfo) {
        if let Some(previous) = chain_link_info.prev {
            match self.get_mut_watched_file(&previous) {
                Some(previous_watched_file) => {
                    let previous_common = previous_watched_file.get_mut_common();
                    previous_common.next = Some(chain_link_info.path);
                }
                None => {
                    error!("Paths inconsistency in setup_chain_link, expect strange results");
                }
            }
        }
    }

    // Checks whether the path is present in the list of watched paths.
    fn get_watched_file(&self, path: &PathBuf) -> Option<&WatchedFile> { self.paths.get(path) }

    fn get_mut_watched_file(&mut self, path: &PathBuf) -> Option<&mut WatchedFile> {
        self.paths.get_mut(path)
    }

    fn get_directory(&self, path: &PathBuf) -> Option<&u32> { self.dirs.get(path) }

    fn get_mut_directory(&mut self, path: &PathBuf) -> Option<&mut u32> { self.dirs.get_mut(path) }

    fn drop_watch(&mut self, path: &PathBuf) -> Option<PathBuf> {
        if let Some(watched_file) = self.paths.remove(path) {
            let common = watched_file.steal_common();
            let dir_path = common.dir_file_name.directory;
            let unwatch_directory = match self.get_mut_directory(&dir_path) {
                Some(count) => {
                    *count -= 1;
                    *count == 0
                }
                None => {
                    error!("Dirs inconsistency in drop_watch, expect strange results");
                    false
                }
            };
            self.symlink_loop_catcher.remove(path);
            let dir_to_unwatch = if unwatch_directory {
                self.dirs.remove(&dir_path);

                Some(dir_path)
            } else {
                None
            };
            // any watch drop means that we don't see the real file anymore
            self.real_file = None;
            if let Some(prev) = common.prev {
                // The `prev` patch may not be watched anymore when we
                // stop watching a bunch of files in a batch (like
                // when we move the watched directory away, so we stop
                // watching everything inside it).
                if let Some(watched_prev) = self.get_mut_watched_file(&prev) {
                    watched_prev.get_mut_common().next = None;
                }
            } else {
                error!("Prev path inconsistency in drop_watch, expect strange results");
            }
            dir_to_unwatch
        } else {
            error!("Paths inconsistency in drop_watch, expect strange results");
            None
        }
    }

    fn symlink_loop(&mut self,
                    path: &PathBuf,
                    path_rest: &VecDeque<OsString>,
                    new_path: &PathBuf,
                    new_path_rest: &VecDeque<OsString>)
                    -> bool {
        let mut merged_path_rest = new_path_rest.clone();
        merged_path_rest.extend(path_rest.iter().cloned());
        let mut merged_path = new_path.clone();
        merged_path.extend(merged_path_rest);
        match self.symlink_loop_catcher.entry(path.clone()) {
            Entry::Occupied(o) => *o.get() == merged_path,
            Entry::Vacant(v) => {
                v.insert(merged_path);
                false
            }
        }
    }

    fn add_regular(&mut self, common: Common) -> LeafResult {
        self.add_leaf_watched_file(WatchedFile::Regular(common))
    }

    // TODO(asymmetric): could these 2 functions be unified?
    // TODO(krnowak): Maybe replace `MissingRegular` and
    // `MissingDirectory` with just one (`Missing`)?
    fn add_missing_regular(&mut self, common: Common) -> LeafResult {
        self.add_leaf_watched_file(WatchedFile::MissingRegular(common))
    }

    fn add_missing_directory(&mut self, common: Common) -> LeafResult {
        self.add_leaf_watched_file(WatchedFile::MissingDirectory(common))
    }

    fn add_leaf_watched_file(&mut self, watched_file: WatchedFile) -> LeafResult {
        debug!("add leaf file: {}", dits!(watched_file));
        let dir_file_name = watched_file.get_common().dir_file_name.clone();
        let needs_watch = self.add_dir(&dir_file_name);
        let chain_link_info = match self.paths.entry(dir_file_name.as_path()) {
            Entry::Occupied(o) => {
                error!("Paths inconsistency in add_leaf_watched_file, expect strange results");
                o.get().get_common().get_chain_link_info()
            }
            Entry::Vacant(v) => v.insert(watched_file).get_common().get_chain_link_info(),
        };
        if needs_watch {
            LeafResult::NewInNewDirectory(chain_link_info, dir_file_name.directory)
        } else {
            LeafResult::NewInOldDirectory(chain_link_info)
        }
    }

    fn get_or_add_directory(&mut self, common: Common) -> BranchResult {
        self.get_or_add_branch_watched_file(WatchedFile::Directory(common))
    }

    fn get_or_add_symlink(&mut self, common: Common) -> BranchResult {
        self.get_or_add_branch_watched_file(WatchedFile::Symlink(common))
    }

    fn get_or_add_branch_watched_file(&mut self, watched_file: WatchedFile) -> BranchResult {
        debug!("get or add branch file: {}", dits!(watched_file));
        if self.paths.contains_key(&watched_file.get_common().path) {
            return BranchResult::AlreadyExists;
        }

        let dir_file_name = watched_file.get_common().dir_file_name.clone();

        let needs_watch = self.add_dir(&dir_file_name);
        let chain_link_info = watched_file.get_common().get_chain_link_info();
        debug!("chain link info: {:?}", dits!(chain_link_info));
        self.paths.insert(dir_file_name.as_path(), watched_file);

        if needs_watch {
            // The directory where the new file is located was not being watched.
            BranchResult::NewInNewDirectory(chain_link_info, dir_file_name.directory)
        } else {
            // The file is new, the directory isn't.
            BranchResult::NewInOldDirectory(chain_link_info)
        }
    }

    // Updates the counter for a directory in the `dirs` HashMap,
    // returning whether the directory has been added for the first
    // time.
    fn add_dir(&mut self, dir_file_name: &DirFileName) -> bool {
        match self.dirs.entry(dir_file_name.directory.clone()) {
            Entry::Occupied(mut o) => {
                *o.get_mut() += 1;

                false
            }
            Entry::Vacant(v) => {
                v.insert(1);

                true
            }
        }
    }

    fn add_path_to_settle(&mut self, path: PathBuf) { self.paths_to_settle.insert(path); }

    fn settle_path(&mut self, path: &Path) { self.paths_to_settle.remove(path); }

    fn set_process_args(&mut self, args: ProcessPathArgs) {
        if match self.process_args_after_settle {
            Some(ref old_args) => args.index < old_args.index,
            None => true,
        } {
            self.process_args_after_settle = Some(args)
        }
    }

    fn process_path_or_defer_if_unsettled(&mut self) -> Option<Vec<PathBuf>> {
        let mut process_args = None;
        swap(&mut process_args, &mut self.process_args_after_settle);
        let (directories, new_args) = match process_args {
            Some(args) => {
                match self.process_path_if_settled(args) {
                    ProcessPathStatus::Executed(v) => (Some(v), None),
                    ProcessPathStatus::NotExecuted(a) => (None, Some(a)),
                }
            }
            None => (None, None),
        };
        self.process_args_after_settle = new_args;
        directories
    }

    fn process_path_if_settled(&mut self, args: ProcessPathArgs) -> ProcessPathStatus {
        if self.paths_to_settle.is_empty() {
            ProcessPathStatus::Executed(self.process_path(args))
        } else {
            ProcessPathStatus::NotExecuted(args)
        }
    }

    fn reset(&mut self) -> Vec<PathBuf> {
        self.paths.clear();
        let mut dirs_to_unwatch = Vec::new();
        dirs_to_unwatch.extend(self.dirs.drain().map(|i| i.0));
        self.paths_to_settle.clear();
        self.process_args_after_settle = None;
        self.symlink_loop_catcher.clear();
        dirs_to_unwatch
    }
}

/// A regular file watcher.
///
/// This type watches for a regular file at any path. The file does
/// not need to exist even - `FileWatcher` will track all the
/// directories and symlinks from the root directory up to the
/// file. If the file or any intermediate directory is missing, then
/// FileWatcher will wait for it to show up.
///
/// `FileWatcher` will use callbacks to notify the user about events
/// that happened to the regular file at the desired path. Note that
/// it will call the file_appeared callback in the first iteration if
/// the file existed when the watcher was created.
pub struct FileWatcher<C: Callbacks, W: Watcher> {
    callbacks: C,
    // The watcher itself.
    watcher: W,
    // A channel for receiving events.
    rx: Receiver<DebouncedEvent>,
    // The paths to watch.
    paths: Paths,
    // Path to the file if it existed when we created the watcher.
    initial_real_file: Option<PathBuf>,
}

/// Convenience function for returning a new file watcher that matches
/// the platform.
pub fn default_file_watcher<P, C>(path: P,
                                  callbacks: C)
                                  -> Result<FileWatcher<C, RecommendedWatcher>>
    where P: Into<PathBuf>,
          C: Callbacks
{
    FileWatcher::<C, RecommendedWatcher>::create(path, callbacks)
}

pub fn default_file_watcher_with_no_initial_event<P, C>(
    path: P,
    callbacks: C)
    -> Result<FileWatcher<C, RecommendedWatcher>>
    where P: Into<PathBuf>,
          C: Callbacks
{
    FileWatcher::<C, RecommendedWatcher>::create_with_no_initial_event(path, callbacks)
}

impl<C: Callbacks, W: Watcher> FileWatcher<C, W> {
    /// Creates a new `FileWatcher`.
    ///
    /// This will create an instance of `W` and start watching the
    /// paths. When looping the file watcher, it will emit an initial
    /// "file appeared" event if the watched file existed when the
    /// file watcher was created.
    ///
    /// Will return `Error::NotifyCreateError` if creating the watcher
    /// fails. In case of watching errors, it returns
    /// `Error::NotifyError`.
    pub fn create<P>(path: P, callbacks: C) -> Result<Self>
        where P: Into<PathBuf>
    {
        Self::create_instance(path, callbacks, true)
    }

    /// Creates a new `FileWatcher`.
    ///
    /// This will create an instance of `W` and start watching the
    /// paths. When looping the file watcher, it will not emit any
    /// initial "file appeared" event even if the watched file existed
    /// when the file watcher was created.
    ///
    /// Will return `Error::NotifyCreateError` if creating the watcher
    /// fails. In case of watching errors, it returns
    /// `Error::NotifyError`.
    pub fn create_with_no_initial_event<P>(path: P, callbacks: C) -> Result<Self>
        where P: Into<PathBuf>
    {
        Self::create_instance(path, callbacks, false)
    }

    // Creates an instance of the FileWatcher.
    fn create_instance<P>(path: P, callbacks: C, send_initial_event: bool) -> Result<Self>
        where P: Into<PathBuf>
    {
        let (tx, rx) = channel();
        let mut watcher =
            W::new(tx, Duration::from_millis(WATCHER_DELAY_MS)).map_err(|err| {
                                                                   Error::NotifyCreateError(err)
                                                               })?;
        let start_path = Self::watcher_path(path.into())?;
        // Initialize the Paths struct, which will hold all state
        // relative to file watching.
        let mut paths = Paths::new(&start_path);

        // Generate list of paths to watch.
        let directories = paths.generate_watch_paths();

        // Start watcher on each path.
        for directory in directories {
            watcher.watch(&directory, RecursiveMode::NonRecursive)
                   .map_err(Error::NotifyError)?;
        }
        let initial_real_file = if send_initial_event {
            paths.real_file.clone()
        } else {
            None
        };

        Ok(Self { callbacks,
                  watcher,
                  rx,
                  paths,
                  initial_real_file })
    }

    /// Get the reference to callbacks.
    #[allow(dead_code)]
    pub fn get_callbacks(&self) -> &C { &self.callbacks }

    /// Get the mutable reference to callbacks.
    #[allow(dead_code)]
    pub fn get_mut_callbacks(&mut self) -> &mut C { &mut self.callbacks }

    /// Get the reference to the underlying watcher.
    #[allow(dead_code)]
    pub fn get_underlying_watcher(&self) -> &W { &self.watcher }

    /// Get the mutable reference to the underlying watcher.
    #[allow(dead_code)]
    pub fn get_mut_underlying_watcher(&mut self) -> &mut W { &mut self.watcher }

    // Turns given path to a simplified absolute path.
    //
    // Simplified means that it is without `.` and `..`.
    fn watcher_path(p: PathBuf) -> Result<PathBuf> {
        let abs_path = if p.is_absolute() {
            p
        } else {
            let cwd = env::current_dir().map_err(Error::Io)?;
            cwd.join(p)
        };
        let simplified_abs_path = simplify_abs_path(&abs_path);
        match DirFileName::split_path(&simplified_abs_path) {
            Some(_) => Ok(simplified_abs_path),
            None => Err(Error::FileWatcherFileIsRoot),
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
        if let Some(ref real_file) = self.initial_real_file {
            self.callbacks.file_appeared(real_file);
        }

        self.initial_real_file = None;

        match self.rx.try_recv() {
            Ok(e) => self.handle_event(e),
            Err(TryRecvError::Empty) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn handle_event(&mut self, event: DebouncedEvent) -> Result<()> {
        let mut actions = VecDeque::new();
        debug!("in handle_event fn");
        debug!("got debounced event: {:?}", event);

        self.emit_directories_for_event(&event);

        // Gather the high-level actions.
        actions.extend(Self::get_paths_actions(&self.paths, event));

        debug!("paths: {}", dits!(self.paths));
        debug!("actions: {:?}", actions);
        // Perform lower-level actions.
        while let Some(action) = actions.pop_front() {
            debug!("action {}", dits!(action));
            match action {
                PathsAction::NotifyFileAppeared(p) => {
                    self.callbacks.file_appeared(p.as_path());
                }
                PathsAction::NotifyFileModified(p) => {
                    self.callbacks.file_modified(p.as_path());
                }
                PathsAction::NotifyFileDisappeared(p) => {
                    self.callbacks.file_disappeared(p.as_path());
                }
                PathsAction::DropWatch(p) => {
                    if let Some(dir_path) = self.paths.drop_watch(&p) {
                        match self.watcher.unwatch(dir_path) {
                            Ok(_) => (),
                            // These probably may happen when the
                            // directory was removed. Ignore them, as
                            // we wanted to drop the watch anyway.
                            Err(notify::Error::PathNotFound)
                            | Err(notify::Error::WatchNotFound) => (),
                            Err(e) => return Err(Error::NotifyError(e)),
                        }
                    }
                }
                PathsAction::AddPathToSettle(p) => {
                    self.paths.add_path_to_settle(p);
                }
                PathsAction::SettlePath(p) => {
                    self.paths.settle_path(&p);
                    actions.extend(self.handle_process_path()?);
                }
                PathsAction::ProcessPathAfterSettle(args) => {
                    self.paths.set_process_args(args);
                    actions.extend(self.handle_process_path()?);
                }
                PathsAction::RestartWatching => {
                    actions.clear();
                    if let Some(ref path) = self.paths.real_file {
                        actions.push_back(PathsAction::NotifyFileDisappeared(path.clone()));
                    }
                    for directory in self.paths.reset() {
                        match self.watcher.unwatch(directory) {
                            Ok(_) => (),
                            // These probably may happen when the
                            // directory was removed. Ignore them, as
                            // we wanted to drop the watch anyway.
                            Err(notify::Error::PathNotFound)
                            | Err(notify::Error::WatchNotFound) => (),
                            Err(e) => return Err(Error::NotifyError(e)),
                        }
                    }
                    let process_args = Paths::path_for_processing(&self.paths.start_path);
                    actions.push_back(PathsAction::ProcessPathAfterSettle(process_args));
                }
            }
        }
        Ok(())
    }

    fn emit_directories_for_event(&mut self, event: &DebouncedEvent) {
        let paths = match event {
            DebouncedEvent::NoticeWrite(ref p)
            | DebouncedEvent::Write(ref p)
            | DebouncedEvent::Chmod(ref p)
            | DebouncedEvent::NoticeRemove(ref p)
            | DebouncedEvent::Remove(ref p)
            | DebouncedEvent::Create(ref p) => vec![p],
            DebouncedEvent::Rename(ref from, ref to) => vec![from, to],
            DebouncedEvent::Rescan => vec![],
            DebouncedEvent::Error(_, ref o) => {
                match o {
                    Some(ref p) => vec![p],
                    None => vec![],
                }
            }
        };
        let mut dirs = vec![];
        for path in paths {
            if let Some(wf) = self.paths.get_watched_file(&path) {
                dirs.push(wf.get_common().dir_file_name.directory.clone());
            } else if self.paths.get_directory(&path).is_some() {
                dirs.push(path.clone());
            } else if let Some(df) = DirFileName::split_path(&path) {
                if self.paths.get_directory(&df.directory).is_some() {
                    dirs.push(df.directory.clone());
                }
            }
        }
        debug!("event in dirs: {:?}", dirs);
        self.callbacks.event_in_directories(&dirs);
    }

    fn handle_process_path(&mut self) -> Result<Vec<PathsAction>> {
        let mut actions = Vec::new();
        match self.paths.process_path_or_defer_if_unsettled() {
            None => (),
            Some(directories) => {
                for directory in directories {
                    self.watcher
                        .watch(&directory, RecursiveMode::NonRecursive)?;
                }
                if let Some(ref path) = self.paths.real_file {
                    actions.push(PathsAction::NotifyFileAppeared(path.clone()));
                }
            }
        }
        Ok(actions)
    }

    // Maps `EventAction`s to one or more lower-level `PathsAction`s.
    fn get_paths_actions(paths: &Paths, event: DebouncedEvent) -> Vec<PathsAction> {
        let mut actions = Vec::new();
        for event_action in Self::get_event_actions(paths, event) {
            debug!("event_action: {}", dits!(event_action));
            let mut tmp_actions = Vec::new();
            match event_action {
                EventAction::Ignore => (),
                EventAction::PlainChange(p) => {
                    tmp_actions.push(PathsAction::NotifyFileModified(p));
                }
                EventAction::RestartWatching => {
                    tmp_actions.push(PathsAction::RestartWatching);
                }
                EventAction::AddRegular(pad) => {
                    let path = pad.dir_file_name.as_path();
                    tmp_actions.push(PathsAction::DropWatch(path.clone()));
                    tmp_actions.push(PathsAction::ProcessPathAfterSettle(pad.args));
                }
                EventAction::DropRegular(pad) => {
                    tmp_actions.extend(Self::drop_common(paths, pad));
                }
                EventAction::AddDirectory(pad) => {
                    let path = pad.dir_file_name.as_path();
                    tmp_actions.push(PathsAction::DropWatch(path.clone()));
                    tmp_actions.push(PathsAction::ProcessPathAfterSettle(pad.args));
                }
                EventAction::DropDirectory(pad) => {
                    tmp_actions.extend(Self::drop_common(paths, pad));
                }
                EventAction::RewireSymlink(pad) => {
                    let path = pad.dir_file_name.as_path();
                    tmp_actions.extend(Self::drop_common(paths, pad));
                    tmp_actions.push(PathsAction::SettlePath(path));
                }
                EventAction::DropSymlink(pad) => {
                    tmp_actions.extend(Self::drop_common(paths, pad));
                }
                EventAction::SettlePath(p) => {
                    tmp_actions.push(PathsAction::SettlePath(p));
                }
            };
            debug!("translated to {:?}", tmp_actions);
            actions.extend(tmp_actions);
        }
        debug!("all actions: {:?}", actions);
        actions
    }

    fn drop_common(paths: &Paths, pad: PathsActionData) -> Vec<PathsAction> {
        let mut actions = Vec::new();
        let path = pad.dir_file_name.as_path();
        actions.push(PathsAction::AddPathToSettle(path.clone()));
        let mut path_to_drop = Some(path);
        while let Some(path) = path_to_drop {
            let maybe_watched_file = paths.get_watched_file(&path);
            actions.push(PathsAction::DropWatch(path));
            path_to_drop = if let Some(watched_file) = maybe_watched_file {
                watched_file.get_common().next.clone()
            } else {
                None
            };
        }
        if let Some(ref path) = paths.real_file {
            actions.push(PathsAction::NotifyFileDisappeared(path.clone()));
        }
        actions.push(PathsAction::ProcessPathAfterSettle(pad.args));
        actions
    }

    // Maps filesystem events to high-level actions.
    fn get_event_actions(paths: &Paths, event: DebouncedEvent) -> Vec<EventAction> {
        // Usual actions on files and resulting events (assuming that
        // a and b are in the same directory which we watch)
        // touch a - Create(a)
        // ln -sf foo a (does not matter if symlink a did exist before)- Create(a)
        // mkdir a - Create(a)
        // mv a b (does not matter if b existed or not) - NoticeRemove(a), Rename(a, b)
        // mv ../a . - Create(a)
        // mv a .. - NoticeRemove(a), Remove(a)
        // rm a - NoticeRemove(a), Remove(a)
        // echo foo >a (assuming a did exist earlier) - NoticeWrite(a), Write(a)
        let event_action = match event {
            // `Write` event will handle that.
            DebouncedEvent::NoticeWrite(_) => EventAction::Ignore,
            // These happen for regular files, just check if it
            // affects the file we are watching.
            //
            // TODO(krnowak): I wonder if we should watch `Chmod`
            // events for directories too. Maybe some permission
            // changes can cause the directory to be unwatchable. Or
            // watchable again for that matter.
            DebouncedEvent::Write(ref p) | DebouncedEvent::Chmod(ref p) => {
                match paths.get_watched_file(p) {
                    Some(&WatchedFile::Regular(_)) => EventAction::PlainChange(p.clone()),
                    _ => EventAction::Ignore,
                }
            }
            DebouncedEvent::NoticeRemove(ref p) => Self::handle_notice_remove_event(paths, p),
            DebouncedEvent::Remove(p) => Self::handle_remove_event(paths, p),
            DebouncedEvent::Create(ref p) => {
                match paths.get_watched_file(p) {
                    None => EventAction::Ignore,
                    Some(&WatchedFile::MissingRegular(ref c)) => {
                        EventAction::AddRegular(c.get_paths_action_data())
                    }
                    // Create event for an already existing file or
                    // directory should not happen, restart watching.
                    Some(&WatchedFile::Regular(_)) | Some(&WatchedFile::Directory(_)) => {
                        EventAction::RestartWatching
                    }
                    Some(&WatchedFile::Symlink(ref c)) => {
                        EventAction::RewireSymlink(c.get_paths_action_data())
                    }
                    Some(&WatchedFile::MissingDirectory(ref c)) => {
                        EventAction::AddDirectory(c.get_paths_action_data())
                    }
                }
            }
            DebouncedEvent::Rename(from, to) => {
                let mut events = Vec::new();
                // Rename is annoying in that it does not come
                // together with `NoticeRemove` of the destination
                // file (it is preceded with `NoticeRemove` of the
                // source file only), so we just going to emulate it
                // and then settle the destination path.
                events.push(Self::handle_notice_remove_event(paths, &to));
                events.push(EventAction::SettlePath(to));
                events.push(Self::handle_remove_event(paths, from));
                debug!("translated to {:?}", events);
                return events;
            }
            DebouncedEvent::Rescan => EventAction::RestartWatching,
            DebouncedEvent::Error(..) => EventAction::RestartWatching,
        };
        debug!("translated to single {}", dits!(event_action));
        vec![event_action]
    }

    fn handle_notice_remove_event(paths: &Paths, p: &PathBuf) -> EventAction {
        match paths.get_watched_file(p) {
            None => EventAction::Ignore,
            // Our directory was removed, moved elsewhere or
            // replaced. I discovered replacement scenario while
            // working on this code. Consider:
            //
            // mkdir a
            // touch a/foo
            // mkdir -p test/a
            // mv a test
            //
            // This will replace the empty directory `yest/a` with
            // `a`, so the file `foo` will be now in `test/a/foo`.
            Some(&WatchedFile::Directory(ref c)) => {
                EventAction::DropDirectory(c.get_paths_action_data())
            }
            // This happens when we expected `p` to be a file, but it
            // was something else and that thing just got removed.
            Some(&WatchedFile::MissingRegular(_)) => EventAction::Ignore,
            Some(&WatchedFile::Regular(ref c)) => {
                EventAction::DropRegular(c.get_paths_action_data())
            }
            Some(&WatchedFile::Symlink(ref c)) => {
                EventAction::DropSymlink(c.get_paths_action_data())
            }
            // This happens when we expected `p` to be a directory,
            // but it was something else and that thing just got
            // removed.
            Some(&WatchedFile::MissingDirectory(_)) => EventAction::Ignore,
        }
    }

    fn handle_remove_event(paths: &Paths, path: PathBuf) -> EventAction {
        match paths.get_watched_file(&path) {
            // We should have dropped the watch of this file in
            // `NoticeRemove`, so this should not happen - restart
            // watching.
            Some(&WatchedFile::Symlink(_))
            | Some(&WatchedFile::Directory(_))
            | Some(&WatchedFile::Regular(_)) => EventAction::RestartWatching,
            // Possibly `path` is something that used to be
            // interesting to us and got removed, try to settle it. If
            // it was not that, then nothing will happen.
            _ => EventAction::SettlePath(path),
        }
    }
}

// For now it's unix only, as we are only testing one k8s related
// scenario, that involves symlinks.
#[cfg(all(unix, test))]
mod tests {
    use std::{collections::{HashMap,
                            HashSet,
                            VecDeque},
              ffi::OsString,
              fmt::{Display,
                    Error,
                    Formatter},
              fs::{self,
                   File},
              io::ErrorKind,
              os::unix::fs as unix_fs,
              path::{Component,
                     Path,
                     PathBuf},
              sync::mpsc::Sender,
              thread,
              time::Duration};

    use notify::{self,
                 DebouncedEvent,
                 RawEvent,
                 RecommendedWatcher,
                 RecursiveMode,
                 Watcher};

    use tempfile::TempDir;

    use super::{Callbacks,
                FileWatcher,
                IndentedStructFormatter,
                IndentedToString,
                WatchedFile};

    // Convenient macro for inline creation of hashmaps.
    macro_rules! hm(
        {$($key:expr => $value:expr),+} => {
            {
                [
                    $(
                        ($key, $value),
                    )+
                ].iter().cloned().collect::<HashMap<_, _>>()
            }
        };
        // This form of the macro is to allow the leading comma.
        { $($key:expr => $value:expr),+, } => {
            hm!{ $($key => $value),+ }
        };
    );

    // Convenient macro for creating PathBufs.
    macro_rules! pb(
        {$str:expr} => {
            PathBuf::from($str)
        };
    );

    // Convenient macro for creating OsStrings.
    macro_rules! os(
        {$str:expr} => {
            OsString::from($str)
        };
    );

    // Add new test cases here.
    fn get_test_cases() -> Vec<TestCase> {
        vec![TestCase { name:  "Basic add/remove directories/files",
                        init:  Init { path:         Some(pb!("/a/b/c")),
                                      commands:     vec![],
                                      initial_file: None, },
                        steps: vec![Step { action: StepAction::Nop,
                                           dirs:   hm! {
                                               pb!("/") => 1,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::MissingDirectory,
                                                   path_rest: vec![os!("b"), os!("c")],
                                                   prev: None,
                                                   next: None,
                                               },
                                           },
                                           events: vec![], },
                                    Step { action: StepAction::MkdirP(pb!("/a")),
                                           dirs:   hm! {
                                               pb!("/") => 1,
                                               pb!("/a") => 1,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("b"), os!("c")],
                                                   prev: None,
                                                   next: Some(pb!("/a/b")),
                                               },
                                               pb!("/a/b") => PathState {
                                                   kind: PathKind::MissingDirectory,
                                                   path_rest: vec![os!("c")],
                                                   prev: Some(pb!("/a")),
                                                   next: None,
                                               },
                                           },
                                           events: vec![], },
                                    Step { action: StepAction::MkdirP(pb!("/a/b")),
                                           dirs:   hm! {
                                               pb!("/") => 1,
                                               pb!("/a") => 1,
                                               pb!("/a/b") => 1,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("b"), os!("c")],
                                                   prev: None,
                                                   next: Some(pb!("/a/b")),
                                               },
                                               pb!("/a/b") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("c")],
                                                   prev: Some(pb!("/a")),
                                                   next: Some(pb!("/a/b/c")),
                                               },
                                               pb!("/a/b/c") => PathState {
                                                   kind: PathKind::MissingRegular,
                                                   path_rest: vec![],
                                                   prev: Some(pb!("/a/b")),
                                                   next: None,
                                               },
                                           },
                                           events: vec![], },
                                    Step { action: StepAction::Touch(pb!("/a/b/c")),
                                           dirs:   hm! {
                                               pb!("/") => 1,
                                               pb!("/a") => 1,
                                               pb!("/a/b") => 1,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("b"), os!("c")],
                                                   prev: None,
                                                   next: Some(pb!("/a/b")),
                                               },
                                               pb!("/a/b") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("c")],
                                                   prev: Some(pb!("/a")),
                                                   next: Some(pb!("/a/b/c")),
                                               },
                                               pb!("/a/b/c") => PathState {
                                                   kind: PathKind::Regular,
                                                   path_rest: vec![],
                                                   prev: Some(pb!("/a/b")),
                                                   next: None,
                                               },
                                           },
                                           events: vec![NotifyEvent::appeared(pb!("/a/b/c"))], },
                                    Step { action: StepAction::RmRF(pb!("/a/b/c")),
                                           dirs:   hm! {
                                               pb!("/") => 1,
                                               pb!("/a") => 1,
                                               pb!("/a/b") => 1,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("b"), os!("c")],
                                                   prev: None,
                                                   next: Some(pb!("/a/b")),
                                               },
                                               pb!("/a/b") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("c")],
                                                   prev: Some(pb!("/a")),
                                                   next: Some(pb!("/a/b/c")),
                                               },
                                               pb!("/a/b/c") => PathState {
                                                   kind: PathKind::MissingRegular,
                                                   path_rest: vec![],
                                                   prev: Some(pb!("/a/b")),
                                                   next: None,
                                               },
                                           },
                                           events: vec![NotifyEvent::disappeared(pb!("/a/b/c"))], },
                                    Step { action: StepAction::RmRF(pb!("/a/b")),
                                           dirs:   hm! {
                                               pb!("/") => 1,
                                               pb!("/a") => 1,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("b"), os!("c")],
                                                   prev: None,
                                                   next: Some(pb!("/a/b")),
                                               },
                                               pb!("/a/b") => PathState {
                                                   kind: PathKind::MissingDirectory,
                                                   path_rest: vec![os!("c")],
                                                   prev: Some(pb!("/a")),
                                                   next: None,
                                               },
                                           },
                                           events: vec![], },
                                    Step { action: StepAction::RmRF(pb!("/a")),
                                           dirs:   hm! {
                                               pb!("/") => 1,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::MissingDirectory,
                                                   path_rest: vec![os!("b"), os!("c")],
                                                   prev: None,
                                                   next: None,
                                               },
                                           },
                                           events: vec![], },], },
             TestCase { name:  "Quick remove directories/files",
                        init:  Init { path:         Some(pb!("/a/b/c")),
                                      commands:     vec![InitCommand::MkdirP(pb!("/a/b")),
                                                         InitCommand::Touch(pb!("/a/b/c")),],
                                      initial_file: Some(pb!("/a/b/c")), },
                        steps: vec![Step { action: StepAction::Nop,
                                           dirs:   hm! {
                                               pb!("/") => 1,
                                               pb!("/a") => 1,
                                               pb!("/a/b") => 1,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("b"), os!("c")],
                                                   prev: None,
                                                   next: Some(pb!("/a/b")),
                                               },
                                               pb!("/a/b") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("c")],
                                                   prev: Some(pb!("/a")),
                                                   next: Some(pb!("/a/b/c")),
                                               },
                                               pb!("/a/b/c") => PathState {
                                                   kind: PathKind::Regular,
                                                   path_rest: vec![],
                                                   prev: Some(pb!("/a/b")),
                                                   next: None,
                                               },
                                           },
                                           events: vec![], },
                                    Step { action: StepAction::RmRF(pb!("/a")),
                                           dirs:   hm! {
                                               pb!("/") => 1,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::MissingDirectory,
                                                   path_rest: vec![os!("b"), os!("c")],
                                                   prev: None,
                                                   next: None,
                                               },
                                           },
                                           events: vec![NotifyEvent::disappeared(pb!("/a/b/c"))], },], },
             TestCase { name:  "Basic symlink tracking",
                        init:  Init { path:         Some(pb!("/a")),
                                      commands:     vec![InitCommand::Touch(pb!("/1")),
                                                         InitCommand::Touch(pb!("/2")),
                                                         InitCommand::Touch(pb!("/3")),
                                                         InitCommand::LnS(pb!("s2"), pb!("/s1")),
                                                         InitCommand::LnS(pb!("/s3"), pb!("/s2")),
                                                         InitCommand::LnS(pb!("/1"), pb!("/s3")),
                                                         InitCommand::LnS(pb!("s1"), pb!("/a")),
                                                         InitCommand::MkdirP(pb!("/tmp")),
                                                         InitCommand::LnS(pb!("3"),
                                                                          pb!("/tmp/link")),],
                                      initial_file: Some(pb!("/1")), },
                        steps: vec![Step { action: StepAction::Nop,
                                           dirs:   hm! {
                                               pb!("/") => 5,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![],
                                                   prev: None,
                                                   next: Some(pb!("/s1")),
                                               },
                                               pb!("/s1") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![],
                                                   prev: Some(pb!("/a")),
                                                   next: Some(pb!("/s2")),
                                               },
                                               pb!("/s2") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![],
                                                   prev: Some(pb!("/s1")),
                                                   next: Some(pb!("/s3")),
                                               },
                                               pb!("/s3") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![],
                                                   prev: Some(pb!("/s2")),
                                                   next: Some(pb!("/1")),
                                               },
                                               pb!("/1") => PathState {
                                                   kind: PathKind::Regular,
                                                   path_rest: vec![],
                                                   prev: Some(pb!("/s3")),
                                                   next: None,
                                               },
                                           },
                                           events: vec![], },
                                    Step { action: StepAction::RmRF(pb!("/s2")),
                                           dirs:   hm! {
                                               pb!("/") => 3,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![],
                                                   prev: None,
                                                   next: Some(pb!("/s1")),
                                               },
                                               pb!("/s1") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![],
                                                   prev: Some(pb!("/a")),
                                                   next: Some(pb!("/s2")),
                                               },
                                               pb!("/s2") => PathState {
                                                   kind: PathKind::MissingRegular,
                                                   path_rest: vec![],
                                                   prev: Some(pb!("/s1")),
                                                   next: None,
                                               },
                                           },
                                           events: vec![NotifyEvent::disappeared(pb!("/1"))], },
                                    Step { action: StepAction::LnS(pb!("/2"), pb!("/s2")),
                                           dirs:   hm! {
                                               pb!("/") => 4,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![],
                                                   prev: None,
                                                   next: Some(pb!("/s1")),
                                               },
                                               pb!("/s1") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![],
                                                   prev: Some(pb!("/a")),
                                                   next: Some(pb!("/s2")),
                                               },
                                               pb!("/s2") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![],
                                                   prev: Some(pb!("/s1")),
                                                   next: Some(pb!("/2")),
                                               },
                                               pb!("/2") => PathState {
                                                   kind: PathKind::Regular,
                                                   path_rest: vec![],
                                                   prev: Some(pb!("/s2")),
                                                   next: None,
                                               },
                                           },
                                           events: vec![NotifyEvent::appeared(pb!("/2"))], },
                                    Step { action: StepAction::Mv(pb!("/tmp/link"), pb!("/s1")),
                                           dirs:   hm! {
                                               pb!("/") => 3,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![],
                                                   prev: None,
                                                   next: Some(pb!("/s1")),
                                               },
                                               pb!("/s1") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![],
                                                   prev: Some(pb!("/a")),
                                                   next: Some(pb!("/3")),
                                               },
                                               pb!("/3") => PathState {
                                                   kind: PathKind::Regular,
                                                   path_rest: vec![],
                                                   prev: Some(pb!("/s1")),
                                                   next: None,
                                               },
                                           },
                                           events: vec![NotifyEvent::disappeared(pb!("/2")),
                                                        NotifyEvent::appeared(pb!("/3")),], },], },
             TestCase { name:  "Emulate Kubernetes ConfigMap",
                        init:  Init { path:         Some(pb!("/a")),
                                      commands:     vec![InitCommand::MkdirP(pb!("/old")),
                                                         InitCommand::MkdirP(pb!("/new")),
                                                         InitCommand::Touch(pb!("/old/a")),
                                                         InitCommand::Touch(pb!("/new/a")),
                                                         InitCommand::LnS(pb!("old"),
                                                                          pb!("/data")),
                                                         InitCommand::LnS(pb!("data/a"),
                                                                          pb!("/a")),
                                                         InitCommand::MkdirP(pb!("/tmp")),
                                                         InitCommand::LnS(pb!("new"),
                                                                          pb!("/tmp/link")),],
                                      initial_file: Some(pb!("/old/a")), },
                        steps: vec![Step { action: StepAction::Nop,
                                           dirs:   hm! {
                                               pb!("/") => 3,
                                               pb!("/old") => 1,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![],
                                                   prev: None,
                                                   next: Some(pb!("/data")),
                                               },
                                               pb!("/data") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![os!("a")],
                                                   prev: Some(pb!("/a")),
                                                   next: Some(pb!("/old")),
                                               },
                                               pb!("/old") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("a")],
                                                   prev: Some(pb!("/data")),
                                                   next: Some(pb!("/old/a")),
                                               },
                                               pb!("/old/a") => PathState {
                                                   kind: PathKind::Regular,
                                                   path_rest: vec![],
                                                   prev: Some(pb!("/old")),
                                                   next: None,
                                               },
                                           },
                                           events: vec![], },
                                    Step { action: StepAction::Mv(pb!("/tmp/link"), pb!("/data")),
                                           dirs:   hm! {
                                               pb!("/") => 3,
                                               pb!("/new") => 1,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![],
                                                   prev: None,
                                                   next: Some(pb!("/data")),
                                               },
                                               pb!("/data") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![os!("a")],
                                                   prev: Some(pb!("/a")),
                                                   next: Some(pb!("/new")),
                                               },
                                               pb!("/new") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("a")],
                                                   prev: Some(pb!("/data")),
                                                   next: Some(pb!("/new/a")),
                                               },
                                               pb!("/new/a") => PathState {
                                                   kind: PathKind::Regular,
                                                   path_rest: vec![],
                                                   prev: Some(pb!("/new")),
                                                   next: None,
                                               },
                                           },
                                           events: vec![NotifyEvent::disappeared(pb!("/old/a")),
                                                        NotifyEvent::appeared(pb!("/new/a")),], },], },
             TestCase { name:  "Symlink loop, pointing to itself",
                        init:  Init { path:         Some(pb!("/a/b/c")),
                                      commands:     vec![InitCommand::MkdirP(pb!("/a")),
                                                         InitCommand::LnS(pb!("b"), pb!("/a/b")),],
                                      initial_file: None, },
                        steps: vec![Step { action: StepAction::Nop,
                                           dirs:   hm! {
                                               pb!("/") => 1,
                                               pb!("/a") => 1,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("b"), os!("c")],
                                                   prev: None,
                                                   next: Some(pb!("/a/b")),
                                               },
                                               pb!("/a/b") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![os!("c")],
                                                   prev: Some(pb!("/a")),
                                                   next: None,
                                               },
                                           },
                                           events: vec![], },
                                    Step { action: StepAction::RmRF(pb!("/a/b")),
                                           dirs:   hm! {
                                               pb!("/") => 1,
                                               pb!("/a") => 1,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("b"), os!("c")],
                                                   prev: None,
                                                   next: Some(pb!("/a/b")),
                                               },
                                               pb!("/a/b") => PathState {
                                                   kind: PathKind::MissingDirectory,
                                                   path_rest: vec![os!("c")],
                                                   prev: Some(pb!("/a")),
                                                   next: None,
                                               },
                                           },
                                           events: vec![], },], },
             TestCase { name:  "Dropping looping symlink and adding a new one instead",
                        init:  Init { path:         Some(pb!("/a/b/c/d")),
                                      commands:     vec![InitCommand::MkdirP(pb!("/a")),
                                                         InitCommand::MkdirP(pb!("/x")),
                                                         InitCommand::LnS(pb!("/x"), pb!("/a/b")),
                                                         InitCommand::LnS(pb!("/a/b/c"),
                                                                          pb!("/x/c")),],
                                      initial_file: None, },
                        steps: vec![Step { action: StepAction::Nop,
                                           dirs:   hm! {
                                               pb!("/") => 2,
                                               pb!("/a") => 1,
                                               pb!("/x") => 1,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("b"), os!("c"), os!("d")],
                                                   prev: None,
                                                   next: Some(pb!("/a/b")),
                                               },
                                               pb!("/a/b") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![os!("c"), os!("d")],
                                                   prev: Some(pb!("/a")),
                                                   next: Some(pb!("/x")),
                                               },
                                               pb!("/x") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("c"), os!("d")],
                                                   prev: Some(pb!("/a/b")),
                                                   next: Some(pb!("/x/c")),
                                               },
                                               pb!("/x/c") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![os!("d")],
                                                   prev: Some(pb!("/x")),
                                                   next: None,
                                               },
                                           },
                                           events: vec![], },
                                    Step { action: StepAction::RmRF(pb!("/x/c")),
                                           dirs:   hm! {
                                               pb!("/") => 2,
                                               pb!("/a") => 1,
                                               pb!("/x") => 1,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("b"), os!("c"), os!("d")],
                                                   prev: None,
                                                   next: Some(pb!("/a/b")),
                                               },
                                               pb!("/a/b") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![os!("c"), os!("d")],
                                                   prev: Some(pb!("/a")),
                                                   next: Some(pb!("/x")),
                                               },
                                               pb!("/x") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("c"), os!("d")],
                                                   prev: Some(pb!("/a/b")),
                                                   next: Some(pb!("/x/c")),
                                               },
                                               pb!("/x/c") => PathState {
                                                   kind: PathKind::MissingDirectory,
                                                   path_rest: vec![os!("d")],
                                                   prev: Some(pb!("/x")),
                                                   next: None,
                                               },
                                           },
                                           events: vec![], },
                                    Step { action: StepAction::Touch(pb!("/x/d")),
                                           dirs:   hm! {
                                               pb!("/") => 2,
                                               pb!("/a") => 1,
                                               pb!("/x") => 1,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("b"), os!("c"), os!("d")],
                                                   prev: None,
                                                   next: Some(pb!("/a/b")),
                                               },
                                               pb!("/a/b") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![os!("c"), os!("d")],
                                                   prev: Some(pb!("/a")),
                                                   next: Some(pb!("/x")),
                                               },
                                               pb!("/x") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("c"), os!("d")],
                                                   prev: Some(pb!("/a/b")),
                                                   next: Some(pb!("/x/c")),
                                               },
                                               pb!("/x/c") => PathState {
                                                   kind: PathKind::MissingDirectory,
                                                   path_rest: vec![os!("d")],
                                                   prev: Some(pb!("/x")),
                                                   next: None,
                                               },
                                           },
                                           events: vec![], },
                                    Step { action: StepAction::LnS(pb!("."), pb!("/x/c")),
                                           dirs:   hm! {
                                               pb!("/") => 2,
                                               pb!("/a") => 1,
                                               pb!("/x") => 2,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("b"), os!("c"), os!("d")],
                                                   prev: None,
                                                   next: Some(pb!("/a/b")),
                                               },
                                               pb!("/a/b") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![os!("c"), os!("d")],
                                                   prev: Some(pb!("/a")),
                                                   next: Some(pb!("/x")),
                                               },
                                               pb!("/x") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("c"), os!("d")],
                                                   prev: Some(pb!("/a/b")),
                                                   next: Some(pb!("/x/c")),
                                               },
                                               pb!("/x/c") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![os!("d")],
                                                   prev: Some(pb!("/x")),
                                                   next: Some(pb!("/x/d")),
                                               },
                                               pb!("/x/d") => PathState {
                                                   kind: PathKind::Regular,
                                                   path_rest: vec![],
                                                   prev: Some(pb!("/x/c")),
                                                   next: None,
                                               },
                                           },
                                           events: vec![NotifyEvent::appeared(pb!("/x/d"))], },], },
             TestCase { name:  "Rewiring symlink loop",
                        init:  Init { path:         Some(pb!("/a/b/c/d")),
                                      commands:     vec![InitCommand::MkdirP(pb!("/a")),
                                                         InitCommand::MkdirP(pb!("/x")),
                                                         InitCommand::LnS(pb!("/x"), pb!("/a/b")),
                                                         InitCommand::LnS(pb!("/a/b/c"),
                                                                          pb!("/x/c")),
                                                         InitCommand::Touch(pb!("/x/d")),
                                                         InitCommand::MkdirP(pb!("/tmp")),
                                                         InitCommand::LnS(pb!("."),
                                                                          pb!("/tmp/link")),],
                                      initial_file: None, },
                        steps: vec![Step { action: StepAction::Nop,
                                           dirs:   hm! {
                                               pb!("/") => 2,
                                               pb!("/a") => 1,
                                               pb!("/x") => 1,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("b"), os!("c"), os!("d")],
                                                   prev: None,
                                                   next: Some(pb!("/a/b")),
                                               },
                                               pb!("/a/b") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![os!("c"), os!("d")],
                                                   prev: Some(pb!("/a")),
                                                   next: Some(pb!("/x")),
                                               },
                                               pb!("/x") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("c"), os!("d")],
                                                   prev: Some(pb!("/a/b")),
                                                   next: Some(pb!("/x/c")),
                                               },
                                               pb!("/x/c") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![os!("d")],
                                                   prev: Some(pb!("/x")),
                                                   next: None,
                                               },
                                           },
                                           events: vec![], },
                                    Step { action: StepAction::Mv(pb!("/tmp/link"), pb!("/x/c")),
                                           dirs:   hm! {
                                               pb!("/") => 2,
                                               pb!("/a") => 1,
                                               pb!("/x") => 2,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("b"), os!("c"), os!("d")],
                                                   prev: None,
                                                   next: Some(pb!("/a/b")),
                                               },
                                               pb!("/a/b") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![os!("c"), os!("d")],
                                                   prev: Some(pb!("/a")),
                                                   next: Some(pb!("/x")),
                                               },
                                               pb!("/x") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("c"), os!("d")],
                                                   prev: Some(pb!("/a/b")),
                                                   next: Some(pb!("/x/c")),
                                               },
                                               pb!("/x/c") => PathState {
                                                   kind: PathKind::Symlink,
                                                   path_rest: vec![os!("d")],
                                                   prev: Some(pb!("/x")),
                                                   next: Some(pb!("/x/d")),
                                               },
                                               pb!("/x/d") => PathState {
                                                   kind: PathKind::Regular,
                                                   path_rest: vec![],
                                                   prev: Some(pb!("/x/c")),
                                                   next: None,
                                               },
                                           },
                                           events: vec![NotifyEvent::appeared(pb!("/x/d"))], },], },
             TestCase { name:  "Moving a directory",
                        init:  Init { path:         Some(pb!("/a/b/c")),
                                      commands:     vec![InitCommand::MkdirP(pb!("/a/b")),
                                                         InitCommand::Touch(pb!("/a/b/c")),],
                                      initial_file: Some(pb!("/a/b/c")), },
                        steps: vec![Step { action: StepAction::Nop,
                                           dirs:   hm! {
                                               pb!("/") => 1,
                                               pb!("/a") => 1,
                                               pb!("/a/b") => 1,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("b"), os!("c")],
                                                   prev: None,
                                                   next: Some(pb!("/a/b")),
                                               },
                                               pb!("/a/b") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("c")],
                                                   prev: Some(pb!("/a")),
                                                   next: Some(pb!("/a/b/c")),
                                               },
                                               pb!("/a/b/c") => PathState {
                                                   kind: PathKind::Regular,
                                                   path_rest: vec![],
                                                   prev: Some(pb!("/a/b")),
                                                   next: None,
                                               },
                                           },
                                           events: vec![], },
                                    Step { action: StepAction::Mv(pb!("/a/b"), pb!("/a/d")),
                                           dirs:   hm! {
                                               pb!("/") => 1,
                                               pb!("/a") => 1,
                                           },
                                           paths:  hm! {
                                               pb!("/a") => PathState {
                                                   kind: PathKind::Directory,
                                                   path_rest: vec![os!("b"), os!("c")],
                                                   prev: None,
                                                   next: Some(pb!("/a/b")),
                                               },
                                               pb!("/a/b") => PathState {
                                                   kind: PathKind::MissingDirectory,
                                                   path_rest: vec![os!("c")],
                                                   prev: Some(pb!("/a")),
                                                   next: None,
                                               },
                                           },
                                           events: vec![NotifyEvent::disappeared(pb!("/a/b/c"))], },], },]
    }

    #[test]
    fn file_watcher() {
        for tc in get_test_cases() {
            let mut runner = TestCaseRunner::new();
            runner.debug_info.add(format!("test case: {}", tc.name));
            runner.run_init_commands(&tc.init.commands);
            let setup = runner.prepare_watcher(&tc.init.path);
            runner.run_steps(setup, &tc.init.initial_file, &tc.steps);
        }
    }

    // Commands that can be executed at the test case init.
    //
    // Tests may come and go, so some of the variants may be unused.
    #[allow(dead_code)]
    enum InitCommand {
        MkdirP(PathBuf),
        Touch(PathBuf),
        LnS(PathBuf, PathBuf),
    }

    // Description of the init phase for test case.
    struct Init {
        // The path to the file that will be watched.
        path: Option<PathBuf>,
        // Commands to be executed before executing the steps.
        commands: Vec<InitCommand>,
        // Optional file to the real file if it exists after
        // performing the initial commands.
        initial_file: Option<PathBuf>,
    }

    // Commands executed as a part of the test case step.
    // Tests may come and go, so some of the variants may be unused.
    #[allow(dead_code)]
    #[derive(Debug)]
    enum StepAction {
        LnS(PathBuf, PathBuf),
        MkdirP(PathBuf),
        Touch(PathBuf),
        Mv(PathBuf, PathBuf),
        RmRF(PathBuf),
        Nop,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    enum PathKind {
        Symlink,
        Regular,
        MissingRegular,
        Directory,
        MissingDirectory,
    }

    // Simplified description of the WatchedFile's Common struct.
    #[derive(Clone)]
    struct PathState {
        kind:      PathKind,
        path_rest: Vec<OsString>,
        prev:      Option<PathBuf>,
        next:      Option<PathBuf>,
    }

    impl IndentedToString for PathState {
        fn indented_to_string(&self, spaces: &str, repeat: usize) -> String {
            let mut formatter = IndentedStructFormatter::new("PathState", spaces, repeat);
            formatter.add_debug("kind", &self.kind);
            formatter.add_debug("path_rest", &self.path_rest);
            formatter.add("prev", &self.prev);
            formatter.add("next", &self.next);
            formatter.fmt()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    enum NotifyEventKind {
        Appeared,
        Modified,
        Disappeared,
    }

    #[derive(Clone, Debug, PartialEq)]
    struct NotifyEvent {
        path: PathBuf,
        kind: NotifyEventKind,
    }

    impl NotifyEvent {
        fn new(path: PathBuf, kind: NotifyEventKind) -> Self { Self { path, kind } }

        fn appeared(path: PathBuf) -> Self { Self::new(path, NotifyEventKind::Appeared) }

        fn modified(path: PathBuf) -> Self { Self::new(path, NotifyEventKind::Modified) }

        fn disappeared(path: PathBuf) -> Self { Self::new(path, NotifyEventKind::Disappeared) }
    }

    // A description of the single step in the test case.
    struct Step {
        // Action to execute at the beginning of the step.
        action: StepAction,
        // Expected watched directories together with the use count,
        // similar to `dirs` field in Paths.
        dirs: HashMap<PathBuf, u32>,
        // Expected watched items, similar to `paths` field in Paths,
        // but a bit simplified.
        paths: HashMap<PathBuf, PathState>,
        // Expected events that happened when executing the step
        // command. The events map to the `file_*` functions in
        // `Callbacks` trait.
        events: Vec<NotifyEvent>,
    }

    impl IndentedToString for Step {
        fn indented_to_string(&self, spaces: &str, repeat: usize) -> String {
            let mut formatter = IndentedStructFormatter::new("Step", spaces, repeat);
            formatter.add_debug("action", &self.action);
            formatter.add("dirs", &self.dirs);
            formatter.add("paths", &self.paths);
            formatter.add_debug("events", &self.events);
            formatter.fmt()
        }
    }

    struct TestCase {
        // Not used directly, but describes the test. Can be used
        // later for debugging.
        #[allow(dead_code)]
        name: &'static str,
        init: Init,
        steps: Vec<Step>,
    }

    // The implementation of `Callbacks` trait for testing purposes.
    #[derive(Default)]
    struct TestCallbacks {
        // A list of events that happened when executing the step.
        events: Vec<NotifyEvent>,
        // A set of ignored directories. Usually it is just `/` and
        // `/tmp`.
        ignored_dirs: HashSet<PathBuf>,
        // Whether this single iteration should be ignored, because it
        // happened in one of the ignored directories.
        ignore: bool,
    }

    impl TestCallbacks {
        fn new(ignored_dirs: &[PathBuf]) -> Self {
            let mut cb = Self::default();
            cb.ignored_dirs.extend(ignored_dirs.iter().cloned());
            cb
        }
    }

    impl Callbacks for TestCallbacks {
        fn file_appeared(&mut self, real_path: &Path) {
            self.events
                .push(NotifyEvent::appeared(real_path.to_owned()));
        }

        fn file_modified(&mut self, real_path: &Path) {
            self.events
                .push(NotifyEvent::modified(real_path.to_owned()));
        }

        fn file_disappeared(&mut self, real_path: &Path) {
            self.events
                .push(NotifyEvent::disappeared(real_path.to_owned()));
        }

        fn event_in_directories(&mut self, paths: &[PathBuf]) {
            for path in paths {
                if self.ignored_dirs.contains(path) {
                    debug!("got event in ignored dirs");
                    self.ignore = true;
                    break;
                }
            }
        }
    }

    // The implementation of notify::Watcher trait for testing
    // purposes.
    struct TestWatcher {
        // The real watcher that does the grunt work.
        real_watcher: RecommendedWatcher,
        // A set of watched dirs. We will use these to correctly
        // compute the number of iterations to perform after executing
        // the step action.
        watched_dirs: HashSet<PathBuf>,
    }

    impl Watcher for TestWatcher {
        fn new_raw(tx: Sender<RawEvent>) -> notify::Result<Self> {
            Ok(TestWatcher { real_watcher: RecommendedWatcher::new_raw(tx)?,
                             watched_dirs: HashSet::new(), })
        }

        fn new(tx: Sender<DebouncedEvent>, d: Duration) -> notify::Result<Self> {
            Ok(TestWatcher { real_watcher: RecommendedWatcher::new(tx, d)?,
                             watched_dirs: HashSet::new(), })
        }

        fn watch<P: AsRef<Path>>(&mut self, path: P, mode: RecursiveMode) -> notify::Result<()> {
            if !self.watched_dirs.insert(path.as_ref().to_owned()) {
                panic!("Trying to watch a path {} we are already watching",
                       path.as_ref().display(),);
            }
            if mode == RecursiveMode::Recursive {
                panic!("Recursive watch should not ever happen");
            }
            self.real_watcher.watch(path, mode)
        }

        fn unwatch<P: AsRef<Path>>(&mut self, path: P) -> notify::Result<()> {
            if !self.watched_dirs.remove(&path.as_ref().to_owned()) {
                panic!("Trying to unwatch a path {} we were not watching",
                       path.as_ref().display(),);
            }
            self.real_watcher.unwatch(path)
        }
    }

    struct DebugInfo {
        logs_per_level: Vec<Vec<String>>,
    }

    impl DebugInfo {
        fn new() -> Self { Self { logs_per_level: vec![Vec::new()], } }

        fn push_level(&mut self) { self.logs_per_level.push(Vec::new()); }

        fn pop_level(&mut self) {
            self.logs_per_level.pop();
            assert!(!self.logs_per_level.is_empty(),
                    "too many pops on DebugInfo");
        }

        fn add(&mut self, str: String) { self.logs_per_level.last_mut().unwrap().push(str); }
    }

    impl Display for DebugInfo {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            writeln!(f, "----------------")?;
            for level_logs in &self.logs_per_level {
                for entry in level_logs {
                    writeln!(f, "{}\n----------------", entry)?;
                }
            }
            Ok(())
        }
    }

    struct WatcherSetup {
        init_path: PathBuf,
        watcher:   FileWatcher<TestCallbacks, TestWatcher>,
    }

    // Structure used for executing the initial commands and step
    // actions.
    struct FsOps<'a> {
        debug_info:   &'a mut DebugInfo,
        root:         &'a PathBuf,
        watched_dirs: Option<&'a HashSet<PathBuf>>,
    }

    impl<'a> FsOps<'a> {
        fn ln_s(&self, target: &PathBuf, path: &PathBuf) -> u32 {
            let pp = self.prepend_root(&path);
            let tt = if target.is_absolute() {
                self.prepend_root(&target)
            } else {
                target.clone()
            };
            unix_fs::symlink(&tt, &pp).unwrap_or_else(|_| {
                                          panic!("could not create symlink at {} pointing to {}, \
                                                  debug info:\n{}",
                                                 pp.display(),
                                                 tt.display(),
                                                 self.debug_info,)
                                      });
            if self.parent_is_watched(&pp) {
                // One event - create.
                1
            } else {
                // No events.
                0
            }
        }

        fn mkdir_p(&self, path: &PathBuf) -> u32 {
            let full_path = self.prepend_root(&path);
            match self.watched_dirs {
                Some(wd) => {
                    let mut test_path = full_path.clone();
                    while !test_path.exists() {
                        test_path = self.get_parent(&test_path);
                    }
                    self.real_mkdir(&full_path);
                    if wd.contains(&test_path) {
                        // One event - create.
                        1
                    } else {
                        // No events.
                        0
                    }
                }
                None => {
                    self.real_mkdir(&full_path);
                    0
                }
            }
        }

        fn real_mkdir(&self, real_path: &PathBuf) {
            fs::create_dir_all(&real_path).unwrap_or_else(|_| {
                                              panic!("could not create directories up to {}, \
                                                      debug info:\n{}",
                                                     real_path.display(),
                                                     self.debug_info,)
                                          });
        }

        fn touch(&self, path: &PathBuf) -> u32 {
            let pp = self.prepend_root(&path);
            File::create(&pp).unwrap_or_else(|_| {
                                 panic!("could not create file {}, debug info:\n{}",
                                        pp.display(),
                                        self.debug_info,)
                             });
            if self.parent_is_watched(&pp) {
                // One event - create.
                1
            } else {
                // No events.
                0
            }
        }

        fn mv(&self, from: &PathBuf, to: &PathBuf) -> u32 {
            let ff = self.prepend_root(&from);
            let tt = self.prepend_root(&to);
            fs::rename(&ff, &tt).unwrap_or_else(|_| {
                                    panic!("could not move from {} to {}, debug info:\n{}",
                                           ff.display(),
                                           tt.display(),
                                           self.debug_info,)
                                });
            match (self.parent_is_watched(&ff), self.parent_is_watched(&tt)) {
                (true, true) | (true, false) => {
                    if self.path_is_watched(&ff) {
                        // Since we are watching both moved path and
                        // its parent, we are going to receive double
                        // notice remove event followed by the rename
                        // event.
                        3
                    } else {
                        // Two events - notice remove, and rename or
                        // remove.
                        2
                    }
                }
                (false, true) => {
                    // One event - create.
                    1
                }
                (false, false) => {
                    // No events.
                    0
                }
            }
        }

        fn rm_rf(&mut self, path: &PathBuf) -> u32 {
            let pp = self.prepend_root(&path);
            let metadata = match pp.symlink_metadata() {
                Ok(m) => m,
                Err(e) => {
                    match e.kind() {
                        ErrorKind::NotFound => return 0,
                        _ => {
                            panic!("Failed to stat {}: {}, debug info:\n{}",
                                   pp.display(),
                                   e,
                                   self.debug_info,)
                        }
                    }
                }
            };
            let event_count = self.get_event_count_on_rm_rf(&pp);
            if metadata.is_dir() {
                fs::remove_dir_all(&pp).unwrap_or_else(|err| {
                                           panic!("failed to remove directory {}: {}, debug \
                                                   info:\n{}",
                                                  pp.display(),
                                                  err,
                                                  self.debug_info,)
                                       });
            } else {
                fs::remove_file(&pp).unwrap_or_else(|err| {
                                        panic!("failed to remove file {}: {}, debug info:\n{}",
                                               pp.display(),
                                               err,
                                               self.debug_info,)
                                    });
            }
            event_count
        }

        fn get_event_count_on_rm_rf(&self, top_path: &PathBuf) -> u32 {
            let mut queue = VecDeque::new();
            queue.push_back(top_path.clone());
            let mut event_count = 0;
            while let Some(path) = queue.pop_front() {
                if !self.parent_is_watched(&path) {
                    continue;
                }
                // Two events for each deletion in the watched path -
                // remove notice and remove.
                event_count += 2;
                let metadata = match path.symlink_metadata() {
                    Ok(m) => m,
                    Err(err) => {
                        panic!("Failed to stat {}: {}, debug info:\n{}",
                               path.display(),
                               err,
                               self.debug_info,)
                    }
                };
                if !metadata.is_dir() {
                    continue;
                }
                queue.extend(self.get_dir_contents(&path));
            }
            event_count
        }

        fn get_dir_contents(&self, path: &PathBuf) -> Vec<PathBuf> {
            fs::read_dir(&path).unwrap_or_else(|err| {
                                   panic!("failed to read directory {}: {}, debug info:\n{}",
                                          path.display(),
                                          err,
                                          self.debug_info,)
                               })
                               .map(|rde| {
                                   rde.unwrap_or_else(|err| {
                                          panic!("failed to get entry for {}: {}, debug info:\n{}",
                                                 path.display(),
                                                 err,
                                                 self.debug_info,)
                                      })
                                      .path()
                               })
                               .collect()
        }

        fn parent_is_watched(&self, path: &PathBuf) -> bool {
            self.path_is_watched(&self.get_parent(&path))
        }

        fn path_is_watched(&self, path: &PathBuf) -> bool {
            match self.watched_dirs {
                Some(wd) => wd.contains(path),
                None => false,
            }
        }

        fn get_parent(&self, path: &PathBuf) -> PathBuf {
            path.parent()
                .unwrap_or_else(|| {
                    panic!("path {} has no parent, debug info:\n{}",
                           path.display(),
                           self.debug_info)
                })
                .to_owned()
        }

        fn prepend_root(&self, p: &PathBuf) -> PathBuf {
            prepend_root_impl(self.root, p, self.debug_info)
        }
    }

    struct TestCaseRunner {
        debug_info: DebugInfo,
        // We don't use this field anywhere, but it will drop the temp
        // dir when TestCaseRunner goes away.
        #[allow(dead_code)]
        tmp_dir: TempDir,
        root: PathBuf,
    }

    impl TestCaseRunner {
        fn new() -> Self {
            let tmp_dir =
                TempDir::new().unwrap_or_else(|_| panic!("couldn't create temporary directory",));
            let root = tmp_dir.path().to_owned();
            Self { debug_info: DebugInfo::new(),
                   tmp_dir,
                   root }
        }

        fn run_init_commands(&mut self, commands: &[InitCommand]) {
            let fs_ops = self.get_fs_ops();
            for command in commands {
                match command {
                    InitCommand::MkdirP(ref path) => {
                        fs_ops.mkdir_p(path);
                    }
                    InitCommand::Touch(ref path) => {
                        fs_ops.touch(path);
                    }
                    InitCommand::LnS(ref target, ref path) => {
                        fs_ops.ln_s(target, path);
                    }
                }
            }
        }

        fn prepare_watcher(&mut self, tc_init_path: &Option<PathBuf>) -> WatcherSetup {
            let init_path = tc_init_path.clone().unwrap_or_else(|| pb!("/a/b/c/d/e/f"));
            let additional_dirs = self.get_additional_directories_from_root();
            let callbacks = TestCallbacks::new(&additional_dirs);
            let watcher = FileWatcher::<_, TestWatcher>::create(self.prepend_root(&init_path),
                                                                callbacks).unwrap_or_else(|_| {
                              panic!("failed to create watcher, debug info:\n{}", self.debug_info,)
                          });
            WatcherSetup { init_path, watcher }
        }

        fn run_steps(&mut self,
                     mut setup: WatcherSetup,
                     tc_initial_file: &Option<PathBuf>,
                     steps: &[Step]) {
            let mut initial_file = tc_initial_file.clone();
            let mut actual_initial_file = setup.watcher.initial_real_file.clone();
            for (step_idx, step) in steps.iter().enumerate() {
                self.debug_info.push_level();
                self.debug_info
                    .add(format!("step {}:\n{}", step_idx, dits!(step)));
                let iterations = self.execute_step_action(&mut setup, &step.action);
                self.spin_watcher(&mut setup, iterations);
                self.test_dirs(&step.dirs, &setup.watcher.paths.dirs);
                self.test_paths(&step.paths, &setup.init_path, &setup.watcher.paths.paths);
                let real_initial_file =
                    self.test_initial_file(iterations, &mut initial_file, &mut actual_initial_file);
                self.test_events(real_initial_file,
                                 &step.events,
                                 &mut setup.watcher.get_mut_callbacks().events);
                self.debug_info.pop_level();
                debug!("\n\n\n++++++++++++++++\n++++STEP+END++++\n++++++++++++++++\n\n\n");
            }
            debug!("\n\n\n================\n=TEST=CASE=ENDS=\n================\n\n\n");
        }

        fn execute_step_action(&mut self, setup: &mut WatcherSetup, action: &StepAction) -> u32 {
            let iterations = {
                let tw = setup.watcher.get_mut_underlying_watcher();
                let mut fs_ops = self.get_fs_ops_with_dirs(&tw.watched_dirs);
                match action {
                    StepAction::LnS(ref target, ref path) => fs_ops.ln_s(target, path),
                    StepAction::MkdirP(ref path) => fs_ops.mkdir_p(path),
                    StepAction::Touch(ref path) => fs_ops.touch(path),
                    StepAction::Mv(ref from, ref to) => fs_ops.mv(from, to),
                    StepAction::RmRF(ref path) => fs_ops.rm_rf(path),
                    StepAction::Nop => 0,
                }
            };
            self.debug_info
                .add(format!("expected iterations: {}", iterations));
            iterations
        }

        fn spin_watcher(&self, setup: &mut WatcherSetup, iterations: u32) {
            let mut iteration = 0;

            // After switching single_iteration() from recv() to try_recv(), this sleep is required
            // for these tests to pass.
            thread::sleep(Duration::from_secs(3));

            while iteration < iterations {
                setup.watcher
                     .single_iteration()
                     .unwrap_or_else(|_| {
                         panic!("iteration failed, debug info:\n{}", self.debug_info,)
                     });
                let cb = setup.watcher.get_mut_callbacks();
                if cb.ignore {
                    debug!("got event below tmpdir, not increasing the iteration counter");
                    cb.ignore = false;
                } else {
                    iteration += 1;
                }
            }
        }

        fn test_dirs(&mut self,
                     step_dirs: &HashMap<PathBuf, u32>,
                     actual_dirs: &HashMap<PathBuf, u32>) {
            let expected_dirs = self.fixup_expected_dirs(step_dirs);
            self.debug_info
                .add(format!("fixed up expected dirs:\n{}", dits!(expected_dirs),));
            assert_eq!(&expected_dirs, actual_dirs,
                       "comparing watched directories, debug info:\n{}",
                       self.debug_info,);
        }

        fn test_paths(&mut self,
                      step_paths: &HashMap<PathBuf, PathState>,
                      init_path: &PathBuf,
                      actual_paths: &HashMap<PathBuf, WatchedFile>) {
            let expected_paths = self.fixup_expected_paths(&step_paths, &init_path);
            self.debug_info
                .add(format!("fixed up expected paths:\n{}", dits!(expected_paths),));
            self.compare_paths(expected_paths, actual_paths);
        }

        fn test_initial_file(&mut self,
                             iterations: u32,
                             initial_file: &mut Option<PathBuf>,
                             actual_initial_file: &mut Option<PathBuf>)
                             -> Option<PathBuf> {
            if iterations > 0 {
                let expected_initial_file =
                    initial_file.take().map(|path| self.prepend_root(&path));
                self.debug_info
                    .add(format!("fixed up initial file: {:?}", expected_initial_file));
                assert_eq!(expected_initial_file,
                           actual_initial_file.take(),
                           "comparing initial path, debug info:\n{}",
                           self.debug_info,);
                expected_initial_file
            } else {
                None
            }
        }

        fn test_events(&mut self,
                       real_initial_file: Option<PathBuf>,
                       step_events: &[NotifyEvent],
                       actual_events: &mut Vec<NotifyEvent>) {
            let expected_events = self.fixup_expected_events(&step_events, real_initial_file);
            self.debug_info
                .add(format!("fixed up expected events: {:?}", expected_events));
            assert_eq!(&expected_events, actual_events,
                       "comparing expected events, debug info:\n{}",
                       self.debug_info,);
            actual_events.clear();
        }

        fn get_fs_ops_with_dirs<'a>(&'a mut self, watched_dirs: &'a HashSet<PathBuf>) -> FsOps<'a> {
            let mut fs_ops = self.get_fs_ops();
            fs_ops.watched_dirs = Some(watched_dirs);
            fs_ops
        }

        fn get_fs_ops(&mut self) -> FsOps<'_> {
            FsOps { debug_info:   &mut self.debug_info,
                    root:         &self.root,
                    watched_dirs: None, }
        }

        fn fixup_expected_dirs(&self, dirs: &HashMap<PathBuf, u32>) -> HashMap<PathBuf, u32> {
            let mut expected_dirs = dirs.iter()
                                        .map(|(p, c)| (self.prepend_root(&p), *c))
                                        .collect::<HashMap<_, _>>();
            let additional_dirs = self.get_additional_directories_from_root();
            expected_dirs.extend(additional_dirs.iter().cloned().map(|d| (d, 1)));
            expected_dirs
        }

        // Get vector for extending dirs. For the root directory like
        // /tmp/foo, the vector will be [`/`, `/tmp`].
        //
        // All this is because we run the tests in a temporary
        // directory (let's say `/tmp/foo`) and if in test case we
        // specify that we expect directories `/` and `/a` to be
        // watched, then in reality these will be `/`, `/tmp`,
        // `/tmp/foo`, `/tmp/foo/a`.
        fn get_additional_directories_from_root(&self) -> Vec<PathBuf> {
            let mut tmp_path = PathBuf::new();
            let mut additional_dirs = Vec::new();

            for component in self.root.components() {
                match component {
                    Component::Prefix(p) => tmp_path.push(p.as_os_str().to_owned()),
                    Component::RootDir | Component::Normal(_) => {
                        tmp_path.push(component.as_os_str().to_owned());
                        additional_dirs.push(tmp_path.clone());
                    }
                    // Respectively the `.`. and `..` components of a path.
                    Component::CurDir | Component::ParentDir => {
                        panic!("the path should be simplified")
                    }
                };
            }
            // Pop the last directory (like `/tmp/foo`), so it will not
            // overwrite an already fixed-up directory in expected_dirs.
            additional_dirs.pop();

            additional_dirs
        }

        fn fixup_expected_paths(&self,
                                paths: &HashMap<PathBuf, PathState>,
                                init_path: &PathBuf)
                                -> HashMap<PathBuf, PathState> {
            let expected_paths = self.get_initial_expected_paths(paths);
            let real_first_expected =
                self.get_real_first_expected_path(&init_path, &expected_paths);
            let additional_paths = self.get_additional_paths(&real_first_expected,
                                                             // The existence of this path in the
                                                             // map is checked in
                                                             // get_real_first_expected_path.
                                                             &expected_paths[&real_first_expected]);

            self.link_expected_paths_with_additional_ones(expected_paths,
                                                          additional_paths,
                                                          real_first_expected)
        }

        fn compare_paths(&mut self,
                         expected_paths: HashMap<PathBuf, PathState>,
                         actual_paths: &HashMap<PathBuf, WatchedFile>) {
            self.compare_path_keys(&expected_paths, &actual_paths);

            for (path, path_state) in expected_paths {
                // This should not panic - we tested the equality of
                // paths in compare_path_keys.
                let watched_file = actual_paths.get(&path).unwrap();

                self.debug_info.push_level();
                self.debug_info.add(format!("path: {}", path.display()));
                self.debug_info
                    .add(format!("watched file:\n{}", dits!(watched_file)));
                self.debug_info
                    .add(format!("path state:\n{}", dits!(path_state)));
                self.compare_path_state_with_watched_file(&path_state, &watched_file);
                self.debug_info.pop_level();
            }
        }

        fn fixup_expected_events(&self,
                                 events: &[NotifyEvent],
                                 real_initial_file: Option<PathBuf>)
                                 -> Vec<NotifyEvent> {
            let mut expected_events = match real_initial_file {
                Some(path) => vec![NotifyEvent::appeared(path)],
                None => Vec::new(),
            };
            expected_events.extend(events.iter()
                                         .map(|e| {
                                             NotifyEvent::new(self.prepend_root(&e.path), e.kind)
                                         }));
            expected_events
        }

        fn get_initial_expected_paths(&self,
                                      paths: &HashMap<PathBuf, PathState>)
                                      -> HashMap<PathBuf, PathState> {
            paths.iter()
                 .map(|(p, s)| {
                     (self.prepend_root(&p),
                      PathState { kind:      s.kind,
                                  path_rest: s.path_rest.clone(),
                                  prev:      match &s.prev {
                                      Some(ref p) => Some(self.prepend_root(&p)),
                                      None => None,
                                  },
                                  next:      match &s.next {
                                      Some(ref p) => Some(self.prepend_root(&p)),
                                      None => None,
                                  }, })
                 })
                 .collect()
        }

        fn get_real_first_expected_path(&self,
                                        init_path: &PathBuf,
                                        expected_paths: &HashMap<PathBuf, PathState>)
                                        -> PathBuf {
            let first_expected = get_first_item(&init_path);
            let real_first_expected = self.prepend_root(&first_expected);
            let first_item = expected_paths.get(&real_first_expected).unwrap_or_else(|| {
                                                                         panic!(
                    "expected watched item for {} (real: {}), it is an error in the test case",
                    first_expected.display(),
                    real_first_expected.display(),
                )
                                                                     });

            assert_eq!(first_item.prev,
                       None,
                       "expected prev member of first expected path ({}, real: {}) to be None, {}",
                       first_expected.display(),
                       real_first_expected.display(),
                       "it is an error in the test case",);

            real_first_expected
        }

        fn prepend_root(&self, p: &PathBuf) -> PathBuf {
            prepend_root_impl(&self.root, p, &self.debug_info)
        }

        fn get_additional_paths(&self,
                                real_first_expected: &PathBuf,
                                first_item: &PathState)
                                -> Vec<(PathBuf, PathState)> {
            let mut ap_vec = Vec::new();
            // empty additional path states
            for path in self.get_additional_paths_from_root() {
                ap_vec.push((path,
                             PathState { kind:      PathKind::Directory,
                                         path_rest: Vec::new(),
                                         prev:      None,
                                         next:      None, }));
            }
            let ap_len = ap_vec.len();
            // Link the path states - set the prev and next
            // members. The first path state will have None `prev` and
            // the last path state will have None `next`.
            for idx in (0..ap_len).skip(1) {
                ap_vec[idx].1.prev = Some(ap_vec[idx - 1].0.clone());
                ap_vec[ap_len - 1 - idx].1.next = Some(ap_vec[ap_len - idx].0.clone())
            }
            // Fill path rests in additional path states.
            let real_first_expected_path_rest = to_path_rest(&real_first_expected);
            for (idx, ap_vec_item) in ap_vec.iter_mut().enumerate() {
                let path_rest = &mut ap_vec_item.1.path_rest;
                // If root/temporary directory is `/tmp/foo/bar` then
                // `/tmp` will have path rest `[foo, bar, <rest from
                // the first item>]`, `/tmp/foo` - `[bar, <rest from
                // the first item>]`, and `/tmp/foo/bar` - `[<rest
                // from the first item>]`.
                path_rest.extend(real_first_expected_path_rest.iter().skip(idx + 1).cloned());
                // Here add the `<rest from the first item>` part.
                path_rest.extend(first_item.path_rest.iter().cloned());
            }

            ap_vec
        }

        fn link_expected_paths_with_additional_ones(&self,
                                                    mut expected_paths: HashMap<PathBuf,
                                                            PathState>,
                                                    mut additional_paths: Vec<(PathBuf,
                                                         PathState)>,
                                                    real_first_expected: PathBuf)
                                                    -> HashMap<PathBuf, PathState> {
            // link last additional path state with the first expected one
            if let Some(last) = additional_paths.last_mut() {
                // The existence of this path in the map is checked in
                // get_real_first_expected_path.
                let mut first_item = expected_paths.get_mut(&real_first_expected).unwrap();

                last.1.next = Some(real_first_expected);
                first_item.prev = Some(last.0.clone());
            }

            expected_paths.extend(additional_paths);
            expected_paths
        }

        fn compare_path_keys(&self,
                             expected_paths: &HashMap<PathBuf, PathState>,
                             actual_paths: &HashMap<PathBuf, WatchedFile>) {
            let mut expected_paths_keys: Vec<&PathBuf> = expected_paths.keys().collect();
            expected_paths_keys.sort();
            let mut actual_paths_keys: Vec<&PathBuf> = actual_paths.keys().collect();
            actual_paths_keys.sort();

            assert_eq!(expected_paths_keys, actual_paths_keys,
                       "comparing paths, debug info:\n{}",
                       self.debug_info,);
        }

        fn compare_path_state_with_watched_file(&self,
                                                path_state: &PathState,
                                                watched_file: &WatchedFile) {
            let common = watched_file.get_common();
            let expected_kind = match watched_file {
                WatchedFile::Regular(_) => PathKind::Regular,
                WatchedFile::MissingRegular(_) => PathKind::MissingRegular,
                WatchedFile::Symlink(_) => PathKind::Symlink,
                WatchedFile::Directory(_) => PathKind::Directory,
                WatchedFile::MissingDirectory(_) => PathKind::MissingDirectory,
            };
            let path_rest: Vec<OsString> = common.path_rest.iter().cloned().collect();

            assert_eq!(path_state.kind, expected_kind,
                       "ensuring proper watched file kind, debug info:\n{}",
                       self.debug_info,);
            assert_eq!(path_state.path_rest, path_rest,
                       "comparing path rest, debug info:\n{}",
                       self.debug_info,);
            assert_eq!(path_state.prev, common.prev,
                       "comparing prev member, debug info:\n{}",
                       self.debug_info,);
            assert_eq!(path_state.next, common.next,
                       "comparing next member, debug info:\n{}",
                       self.debug_info,);
        }

        // Get vector for extending paths. For the root directory like
        // /tmp/foo, the vector will be [`/tmp/`, `/tmp/foo`].
        //
        // All this is because we run the tests in a temporary
        // directory (let's say `/tmp/foo`) and if in test case we
        // specify that we expect the watched items to be `/a` and
        // `/a/b`, then in reality these will be `/tmp`, `/tmp/foo`,
        // `/tmp/foo/a` and `/tmp/foo/a/b`.
        fn get_additional_paths_from_root(&self) -> Vec<PathBuf> {
            let mut tmp_path = PathBuf::new();
            let mut for_paths = Vec::new();

            for component in self.root.components() {
                match component {
                    Component::Prefix(_) | Component::RootDir => {
                        tmp_path.push(component.as_os_str().to_owned())
                    }
                    Component::Normal(c) => {
                        tmp_path.push(c.to_owned());
                        for_paths.push(tmp_path.to_owned());
                    }
                    // Respectively the `.`. and `..` components of a path.
                    Component::CurDir | Component::ParentDir => {
                        panic!("the path should be simplified")
                    }
                };
            }

            for_paths
        }
    }

    fn prepend_root_impl(root: &PathBuf, p: &PathBuf, debug_info: &DebugInfo) -> PathBuf {
        if !p.is_absolute() {
            panic!("expected path {} to be absolute, debug info:\n{}",
                   p.display(),
                   debug_info,);
        }
        root.join(strip_prefix_and_root(p))
    }

    fn get_first_item(path: &PathBuf) -> PathBuf {
        let mut first = PathBuf::new();

        for component in path.components() {
            match component {
                Component::Prefix(p) => first.push(p.as_os_str().to_owned()),
                Component::RootDir => first.push(component.as_os_str().to_owned()),
                Component::Normal(c) => {
                    first.push(c.to_owned());
                    break;
                }
                // Respectively the `.`. and `..` components of a path.
                Component::CurDir | Component::ParentDir => panic!("the path should be simplified"),
            }
        }

        first
    }

    fn to_path_rest(path: &PathBuf) -> Vec<OsString> {
        let mut path_rest = Vec::new();

        for component in path.components() {
            match component {
                Component::Prefix(_) => (),
                Component::RootDir => (),
                Component::Normal(c) => path_rest.push(c.to_owned()),
                // Respectively the `.`. and `..` components of a path.
                Component::CurDir | Component::ParentDir => panic!("the path should be simplified"),
            }
        }

        path_rest
    }

    fn strip_prefix_and_root(path: &PathBuf) -> PathBuf {
        let mut stripped = PathBuf::new();

        for component in path.components() {
            match component {
                Component::Prefix(_) | Component::RootDir => (),
                Component::Normal(_) | Component::CurDir | Component::ParentDir => {
                    stripped.push(component.as_os_str())
                }
            }
        }

        stripped
    }
}
