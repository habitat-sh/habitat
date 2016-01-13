// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::fmt;
use std::io::{Seek, SeekFrom};
use std::path::PathBuf;
use std::str;

use libarchive::writer;
use libarchive::reader::{self, Reader};
use libarchive::archive::{Entry, ReadFilter, ReadFormat};
use regex::Regex;

use error::{BldrResult, BldrError, ErrorKind};
use package::Package;
use util::gpg;

static LOGKEY: &'static str = "PA";

#[derive(Debug)]
pub enum MetaFile {
    CFlags,
    Deps,
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
            MetaFile::Deps => "DEPS",
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

    /// A package struct representing the contents of this archive.
    ///
    /// # Failures
    ///
    /// * If an `IDENT` metafile is not found in the archive
    /// * If the archive cannot be read
    /// * If the archive cannot be verified
    pub fn package(&self) -> BldrResult<Package> {
        let body = try!(self.read_metadata(MetaFile::Ident));
        let mut package = try!(Package::from_ident(&body));
        match self.deps() {
            Ok(Some(deps)) => {
                for dep in deps {
                    package.add_dep(dep);
                }
            }
            Ok(None) => {}
            Err(e) => return Err(e),
        }
        Ok(package)
    }

    /// List of package structs representing the package dependencies for this archive.
    ///
    /// # Failures
    ///
    /// * If a `DEPS` metafile is not found in the archive
    /// * If the archive cannot be read
    /// * If the archive cannot be verified
    pub fn deps(&self) -> BldrResult<Option<Vec<Package>>> {
        match self.read_metadata(MetaFile::Deps) {
            Ok(body) => {
                let dep_strs: Vec<&str> = body.split("\n").collect();
                let mut deps = vec![];
                for dep in &dep_strs {
                    match Package::from_ident(&dep) {
                        Ok(package) => deps.push(package),
                        Err(_) => continue,
                    }
                }
                Ok(Some(deps))
            }
            Err(BldrError{err: ErrorKind::MetaFileNotFound(_), ..}) => Ok(None),
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
    pub fn unpack(&self) -> BldrResult<Package> {
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
        self.package()
    }

    fn read_metadata(&self, file: MetaFile) -> BldrResult<String> {
        let f = self.path.to_str().unwrap().to_string();
        let mut out = try!(gpg::verify(&f));
        try!(out.seek(SeekFrom::Start(0)));
        let mut builder = reader::Builder::new();
        try!(builder.support_format(ReadFormat::All));
        try!(builder.support_filter(ReadFilter::All));
        let mut reader = try!(builder.open_stream(out));
        let re = try!(Regex::new(&format!("/{}$", file)));
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
                    Ok(content) => Ok(content.to_string()),
                    Err(_) => Err(bldr_error!(ErrorKind::MetaFileMalformed)),
                }
            }
            Ok(None) => Err(bldr_error!(ErrorKind::MetaFileMalformed)),
            Err(_) => Err(bldr_error!(ErrorKind::MetaFileMalformed)),
        }
    }
}
