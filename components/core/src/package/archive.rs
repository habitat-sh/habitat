// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;
use std::error;
use std::path::{Path, PathBuf};
use std::result;
use std::str::{self, FromStr};

use libarchive::writer;
use libarchive::reader::{self, Reader};
use libarchive::archive::{Entry, ReadFilter, ReadFormat, ExtractOption, ExtractOptions};
use regex::Regex;

use error::{Error, Result};
use crypto::{artifact, hash};
use package::{Identifiable, PackageIdent, MetaFile};

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
        hash::hash_file(&self.path)
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
    pub fn verify<P: AsRef<Path>>(&self, cache_key_path: &P) -> Result<(String, String)> {
        artifact::verify(&self.path, cache_key_path)
    }

    /// Given a package name and a path to a file as an `&str`, unpack
    /// the package.
    ///
    /// # Failures
    ///
    /// * If the package cannot be unpacked
    pub fn unpack(&self, fs_root_path: Option<&Path>) -> Result<()> {
        let root = fs_root_path.unwrap_or(Path::new("/"));
        let tar_reader = try!(artifact::get_archive_reader(&self.path));
        let mut builder = reader::Builder::new();
        try!(builder.support_format(ReadFormat::Gnutar));
        try!(builder.support_filter(ReadFilter::Xz));
        let mut reader = try!(builder.open_stream(tar_reader));
        let writer = writer::Disk::new();
        let mut extract_options = ExtractOptions::new();
        extract_options.add(ExtractOption::Time);
        try!(writer.set_options(&extract_options));
        try!(writer.set_standard_lookup());
        try!(writer.write(&mut reader, Some(root.to_string_lossy().as_ref())));
        try!(writer.close());
        Ok(())
    }

    fn read_deps(&mut self, file: MetaFile) -> Result<Vec<PackageIdent>> {
        let mut deps: Vec<PackageIdent> = vec![];
        match self.read_metadata(file) {
            Ok(Some(body)) => {
                debug!("body = [{}]", &body);
                let ids: Vec<String> = body.split("\n").map(|d| d.to_string()).collect();
                for id in &ids {
                    debug!("id = [{}]", &id);
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
        let tar_reader = try!(artifact::get_archive_reader(&self.path));
        let mut builder = reader::Builder::new();
        try!(builder.support_format(ReadFormat::Gnutar));
        try!(builder.support_filter(ReadFilter::Xz));
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

            let mut buf = String::new();
            loop {
                match reader.read_block() {
                    Ok(Some(bytes)) => {
                        match str::from_utf8(bytes) {
                            Ok(content) => {
                                buf.push_str(content.trim());
                            }
                            Err(_) => return Err(Error::MetaFileMalformed(matched_type.unwrap())),
                        }
                    }
                    Ok(None) => {
                        debug!("content = {}", &buf);
                        metadata.insert(matched_type.unwrap(), buf);
                        break;
                    }
                    Err(_) => return Err(Error::MetaFileMalformed(matched_type.unwrap())),
                }
            }//inner loop

            if matched_count == METAFILE_REGXS.len() as u8 {
                break;
            }
        }
        self.metadata = Some(metadata);
        Ok(self.metadata.as_ref().unwrap().get(&file))
    }
}

pub trait FromArchive: Sized {
    type Error: error::Error;

    fn from_archive(archive: &mut PackageArchive) -> result::Result<Self, Self::Error>;
}

#[cfg(test)]
mod test {
    use std::env;
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn reading_artifact_metadata() {
        let mut hart = PackageArchive::new(fixtures()
            .join("happyhumans-possums-8.1.4-20160427165340-x86_64-linux.hart"));
        let ident = hart.ident().unwrap();
        assert_eq!(ident.origin, "happyhumans");
        assert_eq!(ident.name, "possums");
        assert_eq!(ident.version, Some("8.1.4".to_string()));
        assert_eq!(ident.release, Some("20160427165340".to_string()));
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

    #[test]
    fn reading_artifact_deps() {
        let mut hart = PackageArchive::new(fixtures()
            .join("happyhumans-possums-8.1.4-20160427165340-x86_64-linux.hart"));
        let _ = hart.deps().unwrap();
        let _ = hart.tdeps().unwrap();
    }

    #[test]
    fn reading_artifact_large_tdeps() {
        let mut hart = PackageArchive::new(fixtures()
            .join("unhappyhumans-possums-8.1.4-20160427165340-x86_64-linux.hart"));
        let tdeps = hart.tdeps().unwrap();
        assert_eq!(1024, tdeps.len());
    }

}
