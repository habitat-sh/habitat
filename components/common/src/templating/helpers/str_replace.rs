use handlebars::{Handlebars,
                 Helper,
                 HelperDef,
                 RenderContext,
                 RenderError};

use super::super::RenderResult;

#[derive(Clone, Copy)]
pub struct StrReplaceHelper;

impl HelperDef for StrReplaceHelper {
    fn call(&self, h: &Helper<'_>, _: &Handlebars, rc: &mut RenderContext<'_>) -> RenderResult<()> {
        let param =
            h.param(0).and_then(|v| v.value().as_str()).ok_or_else(|| {
                                                            RenderError::new("Expected 3 string \
                                                                              parameters for \
                                                                              \"strReplace\"")
                                                        })?;
        let old =
            h.param(1).and_then(|v| v.value().as_str()).ok_or_else(|| {
                                                            RenderError::new("Expected 3 string \
                                                                              parameters for \
                                                                              \"strReplace\"")
                                                        })?;
        let new =
            h.param(2).and_then(|v| v.value().as_str()).ok_or_else(|| {
                                                            RenderError::new("Expected 3 string \
                                                                              parameters for \
                                                                              \"strReplace\"")
                                                        })?;
        rc.writer
          .write_all(param.replace(old, new).into_bytes().as_ref())?;
        Ok(())
    }
}

pub static STR_REPLACE: StrReplaceHelper = StrReplaceHelper;
