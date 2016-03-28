// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::fmt;
use std::io::Read;

use hyper;
use hyper::header::{Accept, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use rustc_serialize::json;

use error::{Error, Result};

// JW TODO: do not hardcode these. Make available through configuration file.
const GH_CLIENT_ID: &'static str = "e98d2a94787be9af9c00";
const GH_CLIENT_SECRET: &'static str = "e5ff94188e3cf01d42f3e2bcbbe4faabe11c71ba";

#[derive(RustcDecodable, RustcEncodable)]
pub struct AuthOk {
    pub access_token: String,
    pub scope: String,
    pub token_type: String,
}

impl AuthOk {
    pub fn has_scope(&self, grant: &str) -> bool {
        self.scope.split(",").collect::<Vec<&str>>().iter().any(|&p| p == grant)
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct AuthErr {
    pub error: String,
    pub error_description: String,
    pub error_uri: String,
}

impl fmt::Display for AuthErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "err={}, desc={}, uri={}",
               self.error,
               self.error_description,
               self.error_uri)
    }
}

#[derive(RustcDecodable, RustcEncodable)]
pub enum AuthResp {
    AuthOk,
    AuthErr,
}

pub fn authenticate(code: &str) -> Result<String> {
    let client = hyper::Client::new();
    let url = hyper::Url::parse(&format!("https://github.com/login/oauth/access_token?client_id={}&client_secret={}&code={}", GH_CLIENT_ID, GH_CLIENT_SECRET, code)).unwrap();
    let request = client.post(url)
                        .header(Accept(vec![qitem(Mime(TopLevel::Application,
                                                       SubLevel::Json,
                                                       vec![]))]));
    let mut response = try!(request.send());
    if response.status.is_success() {
        let mut encoded = String::new();
        try!(response.read_to_string(&mut encoded));
        match json::decode(&encoded) {
            Ok(msg @ AuthOk {..}) => {
                let scope = "user:email".to_string();
                if msg.has_scope(&scope) {
                    Ok(msg.access_token)
                } else {
                    Err(Error::MissingScope(scope))
                }
            }
            Err(_) => {
                let err: AuthErr = try!(json::decode(&encoded));
                Err(Error::from(err))
            }
        }
    } else {
        Err(Error::HTTP(response.status))
    }
}
