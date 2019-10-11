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
use serde::Serialize;
use serde_json::{self,
                 Value as Json};

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
