use log::LevelFilter;
use log4rs::{append::console::ConsoleAppender,
             config::{Appender,
                      Config,
                      Root},
             encode::pattern::PatternEncoder,
             file::Deserializers};
use std::path::PathBuf;

mod env_logger_compatibility;

/// A `log4rs`
/// [PatternEncoder](https://docs.rs/log4rs/0.8.3/log4rs/encode/pattern/index.html)
/// format to mimic that of out-of-the-box `env_logger`.
const DEFAULT_PATTERN: &str = "[{d(%Y-%m-%dT%H:%M:%SZ)(utc)} {l} {module}] {message}{n}";

/// Initialize a log4rs-based logging system.
///
/// Absent any other configuration, a basic logging configuration will
/// be used. Users can also specify configuration using a YAML
/// configuration file (see
/// https://docs.rs/log4rs/0.8.3/log4rs/#configuration).
///
/// We also provide a migration path from our previous
/// `env_logger`-based implementation. If a configuration file is
/// absent, but `RUST_LOG` is present in the environment, we will
/// coerce a `log4rs` configuration from that `env_logger`
/// configuration string.
pub fn init() {
    let file = configuration_file();
    if file.exists() {
        if let Err(e) = log4rs::init_file(&file, Deserializers::default()) {
            eprintln!("Logging configuration file '{}' not valid: {}; using default logging \
                       configuration",
                      file.display(),
                      e);
            log4rs::init_config(default_config()).expect("Tried setting the log configuration, \
                                                          but the global logger had already been \
                                                          set!");
        }
    } else {
        let config = env_logger_compatibility::from_env().unwrap_or_else(|| {
                                                             eprintln!("Logging configuration \
                                                                        file '{}' not found; \
                                                                        using default logging \
                                                                        configuration",
                                                                       file.display());
                                                             default_config()
                                                         });

        log4rs::init_config(config).expect("Tried setting the log configuration, but the global \
                                            logger had already been set!");
    }
}

/// The logging configuration that will be used if a configuration
/// file is not found when the Supervisor is started.
///
/// Logs everything at ERROR and above to standard output. This is
/// kept deliberately "bare bones" (for now), since `log4rs` allows for so much
/// flexibility to end users via its configuration file.
///
/// As we move forward to consolidate more of our output under a
/// unified logging interface (see
/// https://github.com/habitat-sh/habitat/issues/6587, but
/// particularly https://github.com/habitat-sh/habitat/issues/6584),
/// we _may_ want to further customize this default configuration, in
/// order to more closely adhere to our existing non-log-based output
/// (at that point, though, it may be better to define the default in
/// a YAML file and `include!` it here for readability)
fn default_config() -> Config {
    let stdout = ConsoleAppender::builder().encoder(Box::new(PatternEncoder::new(DEFAULT_PATTERN)))
                                           .build();
    // Calling `expect()` is OK because we have full control over this
    // configuration. It can only fail if we create an inconsistent
    // one (e.g., referred to an appender that we didn't
    // create). Also, see the test below.
    Config::builder().appender(Appender::builder().build("stdout", Box::new(stdout)))
                     .build(Root::builder().appender("stdout").build(LevelFilter::Error))
                     .expect("Default log4rs configuration should always be valid")
}

fn configuration_file() -> PathBuf {
    habitat_sup_protocol::sup_root(None).join("config")
                                        .join("log.yml")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This saves us from accidentally creating an inconsistent default
    /// configuration.
    #[test]
    fn default_configuration_does_not_fail() { default_config(); }
}
