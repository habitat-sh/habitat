pub mod download;
pub mod export;
pub mod generate;
pub mod import;
pub mod upload;
pub mod upload_latest;

use std::{fs::File,
          io::{BufRead,
               BufReader},
          path::Path};

use crate::{error::{Error,
                    Result},
            hcore};

// shared between origin::key::upload and origin::key::upload_latest
fn get_name_with_rev(keyfile: &Path, expected_vsn: &str) -> Result<String> {
    let f = File::open(&keyfile)?;
    let f = BufReader::new(f);
    let mut lines = f.lines();
    match lines.next() {
        Some(val) => {
            let val = val?;
            if val != expected_vsn {
                let msg = format!("Unsupported version: {}", &val);
                return Err(Error::HabitatCore(hcore::Error::CryptoError(msg)));
            }
        }
        None => {
            let msg = "Corrupt key file, can't read file version".to_string();
            return Err(Error::HabitatCore(hcore::Error::CryptoError(msg)));
        }
    }
    let name_with_rev = match lines.next() {
        Some(val) => val?,
        None => {
            let msg = "Corrupt key file, can't read name with rev".to_string();
            return Err(Error::HabitatCore(hcore::Error::CryptoError(msg)));
        }
    };
    Ok(name_with_rev)
}
