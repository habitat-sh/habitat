// Validators used for validating inputs

// TODO: These are pub(crate) for now, because this is jut a POC implementation, eventually this
// should move into `habitat_core::common` as `pub` with proper documentation.

// Value parser for URL
#[derive(Clone)]
pub(crate) struct UrlValueParser;

impl clap::builder::TypedValueParser for UrlValueParser {
    type Value = String;

    fn parse_ref(&self,
                 cmd: &clap::Command,
                 arg: Option<&clap::Arg>,
                 value: &std::ffi::OsStr)
                 -> Result<Self::Value, clap::Error> {
        let result = url::Url::parse(value.to_string_lossy().as_ref());
        if result.is_err() {
            let mut err = clap::Error::new(clap::error::ErrorKind::ValueValidation).with_cmd(cmd);
            if let Some(arg) = arg {
                err.insert(clap::error::ContextKind::InvalidArg,
                           clap::error::ContextValue::String(arg.to_string()));
            }
            err.insert(clap::error::ContextKind::InvalidValue,
                       clap::error::ContextValue::String(format!("`{}`: {}",
                                                                 value.to_string_lossy(),
                                                                 result.err().unwrap(),)));
            Err(err)
        } else {
            Ok(value.to_str().unwrap().to_string())
        }
    }
}

use habitat_core::package::PackageIdent;
use std::str::FromStr;

// Hart file parser
#[derive(Clone)]
pub(crate) struct HabHartIdParser;

impl clap::builder::TypedValueParser for HabHartIdParser {
    type Value = String;

    fn parse_ref(&self,
                 cmd: &clap::Command,
                 arg: Option<&clap::Arg>,
                 value: &std::ffi::OsStr)
                 -> Result<Self::Value, clap::Error> {
        let val = value.to_str().unwrap().to_string();
        let result = {
            if val.ends_with(".hart") {
                if std::path::Path::new(&val).is_file() {
                    Ok(())
                } else {
                    Err("Not a valid HART file Path.".to_string())
                }
            } else {
                match PackageIdent::from_str(&val) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("{}", e)),
                }
            }
        };

        if result.is_err() {
            let mut err = clap::Error::new(clap::error::ErrorKind::ValueValidation).with_cmd(cmd);
            if let Some(arg) = arg {
                err.insert(clap::error::ContextKind::InvalidArg,
                           clap::error::ContextValue::String(arg.to_string()));
            }
            err.insert(clap::error::ContextKind::InvalidValue,
                       clap::error::ContextValue::String(format!("`{}`: {}",
                                                                 value.to_string_lossy(),
                                                                 result.err().unwrap(),)));
            Err(err)
        } else {
            Ok(value.to_str().unwrap().to_string())
        }
    }
}
