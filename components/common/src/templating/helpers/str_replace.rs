use handlebars::{Context,
                 Handlebars,
                 Helper,
                 HelperDef,
                 HelperResult,
                 Output,
                 RenderContext,
                 RenderErrorReason};

#[derive(Clone, Copy)]
pub struct StrReplaceHelper;

impl HelperDef for StrReplaceHelper {
    fn call<'reg: 'rc, 'rc>(&self,
                            h: &Helper<'rc>,
                            _: &'reg Handlebars<'reg>,
                            _: &'rc Context,
                            _rc: &mut RenderContext<'reg, 'rc>,
                            out: &mut dyn Output)
                            -> HelperResult {
        let param =
            h.param(0).and_then(|v| v.value().as_str()).ok_or_else(|| {
                RenderErrorReason::ParamTypeMismatchForName(
                    stringify!($helper_fn_name),
                    "0".to_owned(),
                    "string".to_owned())
                                                        })?;
        let old =
            h.param(1).and_then(|v| v.value().as_str()).ok_or_else(|| {
                RenderErrorReason::ParamTypeMismatchForName(
                    stringify!($helper_fn_name),
                    "0".to_owned(),
                    "string".to_owned())
                                                        })?;
        let new =
            h.param(2).and_then(|v| v.value().as_str()).ok_or_else(|| {
                RenderErrorReason::ParamTypeMismatchForName(
                    stringify!($helper_fn_name),
                    "0".to_owned(),
                    "string".to_owned())
                                                        })?;
        out.write(param.replace(old, new).as_ref())?;
        Ok(())
    }
}

pub static STR_REPLACE: StrReplaceHelper = StrReplaceHelper;
