use std::io::Write;

use crate::{export_docker::Result,
            export_k8s::QuoteHelper};

pub struct Values {
    values: Vec<ValuesEntry>,
}

impl Values {
    pub fn new() -> Self { Values { values: Vec::new() } }

    pub fn add_entry(&mut self, variable: &str, value: &str) {
        self.values
            .push(ValuesEntry { variable: variable.to_owned(),
                                value:    value.to_owned(), });
    }

    pub fn generate(&self, write: &mut dyn Write) -> Result<()> {
        let mut out = "".to_owned();
        for entry in &self.values {
            out = out
                  + &format!("{}: {}\n",
                             entry.variable,
                             QuoteHelper::escape(&entry.value));
        }

        write.write_all(out.as_bytes())?;

        Ok(())
    }
}

struct ValuesEntry {
    variable: String,
    value:    String,
}
