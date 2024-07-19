// All Clap v4 Validators used by different crates are maintained here.
//

// Value parser for URL
#[derive(Clone)]
pub struct UrlValueParser;

impl clap_v4::builder::TypedValueParser for UrlValueParser {
    type Value = String;

    fn parse_ref(&self,
                 cmd: &clap_v4::Command,
                 arg: Option<&clap_v4::Arg>,
                 value: &std::ffi::OsStr)
                 -> Result<Self::Value, clap_v4::Error> {
        let result = url::Url::parse(value.to_string_lossy().as_ref());
        if result.is_err() {
            let mut err =
                clap_v4::Error::new(clap_v4::error::ErrorKind::ValueValidation).with_cmd(cmd);
            if let Some(arg) = arg {
                err.insert(clap_v4::error::ContextKind::InvalidArg,
                           clap_v4::error::ContextValue::String(arg.to_string()));
            }
            err.insert(clap_v4::error::ContextKind::InvalidValue,
                       clap_v4::error::ContextValue::String(format!("`{}`: {}",
                                                                    value.to_string_lossy(),
                                                                    result.err().unwrap(),)));
            Err(err)
        } else {
            Ok(value.to_str().unwrap().to_string())
        }
    }
}

/// Struct implementing validator for Hart Package Identifier or Hart File Package Path
///
/// Try parsing the `InstallSource` from the given input, if it's a valid `InstallSource` then the
/// given package identifier is a valid one.
#[derive(Clone)]
pub struct HabPackageInstallSourceValueParser;

use crate::command::package::install::InstallSource;
use std::str::FromStr;

impl clap_v4::builder::TypedValueParser for HabPackageInstallSourceValueParser {
    type Value = String;

    fn parse_ref(&self,
                 cmd: &clap_v4::Command,
                 arg: Option<&clap_v4::Arg>,
                 value: &std::ffi::OsStr)
                 -> Result<Self::Value, clap_v4::Error> {
        let val = value.to_str().unwrap().to_string();

        let result = InstallSource::from_str(&val);
        if result.is_err() {
            let mut err =
                clap_v4::Error::new(clap_v4::error::ErrorKind::ValueValidation).with_cmd(cmd);
            if let Some(arg) = arg {
                err.insert(clap_v4::error::ContextKind::InvalidArg,
                           clap_v4::error::ContextValue::String(arg.to_string()));
            }
            err.insert(clap_v4::error::ContextKind::InvalidValue,
                       clap_v4::error::ContextValue::String(format!("`{}`: {}",
                                                                    value.to_string_lossy(),
                                                                    result.err().unwrap(),)));
            Err(err)
        } else {
            Ok(value.to_str().unwrap().to_string())
        }
    }
}
