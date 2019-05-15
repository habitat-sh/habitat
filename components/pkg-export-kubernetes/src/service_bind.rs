use clap::ArgMatches;
use serde_json;
use std::{result,
          str::FromStr};

use crate::{export_docker::Result,
            hcore::service::ServiceGroup};

use crate::error::Error;

#[derive(Clone, Debug)]
pub struct ServiceBind {
    pub name:          String,
    pub service_group: ServiceGroup,
}

impl ServiceBind {
    pub fn from_args(matches: &ArgMatches<'_>) -> Result<Vec<Self>> {
        let mut binds = Vec::new();

        if let Some(bind_args) = matches.values_of("BIND") {
            for arg in bind_args {
                let b = arg.parse::<Self>()?;

                binds.push(b);
            }
        };

        Ok(binds)
    }

    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "name": self.name,
            "service": self.service_group.service(),
            "group": self.service_group.group(),
        })
    }
}

impl FromStr for ServiceBind {
    type Err = Error;

    fn from_str(bind_str: &str) -> result::Result<Self, Self::Err> {
        let values: Vec<&str> = bind_str.split(':').collect();
        if values.len() != 2 {
            return Err(Error::InvalidBinding(bind_str.to_string()));
        }

        Ok(ServiceBind { name:          values[0].to_string(),
                         service_group: ServiceGroup::from_str(values[1])?, })
    }
}
