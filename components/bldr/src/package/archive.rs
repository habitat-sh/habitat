// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::str::{self, FromStr};

use crypto::sha2::Sha256;
use crypto::digest::Digest;
use libarchive::writer;
use libarchive::reader::{self, Reader};
use libarchive::archive::{Entry, ReadFilter, ReadFormat};
use regex::Regex;

use error::{BldrResult, BldrError, ErrorKind};
use package::PackageIdent;
use util::gpg;

static LOGKEY: &'static str = "PA";

lazy_static! {
    static ref METAFILE_REGXS: HashMap<MetaFile, Regex> = {
        let mut map = HashMap::new();
        map.insert(MetaFile::CFlags, Regex::new(&format!(r"^opt/bldr/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::CFlags)).unwrap());
        map.insert(MetaFile::Config, Regex::new(&format!(r"^opt/bldr/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::Config)).unwrap());
        map.insert(MetaFile::Deps, Regex::new(&format!(r"^opt/bldr/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::Deps)).unwrap());
        map.insert(MetaFile::TDeps, Regex::new(&format!(r"^opt/bldr/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::TDeps)).unwrap());
        map.insert(MetaFile::Exposes, Regex::new(&format!(r"^opt/bldr/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::Exposes)).unwrap());
        map.insert(MetaFile::Ident, Regex::new(&format!(r"^opt/bldr/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::Ident)).unwrap());
        map.insert(MetaFile::LdRunPath, Regex::new(&format!(r"^opt/bldr/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::LdRunPath)).unwrap());
        map.insert(MetaFile::LdFlags, Regex::new(&format!(r"^opt/bldr/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::LdFlags)).unwrap());
        map.insert(MetaFile::Manifest, Regex::new(&format!(r"^opt/bldr/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::Manifest)).unwrap());
        map.insert(MetaFile::Path, Regex::new(&format!(r"^opt/bldr/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::Path)).unwrap());
        map
    };
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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

type Metadata = HashMap<MetaFile, String>;

#[derive(Debug)]
pub struct PackageArchive {
    pub path: PathBuf,
    metadata: Option<Metadata>,
}

impl PackageArchive {
    pub fn new(path: PathBuf) -> Self {
        PackageArchive {
            path: path,
            metadata: None,
        }
    }

    /// Calculate and return the checksum of the package archive in hexadecimal format.
    ///
    /// # Failures
    ///
    /// * If the archive cannot be read
    pub fn checksum(&self) -> BldrResult<String> {
        let mut digest = Sha256::new();
        let mut file = try!(File::open(&self.path));
        let mut buffer = Vec::new();
        try!(file.read_to_end(&mut buffer));
        digest.input(&buffer);
        let hash = digest.result_str();
        Ok(hash)
    }

    pub fn cflags(&mut self) -> BldrResult<Option<String>> {
        match self.read_metadata(MetaFile::CFlags) {
            Ok(data) => Ok(data.cloned()),
            Err(e) => Err(e),
        }
    }

    pub fn config(&mut self) -> BldrResult<Option<String>> {
        match self.read_metadata(MetaFile::Config) {
            Ok(data) => Ok(data.cloned()),
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
    pub fn deps(&mut self) -> BldrResult<Vec<PackageIdent>> {
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
    pub fn tdeps(&mut self) -> BldrResult<Vec<PackageIdent>> {
        self.read_deps(MetaFile::TDeps)
    }

    pub fn exposes(&mut self) -> BldrResult<Vec<u16>> {
        match self.read_metadata(MetaFile::Exposes) {
            Ok(Some(data)) => {
                let ports: Vec<u16> = data.split(" ")
                                          .filter_map(|port| port.parse::<u16>().ok())
                                          .collect();
                Ok(ports)
            }
            Ok(None) => Ok(vec![]),
            Err(e) => Err(e),
        }
    }

    pub fn ident(&mut self) -> BldrResult<PackageIdent> {
        match self.read_metadata(MetaFile::Ident) {
            Ok(None) => Err(bldr_error!(ErrorKind::MetaFileMalformed(MetaFile::Ident))),
            Ok(Some(data)) => PackageIdent::from_str(&data),
            Err(e) => Err(e),
        }
    }

    pub fn ld_run_path(&mut self) -> BldrResult<Option<String>> {
        match self.read_metadata(MetaFile::LdRunPath) {
            Ok(data) => Ok(data.cloned()),
            Err(e) => Err(e),
        }
    }

    pub fn ldflags(&mut self) -> BldrResult<Option<String>> {
        match self.read_metadata(MetaFile::LdFlags) {
            Ok(data) => Ok(data.cloned()),
            Err(e) => Err(e),
        }
    }

    pub fn manifest(&mut self) -> BldrResult<String> {
        match self.read_metadata(MetaFile::Manifest) {
            Ok(None) => Err(bldr_error!(ErrorKind::MetaFileMalformed(MetaFile::Manifest))),
            Ok(Some(data)) => Ok(data.clone()),
            Err(e) => Err(e),
        }
    }

    pub fn path(&mut self) -> BldrResult<Option<String>> {
        match self.read_metadata(MetaFile::Path) {
            Ok(data) => Ok(data.cloned()),
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

    fn read_deps(&mut self, file: MetaFile) -> BldrResult<Vec<PackageIdent>> {
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

    fn read_metadata(&mut self, file: MetaFile) -> BldrResult<Option<&String>> {
        if let Some(ref files) = self.metadata {
            return Ok(files.get(&file));
        }

        let mut metadata = Metadata::new();
        let mut matched_count = 0u8;
        let f = self.path.to_str().unwrap().to_string();
        let mut out = try!(gpg::verify(&f));
        try!(out.seek(SeekFrom::Start(0)));
        let mut builder = reader::Builder::new();
        try!(builder.support_format(ReadFormat::All));
        try!(builder.support_filter(ReadFilter::All));
        let mut reader = try!(builder.open_stream(out));
        loop {
            let mut matched_type: Option<MetaFile> = None;
            if let Some(entry) = reader.next_header() {
                for (matched, regx) in METAFILE_REGXS.iter() {
                    if regx.is_match(entry.pathname()) {
                        matched_type = Some((*matched).clone());
                        matched_count += 1;
                        break;
                    }
                }
            } else {
                break;
            }

            if matched_type.is_none() {
                continue;
            }

            match reader.read_block() {
                Ok(Some(bytes)) => {
                    match str::from_utf8(bytes) {
                        Ok(content) => {
                            metadata.insert(matched_type.unwrap(), content.trim().to_string());
                        }
                        Err(_) => {
                            return Err(bldr_error!(ErrorKind::MetaFileMalformed(matched_type.unwrap())))
                        }
                    }
                }
                Ok(None) => (),
                Err(_) => {
                    return Err(bldr_error!(ErrorKind::MetaFileMalformed(matched_type.unwrap())))
                }
            }

            if matched_count == METAFILE_REGXS.len() as u8 {
                break;
            }
        }
        self.metadata = Some(metadata);
        Ok(self.metadata.as_ref().unwrap().get(&file))
    }
}
