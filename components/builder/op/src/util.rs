// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::hash::Hasher;
use std::path::PathBuf;

use bldr_sessionsrv::server as sessionsrv;
use fnv::FnvHasher;
use hcore::crypto::hash::hash_file;
use protocol::SHARD_COUNT;

const SHARD_MASK: u64 = 0x1FFF;

use config::Config;
use error::{Error, Result};

pub fn hash(config: Config) -> Result<()> {
    let path = PathBuf::from(config.file.unwrap());
    match hash_file(&path) {
        Ok(checksum) => {
            println!("{}", checksum);
            Ok(())
        }
        Err(e) => Err(Error::HabitatCore(e)),
    }
}

pub fn session(encoded_token: &str) -> Result<()> {
    let token = sessionsrv::decode_token(encoded_token).unwrap();
    println!("{:?}", token);
    Ok(())
}

pub fn shard(config: Config) -> u64 {
    let shard = config.shard.unwrap();
    match shard.parse::<u64>() {
        Ok(id) => {
            id & SHARD_MASK
        }
        Err(_) => {
            let mut hasher = FnvHasher::default();
            hasher.write(shard.as_bytes());
            let hval = hasher.finish();
            hval % SHARD_COUNT as u64
        }
    }
}
