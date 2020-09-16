use crate::{error::Result,
            hcore::crypto::Blake2bHash};

pub fn start(src: &str) -> Result<()> {
    let h = Blake2bHash::from_file(&src)?;
    println!("{}  {}", h, src);
    Ok(())
}
