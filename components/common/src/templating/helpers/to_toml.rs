use handlebars::{Handlebars,
                 Helper,
                 HelperDef,
                 RenderContext,
                 RenderError};
use toml;

use super::super::RenderResult;

#[derive(Clone, Copy)]
pub struct ToTomlHelper;

impl HelperDef for ToTomlHelper {
    fn call(&self, h: &Helper<'_>, _: &Handlebars, rc: &mut RenderContext<'_>) -> RenderResult<()> {
        let param = h.param(0)
                     .ok_or_else(|| RenderError::new("Expected 1 parameter for \"toToml\""))?
                     .value();
        let bytes = toml::ser::to_vec(&param).map_err(|e| {
                                                 RenderError::new(format!("Can't serialize \
                                                                           parameter to TOML: {}",
                                                                          e))
                                             })?;
        rc.writer.write_all(bytes.as_ref())?;
        Ok(())
    }
}

pub static TO_TOML: ToTomlHelper = ToTomlHelper;
