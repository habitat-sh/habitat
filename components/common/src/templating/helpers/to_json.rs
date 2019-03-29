use handlebars::{Handlebars,
                 Helper,
                 HelperDef,
                 RenderContext,
                 RenderError};
use serde_json;

use super::super::RenderResult;

#[derive(Clone, Copy)]
pub struct ToJsonHelper;

impl HelperDef for ToJsonHelper {
    fn call(&self, h: &Helper<'_>, _: &Handlebars, rc: &mut RenderContext<'_>) -> RenderResult<()> {
        let param = h.param(0)
                     .ok_or_else(|| RenderError::new("Expected 1 parameter for \"toJson\""))?
                     .value();
        let json = serde_json::to_string_pretty(param).map_err(|e| {
                       RenderError::new(format!("Can't serialize parameter to JSON: {}", e))
                   })?;
        rc.writer.write_all(json.into_bytes().as_ref())?;
        Ok(())
    }
}

pub static TO_JSON: ToJsonHelper = ToJsonHelper;
