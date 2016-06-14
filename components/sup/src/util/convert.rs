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

use std::collections::BTreeMap;

use toml;
use rustc_serialize::json::Json;

pub fn toml_to_json(value: toml::Value) -> Json {
    match value {
        toml::Value::String(s) => Json::String(format!("{}", s)),
        toml::Value::Integer(i) => Json::I64(i as i64),
        toml::Value::Float(i) => Json::F64(i as f64),
        toml::Value::Boolean(b) => Json::Boolean(b),
        toml::Value::Datetime(s) => Json::String(format!("{}", s)),
        toml::Value::Array(a) => toml_vec_to_json(a),
        toml::Value::Table(t) => toml_table_to_json(t),
    }
}

pub fn toml_vec_to_json(toml: Vec<toml::Value>) -> Json {
    let mut mvec = vec![];
    for x in toml.iter() {
        mvec.push(toml_to_json(x.clone()))
    }
    Json::Array(mvec)
}

// Translates a toml table to a mustache datastructure.
pub fn toml_table_to_json(toml: BTreeMap<String, toml::Value>) -> Json {
    let mut hashmap = BTreeMap::new();
    for (key, value) in toml.iter() {
        hashmap.insert(format!("{}", key), toml_to_json(value.clone()));
    }
    Json::Object(hashmap)
}
