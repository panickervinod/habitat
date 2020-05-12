use base64::DecodeError;
use failure;
use rusoto_core::RusotoError;
use rusoto_ecr::GetAuthorizationTokenError;
use std::{process::ExitStatus,
          result};

pub type Result<T> = result::Result<T, failure::Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Base64DecodeError(DecodeError),
    #[fail(display = "Container image build failed with exit code: {}", _0)]
    BuildFailed(ExitStatus),
    #[fail(display = "Could not determine container image ID for: {}", _0)]
    DockerImageIdNotFound(String),
    #[fail(display = "Invalid registry type: {}", _0)]
    InvalidRegistryType(String),
    #[fail(display = "No ECR Tokens returned")]
    NoECRTokensReturned,
    #[fail(display = "{}", _0)]
    TokenFetchFailed(RusotoError<GetAuthorizationTokenError>),
    #[fail(display = "A primary service package could not be determined from: {:?}. At least \
                      one package with a run hook must be provided.",
           _0)]
    PrimaryServicePackageNotFound(Vec<String>),
    #[fail(display = "Container image push failed with exit code: {}", _0)]
    PushImageFailed(ExitStatus),
    #[fail(display = "Removing local container images failed with exit code: {}",
           _0)]
    RemoveImageFailed(ExitStatus),
}