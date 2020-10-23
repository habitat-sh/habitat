//! We currently use both `rustls` and `native-tls` for TLS. The libraries are used in the following
//! places:
//!
//! `rustls`:
//!     - http gateway
//!     - control gateway
//! `native-tls`:
//!     - builder http client
//!     - event stream
//!
//! Historically we used `native-tls` because it allowed reading from the system's certificate
//! store (ie `rustls` did not). `rustls` now allows reading system certificates with the
//! `rustls-native-certs` crate. `rustls` had to be used for the http gateway because `actix-web`
//! does not support `native-tls` as a backend. `native-tls` does not support client side
//! authentication requiring us to use `rustls` for the control gateway. Eventually, we
//! would like to standardize on a single TLS crate. It would be nice to standardize on `rustls` it
//! is an all Rust crate and imposes some [best practices](https://docs.rs/rustls/0.18.1/rustls/#non-features).
//! However, by imposing these best practices not all protocols or certificate formats are
//! supported. Therefore it would be a breaking change for the builder http client and event stream.
//! This breaking change should be considered before standardizing to `rustls`. For now we try to
//! keep logic specific to one of the TLS implementations behind the `native_tls_wrapper` or
//! `rustls_wrapper` modules.

pub mod ctl_gateway;
pub mod native_tls_wrapper;
pub mod rustls_wrapper;
