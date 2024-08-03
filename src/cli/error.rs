use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Git(#[from] crate::git::error::Error),
    #[error(transparent)]
    Profile(#[from] crate::profile::error::Error),
    #[error(transparent)]
    Ssh(#[from] crate::ssh::error::Error),
}