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

use std::collections::BTreeMap;

use serde_json;
use toml;

pub fn toml_to_json(value: toml::Value) -> serde_json::Value {
    match value {
        toml::Value::String(s) => serde_json::Value::String(format!("{}", s)),
        toml::Value::Integer(i) => serde_json::Value::from(i as i64),
        toml::Value::Float(i) => serde_json::Value::from(i as f64),
        toml::Value::Boolean(b) => serde_json::Value::Bool(b),
        toml::Value::Datetime(s) => serde_json::Value::String(format!("{}", s)),
        toml::Value::Array(a) => toml_vec_to_json(a),
        toml::Value::Table(t) => toml_table_to_json(t),
    }
}

fn toml_vec_to_json(toml: Vec<toml::Value>) -> serde_json::Value {
    let mut mvec = vec![];
    for x in toml.iter() {
        mvec.push(toml_to_json(x.clone()))
    }
    serde_json::Value::Array(mvec)
}

// Translates a toml table to a mustache data structure.
fn toml_table_to_json(toml: BTreeMap<String, toml::Value>) -> serde_json::Value {
    let mut map = serde_json::Map::with_capacity(toml.len());
    for (key, value) in toml.iter() {
        map.insert(format!("{}", key), toml_to_json(value.clone()));
    }
    serde_json::Value::Object(map)
}
