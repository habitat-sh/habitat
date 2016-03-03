// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::fmt;
use std::io::{Seek, SeekFrom};
use std::path::PathBuf;
use std::str::{self, FromStr};

use libarchive::writer;
use libarchive::reader::{self, Reader};
use libarchive::archive::{Entry, ReadFilter, ReadFormat};
use regex::Regex;

use error::{BldrResult, BldrError, ErrorKind};
use package::PackageIdent;
use util::gpg;

static LOGKEY: &'static str = "PA";

#[derive(Debug)]
pub enum MetaFile {
    CFlags,
    Config,
    Deps,
    TDeps,
    Exposes,
    Ident,
    LdRunPath,
    LdFlags,
    Manifest,
    Path,
}

impl fmt::Display for MetaFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = match *self {
            MetaFile::CFlags => "CFLAGS",
            MetaFile::Config => "default.toml",
            MetaFile::Deps => "DEPS",
            MetaFile::TDeps => "TDEPS",
            MetaFile::Exposes => "EXPOSES",
            MetaFile::Ident => "IDENT",
            MetaFile::LdRunPath => "LD_RUN_PATH",
            MetaFile::LdFlags => "LDFLAGS",
            MetaFile::Manifest => "MANIFEST",
            MetaFile::Path => "PATH",
        };
        write!(f, "{}", id)
    }
}

#[derive(Debug)]
pub struct PackageArchive {
    pub path: PathBuf,
}

impl PackageArchive {
    pub fn new(path: PathBuf) -> Self {
        PackageArchive { path: path }
    }

    pub fn cflags(&self) -> BldrResult<Option<String>> {
        match self.read_metadata(MetaFile::CFlags) {
            Ok(data) => Ok(data),
            Err(BldrError{err: ErrorKind::MetaFileNotFound(_), ..}) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn config(&self) -> BldrResult<Option<String>> {
        match self.read_metadata(MetaFile::Config) {
            Ok(data) => Ok(data),
            Err(BldrError{err: ErrorKind::MetaFileNotFound(_), ..}) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Returns a list of package identifiers representing the runtime package dependencies for
    /// this archive.
    ///
    /// # Failures
    ///
    /// * If a `DEPS` metafile is not found in the archive
    /// * If the archive cannot be read
    /// * If the archive cannot be verified
    pub fn deps(&self) -> BldrResult<Vec<PackageIdent>> {
        self.read_deps(MetaFile::Deps)
    }

    /// Returns a list of package identifiers representing the transitive runtime package
    /// dependencies for this archive.
    ///
    /// # Failures
    ///
    /// * If a `TDEPS` metafile is not found in the archive
    /// * If the archive cannot be read
    /// * If the archive cannot be verified
    pub fn tdeps(&self) -> BldrResult<Vec<PackageIdent>> {
        self.read_deps(MetaFile::TDeps)
    }

    pub fn exposes(&self) -> BldrResult<Vec<u16>> {
        match self.read_metadata(MetaFile::Exposes) {
            Ok(Some(data)) => {
                let ports: Vec<u16> = data.split(" ")
                                          .filter_map(|port| port.parse::<u16>().ok())
                                          .collect();
                Ok(ports)
            }
            Ok(None) => Ok(vec![]),
            Err(BldrError{err: ErrorKind::MetaFileNotFound(_), ..}) => Ok(vec![]),
            Err(e) => Err(e),
        }
    }

    pub fn ident(&self) -> BldrResult<PackageIdent> {
        match self.read_metadata(MetaFile::Ident) {
            Ok(None) => Err(bldr_error!(ErrorKind::MetaFileMalformed(MetaFile::Ident))),
            Ok(Some(data)) => PackageIdent::from_str(&data),
            Err(e) => Err(e),
        }
    }

    pub fn ld_run_path(&self) -> BldrResult<Option<String>> {
        match self.read_metadata(MetaFile::LdRunPath) {
            Ok(data) => Ok(data),
            Err(BldrError{err: ErrorKind::MetaFileNotFound(_), ..}) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn ldflags(&self) -> BldrResult<Option<String>> {
        match self.read_metadata(MetaFile::LdFlags) {
            Ok(data) => Ok(data),
            Err(BldrError {err: ErrorKind::MetaFileNotFound(_), ..}) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn manifest(&self) -> BldrResult<String> {
        match self.read_metadata(MetaFile::Manifest) {
            Ok(None) => Err(bldr_error!(ErrorKind::MetaFileMalformed(MetaFile::Manifest))),
            Ok(Some(data)) => Ok(data),
            Err(e) => Err(e),
        }
    }

    pub fn path(&self) -> BldrResult<Option<String>> {
        match self.read_metadata(MetaFile::Path) {
            Ok(data) => Ok(data),
            Err(BldrError {err: ErrorKind::MetaFileNotFound(_), ..}) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// A plain string representation of the archive's file name.
    pub fn file_name(&self) -> String {
        self.path.file_name().unwrap().to_string_lossy().into_owned()
    }

    /// Given a package name and a path to a file as an `&str`, verify
    /// the files gpg signature.
    ///
    /// # Failures
    ///
    /// * Fails if it cannot verify the GPG signature for any reason
    pub fn verify(&self) -> BldrResult<()> {
        match gpg::verify(self.path.to_str().unwrap()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Given a package name and a path to a file as an `&str`, unpack
    /// the package.
    ///
    /// # Failures
    ///
    /// * If the package cannot be unpacked via gpg
    pub fn unpack(&self) -> BldrResult<()> {
        let file = self.path.to_str().unwrap().to_string();
        let mut out = try!(gpg::verify(&file));
        try!(out.seek(SeekFrom::Start(0)));
        let mut builder = reader::Builder::new();
        try!(builder.support_format(ReadFormat::All));
        try!(builder.support_filter(ReadFilter::All));
        let mut reader = try!(builder.open_stream(out));
        let writer = writer::Disk::new();
        try!(writer.set_standard_lookup());
        try!(writer.write(&mut reader, Some("/")));
        try!(writer.close());
        Ok(())
    }

    fn read_deps(&self, file: MetaFile) -> BldrResult<Vec<PackageIdent>> {
        let mut deps: Vec<PackageIdent> = vec![];
        match self.read_metadata(file) {
            Ok(Some(body)) => {
                let ids: Vec<String> = body.split("\n").map(|d| d.to_string()).collect();
                for id in &ids {
                    let package = try!(PackageIdent::from_str(id));
                    if !package.fully_qualified() {
                        // JW TODO: use a more appropriate erorr to describe the invalid
                        // user input here. (user because a package was generated by a user
                        // and read into program)
                        return Err(bldr_error!(ErrorKind::InvalidPackageIdent(package.to_string())));
                    }
                    deps.push(package);
                }
                Ok(deps)
            }
            Ok(None) => Ok(vec![]),
            Err(BldrError{err: ErrorKind::MetaFileNotFound(_), ..}) => Ok(deps),
            Err(e) => Err(e),
        }
    }

    fn read_metadata(&self, file: MetaFile) -> BldrResult<Option<String>> {
        let f = self.path.to_str().unwrap().to_string();
        let mut out = try!(gpg::verify(&f));
        try!(out.seek(SeekFrom::Start(0)));
        let mut builder = reader::Builder::new();
        try!(builder.support_format(ReadFormat::All));
        try!(builder.support_filter(ReadFilter::All));
        let mut reader = try!(builder.open_stream(out));
        let re = try!(Regex::new(&format!(r"^opt/bldr/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$",
                                          file)));
        loop {
            {
                if let Some(entry) = reader.next_header() {
                    if re.is_match(entry.pathname()) {
                        break;
                    }
                } else {
                    return Err(bldr_error!(ErrorKind::MetaFileNotFound(file)));
                }
            }
        }
        match reader.read_block() {
            Ok(Some(bytes)) => {
                match str::from_utf8(bytes) {
                    Ok(content) => Ok(Some(content.trim().to_string())),
                    Err(_) => Err(bldr_error!(ErrorKind::MetaFileMalformed(file))),
                }
            }
            Ok(None) => Ok(None),
            Err(_) => Err(bldr_error!(ErrorKind::MetaFileMalformed(file))),
        }
    }
}
