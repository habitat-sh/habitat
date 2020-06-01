use crate::error::Error;
use serde::de::DeserializeOwned;
use std::{error::Error as StdError,
          fs::File,
          io::Read,
          path::Path};

pub trait ConfigFile: DeserializeOwned + Sized {
    type Error: StdError + From<Error>;

    fn from_file<T: AsRef<Path>>(filepath: T) -> Result<Self, Self::Error> {
        let mut file = match File::open(filepath.as_ref()) {
            Ok(f) => f,
            Err(e) => {
                return Err(Self::Error::from(Error::ConfigFileIO(filepath.as_ref()
                                                                         .to_path_buf(),
                                                                 e)));
            }
        };
        let mut raw = String::new();
        match file.read_to_string(&mut raw) {
            Ok(_) => (),
            Err(e) => {
                return Err(Self::Error::from(Error::ConfigFileIO(filepath.as_ref()
                                                                         .to_path_buf(),
                                                                 e)));
            }
        }
        Self::from_raw(&raw)
    }

    fn from_raw(raw: &str) -> Result<Self, Self::Error> {
        let value = toml::from_str(&raw).map_err(Error::ConfigFileSyntax)?;
        Ok(value)
    }
}
