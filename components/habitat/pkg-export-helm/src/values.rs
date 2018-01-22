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

extern crate habitat_pkg_export_docker as export_docker;

use std::io::Write;

use export_docker::Result;

pub struct Values {
    values: Vec<ValuesEntry>,
}

impl Values {
    pub fn new() -> Self {
        Values { values: Vec::new() }
    }

    pub fn add_entry(&mut self, variable: &str, value: &str) {
        self.values.push(ValuesEntry {
            variable: variable.to_owned(),
            value: value.to_owned(),
        });
    }

    pub fn generate(&self, write: &mut Write) -> Result<()> {
        let mut out = "".to_owned();
        for entry in &self.values {
            out = out + &format!("{}: \"{}\"\n", entry.variable, entry.value);
        }

        write.write(out.as_bytes())?;

        Ok(())
    }
}

struct ValuesEntry {
    variable: String,
    value: String,
}
