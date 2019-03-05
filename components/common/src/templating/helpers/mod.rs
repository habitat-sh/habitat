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

mod each_alive;
mod pkg_path_for;
mod str_concat;
mod str_join;
mod str_replace;
mod to_json;
mod to_lowercase;
mod to_toml;
mod to_uppercase;
mod to_yaml;

use serde::Serialize;
use serde_json::{self,
                 Value as Json};

pub use self::{each_alive::EACH_ALIVE,
               pkg_path_for::PKG_PATH_FOR,
               str_concat::STR_CONCAT,
               str_join::STR_JOIN,
               str_replace::STR_REPLACE,
               to_json::TO_JSON,
               to_lowercase::TO_LOWERCASE,
               to_toml::TO_TOML,
               to_uppercase::TO_UPPERCASE,
               to_yaml::TO_YAML};

// Taken from `handlebars::context::JsonTruthy`. The trait is marked public but it's in a private
// module. It's super useful so let's pull it into here.
pub trait JsonTruthy {
    fn is_truthy(&self) -> bool;
}

impl JsonTruthy for Json {
    fn is_truthy(&self) -> bool {
        match *self {
            Json::Bool(ref i) => *i,
            Json::Number(ref n) => n.as_f64().map(|f| f.is_normal()).unwrap_or(false),
            Json::Null => false,
            Json::String(ref i) => !i.is_empty(),
            Json::Array(ref i) => !i.is_empty(),
            Json::Object(ref i) => !i.is_empty(),
        }
    }
}

/// Helper which will serialize to Json the given reference or return `Json::Null`
fn to_json<T>(src: &T) -> Json
    where T: Serialize
{
    serde_json::to_value(src).unwrap_or(Json::Null)
}
