use handlebars::{Context,
                 Handlebars,
                 Helper,
                 HelperDef,
                 HelperResult,
                 Output,
                 RenderContext,
                 RenderErrorReason};

#[derive(Clone, Copy)]
pub struct ToYamlHelper;

impl HelperDef for ToYamlHelper {
    fn call<'reg: 'rc, 'rc>(&self,
                            h: &Helper<'rc>,
                            _: &'reg Handlebars<'reg>,
                            _: &'rc Context,
                            _rc: &mut RenderContext<'reg, 'rc>,
                            out: &mut dyn Output)
                            -> HelperResult {
        let param = h.param(0)
                     .ok_or_else(|| {
                         RenderErrorReason::Other("Expected 1 parameter for \"toYaml\"".to_string())
                     })?
                     .value();
        let yaml = serde_yaml::to_string(param).map_err(|e| {
                       RenderErrorReason::Other(format!("Can't serialize parameter to YAML: {}", e))
                   })?;
        out.write(yaml.as_ref())?;
        Ok(())
    }
}

pub static TO_YAML: ToYamlHelper = ToYamlHelper;
