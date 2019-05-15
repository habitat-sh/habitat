use std::io::Read;

use hyper::client::Response;
use serde;
use serde_json;

use crate::error::{Error,
                   Result};

pub fn decoded_response<T>(mut response: Response) -> Result<T>
    where T: serde::de::DeserializeOwned
{
    let mut encoded = String::new();
    response.read_to_string(&mut encoded).map_err(Error::IO)?;
    debug!("Body: {:?}", encoded);
    let thing = serde_json::from_str(&encoded).map_err(Error::Json)?;
    Ok(thing)
}
