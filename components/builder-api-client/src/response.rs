use crate::error::{Error,
                   Result};
use reqwest::{header::AsHeaderName,
              Response,
              StatusCode};
use std::{fmt,
          io::Read};

#[derive(Clone, Deserialize)]
#[serde(rename = "error")]
pub struct NetError {
    pub code: i32,
    pub msg:  String,
}

impl fmt::Display for NetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[err: {:?}, msg: {}]", self.code, self.msg)
    }
}

pub trait ResponseExt {
    fn ok_if(&mut self, code: reqwest::StatusCode) -> Result<()>;
    fn get_header<'a, K: AsHeaderName>(&'a self, name: K) -> Result<&'a str>;
}

impl ResponseExt for reqwest::Response {
    fn ok_if(&mut self, code: reqwest::StatusCode) -> Result<()> {
        debug!("Response Status: {:?}", self.status());

        if self.status() == code {
            Ok(())
        } else {
            Err(err_from_response(self))
        }
    }

    fn get_header<'a, K>(&'a self, name: K) -> Result<&'a str>
        where K: AsHeaderName
    {
        let hdr_name = name.as_str().to_string();
        self.headers()
            .get(name)
            .ok_or_else(|| Error::MissingHeader(hdr_name.clone()))?
            .to_str()
            .map_err(|_| Error::InvalidHeader(hdr_name.clone()))
    }
}

pub fn err_from_response(response: &mut Response) -> Error {
    if response.status() == StatusCode::UNAUTHORIZED {
        return Error::APIError(response.status(),
                               "Please check that you have specified a valid Personal Access \
                                Token."
                                       .to_string());
    }

    let mut buff = String::new();
    match response.read_to_string(&mut buff) {
        Ok(_) => {
            match serde_json::from_str::<NetError>(&buff) {
                Ok(err) => Error::APIError(response.status(), err.to_string()),
                Err(_) => Error::APIError(response.status(), buff),
            }
        }
        Err(_) => {
            buff.truncate(0);
            Error::APIError(response.status(), buff)
        }
    }
}
