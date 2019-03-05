use base64::DecodeError;
use failure;
use std::result;

pub type Result<T> = result::Result<T, failure::Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Base64DecodeError(DecodeError),
    #[fail(display = "A primary service package could not be determined from: {:?}. At least \
                      one package with a run hook must be provided.",
           _0)]
    PrimaryServicePackageNotFound(Vec<String>),
}
