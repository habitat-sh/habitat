use std::default::Default;
use std::path::Path;
use std::ptr;
use std::ffi::CString;

use libarchive3_sys::ffi;

use archive::{Entry, ExtractOptions, Handle, WriteFilter, WriteFormat};
use reader::{Reader, ReaderEntry};
use error::{ArchiveResult, ArchiveError};

pub struct Writer {
    handle: *mut ffi::Struct_archive,
}

pub struct Disk {
    handle: *mut ffi::Struct_archive,
}

pub struct Builder {
    handle: *mut ffi::Struct_archive,
    consumed: bool,
}

impl Writer {
    pub fn new(handle: *mut ffi::Struct_archive) -> Self {
        Writer { handle: handle }
    }
}

impl Handle for Writer {
    unsafe fn handle(&self) -> *mut ffi::Struct_archive {
        self.handle
    }
}

impl Drop for Writer {
    fn drop(&mut self) {
        unsafe {
            ffi::archive_write_free(self.handle());
        }
    }
}

impl Disk {
    pub fn new() -> Self {
        Disk::default()
    }

    // Retrieve the currently-set value for last block size. A value of -1 here indicates that the
    // library should use default values.
    pub fn bytes_in_last_block(&self) -> i32 {
        unsafe { ffi::archive_write_get_bytes_in_last_block(self.handle) }
    }

    // Retrieve the block size to be used for writing. A value of -1 here indicates that the
    // library should use default values. A value of zero indicates that internal blocking is
    // suppressed.
    pub fn bytes_per_block(&self) -> i32 {
        unsafe { ffi::archive_write_get_bytes_per_block(self.handle) }
    }

    pub fn set_bytes_per_block(&mut self, count: i32) -> ArchiveResult<()> {
        unsafe {
            ffi::archive_write_set_bytes_per_block(self.handle, count);
        }
        ArchiveResult::from(self as &Handle)
    }

    pub fn set_bytes_in_last_block(&mut self, count: i32) -> ArchiveResult<()> {
        unsafe {
            ffi::archive_write_set_bytes_in_last_block(self.handle, count);
        }
        ArchiveResult::from(self as &Handle)
    }

    // Set options for extraction built from `ExtractOptions`
    pub fn set_options(&self, eopt: &ExtractOptions) -> ArchiveResult<()> {
        unsafe {
            ffi::archive_write_disk_set_options(self.handle, eopt.flags);
        }
        ArchiveResult::from(self as &Handle)
    }

    // This convenience function installs a standard set of user and group lookup functions. These
    // functions use getpwnam(3) and getgrnam(3) to convert names to ids, defaulting to the ids if
    // the names cannot be looked up. These functions also implement a simple memory cache to
    // reduce the number of calls to getpwnam(3) and getgrnam(3).
    pub fn set_standard_lookup(&self) -> ArchiveResult<()> {
        unsafe {
            ffi::archive_write_disk_set_standard_lookup(self.handle);
        }
        ArchiveResult::from(self as &Handle)
    }

    // * Failures - HeaderPosition
    pub fn write<T: Reader>(&self, reader: &mut T, prefix: Option<&str>) -> ArchiveResult<usize> {
        if reader.header_position() != 0 {
            return Err(ArchiveError::HeaderPosition);
        }
        let mut bytes: usize = 0;
        let mut write_pending: bool = false;
        loop {
            {
                if let Some(entry) = reader.next_header() {
                    if let Some(pfx) = prefix {
                        let path = Path::new(pfx).join(entry.pathname());
                        try!(entry.set_pathname(path));
                    }
                    match self.write_header(entry) {
                        Ok(()) => (),
                        Err(e) => return Err(e),
                    }
                    if entry.size() > 0 {
                        write_pending = true
                    }
                } else {
                    break;
                }
            }
            if write_pending {
                bytes += try!(self.write_data(reader));
                write_pending = false;
            }
        }
        unsafe {
            match ffi::archive_write_finish_entry(self.handle()) {
                ffi::ARCHIVE_OK => Ok(bytes),
                _ => Err(ArchiveError::from(self as &Handle)),
            }
        }
    }

    pub fn close(&self) -> ArchiveResult<()> {
        unsafe {
            ffi::archive_write_close(self.handle());
        }
        ArchiveResult::from(self as &Handle)
    }

    fn write_data<T: Reader>(&self, reader: &T) -> ArchiveResult<usize> {
        let mut buff = ptr::null();
        let mut size = 0;
        let mut offset = 0;

        unsafe {
            match ffi::archive_read_data_block(reader.handle(), &mut buff, &mut size, &mut offset) {
                ffi::ARCHIVE_EOF => Ok(size),
                ffi::ARCHIVE_OK => {
                    if ffi::archive_write_data_block(self.handle, buff, size, offset) < 0 {
                        Err(ArchiveError::from(self as &Handle))
                    } else {
                        Ok(size)
                    }
                }
                _ => Err(ArchiveError::from(reader as &Handle)),
            }
        }
    }

    fn write_header(&self, entry: &ReaderEntry) -> ArchiveResult<()> {
        unsafe {
            ffi::archive_write_header(self.handle, entry.entry());
        }
        ArchiveResult::from(self as &Handle)
    }
}

impl Handle for Disk {
    unsafe fn handle(&self) -> *mut ffi::Struct_archive {
        self.handle
    }
}

impl Default for Disk {
    fn default() -> Self {
        unsafe {
            let handle = ffi::archive_write_disk_new();
            if handle.is_null() {
                panic!("Allocation error");
            }
            Disk { handle: handle }
        }
    }
}

impl Drop for Disk {
    fn drop(&mut self) {
        self.close().unwrap();
        unsafe {
            ffi::archive_write_free(self.handle());
        }
    }
}

impl Builder {
    pub fn new() -> Self {
        Builder::default()
    }

    pub fn add_filter(&mut self, filter: WriteFilter) -> ArchiveResult<()> {
        match filter {
            WriteFilter::B64Encode => unsafe {
                ffi::archive_write_add_filter_b64encode(self.handle);
            },
            WriteFilter::Bzip2 => unsafe {
                ffi::archive_write_add_filter_bzip2(self.handle);
            },
            WriteFilter::Compress => unsafe {
                ffi::archive_write_add_filter_compress(self.handle);
            },
            WriteFilter::Grzip => unsafe {
                ffi::archive_write_add_filter_grzip(self.handle);
            },
            WriteFilter::Gzip => unsafe {
                ffi::archive_write_add_filter_gzip(self.handle);
            },
            WriteFilter::Lrzip => unsafe {
                ffi::archive_write_add_filter_lrzip(self.handle);
            },
            WriteFilter::Lzip => unsafe {
                ffi::archive_write_add_filter_lzip(self.handle);
            },
            WriteFilter::Lzma => unsafe {
                ffi::archive_write_add_filter_lzma(self.handle);
            },
            WriteFilter::Lzop => unsafe {
                ffi::archive_write_add_filter_lzop(self.handle);
            },
            WriteFilter::None => unsafe {
                ffi::archive_write_add_filter_none(self.handle);
            },
            WriteFilter::Program(prog) => {
                let c_prog = CString::new(prog).unwrap();
                unsafe { ffi::archive_write_add_filter_program(self.handle, c_prog.as_ptr()) };

            }
            WriteFilter::UuEncode => unsafe {
                ffi::archive_write_add_filter_uuencode(self.handle);
            },
            WriteFilter::Xz => unsafe {
                ffi::archive_write_add_filter_xz(self.handle);
            },
        }
        ArchiveResult::from(self as &Handle)
    }

    pub fn set_format(&self, format: WriteFormat) -> ArchiveResult<()> {
        match format {
            WriteFormat::SevenZip => unsafe {
                ffi::archive_write_set_format_7zip(self.handle);
            },
            WriteFormat::ArBsd => unsafe {
                ffi::archive_write_set_format_ar_bsd(self.handle);
            },
            WriteFormat::ArSvr4 => unsafe {
                ffi::archive_write_set_format_ar_svr4(self.handle);
            },
            WriteFormat::Cpio => unsafe {
                ffi::archive_write_set_format_cpio(self.handle);
            },
            WriteFormat::CpioNewc => unsafe {
                ffi::archive_write_set_format_cpio_newc(self.handle);
            },
            WriteFormat::Gnutar => unsafe {
                ffi::archive_write_set_format_gnutar(self.handle);
            },
            WriteFormat::Iso9660 => unsafe {
                ffi::archive_write_set_format_iso9660(self.handle);
            },
            WriteFormat::Mtree => unsafe {
                ffi::archive_write_set_format_mtree(self.handle);
            },
            WriteFormat::MtreeClassic => unsafe {
                ffi::archive_write_set_format_mtree_classic(self.handle);
            },
            WriteFormat::Pax => unsafe {
                ffi::archive_write_set_format_pax(self.handle);
            },
            WriteFormat::PaxRestricted => unsafe {
                ffi::archive_write_set_format_pax_restricted(self.handle);
            },
            WriteFormat::Shar => unsafe {
                ffi::archive_write_set_format_shar(self.handle);
            },
            WriteFormat::SharDump => unsafe {
                ffi::archive_write_set_format_shar_dump(self.handle);
            },
            WriteFormat::Ustar => unsafe {
                ffi::archive_write_set_format_ustar(self.handle);
            },
            WriteFormat::V7tar => unsafe {
                ffi::archive_write_set_format_v7tar(self.handle);
            },
            WriteFormat::Xar => unsafe {
                ffi::archive_write_set_format_xar(self.handle);
            },
            WriteFormat::Zip => unsafe {
                ffi::archive_write_set_format_zip(self.handle);
            },
        }
        ArchiveResult::from(self as &Handle)
    }

    pub fn open_file<T: AsRef<Path>>(mut self, file: T) -> ArchiveResult<Writer> {
        if self.consumed {
            return Err(ArchiveError::Consumed);
        }
        let c_file = CString::new(file.as_ref().to_string_lossy().as_bytes()).unwrap();
        let res = unsafe { ffi::archive_write_open_filename(self.handle, c_file.as_ptr()) };
        match res {
            0 => {
                self.consumed = true;
                Ok(Writer::new(self.handle))
            }
            _ => Err(ArchiveError::from(&self as &Handle)),
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        unsafe {
            let handle = ffi::archive_write_new();
            if handle.is_null() {
                panic!("Allocation error");
            }
            Builder {
                handle: handle,
                consumed: false,
            }
        }
    }
}

impl Handle for Builder {
    unsafe fn handle(&self) -> *mut ffi::Struct_archive {
        self.handle
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        if !self.consumed {
            unsafe {
                ffi::archive_write_free(self.handle);
            }
        }
    }
}
