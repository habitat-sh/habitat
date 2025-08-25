use aws_sdk_ecr::{error::SdkError,
                  operation::get_authorization_token::GetAuthorizationTokenError};
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("Invalid registry type: {0}")]
    InvalidRegistryType(String),
    #[error("No ECR Tokens returned")]
    NoECRTokensReturned,
    #[error(transparent)]
    TokenFetchFailed(SdkError<GetAuthorizationTokenError>),
    #[error("A primary service package could not be determined from: {0:?}. At least one \
             package with a run hook must be provided.")]
    PrimaryServicePackageNotFound(Vec<String>),
}
