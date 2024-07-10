use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Can't clone repository: {0:?}")]
    Clone(git2::Error),
    #[error("Invalid url: {0}")]
    InvalidUrl(String),
    #[error("Can't extract repository name from url: {0}")]
    CantExtractRepo(String),
}
