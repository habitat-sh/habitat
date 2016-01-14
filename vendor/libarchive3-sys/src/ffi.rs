pub use libc::*;

pub const ARCHIVE_EOF: c_int = 1;
pub const ARCHIVE_OK: c_int = 0;
pub const ARCHIVE_RETRY: c_int = -10;
pub const ARCHIVE_WARN: c_int = -20;
pub const ARCHIVE_FAILED: c_int = -25;
pub const ARCHIVE_FATAL: c_int = -30;

pub const ARCHIVE_EXTRACT_OWNER: c_int = 0x0001;
pub const ARCHIVE_EXTRACT_PERM: c_int = 0x0002;
pub const ARCHIVE_EXTRACT_TIME: c_int = 0x0004;
pub const ARCHIVE_EXTRACT_NO_OVERWRITE: c_int = 0x0008;
pub const ARCHIVE_EXTRACT_UNLINK: c_int = 0x0010;
pub const ARCHIVE_EXTRACT_ACL: c_int = 0x0020;
pub const ARCHIVE_EXTRACT_FFLAGS: c_int = 0x0040;
pub const ARCHIVE_EXTRACT_XATTR: c_int = 0x0080;
pub const ARCHIVE_EXTRACT_SECURE_SYMLINKS: c_int = 0x0100;
pub const ARCHIVE_EXTRACT_SECURE_NODOTDOT: c_int = 0x0200;
pub const ARCHIVE_EXTRACT_NO_AUTODIR: c_int = 0x0400;
pub const ARCHIVE_EXTRACT_NO_OVERWRITE_NEWER: c_int = 0x0800;
pub const ARCHIVE_EXTRACT_SPARSE: c_int = 0x1000;
pub const ARCHIVE_EXTRACT_MAC_METADATA: c_int = 0x2000;
pub const ARCHIVE_EXTRACT_NO_HFS_COMPRESSION: c_int = 0x4000;
pub const ARCHIVE_EXTRACT_HFS_COMPRESSION_FORCED: c_int = 0x8000;
pub const ARCHIVE_EXTRACT_SECURE_NOABSOLUTEPATHS: c_int = 0x10000;
pub const ARCHIVE_EXTRACT_CLEAR_NOCHANGE_FFLAGS: c_int = 0x20000;

pub const AE_IFBLK: c_uint = 0o060000;
pub const AE_IFCHR: c_uint = 0o020000;
pub const AE_IFDIR: c_uint = 0o040000;
pub const AE_IFIFO: c_uint = 0o010000;
pub const AE_IFLNK: c_uint = 0o120000;
pub const AE_IFMT: c_uint = 0o170000;
pub const AE_IFREG: c_uint = 0o100000;
pub const AE_IFSOCK: c_uint = 0o140000;

pub enum Struct_archive { }
pub enum Struct_archive_entry { }
pub enum Struct_archive_acl { }
pub enum Struct_archive_entry_linkresolver { }

pub type archive_read_callback = unsafe extern "C" fn(arg1: *mut Struct_archive,
                                                      _client_data: *mut c_void,
                                                      _buffer: *mut *const c_void)
                                                      -> ssize_t;
pub type archive_skip_callback = unsafe extern "C" fn(arg1: *mut Struct_archive,
                                                      _client_data: *mut c_void,
                                                      request: int64_t)
                                                      -> int64_t;
pub type archive_seek_callback = unsafe extern "C" fn(arg1: *mut Struct_archive,
                                                      _client_data: *mut c_void,
                                                      offset: int64_t,
                                                      whence: c_int)
                                                      -> int64_t;
pub type archive_write_callback = unsafe extern "C" fn(arg1: *mut Struct_archive,
                                                       _client_data: *mut c_void,
                                                       _buffer: *const c_void,
                                                       _length: size_t)
                                                       -> ssize_t;
pub type archive_open_callback = unsafe extern "C" fn(arg1: *mut Struct_archive,
                                                      _client_data: *mut c_void)
                                                      -> c_int;
pub type archive_close_callback = unsafe extern "C" fn(arg1: *mut Struct_archive,
                                                       _client_data: *mut c_void)
                                                       -> c_int;
pub type archive_switch_callback = unsafe extern "C" fn(arg1: *mut Struct_archive,
                                                        _client_data1: *mut c_void,
                                                        _client_data2: *mut c_void)
                                                        -> c_int;

#[link(name = "archive")]
extern "C" {
    pub fn archive_version_number() -> c_int;
    pub fn archive_version_string() -> *const c_char;
    pub fn archive_read_new() -> *mut Struct_archive;
    pub fn archive_read_support_compression_all(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_compression_bzip2(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_compression_compress(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_compression_gzip(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_compression_lzip(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_compression_lzma(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_compression_none(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_compression_program(arg1: *mut Struct_archive,
                                                    command: *const c_char)
                                                    -> c_int;
    pub fn archive_read_support_compression_program_signature(arg1: *mut Struct_archive,
                                                              arg2: *const c_char,
                                                              arg3: *const c_void,
                                                              arg4: size_t)
                                                              -> c_int;
    pub fn archive_read_support_compression_rpm(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_compression_uu(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_compression_xz(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_filter_all(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_filter_bzip2(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_filter_compress(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_filter_gzip(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_filter_grzip(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_filter_lrzip(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_filter_lzip(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_filter_lzma(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_filter_lzop(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_filter_none(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_filter_program(arg1: *mut Struct_archive,
                                               command: *const c_char)
                                               -> c_int;
    pub fn archive_read_support_filter_program_signature(arg1: *mut Struct_archive,
                                                         arg2: *const c_char,
                                                         arg3: *const c_void,
                                                         arg4: size_t)
                                                         -> c_int;
    pub fn archive_read_support_filter_rpm(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_filter_uu(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_filter_xz(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_format_7zip(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_format_all(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_format_ar(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_format_by_code(arg1: *mut Struct_archive, arg2: c_int) -> c_int;
    pub fn archive_read_support_format_cab(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_format_cpio(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_format_empty(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_format_gnutar(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_format_iso9660(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_format_lha(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_format_mtree(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_format_rar(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_format_raw(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_format_tar(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_format_xar(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_support_format_zip(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_set_format(arg1: *mut Struct_archive, arg2: c_int) -> c_int;
    pub fn archive_read_append_filter(arg1: *mut Struct_archive, arg2: c_int) -> c_int;
    pub fn archive_read_append_filter_program(arg1: *mut Struct_archive,
                                              arg2: *const c_char)
                                              -> c_int;
    pub fn archive_read_append_filter_program_signature(arg1: *mut Struct_archive,
                                                        arg2: *const c_char,
                                                        arg3: *const c_void,
                                                        arg4: size_t)
                                                        -> c_int;
    pub fn archive_read_set_open_callback(arg1: *mut Struct_archive,
                                          arg2:
                                              *mut ::std::option::Option<extern "C" fn()
                                                                             ->
                                                                                 c_int>)
     -> c_int;
    pub fn archive_read_set_read_callback(arg1: *mut Struct_archive,
                                          arg2:
                                              *mut ::std::option::Option<extern "C" fn()
                                                                             ->
                                                                                 ssize_t>)
     -> c_int;
    pub fn archive_read_set_seek_callback(arg1: *mut Struct_archive,
                                          arg2:
                                              *mut ::std::option::Option<extern "C" fn()
                                                                             ->
                                                                                 int64_t>)
     -> c_int;
    pub fn archive_read_set_skip_callback(arg1: *mut Struct_archive,
                                          arg2:
                                              *mut ::std::option::Option<extern "C" fn()
                                                                             ->
                                                                                 int64_t>)
     -> c_int;
    pub fn archive_read_set_close_callback(arg1: *mut Struct_archive,
                                           arg2:
                                               *mut ::std::option::Option<extern "C" fn()
                                                                              ->
                                                                                  c_int>)
     -> c_int;
    pub fn archive_read_set_switch_callback(arg1: *mut Struct_archive,
                                            arg2:
                                                *mut ::std::option::Option<extern "C" fn()
                                                                               ->
                                                                                   c_int>)
     -> c_int;
    pub fn archive_read_set_callback_data(arg1: *mut Struct_archive, arg2: *mut c_void) -> c_int;
    pub fn archive_read_set_callback_data2(arg1: *mut Struct_archive,
                                           arg2: *mut c_void,
                                           arg3: c_uint)
                                           -> c_int;
    pub fn archive_read_add_callback_data(arg1: *mut Struct_archive,
                                          arg2: *mut c_void,
                                          arg3: c_uint)
                                          -> c_int;
    pub fn archive_read_append_callback_data(arg1: *mut Struct_archive,
                                             arg2: *mut c_void)
                                             -> c_int;
    pub fn archive_read_prepend_callback_data(arg1: *mut Struct_archive,
                                              arg2: *mut c_void)
                                              -> c_int;
    pub fn archive_read_open1(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_open(arg1: *mut Struct_archive,
                             _client_data: *mut c_void,
                             arg2: ::std::option::Option<archive_open_callback>,
                             arg3: ::std::option::Option<archive_read_callback>,
                             arg4: ::std::option::Option<archive_close_callback>)
                             -> c_int;
    pub fn archive_read_open2(arg1: *mut Struct_archive,
                              _client_data: *mut c_void,
                              arg2: *mut ::std::option::Option<extern "C" fn() -> c_int>,
                              arg3: *mut ::std::option::Option<extern "C" fn() -> ssize_t>,
                              arg4: *mut ::std::option::Option<extern "C" fn() -> int64_t>,
                              arg5: *mut ::std::option::Option<extern "C" fn() -> c_int>)
                              -> c_int;
    pub fn archive_read_open_filename(arg1: *mut Struct_archive,
                                      _filename: *const c_char,
                                      _block_size: size_t)
                                      -> c_int;
    pub fn archive_read_open_filenames(arg1: *mut Struct_archive,
                                       _filenames: *mut *const c_char,
                                       _block_size: size_t)
                                       -> c_int;
    pub fn archive_read_open_filename_w(arg1: *mut Struct_archive,
                                        _filename: *const wchar_t,
                                        _block_size: size_t)
                                        -> c_int;
    pub fn archive_read_open_file(arg1: *mut Struct_archive,
                                  _filename: *const c_char,
                                  _block_size: size_t)
                                  -> c_int;
    pub fn archive_read_open_memory(arg1: *mut Struct_archive,
                                    buff: *mut c_void,
                                    size: size_t)
                                    -> c_int;
    pub fn archive_read_open_memory2(a: *mut Struct_archive,
                                     buff: *mut c_void,
                                     size: size_t,
                                     read_size: size_t)
                                     -> c_int;
    pub fn archive_read_open_fd(arg1: *mut Struct_archive,
                                _fd: c_int,
                                _block_size: size_t)
                                -> c_int;
    pub fn archive_read_open_FILE(arg1: *mut Struct_archive, _file: *mut FILE) -> c_int;
    pub fn archive_read_next_header(arg1: *mut Struct_archive,
                                    arg2: *mut *mut Struct_archive_entry)
                                    -> c_int;
    pub fn archive_read_next_header2(arg1: *mut Struct_archive,
                                     arg2: *mut Struct_archive_entry)
                                     -> c_int;
    pub fn archive_read_header_position(arg1: *mut Struct_archive) -> int64_t;
    pub fn archive_read_data(arg1: *mut Struct_archive,
                             arg2: *mut c_void,
                             arg3: size_t)
                             -> ssize_t;
    pub fn archive_seek_data(arg1: *mut Struct_archive, arg2: int64_t, arg3: c_int) -> int64_t;
    pub fn archive_read_data_block(a: *mut Struct_archive,
                                   buff: *mut *const c_void,
                                   size: *mut size_t,
                                   offset: *mut int64_t)
                                   -> c_int;
    pub fn archive_read_data_skip(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_data_into_fd(arg1: *mut Struct_archive, fd: c_int) -> c_int;
    pub fn archive_read_set_format_option(_a: *mut Struct_archive,
                                          m: *const c_char,
                                          o: *const c_char,
                                          v: *const c_char)
                                          -> c_int;
    pub fn archive_read_set_filter_option(_a: *mut Struct_archive,
                                          m: *const c_char,
                                          o: *const c_char,
                                          v: *const c_char)
                                          -> c_int;
    pub fn archive_read_set_option(_a: *mut Struct_archive,
                                   m: *const c_char,
                                   o: *const c_char,
                                   v: *const c_char)
                                   -> c_int;
    pub fn archive_read_set_options(_a: *mut Struct_archive, opts: *const c_char) -> c_int;
    pub fn archive_read_extract(arg1: *mut Struct_archive,
                                arg2: *mut Struct_archive_entry,
                                flags: c_int)
                                -> c_int;
    pub fn archive_read_extract2(arg1: *mut Struct_archive,
                                 arg2: *mut Struct_archive_entry,
                                 arg3: *mut Struct_archive)
                                 -> c_int;
    pub fn archive_read_extract_set_progress_callback(arg1:
                                                          *mut Struct_archive,
                                                      _progress_func:
                                                          ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                                         *mut c_void)
                                                                                    ->
                                                                                        ()>,
                                                      _user_data:
                                                          *mut c_void)
     -> ();
    pub fn archive_read_extract_set_skip_file(arg1: *mut Struct_archive,
                                              arg2: int64_t,
                                              arg3: int64_t)
                                              -> ();
    pub fn archive_read_close(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_free(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_finish(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_new() -> *mut Struct_archive;
    pub fn archive_write_set_bytes_per_block(arg1: *mut Struct_archive,
                                             bytes_per_block: c_int)
                                             -> c_int;
    pub fn archive_write_get_bytes_per_block(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_bytes_in_last_block(arg1: *mut Struct_archive,
                                                 bytes_in_last_block: c_int)
                                                 -> c_int;
    pub fn archive_write_get_bytes_in_last_block(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_skip_file(arg1: *mut Struct_archive,
                                       arg2: int64_t,
                                       arg3: int64_t)
                                       -> c_int;
    pub fn archive_write_set_compression_bzip2(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_compression_compress(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_compression_gzip(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_compression_lzip(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_compression_lzma(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_compression_none(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_compression_program(arg1: *mut Struct_archive,
                                                 cmd: *const c_char)
                                                 -> c_int;
    pub fn archive_write_set_compression_xz(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_add_filter(arg1: *mut Struct_archive, filter_code: c_int) -> c_int;
    pub fn archive_write_add_filter_by_name(arg1: *mut Struct_archive,
                                            name: *const c_char)
                                            -> c_int;
    pub fn archive_write_add_filter_b64encode(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_add_filter_bzip2(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_add_filter_compress(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_add_filter_grzip(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_add_filter_gzip(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_add_filter_lrzip(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_add_filter_lzip(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_add_filter_lzma(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_add_filter_lzop(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_add_filter_none(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_add_filter_program(arg1: *mut Struct_archive,
                                            cmd: *const c_char)
                                            -> c_int;
    pub fn archive_write_add_filter_uuencode(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_add_filter_xz(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_format(arg1: *mut Struct_archive, format_code: c_int) -> c_int;
    pub fn archive_write_set_format_by_name(arg1: *mut Struct_archive,
                                            name: *const c_char)
                                            -> c_int;
    pub fn archive_write_set_format_7zip(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_format_ar_bsd(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_format_ar_svr4(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_format_cpio(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_format_cpio_newc(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_format_gnutar(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_format_iso9660(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_format_mtree(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_format_mtree_classic(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_format_pax(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_format_pax_restricted(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_format_shar(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_format_shar_dump(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_format_ustar(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_format_v7tar(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_format_xar(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_format_zip(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_zip_set_compression_deflate(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_zip_set_compression_store(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_open(arg1: *mut Struct_archive,
                              arg2: *mut c_void,
                              arg3: *mut ::std::option::Option<extern "C" fn() -> c_int>,
                              arg4: *mut ::std::option::Option<extern "C" fn() -> ssize_t>,
                              arg5: *mut ::std::option::Option<extern "C" fn() -> c_int>)
                              -> c_int;
    pub fn archive_write_open_fd(arg1: *mut Struct_archive, _fd: c_int) -> c_int;
    pub fn archive_write_open_filename(arg1: *mut Struct_archive, _file: *const c_char) -> c_int;
    pub fn archive_write_open_filename_w(arg1: *mut Struct_archive,
                                         _file: *const wchar_t)
                                         -> c_int;
    pub fn archive_write_open_file(arg1: *mut Struct_archive, _file: *const c_char) -> c_int;
    pub fn archive_write_open_FILE(arg1: *mut Struct_archive, arg2: *mut FILE) -> c_int;
    pub fn archive_write_open_memory(arg1: *mut Struct_archive,
                                     _buffer: *mut c_void,
                                     _buffSize: size_t,
                                     _used: *mut size_t)
                                     -> c_int;
    pub fn archive_write_header(arg1: *mut Struct_archive,
                                arg2: *mut Struct_archive_entry)
                                -> c_int;
    pub fn archive_write_data(arg1: *mut Struct_archive,
                              arg2: *const c_void,
                              arg3: size_t)
                              -> ssize_t;
    pub fn archive_write_data_block(arg1: *mut Struct_archive,
                                    arg2: *const c_void,
                                    arg3: size_t,
                                    arg4: int64_t)
                                    -> ssize_t;
    pub fn archive_write_finish_entry(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_close(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_fail(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_free(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_finish(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_set_format_option(_a: *mut Struct_archive,
                                           m: *const c_char,
                                           o: *const c_char,
                                           v: *const c_char)
                                           -> c_int;
    pub fn archive_write_set_filter_option(_a: *mut Struct_archive,
                                           m: *const c_char,
                                           o: *const c_char,
                                           v: *const c_char)
                                           -> c_int;
    pub fn archive_write_set_option(_a: *mut Struct_archive,
                                    m: *const c_char,
                                    o: *const c_char,
                                    v: *const c_char)
                                    -> c_int;
    pub fn archive_write_set_options(_a: *mut Struct_archive, opts: *const c_char) -> c_int;
    pub fn archive_write_disk_new() -> *mut Struct_archive;
    pub fn archive_write_disk_set_skip_file(arg1: *mut Struct_archive,
                                            arg2: int64_t,
                                            arg3: int64_t)
                                            -> c_int;
    pub fn archive_write_disk_set_options(arg1: *mut Struct_archive, flags: c_int) -> c_int;
    pub fn archive_write_disk_set_standard_lookup(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_write_disk_set_group_lookup(arg1: *mut Struct_archive,
                                               arg2: *mut c_void,
                                               arg3:
                                                   ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                                  *mut c_void,
                                                                                              arg2:
                                                                                                  *const c_char,
                                                                                              arg3:
                                                                                                  int64_t)
                                                                             ->
                                                                                 int64_t>,
                                               arg4:
                                                   ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                                  *mut c_void)
                                                                             ->
                                                                                 ()>)
     -> c_int;
    pub fn archive_write_disk_set_user_lookup(arg1: *mut Struct_archive,
                                              arg2: *mut c_void,
                                              arg3:
                                                  ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                                 *mut c_void,
                                                                                             arg2:
                                                                                                 *const c_char,
                                                                                             arg3:
                                                                                                 int64_t)
                                                                            ->
                                                                                int64_t>,
                                              arg4:
                                                  ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                                 *mut c_void)
                                                                            ->
                                                                                ()>)
     -> c_int;
    pub fn archive_write_disk_gid(arg1: *mut Struct_archive,
                                  arg2: *const c_char,
                                  arg3: int64_t)
                                  -> int64_t;
    pub fn archive_write_disk_uid(arg1: *mut Struct_archive,
                                  arg2: *const c_char,
                                  arg3: int64_t)
                                  -> int64_t;
    pub fn archive_read_disk_new() -> *mut Struct_archive;
    pub fn archive_read_disk_set_symlink_logical(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_disk_set_symlink_physical(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_disk_set_symlink_hybrid(arg1: *mut Struct_archive) -> c_int;
    // pub fn archive_read_disk_entry_from_file(arg1: *mut Struct_archive,
    //                                          arg2: *mut Struct_archive_entry,
    //                                          arg3: c_int,
    //                                          arg4: *const Struct_stat)
    //                                          -> c_int;
    pub fn archive_read_disk_gname(arg1: *mut Struct_archive, arg2: int64_t) -> *const c_char;
    pub fn archive_read_disk_uname(arg1: *mut Struct_archive, arg2: int64_t) -> *const c_char;
    pub fn archive_read_disk_set_standard_lookup(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_disk_set_gname_lookup(arg1: *mut Struct_archive,
                                              arg2: *mut c_void,
                                              arg3:
                                                  ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                                 *mut c_void,
                                                                                             arg2:
                                                                                                 int64_t)
                                                                            ->
                                                                                *const c_char>,
                                              arg4:
                                                  ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                                 *mut c_void)
                                                                            ->
                                                                                ()>)
     -> c_int;
    pub fn archive_read_disk_set_uname_lookup(arg1: *mut Struct_archive,
                                              arg2: *mut c_void,
                                              arg3:
                                                  ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                                 *mut c_void,
                                                                                             arg2:
                                                                                                 int64_t)
                                                                            ->
                                                                                *const c_char>,
                                              arg4:
                                                  ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                                 *mut c_void)
                                                                            ->
                                                                                ()>)
     -> c_int;
    pub fn archive_read_disk_open(arg1: *mut Struct_archive, arg2: *const c_char) -> c_int;
    pub fn archive_read_disk_open_w(arg1: *mut Struct_archive, arg2: *const wchar_t) -> c_int;
    pub fn archive_read_disk_descend(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_disk_can_descend(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_disk_current_filesystem(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_disk_current_filesystem_is_synthetic(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_disk_current_filesystem_is_remote(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_disk_set_atime_restored(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_read_disk_set_behavior(arg1: *mut Struct_archive, flags: c_int) -> c_int;
    pub fn archive_read_disk_set_matching(arg1: *mut Struct_archive,
                                          _matching: *mut Struct_archive,
                                          _excluded_func:
                                              ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                             *mut Struct_archive,
                                                                                         arg2:
                                                                                             *mut c_void,
                                                                                         arg3:
                                                                                             *mut Struct_archive_entry)
                                                                        ->
                                                                            ()>,
                                          _client_data: *mut c_void)
     -> c_int;
    pub fn archive_read_disk_set_metadata_filter_callback(arg1:
                                                              *mut Struct_archive,
                                                          _metadata_filter_func:
                                                              ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                                             *mut Struct_archive,
                                                                                                         arg2:
                                                                                                             *mut c_void,
                                                                                                         arg3:
                                                                                                             *mut Struct_archive_entry)
                                                                                        ->
                                                                                            c_int>,
                                                          _client_data:
                                                              *mut c_void)
     -> c_int;
    pub fn archive_filter_count(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_filter_bytes(arg1: *mut Struct_archive, arg2: c_int) -> int64_t;
    pub fn archive_filter_code(arg1: *mut Struct_archive, arg2: c_int) -> c_int;
    pub fn archive_filter_name(arg1: *mut Struct_archive, arg2: c_int) -> *const c_char;
    pub fn archive_position_compressed(arg1: *mut Struct_archive) -> int64_t;
    pub fn archive_position_uncompressed(arg1: *mut Struct_archive) -> int64_t;
    pub fn archive_compression_name(arg1: *mut Struct_archive) -> *const c_char;
    pub fn archive_compression(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_errno(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_error_string(arg1: *mut Struct_archive) -> *const c_char;
    pub fn archive_format_name(arg1: *mut Struct_archive) -> *const c_char;
    pub fn archive_format(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_clear_error(arg1: *mut Struct_archive) -> ();
    pub fn archive_set_error(arg1: *mut Struct_archive,
                             _err: c_int,
                             fmt: *const c_char,
                             ...)
                             -> ();
    pub fn archive_copy_error(dest: *mut Struct_archive, src: *mut Struct_archive) -> ();
    pub fn archive_file_count(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_match_new() -> *mut Struct_archive;
    pub fn archive_match_free(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_match_excluded(arg1: *mut Struct_archive,
                                  arg2: *mut Struct_archive_entry)
                                  -> c_int;
    pub fn archive_match_path_excluded(arg1: *mut Struct_archive,
                                       arg2: *mut Struct_archive_entry)
                                       -> c_int;
    pub fn archive_match_exclude_pattern(arg1: *mut Struct_archive, arg2: *const c_char) -> c_int;
    pub fn archive_match_exclude_pattern_w(arg1: *mut Struct_archive,
                                           arg2: *const wchar_t)
                                           -> c_int;
    pub fn archive_match_exclude_pattern_from_file(arg1: *mut Struct_archive,
                                                   arg2: *const c_char,
                                                   _nullSeparator: c_int)
                                                   -> c_int;
    pub fn archive_match_exclude_pattern_from_file_w(arg1: *mut Struct_archive,
                                                     arg2: *const wchar_t,
                                                     _nullSeparator: c_int)
                                                     -> c_int;
    pub fn archive_match_include_pattern(arg1: *mut Struct_archive, arg2: *const c_char) -> c_int;
    pub fn archive_match_include_pattern_w(arg1: *mut Struct_archive,
                                           arg2: *const wchar_t)
                                           -> c_int;
    pub fn archive_match_include_pattern_from_file(arg1: *mut Struct_archive,
                                                   arg2: *const c_char,
                                                   _nullSeparator: c_int)
                                                   -> c_int;
    pub fn archive_match_include_pattern_from_file_w(arg1: *mut Struct_archive,
                                                     arg2: *const wchar_t,
                                                     _nullSeparator: c_int)
                                                     -> c_int;
    pub fn archive_match_path_unmatched_inclusions(arg1: *mut Struct_archive) -> c_int;
    pub fn archive_match_path_unmatched_inclusions_next(arg1: *mut Struct_archive,
                                                        arg2: *mut *const c_char)
                                                        -> c_int;
    pub fn archive_match_path_unmatched_inclusions_next_w(arg1: *mut Struct_archive,
                                                          arg2: *mut *const wchar_t)
                                                          -> c_int;
    pub fn archive_match_time_excluded(arg1: *mut Struct_archive,
                                       arg2: *mut Struct_archive_entry)
                                       -> c_int;
    pub fn archive_match_include_time(arg1: *mut Struct_archive,
                                      _flag: c_int,
                                      _sec: time_t,
                                      _nsec: c_long)
                                      -> c_int;
    pub fn archive_match_include_date(arg1: *mut Struct_archive,
                                      _flag: c_int,
                                      _datestr: *const c_char)
                                      -> c_int;
    pub fn archive_match_include_date_w(arg1: *mut Struct_archive,
                                        _flag: c_int,
                                        _datestr: *const wchar_t)
                                        -> c_int;
    pub fn archive_match_include_file_time(arg1: *mut Struct_archive,
                                           _flag: c_int,
                                           _pathname: *const c_char)
                                           -> c_int;
    pub fn archive_match_include_file_time_w(arg1: *mut Struct_archive,
                                             _flag: c_int,
                                             _pathname: *const wchar_t)
                                             -> c_int;
    pub fn archive_match_exclude_entry(arg1: *mut Struct_archive,
                                       _flag: c_int,
                                       arg2: *mut Struct_archive_entry)
                                       -> c_int;
    pub fn archive_match_owner_excluded(arg1: *mut Struct_archive,
                                        arg2: *mut Struct_archive_entry)
                                        -> c_int;
    pub fn archive_match_include_uid(arg1: *mut Struct_archive, arg2: int64_t) -> c_int;
    pub fn archive_match_include_gid(arg1: *mut Struct_archive, arg2: int64_t) -> c_int;
    pub fn archive_match_include_uname(arg1: *mut Struct_archive, arg2: *const c_char) -> c_int;
    pub fn archive_match_include_uname_w(arg1: *mut Struct_archive, arg2: *const wchar_t) -> c_int;
    pub fn archive_match_include_gname(arg1: *mut Struct_archive, arg2: *const c_char) -> c_int;
    pub fn archive_match_include_gname_w(arg1: *mut Struct_archive, arg2: *const wchar_t) -> c_int;
    pub fn archive_entry_clear(arg1: *mut Struct_archive_entry) -> *mut Struct_archive_entry;
    pub fn archive_entry_clone(arg1: *mut Struct_archive_entry) -> *mut Struct_archive_entry;
    pub fn archive_entry_free(arg1: *mut Struct_archive_entry) -> ();
    pub fn archive_entry_new() -> *mut Struct_archive_entry;
    pub fn archive_entry_new2(arg1: *mut Struct_archive) -> *mut Struct_archive_entry;
    pub fn archive_entry_atime(arg1: *mut Struct_archive_entry) -> time_t;
    pub fn archive_entry_atime_nsec(arg1: *mut Struct_archive_entry) -> c_long;
    pub fn archive_entry_atime_is_set(arg1: *mut Struct_archive_entry) -> c_int;
    pub fn archive_entry_birthtime(arg1: *mut Struct_archive_entry) -> time_t;
    pub fn archive_entry_birthtime_nsec(arg1: *mut Struct_archive_entry) -> c_long;
    pub fn archive_entry_birthtime_is_set(arg1: *mut Struct_archive_entry) -> c_int;
    pub fn archive_entry_ctime(arg1: *mut Struct_archive_entry) -> time_t;
    pub fn archive_entry_ctime_nsec(arg1: *mut Struct_archive_entry) -> c_long;
    pub fn archive_entry_ctime_is_set(arg1: *mut Struct_archive_entry) -> c_int;
    pub fn archive_entry_dev(arg1: *mut Struct_archive_entry) -> dev_t;
    pub fn archive_entry_dev_is_set(arg1: *mut Struct_archive_entry) -> c_int;
    pub fn archive_entry_devmajor(arg1: *mut Struct_archive_entry) -> dev_t;
    pub fn archive_entry_devminor(arg1: *mut Struct_archive_entry) -> dev_t;
    pub fn archive_entry_filetype(arg1: *mut Struct_archive_entry) -> mode_t;
    pub fn archive_entry_fflags(arg1: *mut Struct_archive_entry,
                                arg2: *mut c_ulong,
                                arg3: *mut c_ulong)
                                -> ();
    pub fn archive_entry_fflags_text(arg1: *mut Struct_archive_entry) -> *const c_char;
    pub fn archive_entry_gid(arg1: *mut Struct_archive_entry) -> int64_t;
    pub fn archive_entry_gname(arg1: *mut Struct_archive_entry) -> *const c_char;
    pub fn archive_entry_gname_w(arg1: *mut Struct_archive_entry) -> *const wchar_t;
    pub fn archive_entry_hardlink(arg1: *mut Struct_archive_entry) -> *const c_char;
    pub fn archive_entry_hardlink_w(arg1: *mut Struct_archive_entry) -> *const wchar_t;
    pub fn archive_entry_ino(arg1: *mut Struct_archive_entry) -> int64_t;
    pub fn archive_entry_ino64(arg1: *mut Struct_archive_entry) -> int64_t;
    pub fn archive_entry_ino_is_set(arg1: *mut Struct_archive_entry) -> c_int;
    pub fn archive_entry_mode(arg1: *mut Struct_archive_entry) -> mode_t;
    pub fn archive_entry_mtime(arg1: *mut Struct_archive_entry) -> time_t;
    pub fn archive_entry_mtime_nsec(arg1: *mut Struct_archive_entry) -> c_long;
    pub fn archive_entry_mtime_is_set(arg1: *mut Struct_archive_entry) -> c_int;
    pub fn archive_entry_nlink(arg1: *mut Struct_archive_entry) -> c_uint;
    pub fn archive_entry_pathname(arg1: *mut Struct_archive_entry) -> *const c_char;
    pub fn archive_entry_pathname_w(arg1: *mut Struct_archive_entry) -> *const wchar_t;
    pub fn archive_entry_perm(arg1: *mut Struct_archive_entry) -> mode_t;
    pub fn archive_entry_rdev(arg1: *mut Struct_archive_entry) -> dev_t;
    pub fn archive_entry_rdevmajor(arg1: *mut Struct_archive_entry) -> dev_t;
    pub fn archive_entry_rdevminor(arg1: *mut Struct_archive_entry) -> dev_t;
    pub fn archive_entry_sourcepath(arg1: *mut Struct_archive_entry) -> *const c_char;
    pub fn archive_entry_sourcepath_w(arg1: *mut Struct_archive_entry) -> *const wchar_t;
    pub fn archive_entry_size(arg1: *mut Struct_archive_entry) -> int64_t;
    pub fn archive_entry_size_is_set(arg1: *mut Struct_archive_entry) -> c_int;
    pub fn archive_entry_strmode(arg1: *mut Struct_archive_entry) -> *const c_char;
    pub fn archive_entry_symlink(arg1: *mut Struct_archive_entry) -> *const c_char;
    pub fn archive_entry_symlink_w(arg1: *mut Struct_archive_entry) -> *const wchar_t;
    pub fn archive_entry_uid(arg1: *mut Struct_archive_entry) -> int64_t;
    pub fn archive_entry_uname(arg1: *mut Struct_archive_entry) -> *const c_char;
    pub fn archive_entry_uname_w(arg1: *mut Struct_archive_entry) -> *const wchar_t;
    pub fn archive_entry_set_atime(arg1: *mut Struct_archive_entry,
                                   arg2: time_t,
                                   arg3: c_long)
                                   -> ();
    pub fn archive_entry_unset_atime(arg1: *mut Struct_archive_entry) -> ();
    pub fn archive_entry_set_birthtime(arg1: *mut Struct_archive_entry,
                                       arg2: time_t,
                                       arg3: c_long)
                                       -> ();
    pub fn archive_entry_unset_birthtime(arg1: *mut Struct_archive_entry) -> ();
    pub fn archive_entry_set_ctime(arg1: *mut Struct_archive_entry,
                                   arg2: time_t,
                                   arg3: c_long)
                                   -> ();
    pub fn archive_entry_unset_ctime(arg1: *mut Struct_archive_entry) -> ();
    pub fn archive_entry_set_dev(arg1: *mut Struct_archive_entry, arg2: dev_t) -> ();
    pub fn archive_entry_set_devmajor(arg1: *mut Struct_archive_entry, arg2: dev_t) -> ();
    pub fn archive_entry_set_devminor(arg1: *mut Struct_archive_entry, arg2: dev_t) -> ();
    pub fn archive_entry_set_filetype(arg1: *mut Struct_archive_entry, arg2: c_uint) -> ();
    pub fn archive_entry_set_fflags(arg1: *mut Struct_archive_entry,
                                    arg2: c_ulong,
                                    arg3: c_ulong)
                                    -> ();
    pub fn archive_entry_copy_fflags_text(arg1: *mut Struct_archive_entry,
                                          arg2: *const c_char)
                                          -> *const c_char;
    pub fn archive_entry_copy_fflags_text_w(arg1: *mut Struct_archive_entry,
                                            arg2: *const wchar_t)
                                            -> *const wchar_t;
    pub fn archive_entry_set_gid(arg1: *mut Struct_archive_entry, arg2: int64_t) -> ();
    pub fn archive_entry_set_gname(arg1: *mut Struct_archive_entry, arg2: *const c_char) -> ();
    pub fn archive_entry_copy_gname(arg1: *mut Struct_archive_entry, arg2: *const c_char) -> ();
    pub fn archive_entry_copy_gname_w(arg1: *mut Struct_archive_entry, arg2: *const wchar_t) -> ();
    pub fn archive_entry_update_gname_utf8(arg1: *mut Struct_archive_entry,
                                           arg2: *const c_char)
                                           -> c_int;
    pub fn archive_entry_set_hardlink(arg1: *mut Struct_archive_entry, arg2: *const c_char) -> ();
    pub fn archive_entry_copy_hardlink(arg1: *mut Struct_archive_entry, arg2: *const c_char) -> ();
    pub fn archive_entry_copy_hardlink_w(arg1: *mut Struct_archive_entry,
                                         arg2: *const wchar_t)
                                         -> ();
    pub fn archive_entry_update_hardlink_utf8(arg1: *mut Struct_archive_entry,
                                              arg2: *const c_char)
                                              -> c_int;
    pub fn archive_entry_set_ino(arg1: *mut Struct_archive_entry, arg2: int64_t) -> ();
    pub fn archive_entry_set_ino64(arg1: *mut Struct_archive_entry, arg2: int64_t) -> ();
    pub fn archive_entry_set_link(arg1: *mut Struct_archive_entry, arg2: *const c_char) -> ();
    pub fn archive_entry_copy_link(arg1: *mut Struct_archive_entry, arg2: *const c_char) -> ();
    pub fn archive_entry_copy_link_w(arg1: *mut Struct_archive_entry, arg2: *const wchar_t) -> ();
    pub fn archive_entry_update_link_utf8(arg1: *mut Struct_archive_entry,
                                          arg2: *const c_char)
                                          -> c_int;
    pub fn archive_entry_set_mode(arg1: *mut Struct_archive_entry, arg2: mode_t) -> ();
    pub fn archive_entry_set_mtime(arg1: *mut Struct_archive_entry,
                                   arg2: time_t,
                                   arg3: c_long)
                                   -> ();
    pub fn archive_entry_unset_mtime(arg1: *mut Struct_archive_entry) -> ();
    pub fn archive_entry_set_nlink(arg1: *mut Struct_archive_entry, arg2: c_uint) -> ();
    pub fn archive_entry_set_pathname(arg1: *mut Struct_archive_entry, arg2: *const c_char) -> ();
    pub fn archive_entry_copy_pathname(arg1: *mut Struct_archive_entry, arg2: *const c_char) -> ();
    pub fn archive_entry_copy_pathname_w(arg1: *mut Struct_archive_entry,
                                         arg2: *const wchar_t)
                                         -> ();
    pub fn archive_entry_update_pathname_utf8(arg1: *mut Struct_archive_entry,
                                              arg2: *const c_char)
                                              -> c_int;
    pub fn archive_entry_set_perm(arg1: *mut Struct_archive_entry, arg2: mode_t) -> ();
    pub fn archive_entry_set_rdev(arg1: *mut Struct_archive_entry, arg2: dev_t) -> ();
    pub fn archive_entry_set_rdevmajor(arg1: *mut Struct_archive_entry, arg2: dev_t) -> ();
    pub fn archive_entry_set_rdevminor(arg1: *mut Struct_archive_entry, arg2: dev_t) -> ();
    pub fn archive_entry_set_size(arg1: *mut Struct_archive_entry, arg2: int64_t) -> ();
    pub fn archive_entry_unset_size(arg1: *mut Struct_archive_entry) -> ();
    pub fn archive_entry_copy_sourcepath(arg1: *mut Struct_archive_entry,
                                         arg2: *const c_char)
                                         -> ();
    pub fn archive_entry_copy_sourcepath_w(arg1: *mut Struct_archive_entry,
                                           arg2: *const wchar_t)
                                           -> ();
    pub fn archive_entry_set_symlink(arg1: *mut Struct_archive_entry, arg2: *const c_char) -> ();
    pub fn archive_entry_copy_symlink(arg1: *mut Struct_archive_entry, arg2: *const c_char) -> ();
    pub fn archive_entry_copy_symlink_w(arg1: *mut Struct_archive_entry,
                                        arg2: *const wchar_t)
                                        -> ();
    pub fn archive_entry_update_symlink_utf8(arg1: *mut Struct_archive_entry,
                                             arg2: *const c_char)
                                             -> c_int;
    pub fn archive_entry_set_uid(arg1: *mut Struct_archive_entry, arg2: int64_t) -> ();
    pub fn archive_entry_set_uname(arg1: *mut Struct_archive_entry, arg2: *const c_char) -> ();
    pub fn archive_entry_copy_uname(arg1: *mut Struct_archive_entry, arg2: *const c_char) -> ();
    pub fn archive_entry_copy_uname_w(arg1: *mut Struct_archive_entry, arg2: *const wchar_t) -> ();
    pub fn archive_entry_update_uname_utf8(arg1: *mut Struct_archive_entry,
                                           arg2: *const c_char)
                                           -> c_int;
    // pub fn archive_entry_stat(arg1: *mut Struct_archive_entry) -> *const Struct_stat;
    // pub fn archive_entry_copy_stat(arg1: *mut Struct_archive_entry,
    //                                arg2: *const Struct_stat)
    //                                -> ();
    pub fn archive_entry_mac_metadata(arg1: *mut Struct_archive_entry,
                                      arg2: *mut size_t)
                                      -> *const c_void;
    pub fn archive_entry_copy_mac_metadata(arg1: *mut Struct_archive_entry,
                                           arg2: *const c_void,
                                           arg3: size_t)
                                           -> ();
    pub fn archive_entry_acl_clear(arg1: *mut Struct_archive_entry) -> ();
    pub fn archive_entry_acl_add_entry(arg1: *mut Struct_archive_entry,
                                       arg2: c_int,
                                       arg3: c_int,
                                       arg4: c_int,
                                       arg5: c_int,
                                       arg6: *const c_char)
                                       -> c_int;
    pub fn archive_entry_acl_add_entry_w(arg1: *mut Struct_archive_entry,
                                         arg2: c_int,
                                         arg3: c_int,
                                         arg4: c_int,
                                         arg5: c_int,
                                         arg6: *const wchar_t)
                                         -> c_int;
    pub fn archive_entry_acl_reset(arg1: *mut Struct_archive_entry, arg2: c_int) -> c_int;
    pub fn archive_entry_acl_next(arg1: *mut Struct_archive_entry,
                                  arg2: c_int,
                                  arg3: *mut c_int,
                                  arg4: *mut c_int,
                                  arg5: *mut c_int,
                                  arg6: *mut c_int,
                                  arg7: *mut *const c_char)
                                  -> c_int;
    pub fn archive_entry_acl_next_w(arg1: *mut Struct_archive_entry,
                                    arg2: c_int,
                                    arg3: *mut c_int,
                                    arg4: *mut c_int,
                                    arg5: *mut c_int,
                                    arg6: *mut c_int,
                                    arg7: *mut *const wchar_t)
                                    -> c_int;
    pub fn archive_entry_acl_text_w(arg1: *mut Struct_archive_entry,
                                    arg2: c_int)
                                    -> *const wchar_t;
    pub fn archive_entry_acl_text(arg1: *mut Struct_archive_entry, arg2: c_int) -> *const c_char;
    pub fn archive_entry_acl_count(arg1: *mut Struct_archive_entry, arg2: c_int) -> c_int;
    pub fn archive_entry_acl(arg1: *mut Struct_archive_entry) -> *mut Struct_archive_acl;
    pub fn archive_entry_xattr_clear(arg1: *mut Struct_archive_entry) -> ();
    pub fn archive_entry_xattr_add_entry(arg1: *mut Struct_archive_entry,
                                         arg2: *const c_char,
                                         arg3: *const c_void,
                                         arg4: size_t)
                                         -> ();
    pub fn archive_entry_xattr_count(arg1: *mut Struct_archive_entry) -> c_int;
    pub fn archive_entry_xattr_reset(arg1: *mut Struct_archive_entry) -> c_int;
    pub fn archive_entry_xattr_next(arg1: *mut Struct_archive_entry,
                                    arg2: *mut *const c_char,
                                    arg3: *mut *const c_void,
                                    arg4: *mut size_t)
                                    -> c_int;
    pub fn archive_entry_sparse_clear(arg1: *mut Struct_archive_entry) -> ();
    pub fn archive_entry_sparse_add_entry(arg1: *mut Struct_archive_entry,
                                          arg2: int64_t,
                                          arg3: int64_t)
                                          -> ();
    pub fn archive_entry_sparse_count(arg1: *mut Struct_archive_entry) -> c_int;
    pub fn archive_entry_sparse_reset(arg1: *mut Struct_archive_entry) -> c_int;
    pub fn archive_entry_sparse_next(arg1: *mut Struct_archive_entry,
                                     arg2: *mut int64_t,
                                     arg3: *mut int64_t)
                                     -> c_int;
    pub fn archive_entry_linkresolver_new() -> *mut Struct_archive_entry_linkresolver;
    pub fn archive_entry_linkresolver_set_strategy(arg1: *mut Struct_archive_entry_linkresolver,
                                                   arg2: c_int)
                                                   -> ();
    pub fn archive_entry_linkresolver_free(arg1: *mut Struct_archive_entry_linkresolver) -> ();
    pub fn archive_entry_linkify(arg1: *mut Struct_archive_entry_linkresolver,
                                 arg2: *mut *mut Struct_archive_entry,
                                 arg3: *mut *mut Struct_archive_entry)
                                 -> ();
    pub fn archive_entry_partial_links(res: *mut Struct_archive_entry_linkresolver,
                                       links: *mut c_uint)
                                       -> *mut Struct_archive_entry;
}
