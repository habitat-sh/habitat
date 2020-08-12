use crate::error::{Error,
                   Result};
use std::io::{Error as IoError,
              Write};

use serde::Serialize;
use serde_json::Value as Json;
use tabwriter::TabWriter;

// Returns a library object that implements elastic tabstops
pub fn tabw() -> TabWriter<Vec<u8>> { TabWriter::new(Vec::new()) }

// Format strings with elastic tab stops
pub fn tabify(mut tw: TabWriter<Vec<u8>>, s: &str) -> Result<String> {
    write!(&mut tw, "{}", s)?;
    tw.flush()?;
    let inner = tw.into_inner().map_err(|e| {
                                    IoError::new(e.error().kind(),
                                                 "Unable to flush tabwriter buffer to inner.")
                                })?;
    String::from_utf8(inner).map_err(Error::StringFromUtf8Error)
}

pub trait TabularText {
    fn as_tabbed(&self) -> Result<String>;
}

pub trait PortableText: Serialize {
    fn as_json(&self) -> Result<Json> {
        serde_json::to_value(self).map_err(Error::RenderContextSerialization)
    }
}

impl<T: Serialize> PortableText for T {}
