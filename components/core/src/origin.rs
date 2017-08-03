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

use error::{Error, Result};
use regex::Regex;


pub fn validate_origin(origin: &str) -> Result<()> {
    if origin.len() > 255 {
        return Err(Error::InvalidOrigin);
    }

    let valid_origin = Regex::new(r"^[a-z0-9][\w\-_]*$").expect("Unable to compile regex");

    match valid_origin.is_match(origin) {
        false => Err(Error::InvalidOrigin),
        true => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin() {
        let valid_origin = "my-1st_origin";
        let long_origin = "o".repeat(256);
        let capitalized_origin = "Origin";
        let invalid_chars_origin = "origin!";

        assert!(validate_origin(valid_origin).is_ok());
        assert!(validate_origin(&long_origin).is_err());
        assert!(validate_origin(capitalized_origin).is_err());
        assert!(validate_origin(invalid_chars_origin).is_err());
    }
}
