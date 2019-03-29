
use clap::ArgMatches;
use serde_json;
use std::{result::Result as StdResult,
          str::FromStr};

use crate::export_docker::Result;

use crate::error::Error;

#[derive(Clone, Debug)]
pub struct EnvironmentVariable {
    pub name:  String,
    pub value: String,
}

impl EnvironmentVariable {
    pub fn from_args(matches: &ArgMatches<'_>) -> Result<Vec<Self>> {
        let mut environment = Vec::new();

        if let Some(bind_args) = matches.values_of("ENVIRONMENT") {
            for arg in bind_args {
                let variable = arg.parse::<Self>()?;

                environment.push(variable);
            }
        };

        Ok(environment)
    }

    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "name": self.name,
            "value": self.value,
        })
    }
}

impl FromStr for EnvironmentVariable {
    type Err = Error;

    fn from_str(env_str: &str) -> StdResult<Self, Self::Err> {
        let values: Vec<&str> = env_str.splitn(2, '=').collect();
        if values.len() != 2 || values[0].is_empty() {
            return Err(Error::InvalidEnvironmentVariable(env_str.to_string()));
        }

        Ok(EnvironmentVariable { name:  values[0].to_string(),
                                 value: values[1].to_string(), })
    }
}

#[cfg(test)]
mod tests {
    use super::EnvironmentVariable;

    #[test]
    fn test_env_var_from_str() {
        let valid = vec![("foo=bar", "foo", "bar"),
                         ("foo=", "foo", ""),
                         ("foo=bar=baz", "foo", "bar=baz"),];
        let invalid = vec!["foo", "=bar"];

        for (raw, name, value) in valid {
            let result = raw.parse::<EnvironmentVariable>();
            assert!(result.is_ok(), "failed to parse valid raw string '{}'", raw);
            let env_var = result.unwrap();
            assert_eq!(env_var.name, name);
            assert_eq!(env_var.value, value);
        }

        for raw in invalid {
            assert!(raw.parse::<EnvironmentVariable>().is_err(),
                    "invalid raw string '{}' parsed successfully to EnvironmentVariable",
                    raw);
        }
    }
}
