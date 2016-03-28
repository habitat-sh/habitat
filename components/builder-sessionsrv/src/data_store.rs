// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use redis;

use config::Config;
use error::Result;

pub struct DataStore {
    conn: Option<redis::Connection>,
}

impl DataStore {
    pub fn new() -> Self {
        DataStore { conn: None }
    }

    pub fn open(&mut self, config: &Config) -> Result<()> {
        let client = try!(redis::Client::open(config));
        let conn = try!(client.get_connection());
        self.conn = Some(conn);
        Ok(())
    }
}
