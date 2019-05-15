use handlebars::{self,
                 Handlebars,
                 Helper,
                 HelperDef,
                 RenderContext,
                 RenderError};
use serde::Serialize;
use std::result::Result;

// Kubernetes manifest template
const MANIFESTFILE: &str = include_str!("../defaults/KubernetesManifest.hbs");

#[derive(Clone, Copy)]
pub struct QuoteHelper;

impl QuoteHelper {
    pub fn escape(to_escape: &str) -> String {
        // two for enclosing quotes
        let mut escaped = String::with_capacity(to_escape.len() + 2);

        escaped.push('"');
        for c in to_escape.chars() {
            match c {
                '\\' | '"' => escaped.push('\\'),
                _ => (),
            };
            escaped.push(c);
        }
        escaped.push('"');
        escaped
    }
}

impl HelperDef for QuoteHelper {
    fn call(&self,
            h: &Helper<'_>,
            _r: &Handlebars,
            rc: &mut RenderContext<'_>)
            -> Result<(), RenderError> {
        let to_escape = h.param(0)
                         .ok_or_else(|| {
                             RenderError::new(&format!("Expected exactly one parameter for {}",
                                                       h.name()))
                         })?
                         .value()
                         .as_str()
                         .ok_or_else(|| RenderError::new("Expected a string parameter"))?;
        let escaped = QuoteHelper::escape(to_escape);

        rc.writer.write_all(escaped.into_bytes().as_ref())?;
        Ok(())
    }
}

static QUOTE: QuoteHelper = QuoteHelper;

pub struct Renderer {
    hb: Handlebars,
}

impl Renderer {
    fn new() -> Self {
        let mut hb = Handlebars::new();

        hb.register_helper("quote", Box::new(QUOTE));
        hb.register_escape_fn(handlebars::no_escape);

        Self { hb }
    }

    fn render<T>(&self, data: &T) -> String
        where T: Serialize
    {
        // The Result::expect() usage in this function is justified by
        // the fact that errors can only come from the crate
        // programmer (e.g they messed-up the manifest template or
        // didn't check the user input).
        self.hb
            .template_render(MANIFESTFILE, &data)
            .expect("Rendering of manifest from template failed")
    }
}

pub fn render<T>(data: &T) -> String
    where T: Serialize
{
    Renderer::new().render(data)
}

#[cfg(test)]
mod tests {
    use super::QuoteHelper;

    #[test]
    fn test_quote_helper() {
        let strings = vec![(r#"abc"#, r#""abc""#),
                           (r#"escape " quote"#, r#""escape \" quote""#),
                           (r#"\"double escape\""#, r#""\\\"double escape\\\"""#),
                           (r#"backslash at the end\"#, r#""backslash at the end\\""#),
                           (r#""#, r#""""#),];

        for pair in strings {
            let input = pair.0;
            let expected = pair.1;
            let output = QuoteHelper::escape(input);
            assert_eq!(output, *expected);
        }
    }
}
