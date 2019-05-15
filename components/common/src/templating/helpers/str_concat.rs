use handlebars::{Handlebars,
                 Helper,
                 HelperDef,
                 RenderContext};

use super::super::RenderResult;

#[derive(Clone, Copy)]
pub struct StrConcatHelper;

impl HelperDef for StrConcatHelper {
    fn call(&self, h: &Helper<'_>, _: &Handlebars, rc: &mut RenderContext<'_>) -> RenderResult<()> {
        let list: Vec<String> = h.params()
                                 .iter()
                                 .map(handlebars::ContextJson::value)
                                 .filter(|v| !v.is_object())
                                 .map(|v| v.to_string().replace("\"", ""))
                                 .collect();

        rc.writer.write_all(list.concat().into_bytes().as_ref())?;
        Ok(())
    }
}

pub static STR_CONCAT: StrConcatHelper = StrConcatHelper;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_concat_helper() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("strConcat", Box::new(STR_CONCAT));
        let expected = "foobarbaz";
        assert_eq!(expected,
                   handlebars.template_render("{{strConcat \"foo\" \"bar\" \"baz\"}}", &json!({}))
                             .unwrap());
    }
}
