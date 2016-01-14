use std::default::Default;
use std::ffi::{CStr, CString};
use std::path::PathBuf;
use std::str;

use libarchive3_sys::ffi;
use error::{ArchiveResult, ErrCode};

pub enum ReadCompression {
    All,
    Bzip2,
    Compress,
    Gzip,
    Lzip,
    Lzma,
    None,
    Program(String),
    Rpm,
    Uu,
    Xz,
}

pub enum ReadFormat {
    SevenZip,
    All,
    Ar,
    Cab,
    Cpio,
    Empty,
    Gnutar,
    Iso9660,
    Lha,
    Mtree,
    Rar,
    Raw,
    Tar,
    Xar,
    Zip,
}

pub enum ReadFilter {
    All,
    Bzip2,
    Compress,
    Gzip,
    Grzip,
    Lrzip,
    Lzip,
    Lzma,
    Lzop,
    None,
    Program(String),
    ProgramSignature(String, Option<extern "C" fn() -> ()>, usize),
    Rpm,
    Uu,
    Xz,
}

pub enum WriteFormat {
    SevenZip,
    ArBsd,
    ArSvr4,
    Cpio,
    CpioNewc,
    Gnutar,
    Iso9660,
    Mtree,
    MtreeClassic,
    Pax,
    PaxRestricted,
    Shar,
    SharDump,
    Ustar,
    V7tar,
    Xar,
    Zip,
}

pub enum WriteFilter {
    B64Encode,
    Bzip2,
    Compress,
    Grzip,
    Gzip,
    Lrzip,
    Lzip,
    Lzma,
    Lzop,
    None,
    Program(String),
    UuEncode,
    Xz,
}

pub trait Handle {
    unsafe fn handle(&self) -> *mut ffi::Struct_archive;

    fn err_code(&self) -> ErrCode {
        let code = unsafe { ffi::archive_errno(self.handle()) };
        ErrCode(code)
    }

    fn err_msg(&self) -> String {
        unsafe {
            let c_str = CStr::from_ptr(ffi::archive_error_string(self.handle()));
            let buf = c_str.to_bytes();
            String::from(str::from_utf8(buf).unwrap())
        }
    }
}

pub trait Entry {
    unsafe fn entry(&self) -> *mut ffi::Struct_archive_entry;

    fn pathname(&self) -> &str {
        let c_str: &CStr = unsafe { CStr::from_ptr(ffi::archive_entry_pathname(self.entry())) };
        let buf: &[u8] = c_str.to_bytes();
        str::from_utf8(buf).unwrap()
    }

    fn size(&self) -> i64 {
        unsafe { ffi::archive_entry_size(self.entry()) }
    }

    fn set_pathname(&mut self, path: PathBuf) -> ArchiveResult<()> {
        unsafe {
            let c_str = CString::new(path.to_str().unwrap()).unwrap();
            ffi::archive_entry_set_pathname(self.entry(), c_str.as_ptr());
        }
        Ok(())
    }
}

pub enum ExtractOption {
    // The user and group IDs should be set on the restored file. By default, the user and group
    // IDs are not restored.
    Owner,
    // Full permissions (including SGID, SUID, and sticky bits) should be restored exactly as
    // specified, without obeying the current umask. Note that SUID and SGID bits can only be
    // restored if the user and group ID of the object on disk are correct. If
    // `ExtractOption::Owner` is not specified, then SUID and SGID bits will only be restored if
    // the default user and group IDs of newly-created objects on disk happen to match those
    // specified in the archive entry. By default, only basic permissions are restored, and umask
    // is obeyed.
    Permissions,
    // The timestamps (mtime, ctime, and atime) should be restored. By default, they are ignored.
    // Note that restoring of atime is not currently supported.
    Time,
    // Existing files on disk will not be overwritten. By default, existing regular files are
    // truncated and overwritten; existing directories will have their permissions updated; other
    // pre-existing objects are unlinked and recreated from scratch.
    NoOverwrite,
    // Existing files on disk will be unlinked before any attempt to create them. In some cases,
    // this can prove to be a significant performance improvement. By default, existing files are
    // truncated and rewritten, but the file is not recreated. In particular, the default behavior
    // does not break existing hard links.
    Unlink,
    // Attempt to restore ACLs. By default, extended ACLs are ignored.
    ACL,
    // Attempt to restore extended file flags. By default, file flags are ignored.
    FFlags,
    // Attempt to restore POSIX.1e extended attributes. By default, they are ignored.
    XAttr,
    // Refuse to extract any object whose final location would be altered by a symlink on disk.
    // This is intended to help guard against a variety of mischief caused by archives that
    // (deliberately or otherwise) extract files outside of the current directory. The default is
    // not to perform this check. If ARCHIVE_EXTRACT_UNLINK is specified together with this option,
    // the library will remove any intermediate symlinks it finds and return an error only if such
    // symlink could not be removed.
    SecureSymlinks,
    // Refuse to extract a path that contains a `..` element anywhere within it. The default is to
    // not refuse such paths. Note that paths ending in `..` always cause an error, regardless of
    // this flag.
    SecureNoDotDot,
    // Default: Create parent directories as needed
    NoAutoDir,
    // Default: Overwrite files, even if one on disk is newer
    NoOverwriteNewer,
    // Scan data for blocks of NUL bytes and try to recreate them with holes. This results in
    // sparse files, independent of whether the archive format supports or uses them.
    Sparse,
    // Default: Do not restore Mac extended metadata
    // This has no effect except on Mac OS
    MacMetadata,
    // Default: Use HFS+ compression if it was compressed
    // This has no effect except on Mac OS v10.6 or later
    NoHFSCompression,
    // Default: Do not use HFS+ compression if it was not compressed
    // This has no effect except on Mac OS v10.6 or later
    HFSCompressionForced,
    // Default: Do not reject entries with absolute paths */
    SecureNoAbsolutePaths,
    // Default: Do not clear no-change flags when unlinking object */
    ClearNoChangeFFlags,
}

pub struct ExtractOptions {
    pub flags: i32,
}

impl ExtractOptions {
    pub fn new() -> Self {
        ExtractOptions::default()
    }

    pub fn add(&mut self, opt: ExtractOption) -> &mut Self {
        let flag = match opt {
            ExtractOption::Owner => ffi::ARCHIVE_EXTRACT_OWNER,
            ExtractOption::Permissions => ffi::ARCHIVE_EXTRACT_PERM,
            ExtractOption::Time => ffi::ARCHIVE_EXTRACT_TIME,
            ExtractOption::NoOverwrite => ffi::ARCHIVE_EXTRACT_NO_OVERWRITE,
            ExtractOption::Unlink => ffi::ARCHIVE_EXTRACT_UNLINK,
            ExtractOption::ACL => ffi::ARCHIVE_EXTRACT_ACL,
            ExtractOption::FFlags => ffi::ARCHIVE_EXTRACT_FFLAGS,
            ExtractOption::XAttr => ffi::ARCHIVE_EXTRACT_XATTR,
            ExtractOption::SecureSymlinks => ffi::ARCHIVE_EXTRACT_SECURE_SYMLINKS,
            ExtractOption::SecureNoDotDot => ffi::ARCHIVE_EXTRACT_SECURE_NODOTDOT,
            ExtractOption::NoAutoDir => ffi::ARCHIVE_EXTRACT_NO_AUTODIR,
            ExtractOption::NoOverwriteNewer => ffi::ARCHIVE_EXTRACT_NO_OVERWRITE_NEWER,
            ExtractOption::Sparse => ffi::ARCHIVE_EXTRACT_SPARSE,
            ExtractOption::MacMetadata => ffi::ARCHIVE_EXTRACT_MAC_METADATA,
            ExtractOption::NoHFSCompression => ffi::ARCHIVE_EXTRACT_NO_HFS_COMPRESSION,
            ExtractOption::HFSCompressionForced => ffi::ARCHIVE_EXTRACT_HFS_COMPRESSION_FORCED,
            ExtractOption::SecureNoAbsolutePaths => ffi::ARCHIVE_EXTRACT_SECURE_NOABSOLUTEPATHS,
            ExtractOption::ClearNoChangeFFlags => ffi::ARCHIVE_EXTRACT_CLEAR_NOCHANGE_FFLAGS,
        };
        self.flags |= flag;
        self
    }
}

impl Default for ExtractOptions {
    fn default() -> ExtractOptions {
        ExtractOptions { flags: 0 }
    }
}
