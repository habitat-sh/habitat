// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::any::Any;
use std::ffi::{CStr, CString};
use std::fmt;
use std::fs;
use std::marker::PhantomData;
use std::mem;
use std::path::Path;
use std::ptr;
use std::slice;
use std::sync::{Arc, Mutex};

use bincode::{self, SizeLimit};
use bincode::rustc_serialize::{encode, decode};
use depot_core::data_object::{self, DataObject, ToMdbValue, FromMdbValue};
use libc::{c_void, c_int, c_uint, mode_t, size_t};
use lmdb_sys;
use rustc_serialize::{Encodable, Decodable};

use error::{Error, Result};

lazy_static! {
    static ref OPEN_LOCK: Mutex<()> = Mutex::new(());
}

bitflags! {
    flags EnvFlags: c_uint {
        const ENV_NO_SYNC = lmdb_sys::MDB_NOSYNC,
        const ENV_NO_META_SYNC = lmdb_sys::MDB_NOMETASYNC,
        const ENV_MAP_ASYNC = lmdb_sys::MDB_MAPASYNC,
        const ENV_NO_MEM_INIT = lmdb_sys::MDB_NOMEMINIT,
    }
}

bitflags! {
    flags EnvCreateFlags: c_uint {
        const ENV_CREATE_FIXED_MAP = lmdb_sys::MDB_FIXEDMAP,
        const ENV_CREATE_NO_SUB_DIR = lmdb_sys::MDB_NOSUBDIR,
        const ENV_CREATE_READ_ONLY = lmdb_sys::MDB_RDONLY,
        const ENV_CREATE_WRITE_MAP = lmdb_sys::MDB_WRITEMAP,
        const ENV_CREATE_NO_META_SYNC = lmdb_sys::MDB_NOMETASYNC,
        const ENV_CREATE_NO_SYNC = lmdb_sys::MDB_NOSYNC,
        const ENV_CREATE_MAP_ASYNC = lmdb_sys::MDB_MAPASYNC,
        const ENV_CREATE_NO_TLS = lmdb_sys::MDB_NOTLS,
        const ENV_CREATE_NO_LOCK = lmdb_sys::MDB_NOLOCK,
        const ENV_CREATE_NO_READ_AHEAD = lmdb_sys::MDB_NORDAHEAD,
        const ENV_CREATE_NO_MEM_INIT = lmdb_sys::MDB_NOMEMINIT,
    }
}

bitflags! {
    flags DatabaseFlags: c_uint {
        const DB_REVERSE_KEY = lmdb_sys::MDB_REVERSEKEY,
        const DB_ALLOW_DUPS = lmdb_sys::MDB_DUPSORT,
        const DB_INTEGER_KEY = lmdb_sys::MDB_INTEGERKEY,
        const DB_DUPS_FIXED = lmdb_sys::MDB_DUPFIXED,
        const DB_ALLOW_INT_DUPS = lmdb_sys::MDB_INTEGERDUP,
        const DB_REVERSE_DUPS = lmdb_sys::MDB_REVERSEDUP,
        const DB_CREATE = lmdb_sys::MDB_CREATE,
    }
}

bitflags! {
    flags WriteFlags: c_uint {
        const CURRENT = lmdb_sys::MDB_CURRENT,
        const NO_DUP_DATA = lmdb_sys::MDB_NODUPDATA,
        const NO_OVERWRITE = lmdb_sys::MDB_NOOVERWRITE,
        const RESERVE = lmdb_sys::MDB_RESERVE,
        const APPEND = lmdb_sys::MDB_APPEND,
        const APPEND_DUP = lmdb_sys::MDB_APPENDDUP,
        const MULTIPLE = lmdb_sys::MDB_MULTIPLE,
    }
}

/// Name of the package database
pub const PACKAGE_DB: &'static str = "packages";
/// Name of the package index database
pub const PACKAGE_INDEX: &'static str = "package-index";
/// Name of the views database
pub const VIEW_DB: &'static str = "views";
/// Name of the package to views index database
pub const PACKAGE_VIEW_INDEX: &'static str = "package-view-index";
/// Name of the view to packages index database
pub const VIEW_PACKAGE_INDEX: &'static str = "view-package-index";
/// Value for how many databases can be opened within a DataStore's environment. Increase this
/// count for each new database added and decrease this count if databases are removed from the
/// DataStore.
pub const MAX_DBS: u32 = 5;

macro_rules! try_mdb {
    ($e: expr) => (
        {
            let code = $e;
            match code {
                lmdb_sys::MDB_SUCCESS => (),
                _ => return Err(Error::from(MdbError::from(code)))
            }
        }
    )
}

macro_rules! handle_mdb {
    ($e: expr) => (
        {
            let code = $e;
            match code {
                lmdb_sys::MDB_SUCCESS => Ok(()),
                _ => Err(Error::from(MdbError::from(code)))
            }
        }
    )
}

macro_rules! assert_txn_state_eq {
    ($cur:expr, $exp:expr) => (
        {
            let c = $cur.clone();
            let e = $exp;
            if c == e {
                ()
            } else {
                return Err(Error::from(MdbError::StateError(c, e)))
            }
        }
    )
}

#[derive(Debug)]
pub enum MdbError {
    /// The specified DBI was changed unexpectedly
    BadDbi,
    /// Invalid reuse of reader locktable slot
    BadRslot,
    /// Transaction must abort, has a child, or is invalid
    BadTxn,
    /// Unsupported size of key/database name/data or wrong DUPFIXED size
    BadValSize,
    /// Unable to decode data from database
    DecodingError(bincode::rustc_serialize::DecodingError),
    /// Unable to encode data into database
    EncodingError(bincode::rustc_serialize::EncodingError),
    /// Key/Data pair not found
    NotFound,
    /// Key/Data pair already exists
    KeyExists,
    /// Cursor stack too deep (internal error)
    CursorFull,
    /// Environment maxdbs reached
    DbsFull,
    /// Operation and database incompatible or database type has changed. This can mean:
    ///
    /// * Operation expects an the database to have the `DB_ALLOW_DUPS`/`DB_DUPS_FIXED` flag set
    /// * Opening a named database when the unamed database has the `DB_ALLOW_DUPS`/`DB_INTEGER_KEY`
    ///   flag set
    /// * Accessing a data record as a database or vice versa
    /// * The database was dropped and recreated with different flags
    Incompatible,
    /// File is not a valid LMDB file
    Invalid,
    /// Environment mapsize reached
    MapFull,
    /// Database contents grew beyond environment mapsize
    MapResized,
    /// Page has not enough space (internal error)
    PageFull,
    /// Requested page not found (usually indicates data corruption)
    PageNotFound,
    /// Environment maxreaders reached
    ReadersFull,
    /// Located page was wrong type
    Corrupted,
    /// Update of meta page failed or environment encountered a fatal error
    Panic,
    /// Too many TLS keys in use (Windows only)
    TLSFull,
    /// Trasnaction has too many dirty pages
    TxnFull,
    /// Transaction was in an unexpected state at time of execution
    StateError(TxnState, TxnState),
    /// Catch all for undefined error codes
    Undefined(c_int, String),
    /// Environment version mismatch
    VersionMismatch,
}

impl fmt::Display for MdbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<c_int> for MdbError {
    fn from(code: c_int) -> MdbError {
        match code {
            lmdb_sys::MDB_BAD_DBI => MdbError::BadDbi,
            lmdb_sys::MDB_BAD_RSLOT => MdbError::BadRslot,
            lmdb_sys::MDB_BAD_TXN => MdbError::BadTxn,
            lmdb_sys::MDB_BAD_VALSIZE => MdbError::BadValSize,
            lmdb_sys::MDB_NOTFOUND => MdbError::NotFound,
            lmdb_sys::MDB_KEYEXIST => MdbError::KeyExists,
            lmdb_sys::MDB_CURSOR_FULL => MdbError::CursorFull,
            lmdb_sys::MDB_DBS_FULL => MdbError::DbsFull,
            lmdb_sys::MDB_INCOMPATIBLE => MdbError::Incompatible,
            lmdb_sys::MDB_INVALID => MdbError::Invalid,
            lmdb_sys::MDB_MAP_FULL => MdbError::MapFull,
            lmdb_sys::MDB_MAP_RESIZED => MdbError::MapResized,
            lmdb_sys::MDB_PAGE_FULL => MdbError::PageFull,
            lmdb_sys::MDB_PAGE_NOTFOUND => MdbError::PageNotFound,
            lmdb_sys::MDB_READERS_FULL => MdbError::ReadersFull,
            lmdb_sys::MDB_CORRUPTED => MdbError::Corrupted,
            lmdb_sys::MDB_PANIC => MdbError::Panic,
            lmdb_sys::MDB_TLS_FULL => MdbError::TLSFull,
            lmdb_sys::MDB_TXN_FULL => MdbError::TxnFull,
            lmdb_sys::MDB_VERSION_MISMATCH => MdbError::VersionMismatch,
            _ => {
                let msg = unsafe {
                    String::from_utf8(CStr::from_ptr(lmdb_sys::mdb_strerror(code))
                                          .to_bytes()
                                          .to_vec())
                        .unwrap()
                };
                MdbError::Undefined(code, msg)
            }
        }
    }
}

impl From<bincode::rustc_serialize::DecodingError> for MdbError {
    fn from(value: bincode::rustc_serialize::DecodingError) -> MdbError {
        MdbError::DecodingError(value)
    }
}

impl From<bincode::rustc_serialize::EncodingError> for MdbError {
    fn from(value: bincode::rustc_serialize::EncodingError) -> MdbError {
        MdbError::EncodingError(value)
    }
}

fn create_txn(env: &Environment,
              flags: c_uint,
              parent: Option<&mut lmdb_sys::MDB_txn>)
              -> Result<*mut lmdb_sys::MDB_txn> {
    let mut handle: *mut lmdb_sys::MDB_txn = ptr::null_mut();
    let parent = if parent.is_some() {
        parent.unwrap() as *mut lmdb_sys::MDB_txn
    } else {
        ptr::null_mut() as *mut lmdb_sys::MDB_txn
    };
    unsafe {
        try_mdb!(lmdb_sys::mdb_txn_begin(env.handle, parent, flags, &mut handle));
    }
    Ok(handle)
}

fn cursor_get<K, D>(cursor: *mut lmdb_sys::MDB_cursor,
                    key: Option<&K>,
                    value: Option<&D>,
                    op: CursorOp)
                    -> Result<(Option<K>, D)>
    where K: FromMdbValue + ToMdbValue,
          D: Encodable + Decodable
{
    unsafe {
        let mut kval = if key.is_some() {
            key.unwrap().to_mdb_value()
        } else {
            val_for::<K>(None)
        };
        let mut dval = encoded_val_for::<D>(value);
        let key_ptr = kval.mv_data;
        try_mdb!(lmdb_sys::mdb_cursor_get(cursor, &mut kval, &mut dval, op as u32));
        let kout = if key_ptr != kval.mv_data {
            let kout = K::from_mdb_value(&kval);
            Some(kout)
        } else {
            None
        };
        let bytes: &[u8] = slice::from_raw_parts(dval.mv_data as *const u8, dval.mv_size);
        match decode(bytes) {
            Ok(dout) => Ok((kout, dout)),
            Err(e) => Err(Error::from(MdbError::from(e))),
        }
    }
}

unsafe fn val_for<T>(data: Option<&T>) -> lmdb_sys::MDB_val {
    match data {
        Some(d) => {
            lmdb_sys::MDB_val {
                mv_data: d as *const T as *mut c_void,
                mv_size: mem::size_of::<T>() as size_t,
            }
        }
        None => {
            lmdb_sys::MDB_val {
                mv_data: ptr::null_mut(),
                mv_size: 0,
            }
        }
    }
}

unsafe fn encoded_val_for<T: Encodable>(data: Option<&T>) -> lmdb_sys::MDB_val {
    match data {
        Some(d) => {
            // JW TODO: this should be set to the max size the database allows. Infinite is fine
            // for now.
            let mut encoded: Vec<u8> = encode(d, SizeLimit::Infinite).unwrap();
            let bytes: &mut [u8] = &mut encoded[..];
            lmdb_sys::MDB_val {
                mv_data: bytes.as_ptr() as *mut c_void,
                mv_size: bytes.len() as size_t,
            }
        }
        None => {
            lmdb_sys::MDB_val {
                mv_data: ptr::null_mut() as *mut c_void,
                mv_size: 0,
            }
        }
    }
}

pub struct DataStore {
    pub packages: PkgDatabase,
    pub views: ViewDatabase,
    #[allow(dead_code)]
    env: Arc<Environment>,
}

impl DataStore {
    /// Instantiates a new LMDB backed datastore.
    ///
    /// # Failures
    ///
    /// * Cannot read/write to the given path
    /// * Cannot obtain a lock to create the environment
    /// * Could not create the environment or any of it's databases
    pub fn open(path: &Path) -> Result<Self> {
        let mut flags = EnvCreateFlags::empty();
        flags.toggle(ENV_CREATE_NO_SUB_DIR);
        match path.parent() {
            Some(root) => try!(fs::create_dir_all(root)),
            None => return Err(Error::DbInvalidPath),
        }
        let env = try!(Environment::new()
                           .map_size(1073741824)
                           .max_databases(MAX_DBS)
                           .flags(flags)
                           .open(&path, 0o744));
        let env1 = Arc::new(env);
        let env2 = env1.clone();
        let env3 = env1.clone();
        let view_database = try!(ViewDatabase::new().create(env3));
        let pkg_database = try!(PkgDatabase::new().create(env2));
        Ok(DataStore {
            env: env1,
            packages: pkg_database,
            views: view_database,
        })
    }

    /// Truncates every database in the datastore.
    ///
    /// # Failures
    ///
    /// * If a read-write transaction could not be acquired for any of the databases in the
    ///   datastore
    pub fn clear(&self) -> Result<()> {
        let txn = try!(self.packages.txn_rw());
        try!(self.packages.clear(&txn));
        try!(txn.commit());
        let txn = try!(self.views.txn_rw());
        try!(self.views.clear(&txn));
        txn.commit()
    }
}

pub struct EnvironmentBuilder {
    flags: EnvCreateFlags,
    map_size: Option<u64>,
    max_databases: Option<u32>,
    max_readers: Option<u32>,
}

impl EnvironmentBuilder {
    /// Set environment flags.
    pub fn flags(mut self, flags: EnvCreateFlags) -> Self {
        // JW TODO: don't let people set flags themselves, the lmdb API is hostile and requires
        // that you have knowledge of what flags are and are not allowed to be mixed. Instead expose
        // setters that return a result when setting flags that may or may not be compatible with
        // the current state of the environment being built
        self.flags = flags;
        self
    }

    /// Set the maximum number of named databases for the environment.
    ///
    /// This function is only needed if multiple databases will be used in the environment. Simpler
    /// applications that use the environment as a single unnamed database can ignore this option.
    pub fn max_databases(mut self, count: u32) -> Self {
        self.max_databases = Some(count);
        self
    }

    /// Set the maximum number of threads/reader slots for the environment.
    ///
    /// This defines the number of slots in the lock table that is used to track readers in the
    /// environment. The default is 126. Starting a read-only transaction normally ties a lock table
    /// slot to the current thread untilt he environment closes or the thread exists. If the
    /// `ENV_CREATE_NO_TLS` flag is set, starting a new transaction instead ties the slot to the
    /// transaction until it, or the environment, is destroyed.
    pub fn max_readers(mut self, count: u32) -> Self {
        self.max_readers = Some(count);
        self
    }

    /// Set the size of the memory map to use for this environment.
    ///
    /// The size should be a multiple of the OS page size. The default is 10485760 bytes. The size
    /// of the memory map is also the maximum size of the database. The value should be chosen
    /// as large as possible to accomodate future growth of the database.
    pub fn map_size(mut self, size: u64) -> Self {
        self.map_size = Some(size);
        self
    }

    /// Create/open the database
    pub fn open(self, path: &Path, permissions: u32) -> Result<Environment> {
        let handle: *mut lmdb_sys::MDB_env = ptr::null_mut();
        unsafe {
            try_mdb!(lmdb_sys::mdb_env_create(mem::transmute(&handle)));
        }

        if let Some(map_size) = self.map_size {
            unsafe {
                try_mdb!(lmdb_sys::mdb_env_set_mapsize(handle, map_size as usize));
            }
        }

        if let Some(count) = self.max_databases {
            unsafe { try_mdb!(lmdb_sys::mdb_env_set_maxdbs(handle, count)) }
        }

        if let Some(count) = self.max_readers {
            unsafe { try_mdb!(lmdb_sys::mdb_env_set_maxreaders(handle, count)) }
        }

        // JW TODO: if read only flag is set, lets return a read only environment.

        unsafe {
            let path_str = try!(path.to_str().ok_or(Error::DbInvalidPath));
            let path_ptr = try!(CString::new(path_str).map_err(|_| Error::DbInvalidPath)).as_ptr();

            match lmdb_sys::mdb_env_open(handle,
                                         path_ptr,
                                         self.flags.bits(),
                                         permissions as mode_t) {
                lmdb_sys::MDB_SUCCESS => Ok(Environment { handle: handle }),
                code => {
                    lmdb_sys::mdb_env_close(handle);
                    Err(Error::from(MdbError::from(code)))
                }
            }
        }
    }
}

impl Default for EnvironmentBuilder {
    fn default() -> EnvironmentBuilder {
        EnvironmentBuilder {
            flags: EnvCreateFlags::empty(),
            max_databases: None,
            max_readers: None,
            map_size: None,
        }
    }
}

pub struct Environment {
    handle: *mut lmdb_sys::MDB_env,
}

impl Environment {
    pub fn new() -> EnvironmentBuilder {
        EnvironmentBuilder::default()
    }
}

impl Drop for Environment {
    fn drop(&mut self) {
        if self.handle != ptr::null_mut() {
            unsafe { lmdb_sys::mdb_env_close(self.handle) }
        }
    }
}

unsafe impl Send for Environment {}
unsafe impl Sync for Environment {}

pub enum Txn {
    Read,
    ReadWrite,
}

#[derive(PartialEq, Debug, Eq, Copy, Clone)]
pub enum TxnState {
    Normal,
    Released,
    Invalid,
}

impl Default for TxnState {
    fn default() -> TxnState {
        TxnState::Normal
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
/// Cursor Get operations
///
/// This is the set of all operations for retrieving data using a cursor.
enum CursorOp {
    /// Position at the first key/data item
    First = lmdb_sys::MDB_FIRST as isize,
    /// Position at the first data item of current key. Valid only if the database has the
    /// `DB_ALLOW_DUPS` flag set
    FirstDup = lmdb_sys::MDB_FIRST_DUP as isize,
    /// Position at key/data pair. Valid only if the database has the `DB_ALLOW_DUPS` flag set
    GetBoth = lmdb_sys::MDB_GET_BOTH as isize,
    /// Position at key, nearest data. Valid only if the database has the `DB_ALLOW_DUPS` flag set
    GetBothRange = lmdb_sys::MDB_GET_BOTH_RANGE as isize,
    /// Return the key and data at the cursor's current position
    GetCurrent = lmdb_sys::MDB_GET_CURRENT as isize,
    /// Return the key and up to a page of duplicate data items from the cursor's current position.
    /// Move the cursor to prepare for `NextMultiple` cursor operation. Valid only if the database
    /// has the `DB_DUPS_FIXED` flag set
    GetMultiple = lmdb_sys::MDB_GET_MULTIPLE as isize,
    /// Position the cursor at the last key and data item
    Last = lmdb_sys::MDB_LAST as isize,
    /// Position the cursor at the last data item of the current key. Valid only if the database has
    /// the `DB_ALLOW_DUPS` flag set
    LastDup = lmdb_sys::MDB_LAST_DUP as isize,
    /// Position at the next data item
    Next = lmdb_sys::MDB_NEXT as isize,
    /// Position at the next data item of the current key. Valid only if the database has the
    /// `DB_ALLOW_DUPS` flag set
    NextDup = lmdb_sys::MDB_NEXT_DUP as isize,
    /// Return key and up to a page of duplicate data items from next cursor position. Move cursor
    /// to prepare for `NextMultiple` cursor operation. Valid only if the database has the
    /// `DB_DUPS_FIXED` flag set
    NextMultiple = lmdb_sys::MDB_NEXT_MULTIPLE as isize,
    /// Position at first data item of next key
    NextNoDup = lmdb_sys::MDB_NEXT_NODUP as isize,
    /// Position at previous data item
    Prev = lmdb_sys::MDB_PREV as isize,
    /// Position at previous data item of current key. Valid only if the database has the
    /// `DB_ALLOW_DUPS` flag set
    PrevDup = lmdb_sys::MDB_PREV_DUP as isize,
    /// Position at the last data item of previous key
    PrevNoDup = lmdb_sys::MDB_PREV_NODUP as isize,
    /// Position at the specified key
    Set = lmdb_sys::MDB_SET as isize,
    /// Position at the specified key and return the key and it's data
    SetKey = lmdb_sys::MDB_SET_KEY as isize,
    /// Position at first key greater than or equal to the specified key
    SetRange = lmdb_sys::MDB_SET_RANGE as isize,
}

/// Common behaviour for cursors
pub trait Cursor<'a, 'd, D: 'a + 'd + Database, T: Transaction<'a, D>> {
    /// Returns a raw pointer to the cursor's handle.
    fn handle(&self) -> *mut lmdb_sys::MDB_cursor;

    /// Returns a raw pointer to the cursor's transaction handle.
    fn txn(&self) -> &'a mut lmdb_sys::MDB_txn;

    /// Returns a reference to the current state of the cursor's transaction.
    fn state(&self) -> &TxnState;

    /// Returns the cursor's database handle.
    fn database(&self) -> lmdb_sys::MDB_dbi;

    /// Close an open cursor.
    fn close(self) -> ();

    /// Return count of duplicates for current key.
    ///
    /// This call is only valid on databases that support sorted duplicate items by the
    /// `DB_ALLOW_DUPS` flag.
    fn dup_count(&self) -> Result<u64> {
        assert_txn_state_eq!(self.state(), TxnState::Normal);
        let mut count: size_t = 0;
        unsafe {
            try_mdb!(lmdb_sys::mdb_cursor_count(self.handle(), &mut count));
        }
        Ok(count as u64)
    }

    /// Position cursor at the first key/data item and return the data for the item.
    fn first(&mut self) -> Result<(<D::Object as DataObject>::Key, D::Object)> {
        assert_txn_state_eq!(self.state(), TxnState::Normal);
        match cursor_get::<<D::Object as DataObject>::Key, D::Object>(self.handle(),
                                                                      None,
                                                                      None,
                                                                      CursorOp::First) {
            Ok((Some(key), value)) => Ok((key, value)),
            Ok(_) => unreachable!(),
            Err(e) => Err(e),
        }
    }

    /// Position the cursor at the first data item of the current key.
    ///
    /// This call is only valid on databases that support sorted duplicate items by the
    /// `DB_ALLOW_DUPS` flag.
    fn first_dup(&mut self) -> Result<D::Object> {
        assert_txn_state_eq!(self.state(), TxnState::Normal);
        match cursor_get::<<D::Object as DataObject>::Key, D::Object>(self.handle(),
                                                                      None,
                                                                      None,
                                                                      CursorOp::FirstDup) {
            Ok((_, value)) => Ok(value),
            Err(e) => Err(e),
        }
    }

    /// Position the cursor at the last key/data item and return the data for that item.
    fn last(&mut self) -> Result<D::Object> {
        assert_txn_state_eq!(self.state(), TxnState::Normal);
        match cursor_get::<<D::Object as DataObject>::Key, D::Object>(self.handle(),
                                                                      None,
                                                                      None,
                                                                      CursorOp::Last) {
            Ok((_, value)) => Ok(value),
            Err(e) => Err(e),
        }
    }

    /// Position the cursor at the last data item of the current key.
    ///
    /// This call is only valid on databases that support sorted duplicate items by the
    /// `DB_ALLOW_DUPS` flag.
    fn last_dup(&mut self) -> Result<D::Object> {
        assert_txn_state_eq!(self.state(), TxnState::Normal);
        match cursor_get::<<D::Object as DataObject>::Key, D::Object>(self.handle(),
                                                                      None,
                                                                      None,
                                                                      CursorOp::LastDup) {
            Ok((_, value)) => Ok(value),
            Err(e) => Err(e),
        }
    }

    /// Position the cursor at the next data item.
    fn next(&mut self) -> Result<(<D::Object as DataObject>::Key, D::Object)> {
        assert_txn_state_eq!(self.state(), TxnState::Normal);
        match cursor_get::<<D::Object as DataObject>::Key, D::Object>(self.handle(),
                                                                      None,
                                                                      None,
                                                                      CursorOp::Next) {
            Ok((Some(key), value)) => Ok((key, value)),
            Ok(_) => unreachable!(),
            Err(e) => Err(e),
        }
    }

    /// Position the cursor at the next data item of the current key.
    ///
    /// This call is only valid on databases taht support sorted duplicate items by the
    /// `DB_ALLOW_DUPS` flag.
    fn next_dup(&mut self) -> Result<(<D::Object as DataObject>::Key, D::Object)> {
        assert_txn_state_eq!(self.state(), TxnState::Normal);
        match cursor_get::<<D::Object as DataObject>::Key, D::Object>(self.handle(),
                                                                      None,
                                                                      None,
                                                                      CursorOp::NextDup) {
            Ok((Some(key), value)) => Ok((key, value)),
            Ok(_) => unreachable!(),
            Err(e) => Err(e),
        }
    }

    /// Position the cursor at the first data item of the next key.
    ///
    /// This call is only valid on databases taht support sorted duplicate items by the
    /// `DB_ALLOW_DUPS` flag.
    fn next_nodup(&mut self) -> Result<(<D::Object as DataObject>::Key, D::Object)> {
        assert_txn_state_eq!(self.state(), TxnState::Normal);
        match cursor_get::<<D::Object as DataObject>::Key, D::Object>(self.handle(),
                                                                      None,
                                                                      None,
                                                                      CursorOp::NextNoDup) {
            Ok((Some(key), value)) => Ok((key, value)),
            Ok(_) => unreachable!(),
            Err(e) => Err(e),
        }
    }

    /// Position the cursor at the previous data item.
    fn prev(&mut self) -> Result<(<D::Object as DataObject>::Key, D::Object)> {
        assert_txn_state_eq!(self.state(), TxnState::Normal);
        match cursor_get::<<D::Object as DataObject>::Key, D::Object>(self.handle(),
                                                                      None,
                                                                      None,
                                                                      CursorOp::Prev) {
            Ok((Some(key), value)) => Ok((key, value)),
            Ok(_) => unreachable!(),
            Err(e) => Err(e),
        }
    }

    /// Position the cursor at the previous data item of the current key.
    ///
    /// This call is only valid on databases taht support sorted duplicate items by the
    /// `DB_ALLOW_DUPS` flag.
    fn prev_dup(&mut self) -> Result<(<D::Object as DataObject>::Key, D::Object)> {
        assert_txn_state_eq!(self.state(), TxnState::Normal);
        match cursor_get::<<D::Object as DataObject>::Key, D::Object>(self.handle(),
                                                                      None,
                                                                      None,
                                                                      CursorOp::PrevDup) {
            Ok((Some(key), value)) => Ok((key, value)),
            Ok(_) => unreachable!(),
            Err(e) => Err(e),
        }
    }

    /// Position the cursor at the first data item of the previous key.
    ///
    /// This call is only valid on databases taht support sorted duplicate items by the
    /// `DB_ALLOW_DUPS` flag.
    fn prev_nodup(&mut self) -> Result<(<D::Object as DataObject>::Key, D::Object)> {
        assert_txn_state_eq!(self.state(), TxnState::Normal);
        match cursor_get::<<D::Object as DataObject>::Key, D::Object>(self.handle(),
                                                                      None,
                                                                      None,
                                                                      CursorOp::PrevNoDup) {
            Ok((Some(key), value)) => Ok((key, value)),
            Ok(_) => unreachable!(),
            Err(e) => Err(e),
        }
    }

    /// Position the cursor at the specified key and return the key and data.
    fn set_key(&mut self,
               key: &<D::Object as DataObject>::Key)
               -> Result<(<D::Object as DataObject>::Key, D::Object)> {
        assert_txn_state_eq!(self.state(), TxnState::Normal);
        match cursor_get::<<D::Object as DataObject>::Key, D::Object>(self.handle(),
                                                                      Some(key),
                                                                      None,
                                                                      CursorOp::SetKey) {
            Ok((Some(key), value)) => Ok((key, value)),
            Ok(_) => unreachable!(),
            Err(e) => Err(e),
        }
    }
}

/// Read-only cursor
pub struct RoCursor<'a, D: 'a + Database, T: 'a + Transaction<'a, D>> {
    txn: &'a T,
    cursor: *mut lmdb_sys::MDB_cursor,
    _marker: PhantomData<D>,
}

impl<'a, D, T> RoCursor<'a, D, T>
    where D: Database,
          T: Transaction<'a, D>
{
    fn open(txn: &'a T) -> Result<Self> {
        assert_txn_state_eq!(txn.state(), TxnState::Normal);
        let mut cursor: *mut lmdb_sys::MDB_cursor = ptr::null_mut();
        unsafe {
            try_mdb!(lmdb_sys::mdb_cursor_open(txn.handle(), txn.database().handle(), &mut cursor));
        }
        Ok(RoCursor {
            txn: txn,
            cursor: cursor,
            _marker: PhantomData,
        })
    }
}

impl<'a, 'b, D, T> Cursor<'a, 'b, D, T> for RoCursor<'a, D, T>
    where D: 'a + 'b + Database,
          T: 'a + Transaction<'a, D>
{
    fn close(self) {
        unsafe { lmdb_sys::mdb_cursor_close(self.cursor) }
    }

    fn handle(&self) -> *mut lmdb_sys::MDB_cursor {
        self.cursor
    }

    fn txn(&self) -> &'a mut lmdb_sys::MDB_txn {
        self.txn.handle()
    }

    fn database(&self) -> lmdb_sys::MDB_dbi {
        self.txn.database().handle()
    }

    fn state(&self) -> &TxnState {
        self.txn.state()
    }
}

impl<'a, D, T> Drop for RoCursor<'a, D, T>
    where D: Database,
          T: Transaction<'a, D>
{
    fn drop(&mut self) {
        unsafe { lmdb_sys::mdb_cursor_close(self.cursor) }
    }
}

/// Read-write cursor
pub struct RwCursor<'a, D: 'a + Database> {
    txn: &'a RwTransaction<'a, D>,
    cursor: *mut lmdb_sys::MDB_cursor,
    _marker: PhantomData<D>,
}

impl<'a, D: Database> RwCursor<'a, D> {
    fn open(txn: &'a RwTransaction<D>) -> Result<Self> {
        assert_txn_state_eq!(txn.state(), TxnState::Normal);
        let mut cursor: *mut lmdb_sys::MDB_cursor = ptr::null_mut();
        unsafe {
            try_mdb!(lmdb_sys::mdb_cursor_open(txn.handle(), txn.database().handle(), &mut cursor));
        }
        Ok(RwCursor {
            txn: txn,
            cursor: cursor,
            _marker: PhantomData,
        })
    }
}

impl<'a, 'b, D, T> Cursor<'a, 'b, D, T> for RwCursor<'a, D>
    where D: 'a + 'b + Database,
          T: 'a + Transaction<'a, D>
{
    fn close(self) {
        unsafe { lmdb_sys::mdb_cursor_close(self.cursor) }
    }

    fn handle(&self) -> *mut lmdb_sys::MDB_cursor {
        self.cursor
    }

    fn txn(&self) -> &'a mut lmdb_sys::MDB_txn {
        self.txn.handle()
    }

    fn database(&self) -> lmdb_sys::MDB_dbi {
        self.txn.database().handle()
    }

    fn state(&self) -> &TxnState {
        self.txn.state()
    }
}

impl<'a, D: Database> Drop for RwCursor<'a, D> {
    fn drop(&mut self) {
        unsafe { lmdb_sys::mdb_cursor_close(self.cursor) }
    }
}

pub struct MdbStat(lmdb_sys::MDB_stat);

impl MdbStat {
    pub fn new(native: lmdb_sys::MDB_stat) -> Self {
        MdbStat(native)
    }

    /// Number of internal (non-leaf) pages
    pub fn branch_pages(&self) -> usize {
        self.0.ms_branch_pages
    }

    /// Number of data items
    pub fn entries(&self) -> usize {
        self.0.ms_entries
    }

    /// Number of leaf pages
    pub fn leaf_pages(&self) -> usize {
        self.0.ms_leaf_pages
    }

    /// Number of overflow pages
    pub fn overflow_pages(&self) -> usize {
        self.0.ms_overflow_pages
    }
}

pub struct DatabaseBuilder<T: Database> {
    pub name: Option<&'static str>,
    pub flags: DatabaseFlags,
    txn_flags: c_uint,
    _marker: PhantomData<T>,
}

impl<T: Database> DatabaseBuilder<T> {
    /// Create a new database
    pub fn create(mut self, env: Arc<Environment>) -> Result<T> {
        self.flags = self.flags | DB_CREATE;
        let handle = try!(self.open_database(&env));
        T::open(env, handle)
    }

    /// Open an existing database
    pub fn open(mut self, env: Arc<Environment>) -> Result<T> {
        self.flags = self.flags - DB_CREATE;
        let handle = try!(self.open_database(&env));
        T::open(env, handle)
    }

    /// Configures the database to be a named database
    pub fn named(&mut self, name: &'static str) -> &mut Self {
        self.name = Some(name);
        self
    }

    /// Set database flags
    pub fn flags(&mut self, flags: DatabaseFlags) -> &mut Self {
        self.flags = flags;
        self
    }

    /// Set the database to be read-only
    pub fn readonly(&mut self) -> &mut Self {
        self.txn_flags = lmdb_sys::MDB_RDONLY;
        self
    }

    /// Set the database to be writeable
    pub fn writable(&mut self) -> &mut Self {
        self.txn_flags = 0;
        self
    }

    fn open_database(&self, env: &Environment) -> Result<lmdb_sys::MDB_dbi> {
        match OPEN_LOCK.lock() {
            Ok(_) => {
                let name_ptr = if self.name.is_some() {
                    try!(CString::new(self.name.as_ref().unwrap().as_bytes())).as_ptr()
                } else {
                    ptr::null()
                };
                let mut handle: lmdb_sys::MDB_dbi = 0;
                let txn = try!(create_txn(env, self.txn_flags, None));
                unsafe {
                    match handle_mdb!(lmdb_sys::mdb_dbi_open(txn,
                                                             name_ptr,
                                                             self.flags.bits(),
                                                             &mut handle)) {
                        Ok(()) => {
                            try_mdb!(lmdb_sys::mdb_txn_commit(txn));
                            Ok(handle)
                        }
                        Err(e) => {
                            lmdb_sys::mdb_txn_abort(txn);
                            return Err(e);
                        }
                    }
                }
            }
            Err(e) => panic!("Internal data access error: {:?}", e),
        }
    }
}

/// Common behaviour for databases
pub trait Database : Sized {
    type Object: DataObject + Any;

    /// Open the database
    fn open(env: Arc<Environment>, handle: lmdb_sys::MDB_dbi) -> Result<Self>;

    /// Returns the name of the database if one was provided
    fn name() -> &'static str;

    /// Drop all entries from the database
    fn clear<'a, T: Transaction<'a, Self>>(&'a self, txn: &'a T) -> Result<()> {
        txn.clear()
    }

    /// Write an object into the database
    fn write<'a>(&self, txn: &'a RwTransaction<'a, Self>, object: &Self::Object) -> Result<()> {
        txn.put(object.ident(), object)
    }

    /// Returns a reference to the database's environment
    fn env(&self) -> &Environment;

    /// Returns the database handle
    fn handle(&self) -> lmdb_sys::MDB_dbi;

    /// Retrieve statistics for the database
    fn stat<'a, T: Transaction<'a, Self>>(&'a self, txn: &T) -> Result<MdbStat> {
        unsafe {
            let mut native: lmdb_sys::MDB_stat = mem::zeroed();
            try_mdb!(lmdb_sys::mdb_stat(txn.handle(), self.handle(), &mut native));
            Ok(MdbStat::new(native))
        }
    }

    /// Begin a read-only transaction
    fn txn_ro<'b>(&'b self) -> Result<RoTransaction<'b, Self>> {
        RoTransaction::begin(self)
    }

    /// Begin a read-write transaction
    fn txn_rw<'b>(&'b self) -> Result<RwTransaction<'b, Self>> {
        RwTransaction::begin(self)
    }
}

/// Common behaviour for transactions
pub trait Transaction<'a, D: 'a + Database> : Sized {
    /// Begin a transaction
    fn begin(database: &'a D) -> Result<Self>;

    /// Abandon all the operations of the transaction instead of saving them.
    fn abort(self) -> ();

    /// Commit all the operations of a transaction into the database.
    fn commit(self) -> Result<()>;

    /// Return a reference to the transaction's database
    fn database(&self) -> &'a D;

    /// Return a reference of the transaction's handle
    fn handle(&self) -> &mut lmdb_sys::MDB_txn;

    /// Return a reference of the transaction's current state
    fn state(&self) -> &TxnState;

    /// Returns a read-only cursor
    fn cursor_ro(&'a self) -> Result<RoCursor<'a, D, Self>> {
        RoCursor::open(self)
    }

    /// Begins a read-only nested transaction within the current transaction into the given
    /// database.
    fn new_child<D2: Database>(&'a self, database: &'a D2) -> Result<RoTransaction<'a, D2>> {
        let handle = try!(create_txn(database.env(), lmdb_sys::MDB_RDONLY, Some(self.handle())));
        Ok(RoTransaction {
            database: database,
            parent: Some(self.handle()),
            handle: handle,
            state: TxnState::default(),
        })
    }

    /// Begins a read-write nested transaction within the current transaction into the given
    /// database.
    fn new_child_rw<D2: Database>(&'a self, database: &'a D2) -> Result<RwTransaction<'a, D2>> {
        let handle = try!(create_txn(database.env(), 0, Some(self.handle())));
        Ok(RwTransaction {
            database: database,
            parent: Some(self.handle()),
            handle: handle,
            state: TxnState::default(),
        })
    }

    /// Clear all data in the database.
    fn clear(&self) -> Result<()> {
        assert_txn_state_eq!(self.state(), TxnState::Normal);
        unsafe {
            try_mdb!(lmdb_sys::mdb_drop(self.handle(), self.database().handle(), 0));
        }
        Ok(())
    }

    /// Return a value from the database.
    fn get(&self, k: &<D::Object as DataObject>::Key) -> Result<D::Object> {
        assert_txn_state_eq!(self.state(), TxnState::Normal);
        unsafe {
            let mut key = k.to_mdb_value();
            let mut data = encoded_val_for::<()>(None);
            try_mdb!(lmdb_sys::mdb_get(self.handle() as *mut lmdb_sys::MDB_txn,
                                       self.database().handle(),
                                       &mut key,
                                       &mut data));
            let bytes: &[u8] = slice::from_raw_parts(data.mv_data as *const u8, data.mv_size);
            match decode(bytes) {
                Ok(value) => Ok(value),
                Err(e) => Err(Error::from(MdbError::from(e))),
            }
        }
    }
}

pub struct RoTransaction<'a, D: 'a + Database> {
    database: &'a D,
    #[allow(dead_code)]
    parent: Option<&'a mut lmdb_sys::MDB_txn>,
    handle: *mut lmdb_sys::MDB_txn,
    state: TxnState,
}

impl<'a, D: Database> RoTransaction<'a, D> {
    /// Like `abort()`, but doesn't consume the transaction, keeping it alive for later use.
    #[allow(dead_code)]
    fn reset(&mut self) {
        if *self.state() != TxnState::Normal {
            return ();
        }
        unsafe {
            lmdb_sys::mdb_txn_reset(self.handle());
        }
        self.state = TxnState::Released;
    }

    /// Acquires a new reader lock for a transaction that had been previously released by `renew()`.
    #[allow(dead_code)]
    fn renew(&mut self) -> Result<()> {
        assert_txn_state_eq!(self.state(), TxnState::Released);
        unsafe {
            try_mdb!(lmdb_sys::mdb_txn_renew(self.handle()));
        }
        self.state = TxnState::Normal;
        Ok(())
    }
}

impl<'a, D: 'a + Database> Transaction<'a, D> for RoTransaction<'a, D> {
    fn begin(database: &'a D) -> Result<Self> {
        let handle = try!(create_txn(database.env(), lmdb_sys::MDB_RDONLY, None));
        Ok(RoTransaction {
            database: database,
            parent: None,
            handle: handle,
            state: TxnState::default(),
        })
    }

    fn abort(self) {
        if *self.state() != TxnState::Normal {
            return ();
        }
        unsafe {
            lmdb_sys::mdb_txn_abort(self.handle());
        }
    }

    fn commit(mut self) -> Result<()> {
        assert_txn_state_eq!(self.state(), TxnState::Normal);
        unsafe {
            try_mdb!(lmdb_sys::mdb_txn_commit(self.handle()));
        }
        self.state = TxnState::Released;
        Ok(())
    }

    fn database(&self) -> &'a D {
        self.database
    }

    fn handle(&self) -> &mut lmdb_sys::MDB_txn {
        unsafe { &mut *self.handle }
    }

    fn state(&self) -> &TxnState {
        &self.state
    }
}

impl<'a, D: 'a + Database> Drop for RoTransaction<'a, D> {
    fn drop(&mut self) {
        if self.state == TxnState::Normal {
            unsafe {
                lmdb_sys::mdb_txn_commit(self.handle);
            }
            self.state = TxnState::Released;
        }
    }
}

pub struct RwTransaction<'a, D: 'a + Database> {
    database: &'a D,
    #[allow(dead_code)]
    parent: Option<&'a mut lmdb_sys::MDB_txn>,
    handle: *mut lmdb_sys::MDB_txn,
    state: TxnState,
}

impl<'a, D: Database> RwTransaction<'a, D> {
    /// Returns a read-write cursor
    pub fn cursor_rw(&'a self) -> Result<RwCursor<'a, D>> {
        RwCursor::open(self)
    }

    /// Write a key/value into the database.
    ///
    /// The default behaviour is to enter a new key/data pair, replacing any previously existing
    /// key if duplicates are not allowed, or adding a new duplicate data item if duplicates are
    /// allowed.
    ///
    /// Duplicates can be allowed by setting the `DB_ALLOW_DUPS` flag on the database.
    pub fn put(&self, key: &<D::Object as DataObject>::Key, value: &D::Object) -> Result<()> {
        assert_txn_state_eq!(self.state(), TxnState::Normal);
        // JW TODO: these flags represent different types of "writes" and are dependent upon the
        // flags used to open the database. This would mean that this `put` function is the
        // foundation for writing data into the database and functions like `write`, `update`, and
        // `append` are the public API hanging off of the database itself.
        let flags = 0;
        unsafe {
            let mut kval = key.to_mdb_value();
            let mut dval = encoded_val_for::<D::Object>(Some(value));
            try_mdb!(lmdb_sys::mdb_put(self.handle,
                                       self.database.handle(),
                                       &mut kval,
                                       &mut dval,
                                       flags));
        }
        Ok(())
    }

    /// Delete items from the database.
    ///
    /// If the database does not support duplicate data items the value argument is ignored. If
    /// the database supports duplicate data items and the value argument is `None`, all of the
    /// duplicate data items for the key are deleted. Otherwise, if the data option is present only
    /// the matching data item will be deleted.
    ///
    /// # Failures
    ///
    /// * MdbError::NotFound if the specified key/data pair is not in the database
    pub fn delete(&self,
                  key: &<D::Object as DataObject>::Key,
                  value: Option<&D::Object>)
                  -> Result<()> {
        unsafe {
            let mut kval = key.to_mdb_value();
            let dval: *mut lmdb_sys::MDB_val = {
                if value.is_some() {
                    &mut encoded_val_for::<D::Object>(value)
                } else {
                    ptr::null_mut()
                }
            };
            try_mdb!(lmdb_sys::mdb_del(self.handle, self.database.handle(), &mut kval, dval));
        }
        Ok(())
    }
}

impl<'a, D: 'a + Database> Transaction<'a, D> for RwTransaction<'a, D> {
    fn begin(database: &'a D) -> Result<Self> {
        let handle = try!(create_txn(database.env(), 0, None));
        Ok(RwTransaction {
            database: database,
            parent: None,
            handle: handle,
            state: TxnState::default(),
        })
    }

    fn abort(self) {
        if *self.state() != TxnState::Normal {
            return ();
        }
        unsafe {
            lmdb_sys::mdb_txn_abort(self.handle());
        }
    }

    fn commit(mut self) -> Result<()> {
        assert_txn_state_eq!(self.state(), TxnState::Normal);
        unsafe {
            try_mdb!(lmdb_sys::mdb_txn_commit(self.handle()));
        }
        self.state = TxnState::Invalid;
        Ok(())
    }

    fn database(&self) -> &'a D {
        self.database
    }

    fn handle(&self) -> &mut lmdb_sys::MDB_txn {
        unsafe { &mut *self.handle }
    }

    fn state(&self) -> &TxnState {
        &self.state
    }
}

impl<'a, D: 'a + Database> Drop for RwTransaction<'a, D> {
    fn drop(&mut self) {
        if self.state == TxnState::Normal {
            unsafe {
                lmdb_sys::mdb_txn_commit(self.handle);
            }
            self.state = TxnState::Invalid;
        }
    }
}

/// Contains metadata entries for each package known by the Depot
pub struct PkgDatabase {
    pub index: PkgIndex,
    env: Arc<Environment>,
    handle: lmdb_sys::MDB_dbi,
}

impl PkgDatabase {
    pub fn new() -> DatabaseBuilder<Self> {
        DatabaseBuilder::default()
    }
}

impl Database for PkgDatabase {
    type Object = data_object::Package;

    fn open(env: Arc<Environment>, handle: lmdb_sys::MDB_dbi) -> Result<Self> {
        let env2 = env.clone();
        let index = try!(PkgIndex::new().create(env2));
        Ok(PkgDatabase {
            env: env,
            handle: handle,
            index: index,
        })
    }

    fn name() -> &'static str {
        PACKAGE_DB
    }

    fn clear<'a, T: Transaction<'a, Self>>(&'a self, txn: &'a T) -> Result<()> {
        try!(txn.clear());
        let nested = try!(txn.new_child_rw(&self.index));
        try!(self.index.clear(&nested));
        try!(nested.commit());
        Ok(())
    }

    fn write<'a>(&self, txn: &RwTransaction<'a, Self>, object: &Self::Object) -> Result<()> {
        try!(txn.put(object.ident(), object));
        let nested = try!(txn.new_child_rw(&self.index));
        try!(self.index.write(&nested, &object.ident));
        try!(nested.commit());
        Ok(())
    }

    fn env(&self) -> &Environment {
        &self.env
    }

    fn handle(&self) -> lmdb_sys::MDB_dbi {
        self.handle
    }
}

impl Default for DatabaseBuilder<PkgDatabase> {
    fn default() -> DatabaseBuilder<PkgDatabase> {
        DatabaseBuilder {
            name: Some(PkgDatabase::name()),
            flags: DatabaseFlags::empty(),
            txn_flags: 0,
            _marker: PhantomData,
        }
    }
}

impl Drop for PkgDatabase {
    fn drop(&mut self) {
        unsafe { lmdb_sys::mdb_dbi_close(self.env.handle, self.handle()) }
    }
}

/// Contains an index of package identifiers to easily find the latest version/release of a
/// specified package.
pub struct PkgIndex {
    env: Arc<Environment>,
    handle: lmdb_sys::MDB_dbi,
}

impl PkgIndex {
    pub fn new() -> DatabaseBuilder<Self> {
        DatabaseBuilder::default()
    }
}

impl Database for PkgIndex {
    type Object = data_object::PackageIdent;

    fn open(env: Arc<Environment>, handle: lmdb_sys::MDB_dbi) -> Result<Self> {
        Ok(PkgIndex {
            env: env,
            handle: handle,
        })
    }

    fn name() -> &'static str {
        PACKAGE_INDEX
    }

    fn env(&self) -> &Environment {
        &self.env
    }

    fn handle(&self) -> lmdb_sys::MDB_dbi {
        self.handle
    }

    fn write<'a>(&self, txn: &RwTransaction<'a, Self>, object: &Self::Object) -> Result<()> {
        try!(txn.put(&object.origin_idx(), object));
        try!(txn.put(&object.name_idx(), object));
        try!(txn.put(object.version_idx().as_ref().unwrap(), object));
        Ok(())
    }
}

impl Default for DatabaseBuilder<PkgIndex> {
    fn default() -> DatabaseBuilder<PkgIndex> {
        let mut flags = DatabaseFlags::empty();
        flags.toggle(DB_ALLOW_DUPS);
        DatabaseBuilder {
            name: Some(PkgIndex::name()),
            flags: flags,
            txn_flags: 0,
            _marker: PhantomData,
        }
    }
}

impl Drop for PkgIndex {
    fn drop(&mut self) {
        unsafe { lmdb_sys::mdb_dbi_close(self.env.handle, self.handle()) }
    }
}

/// Contains a mapping of view names and the packages found within that view.
///
/// This is how packages will be "promoted" between environments without duplicating data on disk.
pub struct ViewDatabase {
    pub pkg_view_idx: PkgViewIndex,
    pub view_pkg_idx: ViewPkgIndex,
    env: Arc<Environment>,
    handle: lmdb_sys::MDB_dbi,
}

impl ViewDatabase {
    pub fn new() -> DatabaseBuilder<Self> {
        DatabaseBuilder::default()
    }

    // Associate the given package to the given view
    pub fn associate<'a, T: Database>(&self, txn: &RwTransaction<'a, T>, view: &<Self as Database>::Object, pkg: &<PkgDatabase as Database>::Object) -> Result<()> {
        let nested = try!(txn.new_child_rw(&self.pkg_view_idx));
        try!(nested.put(pkg.ident(), view));
        let nested2 = try!(nested.new_child_rw(&self.view_pkg_idx));
        try!(nested2.put(view.ident(), &pkg.ident));
        Ok(())
    }
}

impl Database for ViewDatabase {
    type Object = data_object::View;

    fn open(env: Arc<Environment>, handle: lmdb_sys::MDB_dbi) -> Result<Self> {
        let env2 = env.clone();
        let env3 = env.clone();
        let pkg_view_idx = try!(PkgViewIndex::new().create(env2));
        let view_pkg_idx = try!(ViewPkgIndex::new().create(env3));
        Ok(ViewDatabase {
            env: env,
            handle: handle,
            pkg_view_idx: pkg_view_idx,
            view_pkg_idx: view_pkg_idx,
        })
    }

    fn name() -> &'static str {
        VIEW_DB
    }

    fn env(&self) -> &Environment {
        &self.env
    }

    fn handle(&self) -> lmdb_sys::MDB_dbi {
        self.handle
    }
}

impl Default for DatabaseBuilder<ViewDatabase> {
    fn default() -> DatabaseBuilder<ViewDatabase> {
        DatabaseBuilder {
            name: Some(ViewDatabase::name()),
            flags: DatabaseFlags::empty(),
            txn_flags: 0,
            _marker: PhantomData,
        }
    }
}

impl Drop for ViewDatabase {
    fn drop(&mut self) {
        unsafe { lmdb_sys::mdb_dbi_close(self.env.handle, self.handle()) }
    }
}

pub struct PkgViewIndex {
    env: Arc<Environment>,
    handle: lmdb_sys::MDB_dbi,
}

impl PkgViewIndex {
    pub fn new() -> DatabaseBuilder<Self> {
        DatabaseBuilder::default()
    }
}

impl Database for PkgViewIndex {
    type Object = data_object::View;

    fn open(env: Arc<Environment>, handle: lmdb_sys::MDB_dbi) -> Result<Self> {
        Ok(PkgViewIndex {
            env: env,
            handle: handle,
        })
    }

    fn name() -> &'static str {
        PACKAGE_VIEW_INDEX
    }

    fn env(&self) -> &Environment {
        &self.env
    }

    fn handle(&self) -> lmdb_sys::MDB_dbi {
        self.handle
    }
}

impl Default for DatabaseBuilder<PkgViewIndex> {
    fn default() -> DatabaseBuilder<PkgViewIndex> {
        let mut flags = DatabaseFlags::empty();
        flags.toggle(DB_ALLOW_DUPS);
        DatabaseBuilder {
            name: Some(PkgViewIndex::name()),
            flags: flags,
            txn_flags: 0,
            _marker: PhantomData,
        }
    }
}

impl Drop for PkgViewIndex {
    fn drop(&mut self) {
        unsafe { lmdb_sys::mdb_dbi_close(self.env.handle, self.handle()) }
    }
}

pub struct ViewPkgIndex {
    env: Arc<Environment>,
    handle: lmdb_sys::MDB_dbi,
}

impl ViewPkgIndex {
    pub fn new() -> DatabaseBuilder<Self> {
        DatabaseBuilder::default()
    }
}

impl Database for ViewPkgIndex {
    type Object = data_object::PackageIdent;

    fn name() -> &'static str {
        VIEW_PACKAGE_INDEX
    }

    fn open(env: Arc<Environment>, handle: lmdb_sys::MDB_dbi) -> Result<Self> {
        Ok(ViewPkgIndex {
            env: env,
            handle: handle,
        })
    }

    fn env(&self) -> &Environment {
        &self.env
    }

    fn handle(&self) -> lmdb_sys::MDB_dbi {
        self.handle
    }
}

impl Default for DatabaseBuilder<ViewPkgIndex> {
    fn default() -> DatabaseBuilder<ViewPkgIndex> {
        let mut flags = DatabaseFlags::empty();
        flags.toggle(DB_ALLOW_DUPS);
        DatabaseBuilder {
            name: Some(ViewPkgIndex::name()),
            flags: flags,
            txn_flags: 0,
            _marker: PhantomData,
        }
    }
}

impl Drop for ViewPkgIndex {
    fn drop(&mut self) {
        unsafe { lmdb_sys::mdb_dbi_close(self.env.handle, self.handle()) }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;
    use error::Error;
    use depot_core::data_object::*;
    use hcore::package;

    // JW TODO: This test is ignored while I track down a bug preventing multiple transactions
    // being opened from different threads.
    #[test]
    #[ignore]
    fn read_write_composite_data_object() {
        let ds = open_datastore();
        let key: package::PackageIdent = package::PackageIdent::new("chef",
                                                                    "redis",
                                                                    Some("3.0.1"),
                                                                    Some("1234"));
        {
            let pkg = Package {
                ident: PackageIdent::new(key.clone()),
                manifest: "my-manifest".to_string(),
                deps: vec![],
                tdeps: vec![],
                exposes: vec![],
                config: Some("configuration".to_string()),
                checksum: "f61d212ed22121dbf443f03f0f3aa577".to_string(),
            };
            let txn = ds.packages.txn_rw().unwrap();
            txn.put(&pkg.ident(), &pkg).unwrap();
            txn.commit().unwrap();
        }
        let txn = ds.packages.txn_ro().unwrap();
        let saved = txn.get(&key.to_string()).unwrap();
        txn.abort();
        assert_eq!(saved.ident(), &key.to_string());
        assert_eq!(saved.manifest, "my-manifest");
        assert_eq!(saved.config, Some("configuration".to_string()));
    }

    // JW TODO: This test is ignored while I track down a bug preventing multiple transactions
    // being opened from different threads.
    #[test]
    #[ignore]
    fn transaction_read_write() {
        let ds = open_datastore();
        {
            let view = View::new("my-view");
            let txn = ds.views.txn_rw().unwrap();
            txn.put(view.ident(), &view).unwrap();
            txn.commit().unwrap();
        };
        let txn = ds.views.txn_ro().unwrap();
        let saved = txn.get(&"my-view".to_string()).unwrap();
        txn.abort();
        assert_eq!(saved.ident(), "my-view");
    }

    // JW TODO: This test is ignored while I track down a bug preventing multiple transactions
    // being opened from different threads.
    #[test]
    #[ignore]
    fn transaction_delete() {
        let ds = open_datastore();
        {
            let view = View::new("my-view");
            let txn = ds.views.txn_rw().unwrap();
            txn.put(view.ident(), &view).unwrap();
            txn.commit().unwrap();
        };
        let txn = ds.views.txn_rw().unwrap();
        txn.delete(&"my-view".to_string(), None).unwrap();
        match txn.get(&"my-view".to_string()) {
            Err(Error::MdbError(MdbError::NotFound)) => {
                txn.abort();
                assert!(true)
            }
            _ => {
                txn.abort();
                assert!(false)
            }
        }
    }

    fn open_datastore() -> DataStore {
        let ds = DataStore::open(Path::new("/opt/bldr/test")).unwrap();
        ds.clear().unwrap();
        ds
    }
}
