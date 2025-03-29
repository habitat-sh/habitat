use handlebars::{Context,
                 Handlebars,
                 Helper,
                 HelperDef,
                 HelperResult,
                 Output,
                 RenderContext};

#[derive(Clone, Copy)]
pub struct StrConcatHelper;

impl HelperDef for StrConcatHelper {
    fn call<'reg: 'rc, 'rc>(&self,
                            h: &Helper<'rc>,
                            _: &'reg Handlebars<'reg>,
                            _: &'rc Context,
                            _rc: &mut RenderContext<'reg, 'rc>,
                            out: &mut dyn Output)
                            -> HelperResult {
        let list: Vec<String> = h.params()
                                 .iter()
                                 .map(handlebars::PathAndJson::value)
                                 .filter(|v| !v.is_object())
                                 .map(|v| v.to_string().replace('\"', ""))
                                 .collect();

        out.write(list.concat().as_ref())?;
        Ok(())
    }
}

pub static STR_CONCAT: StrConcatHelper = StrConcatHelper;

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_concat_helper() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("strConcat", Box::new(STR_CONCAT));
        let expected = "foobarbaz";
        assert_eq!(expected,
                   handlebars.render_template("{{strConcat \"foo\" \"bar\" \"baz\"}}", &json!({}))
                             .unwrap());
    }
}
