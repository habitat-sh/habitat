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
        if param.is_object() {
            let toml = toml::ser::to_string(&param).map_err(|e| {
                                                       RenderError::new(format!("Can't serialize \
                                                                                 parameter to \
                                                                                 TOML: {}",
                                                                                e))
                                                   })?;
            rc.writer.write_all(toml.into_bytes().as_ref())?;
        } else {
            let mut value = String::new();
            serde::Serialize::serialize(
                &param,
                toml::ser::ValueSerializer::new(&mut value)
            ).map_err(|e| {
                                RenderError::new(format!("Can't serialize \
                                                         parameter to TOML: \
                                                         {}",
                                                        e))
                            })?;
            rc.writer.write_all(value.into_bytes().as_ref())?;
        }
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

    #[test]
    fn toml_tables_are_serialized_for_str_array() {
        let mut config = Table::new();
        config.insert("build_targets".to_string(),
                      Value::Array(vec![Value::String("x86_64-linux".to_string())]));
        let mut cfg = Table::new();
        cfg.insert("cfg".to_string(), Value::Table(config));

        assert!(toml::to_string(&cfg).is_ok(),
                "Should be able to render a Table directly as TOML");

        // We should also be able to render the same thing through handlebars
        let mut renderer = TemplateRenderer::new();
        let tmpl = "build_targets = {{toToml cfg.build_targets}}";
        renderer.register_template_string("t", tmpl)
                .expect("Couldn't register template!");
        let result = renderer.render("t", &cfg);
        assert!(result.is_ok());
        let generated_cfg = result.unwrap();
        assert_eq!(generated_cfg, "build_targets = [\"x86_64-linux\"]");
    }

    #[test]
    fn toml_tables_are_serialized_for_values() {
        let mut config = Table::new();
        config.insert("log_level".to_string(), Value::String("info".to_string()));
        config.insert("log_path".to_string(), Value::String("/tmp".to_string()));
        config.insert("job_timeout".to_string(), Value::Integer(60));
        config.insert("build_targets".to_string(),
                      Value::Array(vec![Value::String("x86_64-linux".to_string())]));
        config.insert("features_enabled".to_string(),
                      Value::String("".to_string()));

        let mut cfg = Table::new();
        cfg.insert("cfg".to_string(), Value::Table(config));

        assert!(toml::to_string(&cfg).is_ok(),
                "Should be able to render a Table directly as TOML");

        // We should also be able to render the same thing through handlebars
        let mut renderer = TemplateRenderer::new();
        let tmpl = r#"log_path = {{toToml cfg.log_path}}
job_timeout = {{cfg.job_timeout}}
build_targets = {{toToml cfg.build_targets}}
features_enabled = "{{cfg.features_enabled}}""#;
        renderer.register_template_string("t", tmpl)
                .expect("Couldn't register template!");
        assert!(renderer.render("t", &cfg).is_ok());
    }

    #[test]
    fn toml_tables_are_serialized_for_values_and_partial_toml() {
        let mut config = Table::new();
        config.insert("log_level".to_string(), Value::String("info".to_string()));
        config.insert("log_path".to_string(), Value::String("/tmp".to_string()));
        config.insert("job_timeout".to_string(), Value::Integer(60));
        config.insert("build_targets".to_string(),
                      Value::Array(vec![Value::String("x86_64-linux".to_string())]));
        config.insert("features_enabled".to_string(),
                      Value::String("".to_string()));

        let mut backend = Table::new();
        backend.insert("backend".to_string(), Value::String("local".to_string()));
        config.insert("archive".to_string(), Value::Table(backend));

        let mut cfg = Table::new();
        cfg.insert("cfg".to_string(), Value::Table(config));

        assert!(toml::to_string(&cfg).is_ok(),
                "Should be able to render a Table directly as TOML");

        // We should also be able to render the same thing through handlebars
        let mut renderer = TemplateRenderer::new();
        let tmpl = r#"log_path = "{{cfg.log_path}}"
job_timeout = {{cfg.job_timeout}}
build_targets = {{toToml cfg.build_targets}}
features_enabled = "{{cfg.features_enabled}}"

[archive]
{{toToml cfg.archive}}"#;
        renderer.register_template_string("t", tmpl)
                .expect("Couldn't register template!");
        assert!(renderer.render("t", &cfg).is_ok());
    }
}
