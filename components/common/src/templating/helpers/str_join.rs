use handlebars::{Context,
                 Handlebars,
                 Helper,
                 HelperDef,
                 HelperResult,
                 Output,
                 RenderContext,
                 RenderErrorReason};

#[derive(Clone, Copy)]
pub struct StrJoinHelper;

impl HelperDef for StrJoinHelper {
    fn call<'reg: 'rc, 'rc>(&self,
                            h: &Helper<'rc>,
                            _: &'reg Handlebars<'reg>,
                            _: &'rc Context,
                            _rc: &mut RenderContext<'reg, 'rc>,
                            out: &mut dyn Output)
                            -> HelperResult {
        let list: Vec<String> = h.param(0)
                                 .and_then(|v| v.value().as_array())
                                 .ok_or_else(|| {
                                     RenderErrorReason::ParamTypeMismatchForName(
                    stringify!($helper_fn_name),
                    "0".to_owned(),
                    "string".to_owned())
                                 })?
                                 .iter()
                                 .filter(|v| !v.is_object())
                                 .map(|v| v.to_string().replace('\"', ""))
                                 .collect();
        let seperator = h.param(1)
                         .and_then(|v| v.value().as_str())
                         .ok_or_else(||
                                     RenderErrorReason::ParamTypeMismatchForName(
                    stringify!($helper_fn_name),
                    "0".to_owned(),
                    "string".to_owned()))?;

        out.write(list.join(seperator).as_ref())?;
        Ok(())
    }
}

pub static STR_JOIN: StrJoinHelper = StrJoinHelper;

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_join_helper() {
        let json = json!({
            "list": ["foo", "bar", "baz"]
        });
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("strJoin", Box::new(STR_JOIN));
        let expected = "foo,bar,baz";
        assert_eq!(expected,
                   handlebars.render_template("{{strJoin list \",\"}}", &json)
                             .unwrap());
    }

    #[test]
    fn test_join_helper_errors_on_objects() {
        let json = json!({
            "list": [{
                "foo": "bar"
            }]
        });
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("strJoin", Box::new(STR_JOIN));
        assert_eq!("",
                   handlebars.render_template("{{strJoin list \",\"}}", &json)
                             .unwrap());
    }
}
