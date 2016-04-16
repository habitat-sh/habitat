// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::collections::HashMap;
use std::path::PathBuf;
use std::str::{self, FromStr};

use libarchive::writer;
use libarchive::reader::{self, Reader};
use libarchive::archive::{Entry, ReadFilter, ReadFormat};
use regex::Regex;

use error::{Error, Result};
use crypto;
use package::{PackageIdent, MetaFile};

lazy_static! {
    static ref METAFILE_REGXS: HashMap<MetaFile, Regex> = {
        let mut map = HashMap::new();
        map.insert(MetaFile::CFlags, Regex::new(&format!(r"^hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::CFlags)).unwrap());
        map.insert(MetaFile::Config, Regex::new(&format!(r"^hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::Config)).unwrap());
        map.insert(MetaFile::Deps, Regex::new(&format!(r"^hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::Deps)).unwrap());
        map.insert(MetaFile::TDeps, Regex::new(&format!(r"^hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::TDeps)).unwrap());
        map.insert(MetaFile::Exposes, Regex::new(&format!(r"^hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::Exposes)).unwrap());
        map.insert(MetaFile::Ident, Regex::new(&format!(r"^hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::Ident)).unwrap());
        map.insert(MetaFile::LdRunPath, Regex::new(&format!(r"^hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::LdRunPath)).unwrap());
        map.insert(MetaFile::LdFlags, Regex::new(&format!(r"^hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::LdFlags)).unwrap());
        map.insert(MetaFile::Manifest, Regex::new(&format!(r"^hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::Manifest)).unwrap());
        map.insert(MetaFile::Path, Regex::new(&format!(r"^hab/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::Path)).unwrap());
        map
    };
}

type Metadata = HashMap<MetaFile, String>;

#[derive(Debug)]
pub struct PackageArchive {
    pub path: PathBuf,
    metadata: Option<Metadata>,
}

impl PackageArchive {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        PackageArchive {
            path: path.into(),
            metadata: None,
        }
    }

    /// Calculate and return the checksum of the package archive in base64 format.
    ///
    /// # Failures
    ///
    /// * If the archive cannot be read
    pub fn checksum(&self) -> Result<String> {
        crypto::hash_file(&self.path)
    }

    pub fn cflags(&mut self) -> Result<Option<String>> {
        match self.read_metadata(MetaFile::CFlags) {
            Ok(data) => Ok(data.cloned()),
            Err(e) => Err(e),
        }
    }

    pub fn config(&mut self) -> Result<Option<String>> {
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
    pub fn deps(&mut self) -> Result<Vec<PackageIdent>> {
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
    pub fn tdeps(&mut self) -> Result<Vec<PackageIdent>> {
        self.read_deps(MetaFile::TDeps)
    }

    pub fn exposes(&mut self) -> Result<Vec<u16>> {
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

    pub fn ident(&mut self) -> Result<PackageIdent> {
        match self.read_metadata(MetaFile::Ident) {
            Ok(None) => Err(Error::MetaFileNotFound(MetaFile::Ident)),
            Ok(Some(data)) => PackageIdent::from_str(&data),
            Err(e) => Err(e),
        }
    }

    pub fn ld_run_path(&mut self) -> Result<Option<String>> {
        match self.read_metadata(MetaFile::LdRunPath) {
            Ok(data) => Ok(data.cloned()),
            Err(e) => Err(e),
        }
    }

    pub fn ldflags(&mut self) -> Result<Option<String>> {
        match self.read_metadata(MetaFile::LdFlags) {
            Ok(data) => Ok(data.cloned()),
            Err(e) => Err(e),
        }
    }

    pub fn manifest(&mut self) -> Result<String> {
        match self.read_metadata(MetaFile::Manifest) {
            Ok(None) => Err(Error::MetaFileNotFound(MetaFile::Manifest)),
            Ok(Some(data)) => Ok(data.clone()),
            Err(e) => Err(e),
        }
    }

    pub fn path(&mut self) -> Result<Option<String>> {
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
    /// the files signature.
    ///
    /// # Failures
    ///
    /// * Fails if it cannot verify the signature for any reason
    pub fn verify(&self) -> Result<()> {
        crypto::artifact_verify(self.path.to_str().unwrap())
    }

    /// Given a package name and a path to a file as an `&str`, unpack
    /// the package.
    ///
    /// # Failures
    ///
    /// * If the package cannot be unpacked
    pub fn unpack(&self) -> Result<()> {
        let file = self.path.to_str().unwrap().to_string();
        let tar_reader = try!(crypto::get_artifact_reader(&file));
        let mut builder = reader::Builder::new();
        try!(builder.support_format(ReadFormat::All));
        try!(builder.support_filter(ReadFilter::All));
        let mut reader = try!(builder.open_stream(tar_reader));
        let writer = writer::Disk::new();
        try!(writer.set_standard_lookup());
        try!(writer.write(&mut reader, Some("/")));
        try!(writer.close());
        Ok(())
    }

    fn read_deps(&mut self, file: MetaFile) -> Result<Vec<PackageIdent>> {
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
                        return Err(Error::InvalidPackageIdent(package.to_string()));
                    }
                    deps.push(package);
                }
                Ok(deps)
            }
            Ok(None) => Ok(vec![]),
            Err(Error::MetaFileNotFound(_)) => Ok(deps),
            Err(e) => Err(e),
        }
    }

    fn read_metadata(&mut self, file: MetaFile) -> Result<Option<&String>> {
        if let Some(ref files) = self.metadata {
            return Ok(files.get(&file));
        }

        let mut metadata = Metadata::new();
        let mut matched_count = 0u8;
        let f = self.path.to_str().unwrap().to_string();
        let tar_reader = try!(crypto::get_artifact_reader(&f));
        let mut builder = reader::Builder::new();
        try!(builder.support_format(ReadFormat::All));
        try!(builder.support_filter(ReadFilter::All));
        let mut reader = try!(builder.open_stream(tar_reader));
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
                        Err(_) => return Err(Error::MetaFileMalformed(matched_type.unwrap())),
                    }
                }
                Ok(None) => (),
                Err(_) => return Err(Error::MetaFileMalformed(matched_type.unwrap())),
            }

            if matched_count == METAFILE_REGXS.len() as u8 {
                break;
            }
        }
        self.metadata = Some(metadata);
        Ok(self.metadata.as_ref().unwrap().get(&file))
    }
}

#[cfg(test)]
mod test {
    use std::env;
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn reading_artifact_metadata() {
        let mut hart = PackageArchive::new(fixtures()
                                               .join("core-hab-sup-0.4.0-20160416170100.hab"));
        let ident = hart.ident().unwrap();
        assert_eq!(ident.origin, "core");
        assert_eq!(ident.name, "hab-sup");
        assert_eq!(ident.version, Some("0.4.0".to_string()));
        assert_eq!(ident.release, Some("20160416170100".to_string()));
    }

    pub fn exe_path() -> PathBuf {
        env::current_exe().unwrap()
    }

    pub fn root() -> PathBuf {
        exe_path().parent().unwrap().parent().unwrap().parent().unwrap().join("tests")
    }

    pub fn fixtures() -> PathBuf {
        root().join("fixtures")
    }
}
