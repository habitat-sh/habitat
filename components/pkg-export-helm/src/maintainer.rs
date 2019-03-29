
use clap::ArgMatches;
use serde_json;
use std::{result,
          str::FromStr,
          string::ToString};
use url::Url;

use crate::export_docker::Result;

use crate::error::Error;

#[derive(Clone, Debug)]
pub struct Maintainer {
    name:  String,
    email: Option<String>,
    url:   Option<String>,
}

impl Maintainer {
    pub fn from_args(matches: &ArgMatches<'_>) -> Result<Vec<Self>> {
        let mut maintainers = Vec::new();

        if let Some(args) = matches.values_of("MAINTAINER") {
            for arg in args {
                let m = arg.parse::<Self>()?;

                maintainers.push(m);
            }
        };

        Ok(maintainers)
    }

    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "name": self.name,
            "email": self.email,
            "url": self.url,
        })
    }
}

impl FromStr for Maintainer {
    type Err = Error;

    /// Creates a `Maintainer` struct from a string representation, which must be of the format
    /// `NAME[,EMAIL[,URL]]`.
    ///
    /// # Errors
    ///
    /// * `maintainer_str` is not of the format `NAME[,EMAIL[,URL]`
    /// * An invalid URL is specified
    fn from_str(maintainer_str: &str) -> result::Result<Self, Self::Err> {
        let values: Vec<&str> = maintainer_str.split(',').collect();
        if values.is_empty() || values.len() > 3 {
            return Err(Error::InvalidMaintainer(maintainer_str.to_owned()));
        }

        let name = values[0].to_string();
        // FIXME: Check validity of email address
        let email = values.get(1).map(|&s| s.to_owned());
        let url = values.get(2).map(|&s| s.to_owned());
        if let Some(ref u) = url {
            Url::parse(&u).map_err(|e| Error::InvalidUrl(u.to_owned(), format!("{}", e)))?;
        };

        Ok(Maintainer { name, email, url })
    }
}
