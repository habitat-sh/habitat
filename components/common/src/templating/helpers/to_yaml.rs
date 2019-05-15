use handlebars::{Handlebars,
                 Helper,
                 HelperDef,
                 RenderContext,
                 RenderError};
use serde_yaml;

use super::super::RenderResult;

#[derive(Clone, Copy)]
pub struct ToYamlHelper;

impl HelperDef for ToYamlHelper {
    fn call(&self, h: &Helper<'_>, _: &Handlebars, rc: &mut RenderContext<'_>) -> RenderResult<()> {
        let param = h.param(0)
                     .ok_or_else(|| RenderError::new("Expected 1 parameter for \"toYaml\""))?
                     .value();
        let yaml = serde_yaml::to_string(param).map_err(|e| {
                                                   RenderError::new(format!("Can't serialize \
                                                                             parameter to YAML: \
                                                                             {}",
                                                                            e))
                                               })?;
        rc.writer.write_all(yaml.into_bytes().as_ref())?;
        Ok(())
    }
}

pub static TO_YAML: ToYamlHelper = ToYamlHelper;
