// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use std::{collections::{HashMap,
                        HashSet},
          ffi::OsString,
          fmt::Debug,
          path::PathBuf};

pub trait IndentedToString {
    fn indented_to_string(&self, spaces: &str, repeat: usize) -> String;
}

// indented to string
macro_rules! its(
    {$value:expr, $spaces:expr, $repeat:expr} => {
        {
            $value.indented_to_string($spaces, $repeat)
        }
    };
);

// default indented to string
macro_rules! dits(
    {$value:expr} => {
        its!($value, "  ", 0)
    };
);

pub struct IndentedStructFormatter {
    name: String,
    fields: Vec<(String, String)>,
    spaces: String,
    repeat: usize,
}

impl IndentedStructFormatter {
    pub fn new(name: &str, spaces: &str, repeat: usize) -> Self {
        Self {
            name: name.to_string(),
            fields: Vec::new(),
            spaces: spaces.to_string(),
            repeat,
        }
    }

    pub fn add_string(&mut self, field_name: &str, field_value: String) {
        self.fields.push((field_name.to_string(), field_value));
    }

    pub fn add_debug<T: Debug>(&mut self, field_name: &str, field_value: &T) {
        self.add_string(field_name, format!("{:?}", field_value));
    }

    pub fn add<T: IndentedToString>(&mut self, field_name: &str, field_value: &T) {
        let spaces = self.spaces.to_string();
        let repeat = self.repeat + 1;
        self.add_string(field_name, its!(field_value, &spaces, repeat));
    }

    pub fn fmt(&mut self) -> String {
        let indent = self.spaces.repeat(self.repeat);
        let field_indent = self.spaces.repeat(self.repeat + 1);
        // 5 - space between name and opening brace, opening brace, newline
        // after opening brace, closing brace, terminating zero
        let mut capacity = self.name.len() + 5 + indent.len();
        for pair in &self.fields {
            // 4 - colon after name, space, comma, newline after value
            capacity += field_indent.len() + pair.0.len() + 4 + pair.1.len();
        }
        let mut str = String::with_capacity(capacity);
        str.push_str(&format!("{} {{\n", self.name,));
        for pair in &self.fields {
            str.push_str(&format!("{}{}: {},\n", field_indent, pair.0, pair.1));
        }
        str.push_str(&format!("{}}}", indent));
        str
    }
}

impl IndentedToString for u32 {
    fn indented_to_string(&self, _: &str, _: usize) -> String { self.to_string() }
}

impl IndentedToString for PathBuf {
    fn indented_to_string(&self, _: &str, _: usize) -> String { self.display().to_string() }
}

impl IndentedToString for OsString {
    fn indented_to_string(&self, _: &str, _: usize) -> String { self.to_string_lossy().to_string() }
}

impl<T: IndentedToString> IndentedToString for Option<T> {
    fn indented_to_string(&self, spaces: &str, repeat: usize) -> String {
        match self {
            Some(ref v) => format!("Some({})", its!(v, spaces, repeat)),
            None => "None".to_string(),
        }
    }
}

impl<V: IndentedToString> IndentedToString for HashMap<PathBuf, V> {
    fn indented_to_string(&self, spaces: &str, repeat: usize) -> String {
        let mut paths = self.keys().collect::<Vec<&PathBuf>>();
        paths.sort();
        let indent = spaces.repeat(repeat + 1);
        let mut str = String::new();
        str.push_str("{\n");
        for path in paths {
            str.push_str(&format!(
                "{}{}: {},\n",
                indent,
                path.display(),
                its!(self.get(path).unwrap(), spaces, repeat + 1),
            ));
        }
        str.push_str(&format!("{}}}", spaces.repeat(repeat)));
        str
    }
}

impl IndentedToString for HashSet<PathBuf> {
    fn indented_to_string(&self, spaces: &str, repeat: usize) -> String {
        let mut paths = self.iter().collect::<Vec<&PathBuf>>();
        paths.sort();
        let indent = spaces.repeat(repeat + 1);
        let mut str = String::new();
        str.push_str("{\n");
        for path in paths {
            str.push_str(&format!("{}{},\n", indent, path.display(),));
        }
        str.push_str(&format!("{}}}", spaces.repeat(repeat)));
        str
    }
}
