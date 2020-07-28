use std::{fs::File,
          io,
          path::Path};

use crate::hcore::crypto::RingKey;

use crate::error::Result;

pub fn start<P>(ring: &str, cache: P) -> Result<()>
    where P: AsRef<Path>
{
    let latest = RingKey::get_latest_pair_for(ring, &cache)?;
    let path = RingKey::cached_path(&latest.name_with_rev(), &cache)?;
    let mut file = File::open(&path)?;
    debug!("Streaming file contents of {} to standard out",
           &path.display());
    io::copy(&mut file, &mut io::stdout())?;
    Ok(())
}
