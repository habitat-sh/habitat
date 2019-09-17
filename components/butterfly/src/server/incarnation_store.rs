//! Provide the means to persist a Supervisor's own incarnation
//! number across restarts.

use crate::{error::{Error,
                    Result},
            member::Incarnation};
use habitat_core::fs::atomic_write;
use std::{fs::File,
          io::{self,
               Read},
          num,
          path::{Path,
                 PathBuf}};

/// Provide storage of an incarnation number that can persist across
/// Supervisor restarts.
#[derive(Clone, Debug)]
pub struct IncarnationStore {
    /// Path to the file that backs this IncarnationStore.
    path: PathBuf,
}

impl IncarnationStore {
    /// Create a new `IncarnationStore`, backed by the file at `path`.
    pub fn new<P>(path: P) -> Self
        where P: AsRef<Path>
    {
        IncarnationStore { path: path.as_ref().to_path_buf(), }
    }

    /// Ensure that the `IncarnationStore` is backed by a suitable
    /// file on disk. If the file does not already exist, create it
    /// with an initial incarnation number of 0. If the file does
    /// exist, an error will be returned if the contents cannot be
    /// parsed.
    pub fn initialize(&mut self) -> Result<()> {
        if self.path.exists() {
            self.load()?;
        } else {
            self.store(Incarnation::default())?;
        }
        Ok(())
    }

    /// Returns the incarnation value found within the file.
    ///
    /// Returns an error if the file cannot be read or parsed for any
    /// reason.
    pub fn load(&self) -> Result<Incarnation> {
        let into_err = |e: io::Error| Error::IncarnationIO(self.path.clone(), e);

        let mut file = File::open(&self.path).map_err(into_err)?;
        let mut incarnation = String::new();
        file.read_to_string(&mut incarnation).map_err(into_err)?;
        incarnation.trim()
                   .parse()
                   .map_err(|e: num::ParseIntError| Error::IncarnationParse(self.path.clone(), e))
    }

    /// Store the given `new_incarnation` to disk.
    pub fn store(&mut self, new_incarnation: Incarnation) -> Result<()> {
        let into_err = |e: io::Error| Error::IncarnationIO(self.path.clone(), e);

        atomic_write(&self.path, new_incarnation.to_string()).map_err(into_err)?;
        // TODO (CM): set appropriate file permissions here
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use io::Write;
    use mktemp::Temp;
    use std::path::Path;

    #[test]
    fn happy_path() {
        let dir = Temp::new_dir().expect("Could not create temp dir");
        let path = dir.as_ref().join("my_incarnation_store");
        assert!(!path.exists());

        let mut incarnation_store = IncarnationStore::new(&path);
        incarnation_store.initialize()
                         .expect("couldn't initialize incarnation store");

        incarnation_store.store(Incarnation::from(100))
                         .expect("Couldn't store value");
        let i = incarnation_store.load().expect("Couldn't load value");
        assert_eq!(i, Incarnation::from(100));
    }

    #[test]
    fn retrieving_from_a_nonexistent_file_is_an_error() {
        let path = Path::new("/omg/wtf/this-is-not-a-real-file");
        assert!(!path.exists(),
                "The path {:?} shouldn't exist, but it does",
                path);

        let i = IncarnationStore::new(&path);
        assert!(i.load().is_err());
    }

    #[test]
    fn unparseable_incarnation_file_is_an_error() {
        let path = Temp::new_file().expect("Could not create temp file");
        let mut buffer = File::create(&path).expect("could not create file");
        buffer.write_all(b"this is not a u64")
              .expect("could not write file");

        let i = IncarnationStore::new(&path);
        assert!(i.load().is_err());
    }

    #[test]
    fn can_load_valid_values_from_disk() {
        let path = Temp::new_file().expect("Could not create temp file");
        let mut buffer = File::create(&path).expect("could not create file");
        buffer.write_all(b"42").expect("could not write file");

        let i = IncarnationStore::new(&path);
        assert_eq!(i.load().unwrap(), Incarnation::from(42));
    }

    #[test]
    fn can_store_a_new_incarnation_number() {
        let path = Temp::new_file().expect("Could not create temp file");

        let mut i = IncarnationStore::new(&path);
        i.store(Incarnation::from(2112))
         .expect("Should be able to store the number");

        assert_eq!(i.load().unwrap(), Incarnation::from(2112));
    }

    #[test]
    fn initialize_creates_file_with_the_default_incarnation_if_file_does_not_exist() {
        let dir = Temp::new_dir().expect("Could not create temp dir");
        let path = dir.as_ref().join("my_incarnation_store");
        assert!(!path.exists());

        let mut i = IncarnationStore::new(&path);
        i.initialize()
         .expect("`initialize` should return the initial value");

        assert!(path.exists(),
                "The incarnation file should have been created by calling `initialize`");

        let initial_value = i.load().expect("Could not load incarnation number");
        assert_eq!(initial_value, Incarnation::default());
    }

    #[test]
    fn initialize_returns_an_error_if_file_exists_but_is_unparseable() {
        let tempfile = Temp::new_file().expect("Could not create temp file");
        let path = tempfile.as_ref();

        let mut buffer = File::create(&path).expect("could not create file");
        buffer.write_all(b"this, also, is not a u64")
              .expect("could not write file");

        assert!(path.exists());

        let mut i = IncarnationStore::new(&path);
        assert!(i.initialize().is_err());
    }
}
