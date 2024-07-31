use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Property {0} is empty")]
    EmptyProperty(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    LibGit2(#[from] git2::Error),
}