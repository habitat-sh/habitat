//! Transition code to bridge users from `env_logger` to `log4rs`.
//!
//! Takes a configuration string (e.g., the content of the environment
//! variable `RUST_LOG`) for configuring `env_logger` and transforms
//! it (as much as possible) into a valid `log4rs` configuration.
//!
//! An important difference is in how module targeting works. With
//! `env_logger`, you could do something like this:
//!
//! ```text
//! RUST_LOG=debug,tokio=error
//! ```
//!
//! This would ensure that all `debug` and higher severity messages
//! were logged, but every module path that started with the string
//! "tokio" would only log for `error` and higher (Tokio can be rather
//! verbose at the lower logging levels).
//!
//! The `log4rs` library treats this target slightly differently; it
//! *must* be a Rust module path. That is, the same "tokio=error"
//! targeting string would match the module `tokio`, as well as
//! `tokio::foo`, `tokio::foo::bar`, etc. However, for this particular
//! example, there isn't actually a `tokio` module at the root;
//! instead you have things like `tokio_reactor`, `tokio_threadpool`,
//! etc. In `log4rs`, "tokio" will _not_ match `tokio_reactor` or
//! `tokio_threadpool`; thus, you would see debug-level messages from
//! these modules! To filter these in `log4rs`, you would need to
//! explicitly list the module prefixes, like so:
//!
//! ```text
//! RUST_LOG=debug,tokio_reactor=error,tokio_threadpool=error
//! ```
//!
//! In other words, `log4rs` is much stricter in its notion of
//! what qualifies as a module prefix (indeed, it actually _is_ a
//! module prefix, rather than a simple string prefix).
//!
//! Another notable difference is that while `env_logger` permits the
//! addition of a regular expression filter (e.g., only print messages
//! that match the expression "foo*"), we do *not*. Any filter that is
//! present will be ignored.
//!
//! We also don't honor the `RUST_LOG_STYLE` variable; no output is
//! colored at all.
//!
//! See https://docs.rs/env_logger/0.6.1/env_logger/ for more details.
use log::LevelFilter;
use log4rs::{append::console::ConsoleAppender,
             config::{Appender,
                      Config,
                      Logger,
                      Root},
             encode::pattern::PatternEncoder};
use std::{collections::HashMap,
          str::FromStr};

pub(super) fn from_env() -> Option<Config> {
    std::env::var("RUST_LOG").ok().map(|env_var| {
                                      eprintln!("RUST_LOG environment variable found; using it \
                                                 to configure log4rs");
                                      env_var.parse::<EnvLogConfig>()
                                             .expect("RUST_LOG parsing can't fail")
                                             .into()
                                  })
}

/// Encapsulates the relevant parts of an `env_logger` configuration
/// string that we care to replicate.
#[derive(Eq, PartialEq, Clone, Debug)]
struct EnvLogConfig {
    /// The base filtering level. Messages of lower severity than
    /// this will not be printed.
    root_level: LevelFilter,
    /// Optional filtering customizations on a per-module basis.
    module_filters: HashMap<String, LevelFilter>,
}

impl Default for EnvLogConfig {
    /// This is the same unsetting `RUST_LOG` (or, alternatively,
    /// `RUST_LOG=error`).
    fn default() -> Self {
        EnvLogConfig { root_level:     LevelFilter::Error,
                       module_filters: HashMap::new(), }
    }
}

impl Into<Config> for EnvLogConfig {
    /// Actually create a `log4rs` configuration. This is
    /// infallible because we'll always create something valid.
    fn into(self: Self) -> Config {
        let stdout = ConsoleAppender::builder().encoder(Box::new(PatternEncoder::new(super::DEFAULT_PATTERN)))
                                               .build();
        let loggers = self.module_filters
                          .into_iter()
                          .map(|(module, filter)| Logger::builder().build(module, filter));
        Config::builder().appender(Appender::builder().build("stdout", Box::new(stdout)))
                         .loggers(loggers)
                         .build(Root::builder().appender("stdout").build(self.root_level))
                         .expect("Should always generate a valid log4rs configuration")
    }
}

impl FromStr for EnvLogConfig {
    // We skip over any "bad" parts of the string and always end up
    // returning a valid `EnvLogConfig`.
    type Err = std::convert::Infallible;

    // Copied in large part (with appropriate modifications) from
    // [env_logger::filter::parse_spec](https://github.com/sebasmagri/env_logger/blob/811db3240831049788c5500ab9ed1b481d79476f/src/filter/mod.rs#L302).
    //
    // It's not necessarily how I would have written it from scratch,
    // but I felt like keeping the code as similar as possible to the
    // original would be better, both for comparison purposes, as well
    // as for adhering as closely as possible to the behavior we're
    // trying to emulate.
    fn from_str(spec: &str) -> Result<Self, Self::Err> {
        let mut config = EnvLogConfig::default();

        let mut parts = spec.split('/');
        let mods = parts.next();
        if parts.next().is_some() {
            eprintln!("Ignoring regex filter; not supported in log4rs");
        }
        if parts.next().is_some() {
            eprintln!("warning: invalid logging spec '{}', ignoring it (too many '/'s)",
                      spec);
            return Ok(config);
        }

        if let Some(m) = mods {
            for s in m.split(',') {
                if s.is_empty() {
                    continue;
                }
                let mut parts = s.split('=');
                let (log_level, name) =
                    match (parts.next(), parts.next().map(str::trim), parts.next()) {
                        (Some(part0), None, None) => {
                            // if the single argument is a log-level string or number,
                            // treat that as a global fallback
                            match part0.parse() {
                                Ok(num) => (num, None),
                                Err(_) => (LevelFilter::max(), Some(part0)),
                            }
                        }
                        (Some(part0), Some(""), None) => (LevelFilter::max(), Some(part0)),
                        (Some(part0), Some(part1), None) => {
                            match part1.parse() {
                                Ok(num) => (num, Some(part0)),
                                _ => {
                                    eprintln!("warning: invalid logging spec '{}', ignoring it",
                                              part1);
                                    continue;
                                }
                            }
                        }
                        _ => {
                            eprintln!("warning: invalid logging spec '{}', ignoring it", s);
                            continue;
                        }
                    };
                if let Some(target) = name {
                    config.module_filters.insert(target.to_string(), log_level);
                } else {
                    config.root_level = log_level
                };
            }
        };

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function for creating an `EnvLogConfig` struct with a
    /// minimum of boilerplate.
    fn config(root_level: LevelFilter, filters: Vec<(&str, LevelFilter)>) -> EnvLogConfig {
        EnvLogConfig { root_level,
                       module_filters: filters.into_iter()
                                              .map(|(m, f)| (m.to_string(), f))
                                              .collect() }
    }

    #[test]
    fn bare_module_name_gets_converted_to_maximum_filter_level() {
        assert_eq!("my::cool::module".parse::<EnvLogConfig>().unwrap(),
                   config(LevelFilter::Error,
                          vec![("my::cool::module", LevelFilter::Trace)]));

        assert_eq!("my::cool::module,another::nifty::module,snazzy".parse::<EnvLogConfig>()
                                                                   .unwrap(),
                   config(LevelFilter::Error,
                          vec![("my::cool::module", LevelFilter::Trace),
                               ("another::nifty::module", LevelFilter::Trace),
                               ("snazzy", LevelFilter::Trace)]));
    }

    #[test]
    fn can_disable_logging() {
        assert_eq!("off".parse::<EnvLogConfig>().unwrap(),
                   config(LevelFilter::Off, vec![]));

        assert_eq!("debug,my::chatty::module=off".parse::<EnvLogConfig>()
                                                 .unwrap(),
                   config(LevelFilter::Debug,
                          vec![("my::chatty::module", LevelFilter::Off)]));
    }

    #[test]
    fn parse_the_spec() {
        assert_eq!("debug".parse::<EnvLogConfig>().unwrap(),
                   config(LevelFilter::Debug, vec![]));

        assert_eq!("debug,foo::bar::baz=trace".parse::<EnvLogConfig>().unwrap(),
                   config(LevelFilter::Debug,
                          vec![("foo::bar::baz", LevelFilter::Trace)]));

        assert_eq!("monkeys,foo::bar::baz=trace".parse::<EnvLogConfig>()
                                                .unwrap(),
                   config(LevelFilter::Error,
                          vec![("monkeys", LevelFilter::Trace),
                               ("foo::bar::baz", LevelFilter::Trace)]));
    }

    #[test]
    fn regexes_are_ignored() {
        assert_eq!("debug/foo".parse::<EnvLogConfig>().unwrap(),
                   config(LevelFilter::Debug, vec![]));
        assert_eq!("debug,foo::bar::baz=trace/foo".parse::<EnvLogConfig>()
                                                  .unwrap(),
                   config(LevelFilter::Debug,
                          vec![("foo::bar::baz", LevelFilter::Trace)]));
    }

    #[test]
    fn bad_stuff_is_ignored() {
        assert_eq!("debug/foo/foo".parse::<EnvLogConfig>().unwrap(),
                   EnvLogConfig::default());
        assert_eq!("debug/foo,foo::bar::baz=trace/foo".parse::<EnvLogConfig>()
                                                      .unwrap(),
                   EnvLogConfig::default());
    }
}
