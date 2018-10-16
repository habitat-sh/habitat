use clap::{App, ArgMatches};
use std::fmt;
use std::iter::FromIterator;

#[derive(Debug)]
pub enum Expectation<'a> {
    Value(&'a str),
    Values(Vec<String>),
    Boolean(bool),
}

use self::Expectation::{Boolean, Value, Values};

#[derive(Debug)]
enum CliTestError {
    ValueMismatch(String, String, String),
    MultiValueMismatch(String, Vec<String>, Vec<String>),
    MissingFlag(String),
    MissingSubcmd(),
    ClapError(String),
}

impl fmt::Display for CliTestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let content = match self {
            CliTestError::ValueMismatch(flag, expected, actual) => format!(
                r#"
'{}' validation failed:
  Expected: '{}'
  Actual:   '{}'
"#,
                flag, expected, actual
            ),
            CliTestError::MultiValueMismatch(flag, expected, actual) => format!(
                r#"
'{}' validation failed:
  Expected: '{:#?}'
  Actual:   '{:#?}'
"#,
                flag, expected, actual,
            ),
            CliTestError::MissingFlag(flag) => format!("Expected {} flag was missing.", flag),
            CliTestError::MissingSubcmd() => {
                format!("Expected a subcommand, but none was specified")
            }
            CliTestError::ClapError(err) => format!(
                r#"
Command Parse error:
****************************************
{}
****************************************
"#,
                err.to_string()
            ),
        };
        write!(f, "{}", content)
    }
}

pub fn assert_command(app: App, cmd: &str, assertions: Vec<(&str, Expectation)>) {
    let cmd_vec = Vec::from_iter(cmd.split_whitespace());
    let errors = match app.get_matches_from_safe(cmd_vec) {
        Ok(matches) => match matches.subcommand() {
            (_, Some(matches)) => assert_matches(matches, assertions),
            (_, None) => vec![CliTestError::MissingSubcmd()],
        },
        Err(err) => vec![CliTestError::ClapError(err.to_string())],
    };

    if !errors.is_empty() {
        let error_string = errors
            .into_iter()
            .map(|error| format!("{}", error))
            .collect::<Vec<_>>()
            .join("\n");

        panic!(
            r#"

Failed assertions for command: '{}'

{}

"#,
            cmd, error_string
        );
    }
}

fn assert_matches(matches: &ArgMatches, assertions: Vec<(&str, Expectation)>) -> Vec<CliTestError> {
    let mut errs = Vec::new();

    for (flag, expected_value) in assertions {
        match expected_value {
            Value(expected) => match matches.value_of(flag) {
                Some(actual) => {
                    if actual != expected.to_string() {
                        errs.push(CliTestError::ValueMismatch(
                            flag.to_string(),
                            expected.to_string(),
                            actual.to_string(),
                        ));
                    }
                }
                None => {
                    errs.push(CliTestError::MissingFlag(flag.to_string()));
                }
            },
            Values(expected) => match matches.values_of(flag) {
                Some(actual_values) => {
                    let actual = actual_values.map(|v| v.to_string()).collect();
                    if actual != expected {
                        errs.push(CliTestError::MultiValueMismatch(
                            flag.to_string(),
                            expected,
                            actual,
                        ));
                    }
                }
                None => {
                    errs.push(CliTestError::MissingFlag(flag.to_string()));
                }
            },
            Boolean(expected) => {
                if matches.is_present(flag) != expected {
                    errs.push(CliTestError::ValueMismatch(
                        flag.to_string(),
                        expected.to_string(),
                        (!expected).to_string(),
                    ));
                }
            }
        }
    }

    errs
}

#[macro_export]
macro_rules! assertion {
    ($flag:expr, true) => {
        ($flag, $crate::cli_test_helpers::Expectation::Boolean(true))
    };
    ($flag:expr, false) => {
        ($flag, $crate::cli_test_helpers::Expectation::Boolean(false))
    };
    ($flag:expr, [ $( $value:expr ),+ ]) => {
        ($flag, $crate::cli_test_helpers::Expectation::Values(vec![ $( $value.to_string() ),* ]))
    };
    ($flag:expr, $value:expr) => {
        ($flag, $crate::cli_test_helpers::Expectation::Value($value))
    };
}

#[macro_export]
macro_rules! assert_cmd {
    ($app:expr, $cmd:expr) => {
        $crate::cli_test_helpers::assert_command($app, $cmd, Vec::new());
    };
    ($app:expr, $cmd:expr, $($key:expr => $value:tt),* ) => ({
        let mut assertions = Vec::new();

        $( assertions.push(assertion!($key, $value)); )+

            $crate::cli_test_helpers::assert_command($app, $cmd, assertions);
    });
}
