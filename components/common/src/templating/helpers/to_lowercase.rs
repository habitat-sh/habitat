use handlebars::{Handlebars,
                 Helper,
                 HelperDef,
                 RenderContext,
                 RenderError};

use super::super::RenderResult;

#[derive(Clone, Copy)]
pub struct ToLowercaseHelper;

impl HelperDef for ToLowercaseHelper {
    fn call(&self, h: &Helper<'_>, _: &Handlebars, rc: &mut RenderContext<'_>) -> RenderResult<()> {
        let param =
            h.param(0).and_then(|v| v.value().as_str()).ok_or_else(|| {
                                                            RenderError::new("Expected a string \
                                                                              parameter for \
                                                                              \"toLowercase\"")
                                                        })?;
        rc.writer
          .write_all(param.to_lowercase().into_bytes().as_ref())?;
        Ok(())
    }
}

pub static TO_LOWERCASE: ToLowercaseHelper = ToLowercaseHelper;
