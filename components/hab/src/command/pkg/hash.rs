use crate::hcore::crypto::hash;

use crate::error::Result;

pub fn start(src: &str) -> Result<()> {
    let h = hash::hash_file(&src)?;
    println!("{}  {}", h, src);
    Ok(())
}
