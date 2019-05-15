use handlebars::{Handlebars,
                 Helper,
                 HelperDef,
                 RenderContext,
                 RenderError};

use super::super::RenderResult;

#[derive(Clone, Copy)]
pub struct StrJoinHelper;

impl HelperDef for StrJoinHelper {
    fn call(&self, h: &Helper<'_>, _: &Handlebars, rc: &mut RenderContext<'_>) -> RenderResult<()> {
        let list: Vec<String> =
            h.param(0)
             .and_then(|v| v.value().as_array())
             .ok_or_else(|| RenderError::new("Expected 2 parameters for \"strJoin\""))?
             .iter()
             .filter(|v| !v.is_object())
             .map(|v| v.to_string().replace("\"", ""))
             .collect();
        let seperator = h.param(1)
                         .and_then(|v| v.value().as_str())
                         .ok_or_else(|| RenderError::new("Expected 2 parameters for \"strJoin\""))?;

        rc.writer
          .write_all(list.join(seperator).into_bytes().as_ref())?;
        Ok(())
    }
}

pub static STR_JOIN: StrJoinHelper = StrJoinHelper;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_join_helper() {
        let json = json!({
            "list": ["foo", "bar", "baz"]
        });
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("strJoin", Box::new(STR_JOIN));
        let expected = "foo,bar,baz";
        assert_eq!(expected,
                   handlebars.template_render("{{strJoin list \",\"}}", &json)
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
                   handlebars.template_render("{{strJoin list \",\"}}", &json)
                             .unwrap());
    }
}
