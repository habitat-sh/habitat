// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

//! Provide the means to persist a Supervisor's own incarnation
//! number across restarts.

use std::fs::{self, File};
use std::io::{BufWriter, Read, Write};
use std::path::PathBuf;

use error::{Error, Result};
use member::DEFAULT_INCARNATION;
use std::io;
use std::num;

/// Provide storage of an incarnation number that can persist across
/// Supervisor restarts.
#[derive(Clone, Debug)]
pub struct IncarnationStore {
    path: PathBuf,
}

impl From<PathBuf> for IncarnationStore {
    fn from(path: PathBuf) -> Self {
        IncarnationStore { path: path }
    }
}

impl IncarnationStore {
    /// Ensure that the `IncarnationStore` is backed by a suitable
    /// file on disk. If the file does not already exist, create it
    /// with an initial incarnation number of 0. If the file does
    /// exist, an error will be returned if the contents cannot be
    /// parsed.
    ///
    /// Returns the incarnation that is currently stored in the
    /// backing file (even if we just created it, in which case we
    /// return `0`).
    pub fn initialize(&self) -> Result<u64> {
        if !self.path.exists() {
            let initial_value = DEFAULT_INCARNATION;
            self.store(initial_value)?;
            // TODO (CM): set appropriate file permissions here
            Ok(initial_value)
        } else {
            self.retrieve()
        }
    }

    /// Returns the incarnation value found within the file.
    ///
    /// Returns an error if the file cannot be read or parsed for any
    /// reason.
    pub fn retrieve(&self) -> Result<u64> {
        File::open(&self.path)
            .map_err(|e: io::Error| Error::IncarnationIO(self.path.clone(), e))
            .and_then(|mut file| {
                let mut incarnation = String::new();
                file.read_to_string(&mut incarnation)
                    .map_err(|e: io::Error| Error::IncarnationIO(self.path.clone(), e))
                    .and_then(|_| {
                        incarnation.trim().parse().map_err(|e: num::ParseIntError| {
                            Error::IncarnationParse(self.path.clone(), e)
                        })
                    })
            })
    }

    /// Store the given `new_incarnation` to disk.
    pub fn store(&self, new_incarnation: u64) -> Result<()> {
        let tmp = self.path.with_extension("tmp");
        let f =
            File::create(&tmp).map_err(|e: io::Error| Error::IncarnationIO(self.path.clone(), e))?;
        let mut buf = BufWriter::new(f);
        buf.write(new_incarnation.to_string().as_bytes())
            .map_err(|e: io::Error| Error::IncarnationIO(self.path.clone(), e))?;
        buf.flush()
            .map_err(|e: io::Error| Error::IncarnationIO(self.path.clone(), e))?;
        fs::rename(&tmp, &self.path)
            .map_err(|e: io::Error| Error::IncarnationIO(self.path.clone(), e))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mktemp::Temp;
    use std::path::Path;

    #[test]
    fn happy_path() {
        let dir = Temp::new_dir().expect("Could not create temp dir");
        let path = dir.to_path_buf().join("my_incarnation_store");
        assert!(!path.exists());

        let incarnation_store = IncarnationStore::from(path);
        let initial_value = incarnation_store
            .initialize()
            .expect("couldn't initialize incarnation store");
        assert_eq!(initial_value, DEFAULT_INCARNATION);

        incarnation_store.store(100).expect("Couldn't store value");
        let i = incarnation_store
            .retrieve()
            .expect("Couldn't retrieve value");
        assert_eq!(i, 100);
    }

    #[test]
    fn retrieving_from_a_nonexistent_file_is_an_error() {
        let path = Path::new("/omg/wtf/this-is-not-a-real-file");
        assert!(
            !path.exists(),
            "The path {:?} shouldn't exist, but it does",
            path
        );

        let i: IncarnationStore = path.to_path_buf().into();
        assert!(i.retrieve().is_err());
    }

    #[test]
    fn unparseable_incarnation_file_is_an_error() {
        let tmpfile = Temp::new_file().expect("Could not create temp file");
        let path = tmpfile.to_path_buf();
        let mut buffer = File::create(&path).expect("could not create file");
        buffer
            .write_all(b"this is not a u64")
            .expect("could not write file");

        let i: IncarnationStore = path.into();
        assert!(i.retrieve().is_err());
    }

    #[test]
    fn can_retrieve_valid_values_from_disk() {
        let tmpfile = Temp::new_file().expect("Could not create temp file");
        let path = tmpfile.to_path_buf();

        let mut buffer = File::create(&path).expect("could not create file");
        buffer.write_all(b"42").expect("could not write file");

        let i: IncarnationStore = path.into();
        assert_eq!(i.retrieve().unwrap(), 42);
    }

    #[test]
    fn can_store_a_new_incarnation_number() {
        let tmpfile = Temp::new_file().expect("Could not create temp file");
        let path = tmpfile.to_path_buf();

        let i: IncarnationStore = path.into();
        i.store(2112).expect("Should be able to store the number");

        assert_eq!(i.retrieve().unwrap(), 2112);
    }

    #[test]
    fn initialize_creates_file_with_the_default_incarnation_if_file_does_not_exist() {
        let dir = Temp::new_dir().expect("Could not create temp dir");

        let path = dir.to_path_buf().join("my_incarnation_store");
        assert!(!path.exists());

        let i: IncarnationStore = path.clone().into();
        let initial_value = i
            .initialize()
            .expect("`initialize` should return the initial value");

        assert!(
            path.exists(),
            "The incarnation file should have been created by calling `initialize`"
        );

        assert_eq!(initial_value, DEFAULT_INCARNATION);
    }

    #[test]
    fn initialize_returns_an_error_if_file_exists_but_is_unparseable() {
        let tmpfile = Temp::new_file().expect("Could not create temp file");
        let path = tmpfile.to_path_buf();
        let mut buffer = File::create(&path).expect("could not create file");
        buffer
            .write_all(b"this, also, is not a u64")
            .expect("could not write file");

        assert!(path.exists());

        let i: IncarnationStore = path.into();
        assert!(i.initialize().is_err());
    }
}
