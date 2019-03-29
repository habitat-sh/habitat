
use std::{fmt,
          result,
          str::FromStr};

use crate::error::Error;

#[derive(Clone, Debug)]
pub enum Topology {
    Standalone,
    Leader,
}

impl Topology {
    fn as_str(&self) -> &str {
        match *self {
            Topology::Leader => "leader",
            Topology::Standalone => "standalone",
        }
    }
}

impl fmt::Display for Topology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_str()) }
}

impl FromStr for Topology {
    type Err = Error;

    fn from_str(topology: &str) -> result::Result<Self, Self::Err> {
        match topology {
            "leader" => Ok(Topology::Leader),
            "standalone" => Ok(Topology::Standalone),
            _ => Err(Error::InvalidTopology(String::from(topology))),
        }
    }
}

impl Default for Topology {
    fn default() -> Topology { Topology::Standalone }
}
