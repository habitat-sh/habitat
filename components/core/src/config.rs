// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

use std::{error::Error as StdError,
          fs::File,
          io::Read,
          path::Path};

use serde::de::DeserializeOwned;
use toml;

use crate::error::Error;

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
