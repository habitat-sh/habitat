#[macro_use]
extern crate log;

mod api_client;
mod error;

pub use crate::{api_client::ApiClient,
                error::{Error,
                        Result}};
