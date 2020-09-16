use crate::hcore::crypto::hash::Blake2bHash;

use crate::error::Result;

pub fn start(src: &str) -> Result<()> {
    let h = Blake2bHash::from_file(&src)?;
    println!("{}  {}", h, src);
    Ok(())
}
