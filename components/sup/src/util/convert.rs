// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

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
