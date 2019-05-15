use std::{io,
          path::Path};

pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    unimplemented!();
}
