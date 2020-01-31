use crate::error::{Error,
                   Result};
use reqwest::{header::AsHeaderName,
              Response,
              StatusCode};
use std::fmt;

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

pub async fn ok_if(response: Response,
                   code: impl IntoIterator<Item = &reqwest::StatusCode>)
                   -> Result<Response> {
    debug!("Response Status: {:?}", response.status());
    if code.into_iter().any(|&code| code == response.status()) {
        Ok(response)
    } else {
        Err(err_from_response(response).await)
    }
}

pub async fn ok_if_unit(response: Response,
                        code: impl IntoIterator<Item = &reqwest::StatusCode>)
                        -> Result<()> {
    ok_if(response, code).await.map(|_| ())
}

pub fn get_header<K>(response: &Response, name: K) -> Result<String>
    where K: AsHeaderName
{
    let hdr_name = name.as_str().to_string();
    response.headers()
            .get(name)
            .ok_or_else(|| Error::MissingHeader(hdr_name.clone()))?
            .to_str()
            .map_err(|_| Error::InvalidHeader(hdr_name.clone()))
            .map(String::from)
}

pub async fn err_from_response(response: Response) -> Error {
    let status = response.status();
    if status == StatusCode::UNAUTHORIZED {
        return Error::APIError(status,
                               "Please check that you have specified a valid Personal Access \
                                Token."
                                       .to_string());
    }

    match response.text().await {
        Ok(buff) => {
            match serde_json::from_str::<NetError>(&buff) {
                Ok(err) => Error::APIError(status, err.to_string()),
                Err(_) => Error::APIError(status, buff),
            }
        }
        Err(_) => Error::APIError(status, String::new()),
    }
}
