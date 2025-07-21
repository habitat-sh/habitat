pub(crate) mod cli;

habitat_core::env_config_int!(/// Represents how many threads to start for our main Tokio runtime
                              #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
                              TokioThreadCount,
                              usize,
                              HAB_TOKIO_THREAD_COUNT,
                              // This is the same internal logic used in Tokio itself.
                              // https://docs.rs/tokio/0.1.12/src/tokio/runtime/builder.rs.html#68
                              num_cpus::get().max(1));

#[cfg(test)]
#[cfg(feature = "v2")]
mod cli_tests;

#[cfg(feature = "v2")]
mod main_v2;

mod main_v4;

fn main() {
    if cfg!(feature = "v4") {
        main_v4::main_v4()
    } else {
        #[cfg(feature = "v2")]
        main_v2::main_v2()
    }
}
