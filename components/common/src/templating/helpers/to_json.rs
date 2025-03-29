use handlebars::{Context,
                 Handlebars,
                 Helper,
                 HelperDef,
                 HelperResult,
                 Output,
                 RenderContext,
                 RenderErrorReason};

#[derive(Clone, Copy)]
pub struct ToJsonHelper;

impl HelperDef for ToJsonHelper {
    fn call<'reg: 'rc, 'rc>(&self,
                            h: &Helper<'rc>,
                            _: &'reg Handlebars<'reg>,
                            _: &'rc Context,
                            _rc: &mut RenderContext<'reg, 'rc>,
                            out: &mut dyn Output)
                            -> HelperResult {
        let param = h.param(0)
                     .ok_or_else(|| {
                         RenderErrorReason::ParamTypeMismatchForName(stringify!($helper_fn_name),
                                                                     "0".to_owned(),
                                                                     "string".to_owned())
                     })?
                     .value();
        let json = serde_json::to_string_pretty(param).map_err(RenderErrorReason::SerdeError)?;
        out.write(json.as_ref())?;
        Ok(())
    }
}

pub static TO_JSON: ToJsonHelper = ToJsonHelper;
