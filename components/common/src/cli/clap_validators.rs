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

/// Struct implementing validator for Habitat Origin
///
/// Validates with `habitat_core::origin::Origin::validate` function.
#[derive(Clone)]
pub struct HabOriginValueParser;

use habitat_core::origin::Origin;

impl clap_v4::builder::TypedValueParser for HabOriginValueParser {
    type Value = Origin;

    fn parse_ref(&self,
                 cmd: &clap_v4::Command,
                 arg: Option<&clap_v4::Arg>,
                 value: &std::ffi::OsStr)
                 -> Result<Self::Value, clap_v4::Error> {
        let val = value.to_str().unwrap().to_string();

        let result = habitat_core::origin::Origin::validate(val);
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
            Ok(Origin::from_str(value.to_str().unwrap()).unwrap())
        }
    }
}

/// Struct implimenting validator that validates the value is a valid path
#[derive(Clone)]
pub struct FileExistsValueParser;

use std::path::PathBuf;

impl clap_v4::builder::TypedValueParser for FileExistsValueParser {
    type Value = PathBuf;

    fn parse_ref(&self,
                 cmd: &clap_v4::Command,
                 arg: Option<&clap_v4::Arg>,
                 value: &std::ffi::OsStr)
                 -> Result<Self::Value, clap_v4::Error> {
        parse_ref_internal(cmd, arg, value, false, false, "is not a valid file").map(Into::<PathBuf>::into)
    }
}

// TODO: This will be used by `hab config` (this implements the functionality of
// `file_exists_or_stdin` validator in Clap v2.
/// Struct implementing validator that validates the valie is a valid 'file' or 'stdin'
#[derive(Clone)]
pub struct FileExistsOrStdinValueParser;

impl clap_v4::builder::TypedValueParser for FileExistsOrStdinValueParser {
    type Value = String;

    fn parse_ref(&self,
                 cmd: &clap_v4::Command,
                 arg: Option<&clap_v4::Arg>,
                 value: &std::ffi::OsStr)
                 -> Result<Self::Value, clap_v4::Error> {
        parse_ref_internal(cmd, arg, value, false, true, "is not a valid file or stdin")
    }
}

/// Struct implemeitng validator that valudates the value is a valid directory
///
/// Internally uses `ValidPathValueParser`
#[derive(Clone)]
pub struct DirExistsValueParser;

impl clap_v4::builder::TypedValueParser for DirExistsValueParser {
    type Value = std::path::PathBuf;

    fn parse_ref(&self,
                 cmd: &clap_v4::Command,
                 arg: Option<&clap_v4::Arg>,
                 value: &std::ffi::OsStr)
                 -> Result<Self::Value, clap_v4::Error> {
        parse_ref_internal(cmd, arg, value, true, false, "is not a valid directory").map(|x| {
                                                                                        x.into()
                                                                                    })
    }
}

// Checks whether a give path is a file or a dir or stdin, used internally by the validators
//
// eg. FileExistsValueParser will call this command with both `check_dir` and `check_stdin` set to
// false. DirExistsValueParser will call this command with `check_dir` set to `true` and
// `check_stdin` set to `false` etc.
fn check_valid_file_dir_stdin(path: &std::path::Path, check_dir: bool, check_stdin: bool) -> bool {
    let mut is_valid = path.is_file();

    if !is_valid && check_dir {
        is_valid = path.is_dir();
    }

    is_valid = if check_stdin {
        if is_valid {
            is_valid
        } else if let Some(v) = path.to_str() {
            v == "-"
        } else {
            false
        }
    } else {
        is_valid
    };

    is_valid
}

fn parse_ref_internal(cmd: &clap_v4::Command,
                      arg: Option<&clap_v4::Arg>,
                      value: &std::ffi::OsStr,
                      check_dir: bool,
                      check_stdin: bool,
                      err_str: &str)
                      -> Result<String, clap_v4::Error> {
    let val = value.to_str().unwrap().to_string();

    let result = std::path::Path::new(&val);
    if !check_valid_file_dir_stdin(result, check_dir, check_stdin) {
        let mut err = clap_v4::Error::new(clap_v4::error::ErrorKind::ValueValidation).with_cmd(cmd);
        if let Some(arg) = arg {
            err.insert(clap_v4::error::ContextKind::InvalidArg,
                       clap_v4::error::ContextValue::String(arg.to_string()));
        }
        err.insert(clap_v4::error::ContextKind::InvalidValue,
                   clap_v4::error::ContextValue::String(format!("`{}`: {}",
                                                                value.to_string_lossy(),
                                                                err_str,)));
        Err(err)
    } else {
        Ok(value.to_str().unwrap().to_string())
    }
}
/// Validate a given file is a 'toml' file or contains valid package idents only.
///
/// Packages to be installed can be read from a 'toml' file or a file containing package idents
/// only. The actual validation of whether the contents of the 'toml' file are correct is performed
/// by the actual command that calls this validation. This validation will succeed if a file is a
/// 'toml' file (possibly in wrong format) or the file contains packaged identifiers, one per line.
#[derive(Clone)]
pub struct TomlOrPkgIdentFileValueParser;

use crate::cli::{file_into_idents,
                 is_toml_file};

impl clap_v4::builder::TypedValueParser for TomlOrPkgIdentFileValueParser {
    type Value = String;

    fn parse_ref(&self,
                 cmd: &clap_v4::Command,
                 arg: Option<&clap_v4::Arg>,
                 value: &std::ffi::OsStr)
                 -> Result<Self::Value, clap_v4::Error> {
        let val = value.to_str().unwrap().to_string();

        if is_toml_file(&val) {
            return Ok(val);
        }

        let result = file_into_idents(&val);
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
            Ok(val)
        }
    }
}

/// Validates whether given input is a valid Package Identifier
///
/// This validator returns success if the given input is a valid simple Package Identifier or a
/// fully qualified PackageIdentifier
///
/// Use `value_parser = HabPkgIdentValueParser::simple()` for simple Package Identifier.
/// Use `value_parser = HabPkgIdentValueParser::full()` for fully qualified Package Identifier.
#[derive(Clone)]
pub struct HabPkgIdentValueParser {
    fully_qualified: bool,
}

impl HabPkgIdentValueParser {
    /// For Simple Package Identifier of the form 'origin/name'
    pub fn simple() -> Self { Self { fully_qualified: false, } }

    /// For Full Package Identifier of the form 'origin/name/version/release'
    pub fn full() -> Self { Self { fully_qualified: true, } }
}

use habitat_core::package::ident::{FullyQualifiedPackageIdent,
                                   PackageIdent};

impl clap_v4::builder::TypedValueParser for HabPkgIdentValueParser {
    type Value = PackageIdent;

    fn parse_ref(&self,
                 cmd: &clap_v4::Command,
                 arg: Option<&clap_v4::Arg>,
                 value: &std::ffi::OsStr)
                 -> Result<Self::Value, clap_v4::Error> {
        let val = value.to_str().unwrap().to_string();

        let result = if self.fully_qualified {
            FullyQualifiedPackageIdent::from_str(&val).err()
        } else {
            PackageIdent::from_str(&val).err()
        };

        if result.is_some() {
            let mut err =
                clap_v4::Error::new(clap_v4::error::ErrorKind::ValueValidation).with_cmd(cmd);
            if let Some(arg) = arg {
                err.insert(clap_v4::error::ContextKind::InvalidArg,
                           clap_v4::error::ContextValue::String(arg.to_string()));
            }
            err.insert(clap_v4::error::ContextKind::InvalidValue,
                       clap_v4::error::ContextValue::String(format!("`{}`: {}",
                                                                    value.to_string_lossy(),
                                                                    result.unwrap(),)));
            Err(err)
        } else {
            Ok(val.into())
        }
    }
}

// TODO: Add Unit tests for all validators
