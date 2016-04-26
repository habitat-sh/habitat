// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use toml;

use error::Error;

pub trait ConfigFile: Sized {
    type Error: std::error::Error + From<Error>;

    fn from_file<T: AsRef<Path>>(filepath: T) -> Result<Self, Self::Error> {
        let mut file = match File::open(filepath.as_ref()) {
            Ok(f) => f,
            Err(e) => return Err(Self::Error::from(Error::ConfigFileIO(e))),
        };
        let mut raw = String::new();
        match file.read_to_string(&mut raw) {
            Ok(_) => (),
            Err(e) => return Err(Self::Error::from(Error::ConfigFileIO(e))),
        }
        let mut parser = toml::Parser::new(&raw);
        match parser.parse() {
            Some(toml) => Self::from_toml(toml),
            None => {
                let msg = format_errors(&parser);
                Err(Self::Error::from(Error::ConfigFileSyntax(msg)))
            }
        }
    }

    fn from_toml(toml: toml::Table) -> Result<Self, Self::Error>;
}

fn format_errors(parser: &toml::Parser) -> String {
    let mut msg = String::new();
    for err in &parser.errors {
        let (loline, locol) = parser.to_linecol(err.lo);
        let (hiline, hicol) = parser.to_linecol(err.hi);
        msg.push_str(&format!("\t{}:{}-{}:{} error: {}\n",
                              loline,
                              locol,
                              hiline,
                              hicol,
                              err.desc));
    }
    msg
}
