use super::super::RenderResult;
use handlebars::{Handlebars,
                 Helper,
                 HelperDef,
                 RenderContext,
                 RenderError};

#[derive(Clone, Copy)]
pub struct ToTomlHelper;

impl HelperDef for ToTomlHelper {
    fn call(&self, h: &Helper<'_>, _: &Handlebars, rc: &mut RenderContext<'_>) -> RenderResult<()> {
        let param = h.param(0)
                     .ok_or_else(|| RenderError::new("Expected 1 parameter for \"toToml\""))?
                     .value();
        // Since `param` is a JSON object, this only works reliably if
        // `serde_json` has been compiled with the `preserve_order`
        // feature, since order *is* important for TOML (values must
        // be emitted before tables).
        let toml = toml::ser::to_string(&param).map_err(|e| {
                                                   RenderError::new(format!("Can't serialize \
                                                                             parameter to TOML: \
                                                                             {}",
                                                                            e))
                                               })?;
        rc.writer.write_all(toml.into_bytes().as_ref())?;
        Ok(())
    }
}

pub static TO_TOML: ToTomlHelper = ToTomlHelper;

#[cfg(test)]
mod tests {
    use crate::templating::TemplateRenderer;
    use toml::{self,
               value::{Table,
                       Value}};

    /// Inspired by a reported issue... here, we are creating the
    /// following TOML:
    ///
    ///     [[inputs.prometheus]]
    ///     urls = ["http://127.0.0.1:15000/metics"]
    ///     [inputs.prometheus.tags]
    ///     service = "service_name"
    ///
    /// The idea is that you have that config in a file, run `hab
    /// config apply` on it, and then attempt to render a file with
    /// the template
    ///
    ///     {{ toToml cfg }}
    ///
    /// The `toToml` helper should be able to handle this.
    #[test]
    fn toml_tables_are_serialized_appropriately() {
        let mut tags = Table::new();
        tags.insert("service".to_string(),
                    Value::String("service_name".to_string()));

        let urls = Value::Array(vec![Value::String("http://127.0.0.1:15000/metrics".to_string())]);

        let mut prometheus = Table::new();
        prometheus.insert("urls".to_string(), urls);
        prometheus.insert("tags".to_string(), Value::Table(tags));

        let mut inputs = Table::new();
        inputs.insert("prometheus".to_string(),
                      Value::Array(vec![Value::Table(prometheus)]));

        let mut the_stuff = Table::new();
        the_stuff.insert("inputs".to_string(), Value::Table(inputs));

        let mut cfg = Table::new();
        cfg.insert("cfg".to_string(), Value::Table(the_stuff));

        assert!(toml::to_string(&cfg).is_ok(),
                "Should be able to render a Table directly as TOML");

        // We should also be able to render the same thing through handlebars
        let mut renderer = TemplateRenderer::new();
        renderer.register_template_string("t", "{{ toToml cfg }}")
                .expect("Couldn't register template!");

        assert!(renderer.render("t", &cfg).is_ok());
    }
}
